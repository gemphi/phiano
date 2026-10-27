# 198 · The band — a restriction of influence, and the templates it shows

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Played step by step, one variable at a time, on the English guide. Every number below is from a run; the conditions of satisfaction were fixed before each measurement, and two of the four fixes measured NEGATIVE and are kept here as read.

## 1 · The data was broken, and the yardstick with it

The guide carries **8,199 typographic apostrophes (U+2019)** and not one straight `'`. The tokenizer accepted only the straight one, so every contraction shattered: `don’t` → `don` + `t`, `it’s` → `it` + `s`. The fragment `t` was the **6th most frequent token of the whole corpus** (3,105), `s` the 12th.

Fixed (`phiecs::speak::held`): U+2019 is an apostrophe only BETWEEN two alphanumerics, so a closing quote stays punctuation. Measured, before → after:

| | counts | concepts | phasors | gap (counts − phasors) |
|---|---|---|---|---|
| broken tokens | 21.4% | — | 13.9% | 7.5 |
| fixed tokens | **15.2%** | **19.3%** | **12.7%** | **2.5** |

Read it honestly. The gap narrowed by 5.0 points, but NOT because the phasors improved — they fell 1.2. Counts fell 6.2, because `don` → `t` was a free win they were farming 8,199 times. **The old "counts beat the phasors by 7.5 on prose" was partly an artifact of a broken tokenizer.** Key completion moved the same way: phasors 15.7% → 14.2%, counts 20.1% → 17.2%. Condition "phasors rise ≥0.5 on the Key" NOT MET — they fell 1.5. The fix corrected the yardstick, not the model, and saying otherwise would be a lie.

## 2 · A gate on the margin: measured negative, abandoned

The voice decayed into `t the any the every the`. First idea: speak only where the first read stands clear of the second (`Ecs::clearance`). Swept over ten questions:

| clearance τ | median answer | distinct ratio | function-word share |
|---|---|---|---|
| 0.00 | 14 words | 0.757 | 78.6% |
| 0.02 | 1.5 words | 1.00 | 64.3% |
| 0.05+ | 0 words | — | — |

Either nothing changes or the voice collapses. NOT MET at every threshold; not tuned further (a measured negative is not a knob to turn). But the sweep said something: the top-vs-second margin is almost always under 2%. The reads are FLAT.

## 3 · Why they are flat: counts came back through amplitude

`reads` scored a candidate `cos(θ − target) × √amplitude`, and `amplitude = ln(1+evidence)` — a COUNT. For `the` (n≈7,000) that factor is 2.98; for a word said once, 0.83. Agreement only varies ±1. **Frequency was swamping phase**: nothing learns from counts, except this read did. The read after `the` ranked 4,763 candidates from 3.87 to 3.59 and answered `a, i, is, you, and, we, do, be, have, are` — not one of which can follow `the`.

Agreement alone flips the bias instead of curing it: `couple` steps `1/(evidence+1)`, so a word told ONCE lands exactly on its target and scores exactly 1.0. Every rare word ties at the top (`satisfying, settings, map`).

## 4 · Domain and range: one angle cannot hold two roles

Telling `a next b` moved BOTH words on the same channel `θ(next)`: `a` toward its followers − turn, `b` toward its predecessors + turn. A word that is both a heavy predecessor and a heavy follower — `the` — has its one angle torn between the two pulls, and they cancel. A relation has a DOMAIN and a RANGE, and a word sits in both at different angles, so it needs two channels: the kind's own for the thing reaching, `<kind>>` for the thing reached.

## 5 · The band: membership, not ranking

A band is where the restriction of influence lives: phase decides WHO MAY INFLUENCE this read at all, and among the members — all of whom agree well enough — confidence orders. Without such a restriction there is one crowd and no groups. The read after `the`, by band width (in turns of TAU/8):

| band | what follows `the` |
|---|---|
| 1.00 | you, in, is, of, and, have, it |
| 0.50 | you, in, is, of, and, have, it |
| 0.25 | more, job, lot, good, verb, last, new |
| 0.125 | day, few, next, end, room, first |
| 0.0625 | morning, children, evening, future, noise, books |

The band width selects a GRAMMATICAL CLASS. Wide, and frequency still rules; narrow, and what is left is what actually follows `the` in English — and narrower still, pure nouns. Other conditions at 0.125: `present perfect` → **simple**, present, for; `been waiting` → **since**, hard, there. The groups are not given; they appear when influence is restricted.

## 6 · And it predicts WORSE

Condition fixed in advance: Murphy prose phasor top-1 rises ≥0.5 from 12.7%. Measured at band 0.125: **7.8%** — 4.9 points WORSE. NOT MET, reported straight.

The failure is the finding. Top-1 next-word accuracy rewards guessing the crowd; the band removes the crowd on purpose and returns the right class with an individually unlikely member. `the` → `morning` is good grammar and a bad bet. **A template's job is to CONTAIN the truth, not to BE it**, so exact-word accuracy is the wrong yardstick for what a band produces, and the next rung must measure a template as a template: does the true next word fall inside the band's group, and which band it falls in.

## 7 · What this is for

Everything here is becoming an operator in the compiler, and over time the operators are the templates of language generation — at most 1024 of them, with the open question being how templates interchange at each level (sentence, paragraph, chapter) and what links one level to the next: the bands. Phient is the first CLM, and it will use its CLM to know what to call, through an intermediate language of a single type, BCPL's kind, that the engine understands: learning low, saying high — compression and expansion.

## Status

2026-10-26. Tokenizer fixed with tests (`a_typeset_contraction_is_one_word_and_a_closing_quote_is_not`); `Ecs::ranked` and `Ecs::clearance` added; the read modes (`count`, `agree`, `band`, `domain`, `bound`) live behind `PHI_READ` so each is measurable against the others, and `count` is still the default — nothing was switched on the strength of a qualitative win. Next: measure the band as a template (does it contain the truth), then make the surviving bands operators.
