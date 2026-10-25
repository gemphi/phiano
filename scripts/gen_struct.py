#!/usr/bin/env python3
"""
gen_struct.py — record phi-4 INTO our dimension structure, time-boxed (docs/157).

For each word, ask phi-4 for the same dimensions WordNet gives — definition,
part of speech, synonyms, is-a, antonyms, and real example sentences (its use) —
as strict JSON, and append it to data/lex/phi4_dict.jsonl in our schema. This
seeds the structure for the words no dictionary has (your coinages: loveful,
downed, andn, evertihng, …) and gives a phi-4 reading beside WordNet's for the
rest.

Runs for a fixed budget (default 15 minutes), checkpoints after every word, and
resumes where it stopped. Needs only a local ollama with phi-4.

Usage (on the machine where ollama runs), from the repo root:
    python crates/phiano/scripts/gen_struct.py --model phi4:latest --minutes 15

Then come back and I ingest phi4_dict.jsonl beside dict.jsonl and re-draw the
dimension trees (the red words fill in) and re-measure.
"""
import argparse, json, os, re, sys, time, urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(HERE)                       # crates/phiano
LEX = os.path.join(REPO, "data", "lex")
TOK = re.compile(r"[a-z]+")

SCHEMA = (
    'Return ONLY a JSON object, no prose, with exactly these keys for the word '
    '"{w}":\n'
    '{{"def": [up to 3 short definition strings],'
    ' "pos": "noun|verb|adjective|adverb|other",'
    ' "syn": [up to 6 synonyms], "isa": [up to 4 more-general words it is a kind of],'
    ' "ant": [up to 4 antonyms], "ex": [up to 4 short sentences that USE the word naturally]}}\n'
    'If the word is a coinage or misspelling, give your best reading of what it '
    'would mean and still fill every key. Output the JSON object only.'
)


def ollama(model, prompt, host, timeout=120):
    body = json.dumps({"model": model, "prompt": prompt, "stream": False,
                       "format": "json", "options": {"temperature": 0.4, "num_predict": 500}}).encode()
    req = urllib.request.Request(f"{host}/api/generate", data=body,
                                 headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=timeout) as r:
        return json.loads(r.read())["response"]


def parse(resp):
    """Pull the first JSON object out of the model's reply, tolerantly."""
    try:
        return json.loads(resp)
    except Exception:
        pass
    m = re.search(r"\{.*\}", resp, re.S)
    if m:
        try:
            return json.loads(m.group(0))
        except Exception:
            return None
    return None


def freq_map():
    fp = os.path.join(LEX, "freq.tsv")
    d = {}
    if os.path.exists(fp):
        for line in open(fp):
            p = line.split("\t")
            if len(p) == 2:
                try: d[p[0]] = float(p[1])
                except ValueError: pass
    return d


def target_words():
    """Story words first (the ones the test asks about), coinages at the front."""
    have = set()
    dp = os.path.join(LEX, "dict.jsonl")
    if os.path.exists(dp):
        for line in open(dp):
            try: have.add(json.loads(line)["w"])
            except Exception: pass
    words, seen = [], set()
    sdir = os.path.join(REPO, "data", "text", "story")
    texts = []
    if os.path.isdir(sdir):
        for f in os.listdir(sdir):
            if f.endswith(".txt"):
                texts.append(open(os.path.join(sdir, f), encoding="utf-8", errors="ignore").read())
    for t in texts:
        for w in TOK.findall(t.lower()):
            if w not in seen:
                seen.add(w); words.append(w)
    # coinages (not in WordNet) first — that's where the structure has holes
    return [w for w in words if w not in have] + [w for w in words if w in have]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--model", default="phi4:latest")
    ap.add_argument("--host", default="http://localhost:11434")
    ap.add_argument("--minutes", type=float, default=15.0)
    ap.add_argument("--out", default=os.path.join(LEX, "phi4_dict.jsonl"))
    args = ap.parse_args()

    fr = freq_map()
    done_path = args.out + ".done"
    done = set(open(done_path).read().split()) if os.path.exists(done_path) else set()
    words = [w for w in target_words() if w not in done]

    try:
        ollama(args.model, "ok", args.host, timeout=60)
    except Exception as e:
        sys.exit(f"cannot reach ollama at {args.host} with {args.model}: {e}")

    print(f"recording into {args.out} for {args.minutes:.0f} min · model {args.model} "
          f"· {len(words)} words queued ({len(done)} done)", flush=True)
    end = time.time() + args.minutes * 60
    n = 0
    with open(args.out, "a", encoding="utf-8") as out, open(done_path, "a", encoding="utf-8") as dlog:
        for w in words:
            if time.time() >= end:
                break
            try:
                rec = parse(ollama(args.model, SCHEMA.format(w=w), args.host))
            except Exception as e:
                print(f"  skip {w}: {e}", flush=True); continue
            if not isinstance(rec, dict):
                continue
            row = {"w": w, "freq": round(fr.get(w, 0.0), 9), "src": "phi4",
                   "pos": rec.get("pos", ""), "def": rec.get("def", [])[:3],
                   "syn": rec.get("syn", [])[:6], "isa": rec.get("isa", [])[:4],
                   "ant": rec.get("ant", [])[:4], "ex": rec.get("ex", [])[:4]}
            out.write(json.dumps(row, ensure_ascii=False) + "\n"); out.flush()
            dlog.write(w + "\n"); dlog.flush()
            n += 1
            left = int(end - time.time())
            print(f"  [{n}] {w}: {len(row['def'])} defs, {len(row['ex'])} ex · {left}s left", flush=True)

    print(f"done: recorded {n} words this run ({len(done)+n} total) in {args.out}", flush=True)


if __name__ == "__main__":
    main()
