# 200 · The template — a sentence compressed to its types, and the circle's own grammar

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

TRN for prose (docs/199): a sentence compressed to the sequence of its TYPES. No part of speech is given anywhere. A word's type is where the phasors hold it under `next>` — the range channel, where it sits as the thing reached — cut into K sectors of the circle (`examples/template.rs`, `Ecs::angle`). Conditions fixed before the run: (C1) the commonest 1024 templates cover ≥50% of the English guide's sentences at some K; (C2) a sector's members read as one class.

## Measured

| K | distinct templates | top-1024 cover | largest type |
|---|---|---|---|
| 16 | 5,915 | **76.0%** | 2,079 words — `the to i a you in is of we it have and` |
| 8, both angles | 6,122 | 75.5% | 2,062 words — the same crowd, on both angles |
| 32 | — | — | 1,049 / 1,030 — the crowd halves |
| 128 | — | — | 385 — it keeps splitting |

C1 is met and hollow: the commonest templates are `6 6 6 6 …` — the crowd, the crowd, the crowd. The 2,000 most frequent words are reached from everywhere, so under the plain channel their angle is the global mean, and typing them by a second angle (what they reach) does not part them: they sit at the mean there too.

C2 is met where the crowd is not. By angle alone, at K=8 with both angles: (3,3) = `my your his new another american specific stupid lovely` — determiners and adjectives; (2,4) = `accident situation airport company film street city exam`; (1,4) = `exercise examples world river shops dog hand father baby`; (2,2) = `years simple continuous ago hours recently` — time and tense. Nobody told the world what a noun is.

## The circle's own grammar

At K=32 the crowd is not a point but a dense ARC, and the arc has an order. Walking the sectors:

| sector | members (most-told first) |
|---|---|
| 11 | 4 perfect years simple continuous ago hours recently adjectives |
| 12 | i you and do be use don't |
| 13 | the to a in is of we it have was not for |
| 14 | car lot verb last new day few holiday money next phone most |
| 15 | job house party same friend hotel end way weather sentence bus words |
| 16 | english own accident form situation company airport name film street city exam |

Time → subject and auxiliary → determiner, preposition, copula → nouns, nouns, nouns. The range angle under `next` runs from the closed stratum (docs/157: the words that carry the grammar) to the open one (the words that carry the matter), continuously. The type a band selects is a segment of this arc, which is why the band at a quarter-turn handed back `more job lot good` and at a sixteenth `morning children evening` (docs/198): narrower bands walk further into the open stratum.

## What the template is, then, and what it is not yet

A sentence's template under this typing is the pattern of OPEN words among CLOSED ones — the skeleton — and the skeleton recurs (76% of sentences within 1,024 shapes). That is a real compression and the right first rung: BCPL's TRN keeps the shape and drops the names. What it is not yet: a typing of the crowd itself. `the` and `is` and `we` are one type under the plain channel because the plain channel averages their every use; they part only under a deeper condition — the `after <word>` channels the arena already reads deepest-first. Typing the closed stratum by its deep channels is the next rung, and the test is the same: cover and class.

## Status

2026-10-27. `examples/template.rs` (K sectors, `--pair` for both angles, the sector listing as the readout), `Ecs::angle`. Default read unchanged. Next: type the crowd under the deep channels; then the within-type swap test under `sure` (nouns among nouns: does the read notice who?), taped.
