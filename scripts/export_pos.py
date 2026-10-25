import wn, re, collections
w = wn.Wordnet('oewn:2024')
ok = re.compile(r'^[a-z]+$')
pos = collections.defaultdict(set)
for ss in w.synsets():
    p = ss.pos
    p = 'a' if p=='s' else p
    for l in ss.lemmas():
        ll=l.lower()
        if ok.match(ll): pos[ll].add(p)
with open('data/lex/pos.tsv','w') as f:
    for wd,ps in pos.items():
        f.write(f"{wd}\t{''.join(sorted(ps))}\n")
print("content words with POS:", len(pos))
