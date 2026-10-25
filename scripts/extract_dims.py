import json, re, wn
from wordfreq import word_frequency
W=wn.Wordnet('oewn:2024')
D={}
for line in open("/home/claude/phinet/crates/phiano/data/lex/dict.jsonl"):
    e=json.loads(line); D[e["w"]]=e
tok=re.compile(r"[a-z]+")
POSN={'n':'noun','v':'verb','a':'adjective','s':'adjective','r':'adverb'}
ch1="He remained presumptuous even if the evidence was pointing to the direction he had been struggling to reach. It suddenly happened that she came to know exactly that she was in love with her boss, but only circumstances needed to be right. She suddenly had an idea: their new investment would demand them to meet, and she was going to make an irresistible move on this young CEO of an AI company."
ch2="IT WAS ON A SUNDAY AFTERNOON WHEN IT FINALLY DOWNED ON OUR GENTLEMAN THAT HE HAS EXCEEDED THE THRESHOLD OF HOLDING ONTO HIS EGO ANDN HAPPENED TO BE IN A GREAT POSITION AND HE WAS IN THE PROCESS OF LAUNCHING HIS BUSINESS WITH HIS AMERICAN COUNTER PARTS IN THAT MOMENT IT SUDDENLY OCCURED TO HIS WITTY SENSES THAT SOMEHOW EVERTIHNG HAD ALWAYS WORKED FOR THE BEST OUTCOMES AND THAT MOVED HIS HEART TO BE A STRONG AND LOVEFUL MACHINE THAT INSULATED HIS ACTION THAT LOVE."
FUNC=set(open("/home/claude/phinet/crates/phiano/data/lex/function.txt").read().split())

def words(t):
    out=[];seen=set()
    for w in tok.findall(t.lower()):
        if w not in seen: seen.add(w); out.append(w)
    return out[:64]

def ling_and_ex(w):
    exs=[]; frames=set(); poss=set()
    for ss in W.synsets(w):
        p=ss.pos; poss.add(POSN.get(p,p))
        for e in ss.examples(): exs.append(e)
        for sv in ss.senses():
            try:
                for fr in (sv.frames() or []): frames.add(fr)
            except Exception: pass
    # dedup examples, keep short ones
    seen=set(); ex=[]
    for e in exs:
        k=e.lower()
        if k not in seen and 2<=len(e.split())<=14: seen.add(k); ex.append(e)
    ling=", ".join(sorted(poss))
    return ex[:4], ling, sorted(frames)[:4]

def isa_climb(ws,depth=4):
    edges=set();frontier=set(ws)
    for _ in range(depth):
        nxt=set()
        for w in frontier:
            for p in D.get(w,{}).get("isa",[])[:2]:
                edges.add((w,p));nxt.add(p)
        frontier=nxt-{e[0] for e in edges}
        if not frontier:break
    return sorted(edges)

def story(name,t):
    ws=words(t);entries={}
    for w in ws:
        e=D.get(w,{})
        ex,ling,frames = ling_and_ex(w) if w in D else ([],"",[])
        entries[w]={"freq":e.get("freq",0),"pos":e.get("pos",""),"sense":e.get("sense",""),
            "def":(e.get("def") or [""])[0],"syn":e.get("syn",[])[:6],"isa":e.get("isa",[])[:4],
            "ant":e.get("ant",[])[:4],"ex":ex,"ling":ling,"frames":frames,
            "func":w in FUNC,"known":w in D}
    groups={}
    for w in ws:
        g="function" if w in FUNC else (entries[w]["sense"] or ("unknown" if w not in D else "other"))
        groups.setdefault(g,[]).append(w)
    return {"name":name,"n":len(ws),"words":ws,"dim":entries,
            "isa_edges":isa_climb([w for w in ws if w in D and w not in FUNC]),"groups":groups}

out={"ch1":story("ch1 — the presumptuous man",ch1),"ch2":story("ch2 — the sunday afternoon",ch2)}
json.dump(out,open("/mnt/user-data/outputs/stories_dims.json","w"),ensure_ascii=False)
for k,s in out.items():
    exc=sum(1 for w in s["words"] if s["dim"][w]["ex"])
    frc=sum(1 for w in s["words"] if s["dim"][w]["frames"])
    print(f"{k}: {s['n']} words, {exc} with examples, {frc} with verb frames")
