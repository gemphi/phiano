# 175 · Does the DAG earn its keep?

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/174 discovered the families. This doc asks the only question that matters about them — the one docs/170 fixed for Q2: **does reading a word under its concept beat reading it from the flat window, at the same budget?** — and reports the answer honestly, which means reporting that on the toy curriculum it is NO.

## The machinery

`prose/under.rs`: `Knowing::refile(at_least)` re-tells every paragraph under EVERY family it shows (`found/<a>_<b>/eN`), so the discovered DAG becomes scope paths and the cast can read a word under its concept with no new reading machinery; `Knowing::scope_said` picks the concept from what was said so far (causal — the most recent construction the prefix shows that the tree knows as a family). `Knowing::known_words` makes the closed stratum one pass, so family lookups cost a lookup instead of a scan (the toy did not need this; the scale run of docs/176 would not run without it).

## On the toy curriculum: NOT MET

41 sentences told (book + the teacher's uses), 12 held-out sentences never told, same positions and floor for both readings. Two position sets fixed before reading: all of them, and the discriminating cues — those whose last word is shown by two or more families, so a concept has something to choose between (computed from what was told, never from the answer).

| positions | flat | under its concept | lift |
|---|---|---|---|
| all (63) | 28.6% | 27.0% | −1.6 pts |
| discriminating cues (28) | 28.6% | 25.0% | −3.6 pts |

A rank fusion of the two readings was also tried (reciprocal rank, standard k = 60, nothing tuned on the test set) and agreed with the concept reading on 62 of 63 positions — the two readings barely differ, so the toy test is underpowered as well as negative.

**Why, diagnosed:** each family on the toy holds two or three sentences. Conditioning splits evidence that is already minute, and sparse conditioning loses to dense pooling — the same trade `relational.rs` measured years of rounds ago (relation-specific channels: 50.1% against the shipped 61.5%). The hierarchy cannot add information it does not have the counts to carry.

## At scale: MET (docs/176)

The same comparison, same protocol, on 3,000 sentences of real English prose (27,607 prequential positions, vocabulary 4,510) inside the arena of docs/176, where families hold hundreds of sentences rather than two:

| reader | top-1 | first → last slice |
|---|---|---|
| concepts (the family, backing off onto the counts) | **32.7%** | 31.1% → 33.9% |
| the counts alone (the word before) | 30.3% | 29.4% → 32.2% |

**+2.4 points at the same budget, both still rising.** Q2's condition is met where the counts exist to carry the conditioning, and the toy's failure is explained rather than explained away: the DAG needs a corpus, and 41 sentences is not one.

## Status

2026-10-24. `prose/under.rs` (+ tests), the student's held-out lesson (the negative, kept), and the arena reader `phiano::arena::concept` (the positive). Both numbers stand in the repository; neither replaces the other.
