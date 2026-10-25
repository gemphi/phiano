# 180 · The writer and the judge — the loop with no model in it

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The loop

`examples/writer.rs`, 198 lines, and NOTHING in it is a language model — that was the point (docs/169 §4, and the standing rule: this is a new paradigm; a transformer wants to know everything before it can start, and this starts from almost nothing). The student is SHOWN two working programs (the builders' move: a use, not a rule). It WRITES by sampling its own counts — the last two tokens consulted first, backing off to the last one (depth, docs/170), exploring the alphabet at 8% (curiosity). The only teacher is `phicpl::judge`: a program that compiles AND runs is told back into the counts in place; a failure teaches nothing and costs nothing. The floor is token soup — the same alphabet, the same rounds, no learning.

Fixed conditions (docs/179): the compile-and-run rate rises as verdicts are told back, and beats the soup.

## Run 1 — the collapse, kept in the record

Telling back EVERY success: rate 0% → 51% (overall 35.3%, soup 0.0%) — condition formally met — and **4 distinct programs kept**, all degenerate (`let start ( ) = ( 3 )` and kin). The judge rewards executing, so the writer found the easiest executable niche and farmed it. This is the collapse every reward loop invites, caught by reporting what was kept, not just the rate.

## Run 2 — the interest rule

One change, straight from the user's doctrine (interest must GROW, docs/174): **only a NOVEL success is told back.** A program already kept reinforces nothing, so collapsing onto an easy win starves itself, and exploration is the only way to keep being fed. 10,000 rounds, 0.23 seconds:

| measure | soup | writer |
|---|---|---|
| compile-and-run rate | 0.0% | **7.4%**, rising 1.3% → 9.7% |
| distinct programs kept | 0 | **16**, structured |

The kept programs are not the shown ones: `let start ( ) = rv valof $( let x = 0 for n = 1 to 9 do x := x $)`, `… resultis x $)`, FOR loops with new bounds, VALOF blocks rearranged — it even discovered `rv valof $( … $)`, reading the store at the returned value, which is legal under the one-word-type machine and nothing we showed it. Condition MET: rising rate, soup at zero. Earlier in the round, two bugs this loop caught in itself: the shown program must be tokenised by the JUDGE'S OWN lexer (prose's splitter breaks `->` into `-` `>`, and nothing it learns can ever compile), and order-1 counts scramble any program whose tokens repeat — depth-2 with backoff fixed it, which is docs/170's lesson arriving from a second direction.

## What the two runs say together

The rate measures exploitation; the kept-set measures learning. Run 1 maximised the first and learned four strings. Run 2 trades rate for growth and is the honest loop: 16 programs and counting, each one a new executable sentence of the language, found by writing, judged by running, learned in place. The novelty trickle is steady rather than accelerating (the interest curve is flat at ~0.1–0.2% per slice — PARTIAL, reported straight): acceleration needs the writer to compose at the CONSTRUCTION level, not the token level — `Knowing::families` as the writing vocabulary — which joins this loop to Q2's machinery and is the next rung.

## Status

2026-10-24. `arena::Count` gained `choices` (the same counts read as a distribution). Workspace 33 test binaries, 0 failures. `cargo run --release -p phiano --example writer -- [--rounds 10000]`; `WRITER_DEBUG=1` prints the first five programs and their verdicts.
