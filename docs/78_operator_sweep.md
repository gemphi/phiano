# 78 · The operator sweep: softmax over the table, wider and deeper

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. The operator is the
thing that moves a word's function toward the next word's, and docs/76 named
its ranking quality as the ceiling on everything the composer writes.

## What was run

`bin/operator_sweep` — Austen (Emma, Persuasion, Sense and Sensibility; split
seed 7), the shipped voice lexicon read directed, pool of the 1,228 words seen
20+ times, 204,592 training examples, 2,000 validation and 2,000 test examples.
Chance MRR 0.0063; identity (no operator) 7.9× chance valid, 8.1× test. Every
arm: three epochs, batch 64, seed 7, best epoch by validation MRR, test scored
once on that epoch. Arms differ in one thing each.

| arm | params | objective | optimiser | best epoch | valid MRR ×chance | test MRR ×chance | test hit@10 | s |
|---|---|---|---|---|---|---|---|---|
| hinge (shipped) | 8,360 | hinge vs one decoy | Adam | 3 | 14.7× | 14.6× | 0.216 | 142 |
| softmax β=8 | 8,360 | cross-entropy over the table | Adam | 3 | 18.0× | 18.8× | 0.246 | 395 |
| **softmax, wide+deep** | 50,016 | cross-entropy over the table | Adam | 3 | **27.6×** | **27.6×** | **0.335** | 1,157 |
| softmax, wide+deep, AdamW+cosine | 50,016 | cross-entropy over the table | AdamW 1e-3, warm-up + cosine | 3 | 24.5× | 24.3× | 0.311 | 984 |
| softmax, wide+deep, continued to 8 | 50,016 | cross-entropy over the table | Adam | 8 | **30.7×** | **30.3×** | **0.348** | 3,189 |

The last row resumes the third from its epoch-3 checkpoint: validation went
27.6 → 29.3 → 29.7 → 29.8 → 30.7 → 30.7 over epochs 3–8, so it is flattening,
and the 8-epoch operator is what ships.

For scale: the shipped hinge operator at eight epochs (docs/76) was 17.4× valid,
17.1× test, hit@10 0.248. Three epochs of full-table softmax on the same 8k
parameters already pass that; wide (16) and deep (3 layers, 16 modes) on the
same objective reach 27.6×, with a third of held-out targets in the top ten of
1,228. The cosine schedule with decoupled decay did not help at three epochs —
its warm-up spends the first epoch at a lower rate, and the run is too short to
pay that back; it is not evidence against the schedule at eight.

Validation and test agree to the decimal in every row, so the number is not a
selection artefact of picking the best epoch.

## What it does to the text

`bin/decode_eval` (40 held-out prompts × 30 tokens, k = 32), the operator
replaced by the wide+deep softmax one, everything else as in docs/76:

| arm | KN perplexity (3-epoch → 8-epoch operator) | evaluator | distinct | novel bigrams | docs/76 KN |
|---|---|---|---|---|---|
| trigram sampler | 13.9 → 13.3 | 0.671 | 0.884 | 0.0% | 13.6 |
| operator nearest-k | 73.6 → **66.9** | 0.673 | 0.889 | 0.4% | 97.2 |
| union | 67.5 → **66.8** | 0.670 | 0.897 | 0.3% | 86.2 |

(The trigram arm moves too because the guide re-weights every candidate by
its cosine against the prediction, whichever arm proposes the candidates.)

Her own sentences score 187.5. The operator's text moved a third of the way
from where it was toward the trigram sampler's perplexity (97 → 67) while its
distinctness rose above the sampler's; it is still far from stitched-Austen
fluency, and the last five epochs of ranking (27.6× → 30.7×) bought seven
points of perplexity — the ranking is no longer the whole ceiling.

Reading the first three samples suggested one attractor ("morning to think of
some time for it") owned every prompt, so that was measured rather than
believed: the share of a continuation's 4-grams that also occur in another
prompt's continuation, over all 40 prompts.

| arm | shared 4-grams |
|---|---|
| trigram sampler | 25.7% |
| operator nearest-k | 8.7% |
| union | 11.3% |

The trigram sampler is the one that repeats itself across prompts; the operator
repeats a third as much. The phrase in the samples was three prompts, not
forty. A prompt *anchor* — the prompt's mean function added to every step's
context with weight α, the condition the words keep moving under — was tried
at α = 0.5, 1, 2 and made everything worse (operator KN 86 / 98 / 119, distinct
0.84 / 0.78 / 0.75, novel bigrams up to 4%): a standing pull toward the prompt's
neighbourhood narrows the candidate set at every step instead of steering it.
It ships at α = 0 (`OperatorGuide::anchor`, `PHIANO_ANCHOR` to try another).

## How it was run

A full-table softmax epoch costs 14 minutes here, and the two wide arms about
20, so the sweep is resumable: it checkpoints the operator with its optimiser
moments and step count every quarter-epoch (`data/sweep/<arm>.ckpt`, save format
v2), stops when its time budget is spent, and continues on the next call. An
epoch's example order is fixed by its seed and the quarters are consecutive
slices of it, so the arithmetic is that of one uninterrupted run; the `s` column
is wall clock over the quarters. Finished arms are rows in
`data/sweep/results.md`. The winner is `data/austen_operator.fno` (format v2,
50,016 parameters), which `bin/decode_eval` and `OperatorGuide` read.

Two engine changes made this affordable (phinet, see the benchmarks page): the
`cosine_table` backward no longer computes a gradient for a side that needs none
— the table is a constant while the operator trains — and the profile that found
it shows the engine allocation-bound at these sizes, which is where the next
speed-up is.

## What remains

The schedule at eight epochs; the gap
between the operator's text (73.6) and the trigram sampler's (13.9) with the
sampler's repetitiveness (25.7% shared) as the price to avoid; the
author-separating sector; the three missing novels.
