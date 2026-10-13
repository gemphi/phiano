import json, os, re
src="/home/claude/phinet/crates/phiano/data/lex/dict.jsonl"
out="/home/claude/phinet/crates/phiano/data/text/lex"
os.makedirs(out, exist_ok=True)
tok=re.compile(r"[a-z]+")
def clean(s): return " ".join(tok.findall(s.lower()))
# Keep reasonably common words (their use matters for Emma); freq >= 2e-7.
rows=[]
for line in open(src):
    e=json.loads(line)
    if e["freq"] < 2e-7: continue
    w=e["w"]
    sents=[]
    for d in e["def"]:
        d=clean(d)
        if d: sents.append(f"{w} is {d}")      # the word used with its defining words
    for isa in e["isa"][:4]:
        sents.append(f"{w} is a kind of {isa}") # is-a as use
    if e["syn"]:
        sents.append(f"{w} means {' and '.join(e['syn'][:4])}")  # synonym company
    if sents: rows.append((w,sents))
# write ~400 words per file, each word a blank-line-separated paragraph
per=400; n=0
for i in range(0,len(rows),per):
    with open(f"{out}/lex_{i//per:03d}.txt","w") as f:
        for w,sents in rows[i:i+per]:
            f.write(". ".join(sents)+" .\n\n")
    n+=1
tot=sum(len(s) for _,s in rows)
print(f"words {len(rows)} | files {n} | sentences {tot} | approx tokens {sum(sum(len(x.split()) for x in s) for _,s in rows)}")
