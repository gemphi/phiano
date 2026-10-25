#!/usr/bin/env python3
"""
gen_prose.py — accumulate USE, in the right register (docs/157).

The open stratum needs real-register usage, not dictionary glosses. This has a
local model (phi-4 via ollama) write natural sentences that USE each word —
not define it — so the tree counts real company. It is the recursive
"accumulate use -> meaning" loop: start from the function-word core, widen to
the content words, and (optionally) write in a persona's voice.

Runs for hours, checkpoints after every word, and resumes where it stopped —
so you can keep checking it. Output goes to _corpus/prose/*.txt as paragraphs
the Rust tree ingests directly (one word's sentences per paragraph).

Needs only a local ollama with a phi-4 model. No network beyond localhost.

Usage (on the machine where ollama runs):
    python gen_prose.py --model phi4 --n 6 --vocab emma
    python gen_prose.py --model phi4:14b --n 8 --persona "James Joyce" --max 2000

Then, in the phinet repo, point the reader at _corpus/prose as a scope and
re-run `ladder` / `story` to measure whether the open stratum climbs.
"""
import argparse, json, os, re, sys, time, urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(HERE)                      # crates/phiano
DEFAULT_OUT = os.path.join(os.path.dirname(os.path.dirname(REPO)), "_corpus", "prose")
LEX = os.path.join(REPO, "data", "lex")
TOK = re.compile(r"[a-z]+")


def ollama(model, prompt, host, timeout=180):
    """One non-streaming completion from a local ollama."""
    body = json.dumps({
        "model": model,
        "prompt": prompt,
        "stream": False,
        "options": {"temperature": 0.9, "num_predict": 1200},
    }).encode()
    req = urllib.request.Request(f"{host}/api/generate", data=body,
                                 headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=timeout) as r:
        return json.loads(r.read())["response"]


def vocabulary(which, limit):
    """The words to generate usage for, most worth-it first."""
    words = []
    if which == "story":
        # The words of the told stories (data/text/story/*.txt) — the 64-word
        # test set. 64 examples each spread a word's meanings over its circle.
        sdir = os.path.join(REPO, "data", "text", "story")
        seen = []
        if os.path.isdir(sdir):
            for f in sorted(os.listdir(sdir)):
                if f.endswith(".txt"):
                    for w in TOK.findall(open(os.path.join(sdir, f),
                                              encoding="utf-8", errors="ignore").read().lower()):
                        if w not in seen:
                            seen.append(w)
        words = seen
    elif which == "emma":
        # Emma's own words (what the held-out test actually asks about), by
        # frequency so common words come first.
        fp = os.path.join(LEX, "freq.tsv")
        freq = {}
        if os.path.exists(fp):
            for line in open(fp):
                p = line.split("\t")
                if len(p) == 2:
                    try: freq[p[0]] = float(p[1])
                    except ValueError: pass
        emma = os.path.join(REPO, "data", "text", "emma")
        seen = {}
        for root, _, files in os.walk(emma):
            for f in files:
                if f.endswith(".txt"):
                    for w in TOK.findall(open(os.path.join(root, f),
                                              encoding="utf-8", errors="ignore").read().lower()):
                        seen[w] = seen.get(w, 0) + 1
        words = sorted(seen, key=lambda w: -freq.get(w, 0))
    else:  # "freq": the most common English words overall
        fp = os.path.join(LEX, "freq.tsv")
        words = [l.split("\t")[0] for l in open(fp)] if os.path.exists(fp) else []
    return words[:limit] if limit else words


def prompt_for(word, batch, persona):
    voice = f" Write in the voice and style of {persona}." if persona else ""
    return (
        f"Write {batch} short, natural English sentences that USE the word "
        f'"{word}" — spread across its DIFFERENT meanings and associations, '
        f"not the same sense repeated. Do NOT define or explain the word. "
        f"Vary the grammar, subject and sense.{voice}\n"
        f"Output only the sentences, one per line, no numbering."
    )


def gather(model, word, n, persona, host):
    """Accumulate up to `n` distinct usage sentences, batching across calls —
    64 is the floor, this scales to 1024+ by looping."""
    batch = 24
    got, seen, tries = [], set(), 0
    while len(got) < n and tries < (n // batch) + 6:
        tries += 1
        try:
            resp = ollama(model, prompt_for(word, batch, persona), host)
        except Exception:
            break
        for line in resp.splitlines():
            c = clean(line)
            k = c.lower()
            if c and k not in seen:
                seen.add(k); got.append(c)
    return got[:n]


def clean(line):
    line = re.sub(r"^\s*[-*\d.)\]]+\s*", "", line).strip()
    return line if len(line.split()) >= 3 else ""


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--model", default="phi4")
    ap.add_argument("--host", default="http://localhost:11434")
    ap.add_argument("--n", type=int, default=64,
                    help="examples per word = the resolution of its circle; "
                         "64 is the minimum, then 128/256/512/1024 and up (keep it power-of-two for the operator)")
    ap.add_argument("--vocab", choices=["story", "emma", "freq"], default="story")
    ap.add_argument("--max", type=int, default=0, help="cap words (0 = all)")
    ap.add_argument("--persona", default="", help="optional voice, e.g. 'James Joyce'")
    ap.add_argument("--out", default=DEFAULT_OUT)
    args = ap.parse_args()

    os.makedirs(args.out, exist_ok=True)
    done_path = os.path.join(args.out, ".done")
    done = set(open(done_path).read().split()) if os.path.exists(done_path) else set()

    words = [w for w in vocabulary(args.vocab, args.max) if w and w not in done]
    print(f"{len(words)} words to do ({len(done)} already done) · model {args.model} "
          f"· {args.n}/word · persona {args.persona or 'none'} · out {args.out}", flush=True)

    # sanity: is ollama up?
    try:
        ollama(args.model, "ok", args.host, timeout=60)
    except Exception as e:
        sys.exit(f"cannot reach ollama at {args.host} with model {args.model}: {e}")

    shard = len(done) // 500
    t0 = time.time()
    for i, w in enumerate(words):
        sents = gather(args.model, w, args.n, args.persona, args.host)
        if not sents:
            continue
        para = ". ".join(s.rstrip(" .") for s in sents) + " .\n\n"
        with open(os.path.join(args.out, f"prose_{shard:04d}.txt"), "a", encoding="utf-8") as f:
            f.write(para)
        done.add(w)
        with open(done_path, "a", encoding="utf-8") as f:
            f.write(w + "\n")
        if (i + 1) % 500 == 0:
            shard += 1
        if (i + 1) % 25 == 0:
            rate = (i + 1) / (time.time() - t0)
            print(f"  {i+1}/{len(words)} · {rate:.1f} words/s · last: {w} -> {len(sents)} sents", flush=True)

    print(f"done: {len(done)} words total in {args.out}", flush=True)


if __name__ == "__main__":
    main()
