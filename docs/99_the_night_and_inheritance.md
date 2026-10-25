# 99 · The night's quests, and inheritance as structure

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition.

## 1. The night (docs/96)

`scripts/overnight.ps1` ran unattended from 03:36 to about 11:00 local:
eight quests from bread, river, hammer, doctor, tree, car, computer, music,
110 one-kind questions each (about 50 minutes a seed — faster than the 30 s
a question estimated), every one completed. Merged with `phi4_laws`:
**3,630 rows, 1,583 words** (`all.tsv`). Held out 745 rows; pool 1,583;
chance 0.005; prior 0.053 / within kind 0.054.

The operator alone, four seeds (the night's own `operator_seed*.txt`; the
same numbers reproduced in the container):

| rows | held out MRR | within kind | prior (within) |
|---|---|---|---|
| 704 (docs/95) | 0.059 | 0.088 | 0.059 |
| 1,148 (docs/95) | 0.088 ± 0.011 | 0.127 ± 0.011 | 0.065 |
| **3,630 (tonight)** | **0.096 ± 0.005** | **0.144 ± 0.003** | 0.054 |

Within kind keeps rising with rows, from 1.5× to 2.7× the prior, on a pool
2.4 times larger. Soft laws (closure at 30%) lowered within kind to 0.116
at seed 7; counted by spelling, no law passed support 10 on these tables.

## 2. Inheritance as structure (docs/97 §1)

`phinet::relate::inherit`: for the kinds has, made-of, does and used-for,
the operator's reading of `w · k` is its own prediction plus its reading of
each told `w is-a p` parent under `k` — in training (`inherit_epoch`) and in
reading (`score_inherit`). `kind_operator … fno+inherit`. Same rows, same
four seeds:

| | held out MRR | within kind | hit@5 | has | made-of | does | used-for |
|---|---|---|---|---|---|---|---|
| operator | 0.096 ± 0.005 | 0.144 ± 0.003 | 15.7% | 0.112 | 0.099 | 0.062 | 0.092 |
| **+ inheritance** | **0.108 ± 0.010** | 0.143 ± 0.010 | **18.9%** | **0.130** | **0.121** | **0.082** | **0.099** |

Higher over the whole pool on **every seed** (0.119 / 0.107 / 0.096 / 0.111
against 0.097 / 0.095 / 0.089 / 0.102), hit@5 +3.2 points, the four
inheriting kinds all up; within kind unchanged. Read plainly: the declared
law puts the right *kind* of answer — the parent's answers — higher among
all 1,583 words; among the kind's own answers it does not yet pick the
right one more often. On the 1,148-row table (one seed) the same arm gave
0.093 against 0.088 and within kind 0.120 against 0.128. Unlike the counted
soft laws, the declared one helps where the data is broadest.
