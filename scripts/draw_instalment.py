"""Draws one instalment's text for the learner's curriculum.

An instalment is 80,000 English sentences from Tatoeba, drawn under the
series' rule and disjoint from every sentence drawn before it. The rule was
read off samples 15-18 and is written down here because losing it cost a
row: sample 19 was drawn by hand on 2026-11-25 without it, and the
instalment carried longer, dirtier text beside the text it meant to carry
(docs/241). A drawn sample is never redrawn - the file on disk is the
record of what was told.

    python draw_instalment.py 20            # writes tatoeba/sample80k_20.txt
    python draw_instalment.py 20 --count    # says what the pool holds, writes nothing

The rule, one sentence a line:
  * four to fourteen words, counted as the shell counts them;
  * no digit anywhere - the reader drops a token with a digit in it, which
    would leave a hole in the sentence rather than a word;
  * ASCII only, and no quote mark;
  * one terminator, at the end: `.`, `?` or `!`, never inside, so the
    reader's cut at the full stops leaves the line whole;
  * not a line any earlier draw took, the held-out registers included.

The seed is the instalment's own number, so a draw is repeatable.
"""

import bz2
import random
import sys
from pathlib import Path

CORPUS = Path(r"C:\Users\phiac\Workspace\gemphi\phi-old\_corpus\tatoeba")
RAW = CORPUS / "raw" / "eng_sentences.tsv.bz2"
LINES = 80_000
WORDS = (4, 14)


def allowed(s: str) -> bool:
    """Whether the sentence may be drawn, by the rule above."""
    if not s.isascii() or any(c.isdigit() for c in s) or '"' in s:
        return False
    if s[-1] not in ".?!" or any(c in ".?!" for c in s[:-1]):
        return False
    return WORDS[0] <= len(s.split()) <= WORDS[1]


def drawn_already() -> set[str]:
    """Every sentence an earlier draw took, and the held-out registers."""
    told: set[str] = set()
    for f in sorted(CORPUS.glob("*.txt")):
        told.update(f.read_text(encoding="utf-8").splitlines())
    return told


def pool(told: set[str]) -> list[str]:
    """The sentences left to draw from, in the file's own order."""
    left = []
    with bz2.open(RAW, "rt", encoding="utf-8") as f:
        for line in f:
            part = line.rstrip("\n").split("\t")
            if len(part) < 3:
                continue
            s = part[2].strip()
            if s not in told and allowed(s):
                left.append(s)
    return left


def main() -> int:
    if len(sys.argv) < 2 or not sys.argv[1].isdigit():
        print(__doc__)
        return 2
    n = int(sys.argv[1])
    out = CORPUS / f"sample80k_{n}.txt"
    if out.exists():
        print(f"{out.name} is already drawn, and a drawn sample is never redrawn")
        return 1
    told = drawn_already()
    left = pool(told)
    print(f"{len(told)} sentences drawn before · {len(left)} left under the rule")
    if "--count" in sys.argv:
        return 0
    if len(left) < LINES:
        print(f"the well is down to {len(left)}: less than one instalment")
        return 1
    random.Random(n).shuffle(left)
    out.write_text("\n".join(left[:LINES]) + "\n", encoding="utf-8", newline="\n")
    print(f"wrote {out} ({LINES} sentences, seed {n}) · {len(left) - LINES} left after it")
    return 0


if __name__ == "__main__":
    sys.exit(main())
