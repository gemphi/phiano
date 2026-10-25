import wn, re, os
w = wn.Wordnet('oewn:2024')
tok=re.compile(r"[a-z]+")
def clean(s): return " ".join(tok.findall(s.lower()))
out="/home/claude/phinet/crates/phiano/data/text/use"
os.makedirs(out, exist_ok=True)
sents=[]
for ss in w.synsets():
    for ex in ss.examples():
        c=clean(ex)
        if len(c.split())>=3: sents.append(c)
# dedup, write ~600 per file, one sentence per paragraph
seen=set(); uniq=[]
for s in sents:
    if s not in seen: seen.add(s); uniq.append(s)
per=600
for i in range(0,len(uniq),per):
    with open(f"{out}/use_{i//per:03d}.txt","w") as f:
        for s in uniq[i:i+per]:
            f.write(s+" .\n\n")
print(f"real-usage sentences {len(uniq)} | files {(len(uniq)+per-1)//per} | tokens {sum(len(s.split()) for s in uniq)}")
