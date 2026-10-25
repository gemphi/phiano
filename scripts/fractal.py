import json, re, wn
from wordfreq import word_frequency
W=wn.Wordnet('oewn:2024')
tok=re.compile(r"[a-z]+")
POSN={'n':'noun','v':'verb','a':'adjective','s':'adjective','r':'adverb'}
FUNC=set(open("/home/claude/phinet/crates/phiano/data/lex/function.txt").read().split())
ch1="He remained presumptuous even if the evidence was pointing to the direction he had been struggling to reach. It suddenly happened that she came to know exactly that she was in love with her boss, but only circumstances needed to be right. She suddenly had an idea their new investment would demand them to meet and she was going to make an irresistible move on this young CEO of an AI company."
ch2="IT WAS ON A SUNDAY AFTERNOON WHEN IT FINALLY DOWNED ON OUR GENTLEMAN THAT HE HAS EXCEEDED THE THRESHOLD OF HOLDING ONTO HIS EGO ANDN HAPPENED TO BE IN A GREAT POSITION AND HE WAS IN THE PROCESS OF LAUNCHING HIS BUSINESS WITH HIS AMERICAN COUNTER PARTS IN THAT MOMENT IT SUDDENLY OCCURED TO HIS WITTY SENSES THAT SOMEHOW EVERTIHNG HAD ALWAYS WORKED FOR THE BEST OUTCOMES AND THAT MOVED HIS HEART TO BE A STRONG AND LOVEFUL MACHINE THAT INSULATED HIS ACTION THAT LOVE."

def uniq(t):
    out=[];seen=set()
    for w in tok.findall(t.lower()):
        if w not in seen: seen.add(w); out.append(w)
    return out[:64]

def isa_chain(ss, depth=4):
    # recursive is-a (fractal upward): nested parents
    out=[]
    for h in ss.hypernyms()[:2]:
        lem=[l.lower() for l in h.lemmas() if tok.match(l.lower())]
        name=lem[0] if lem else "?"
        out.append({"w":name,"up":isa_chain(h,depth-1) if depth>1 else []})
    return out

def kinds_of(word):
    """The fractal: word -> each sense -> pos / what-kind (supersense) /
    which-sense (gloss) / branches (is-a up, frame, examples)."""
    senses=[]
    for ss in W.synsets(word):
        p=ss.pos
        frames=set()
        for sv in ss.senses():
            try:
                for f in (sv.frames() or []): frames.add(f)
            except Exception: pass
        senses.append({
            "pos":POSN.get(p,p),                 # kind: what it is
            "kind":ss.lexfile() or "",           # what KIND (supersense): what verb / what noun
            "gloss":ss.definition() or "",        # which sense
            "syn":[l.lower() for l in ss.lemmas() if l.lower()!=word][:5],
            "isa":isa_chain(ss),                  # fractal upward
            "frame":sorted(frames)[:3],           # linguistic designation
            "ex":[e for e in ss.examples() if 2<=len(e.split())<=14][:3],
        })
    # count branches (the 16-64 kinds)
    def count(nodes):
        c=0
        for n in nodes: c+=1+count(n.get("up",[]))
        return c
    kinds=sum(1+len(s["frame"])+len(s["ex"])+count(s["isa"]) for s in senses)
    return senses, kinds

def story(name,t):
    ws=uniq(t); out={}
    for w in ws:
        if w in FUNC or not W.synsets(w):
            out[w]={"func":w in FUNC,"known":bool(W.synsets(w)),"senses":[],"kinds":0}
        else:
            senses,kinds=kinds_of(w)
            out[w]={"func":False,"known":True,"freq":round(word_frequency(w,'en'),9),"senses":senses,"kinds":kinds}
    return {"name":name,"words":ws,"dim":out}

data={"ch1":story("ch1 — the presumptuous man",ch1),"ch2":story("ch2 — the sunday afternoon",ch2)}
json.dump(data,open("/mnt/user-data/outputs/fractal.json","w"),ensure_ascii=False)
for k,s in data.items():
    tot=sum(s["dim"][w]["kinds"] for w in s["words"])
    rich=sorted(((s["dim"][w]["kinds"],w) for w in s["words"] if s["dim"][w]["known"]),reverse=True)[:5]
    print(f"{k}: total kind-branches {tot}; richest: {rich}")
