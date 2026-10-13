import wn, re, collections
w = wn.Wordnet('oewn:2024')
ok = re.compile(r'^[a-z]+$')
near = collections.defaultdict(lambda: collections.defaultdict(float))
def lem(ss): return [l.lower() for l in ss.lemmas() if ok.match(l.lower())]
for ss in w.synsets():
    ls = list(dict.fromkeys(lem(ss)))
    hyper = [x for h in ss.hypernyms() for x in lem(h)]
    hypo  = [x for h in ss.hyponyms()  for x in lem(h)]
    for a in ls:
        for b in ls:
            if a!=b: near[a][b]=max(near[a][b],1.0)
        for b in hyper:
            if b!=a: near[a][b]=max(near[a][b],0.6)
        for b in hypo:
            if b!=a: near[a][b]=max(near[a][b],0.4)
with open('data/lex/near.tsv','w') as f:
    for a,nb in near.items():
        for b,wt in sorted(nb.items(), key=lambda x:-x[1])[:24]:
            f.write(f"{a}\t{b}\t{wt}\n")
print("words", len(near))
