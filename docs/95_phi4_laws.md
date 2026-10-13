# 95 · Laws in Phi-4's answers

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. The one-kind-at-a-time
quest (docs/94) on Phi-4: seed `cat`, kinds is-a, has, part-of, made-of,
used-for, does, about 30 s a question on the user's CPU. Read at 241
questions (the run continues): **704 rows, 434 words**, every answer in the
asked format (three lines, pipes kept) — the short prompt fixed the five
answer shapes of docs/88.

## 1. The laws Phi-4's answers obey

Counted on the told rows (`find_laws`, `find_inverses`, support ≥ 10):

| law | holds | of | reading |
|---|---|---|---|
| is-a ⇒ made-of | 50.0% | 24 | a thing is made of what its kind is made of |
| is-a ⇒ is-a | 50.0% | 24 | a kind's kind (cat → mammal → animal) |
| is-a ⇒ has | 39.1% | 23 | **inheritance**: a cat has what a mammal has |
| part-of ⇒ is-a | 37.0% | 27 | |
| is-a ⇒ does | 34.8% | 23 | |
| is-a ⇐⇒ is-a (inverse) | 41.7% | 24 | Phi-4 says both directions (`feline is-a cat`, `cat is-a feline`) |
| has ⇐⇒ has (inverse) | 31.6% | 19 | |

None reaches the synthetic quest's 100%; language's laws are soft, and Phi-4
names one thing in several words (fur, hair, covering). Inheritance is there
at 39%.

## 2. Training on them

`kind_operator`, 60 epochs, 138 of 704 rows held out (pool 434, chance 0.015,
prior 0.053), laws and inverses kept at ≥ 30% (`PHIANO_LAW_MIN=0.3`), four
seeds:

| seed | operator: held out / within kind | + laws (closure + inverse): held out / within kind |
|---|---|---|
| 7 | 0.053 / 0.081 | 0.115 / 0.144 |
| 8 | 0.066 / 0.099 | 0.093 / 0.139 |
| 9 | 0.054 / 0.082 | 0.061 / 0.122 |
| 10 | 0.062 / 0.090 | 0.056 / 0.114 |
| **mean** | **0.059 / 0.088** | **0.081 / 0.130** |

Seed 7 alone (closure only 0.087, inverse only 0.076, both 0.115) looked
like a doubling; four seeds say less. **Within kind the laws help on every
seed** (0.088 → 0.130, +0.042 each time, against a within-kind prior of
0.059): among the answers of the right kind, the untold one is ranked higher.
**Over the whole pool the gain is not robust** (0.059 → 0.081, seeds 0.056 to
0.115). Told rows cost little (0.534 → 0.524 at seed 7).

What "right" means here: agreement with Phi-4 on rows it said and the model
was not shown — including Phi-4's own inconsistencies. The `is-a` inverse is
mostly Phi-4 saying both directions, and training it teaches the model to do
the same.

## 3. At 400 questions

The run finished: 400 questions, **1,148 rows, 652 words**, 240 held out
(pool 652, chance 0.011, prior 0.062 / within kind 0.065). The laws got
*weaker* as the quest left cat's neighbourhood: is-a ⇒ has 39% → 25% of 36,
is-a ⇒ made-of 50% → 36% of 39, part-of ⇒ is-a 35% of 43, the is-a inverse
42% → 24% (below the 30% bar, so no implied rows). Four seeds:

| | held out MRR (mean ± sd) | within kind | hit@5 |
|---|---|---|---|
| prior | 0.062 | 0.065 | 10.0% |
| operator | **0.088 ± 0.011** | **0.127 ± 0.011** | 14.9% |
| operator + closure (321 pairs) | 0.092 ± 0.011 | 0.128 ± 0.010 | 15.0% |

Two readings. **The plain operator now infers past the type**: within kind
0.127 against a prior of 0.065 on every seed, where at 704 rows it was 0.088
against 0.059 — more rows, more of the untold answer. **The soft laws add
nothing measurable at this size** (0.092 vs 0.088, inside one sd): at 704
rows their within-kind gain stood in for missing data, and at 1,148 the data
carries it. The laws in language are real (inheritance at 25–39%) but too
soft, counted by spelling, to be trained as laws.

## 4. Next

The run is at 266 of 400 questions; the same table at 400, and then a second
seed word, give the laws more support. Laws with a synonym step (fur ~ hair)
need the words' own closeness, which the operator's table already holds:
count agreement by nearest functions, not by spelling.
