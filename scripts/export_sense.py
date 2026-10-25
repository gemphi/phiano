import wn, re, collections
w = wn.Wordnet('oewn:2024')
ok = re.compile(r'^[a-z]+$')
sense = collections.defaultdict(collections.Counter)
for ss in w.synsets():
    p = ss.pos; p = 'a' if p=='s' else p
    try:
        lf = ss.lexfile() or f"{p}.other"
    except Exception:
        lf = f"{p}.other"
    for l in ss.lemmas():
        ll=l.lower()
        if ok.match(ll): sense[ll][lf]+=1
cats=collections.Counter()
with open('/home/claude/phinet/crates/phiano/data/lex/sense.tsv','w') as f:
    for wd,c in sense.items():
        top=c.most_common(1)[0][0]; cats[top]+=1
        f.write(f"{wd}\t{top}\n")
print("sense words", len(sense), "| distinct supersenses", len(cats))
print(sorted(cats))
