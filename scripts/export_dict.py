import wn, re, json, collections
w = wn.Wordnet('oewn:2024')
ok = re.compile(r'^[a-z][a-z-]*$')
from wordfreq import word_frequency
entries = collections.defaultdict(lambda: {"pos":set(),"sense":collections.Counter(),"defs":[],"syn":set(),"isa":set(),"ant":set()})
for ss in w.synsets():
    p = ss.pos; p='a' if p=='s' else p
    try: lf = ss.lexfile() or f"{p}.other"
    except Exception: lf=f"{p}.other"
    lemmas=[l.lower() for l in ss.lemmas() if ok.match(l.lower())]
    hyper=[x.lower() for h in ss.hypernyms() for x in h.lemmas() if ok.match(x.lower())]
    defn=ss.definition() or ""
    for a in lemmas:
        e=entries[a]; e["pos"].add(p); e["sense"][lf]+=1
        if defn and len(e["defs"])<4: e["defs"].append(defn)
        for b in lemmas:
            if b!=a: e["syn"].add(b)
        for b in hyper: e["isa"].add(b)
    for sense in ss.senses():
        for a in sense.get_related('antonym'):
            aw=a.word().lemma().lower()
            for l in lemmas:
                if ok.match(aw): entries[l]["ant"].add(aw)
out="/home/claude/phinet/crates/phiano/data/lex/dict.jsonl"
n=0
with open(out,'w') as f:
    for wd,e in entries.items():
        rec={"w":wd,"freq":round(word_frequency(wd,'en'),9),
             "pos":"".join(sorted(e["pos"])),
             "sense":e["sense"].most_common(1)[0][0] if e["sense"] else "",
             "def":e["defs"][:3],
             "syn":sorted(e["syn"])[:12],
             "isa":sorted(e["isa"])[:8],
             "ant":sorted(e["ant"])[:6]}
        f.write(json.dumps(rec,ensure_ascii=False)+"\n"); n+=1
print("dictionary entries:",n)
