# 101 · Declared laws as structure, measured: transitive and converse

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Docs/97 made inheritance a law the model *follows*: `has(cat) = own(cat) ⊕
λ·has(mammal)`, in training and reading alike. The persona files declare two
more laws (`transitive: is-a, part-of`, `inverse: has>part-of`). This doc
makes them structure the same way, in phinet, and measures them.

## What was built (phinet `relate`)

- **Transitive** (`Inherit::with_transitive`). A transitive kind reads its
  own told parents: `is-a(cat) = own ⊕ λ · is-a(mammal)`. Inheritance and
  transitivity are one structure: each kind names the kind its parents come
  from (`via`). Has, made-of, does and used-for go via is-a. Is-a and
  part-of go via themselves.
- **Converse** (`Converse`, `laws_epoch`, `score_laws`). `w has v ⇔ v part-of
  w`, as structure: the score of `w · k · v` is `cos(F(w+k), v) + μ ·
  cos(F(v+k′), w)`. What the candidate says under the converse kind is part
  of every answer. Nothing is added to the data, and the one operator is
  read in both directions. In training, the hinge compares the two-way
  score of the answer with the two-way score of the decoy.
- `inherit_epoch` is now `laws_epoch` with no converse. It keeps the same
  batches and the same seeds, so earlier numbers are reproduced.
- Tests (8 in `relate`):
  - a transitive kind reads its parents' parents;
  - a converse reads the untold direction;
  - with no converse, the reading equals inheritance alone.
- `kind_operator` arms: `+transitive` (`PHIANO_TRANSITIVE`) and `+converse`
  (`PHIANO_CONVERSE`).

## Measured

The night's table: 3,630 rows from Phi-4, 30 epochs, R = 64, 4 seeds
(7–10). Mean ± spread:

| arm | held out MRR | within kind | hit@5 | two hops MRR |
|---|---|---|---|---|
| inherit (docs/99) | **0.112 ± 0.008** | **0.140 ± 0.010** | **19.2%** | 0.102 ± 0.025 |
| inherit + transitive | 0.085 ± 0.002 | 0.134 ± 0.002 | 15.1% | **0.131 ± 0.004** |
| inherit + converse | 0.096 ± 0.009 | 0.123 ± 0.014 | 15.5% | 0.088 ± 0.023 |
| inherit + both | 0.081 ± 0.002 | 0.133 ± 0.003 | 15.2% | 0.121 ± 0.004 |
| prior | 0.053 | 0.054 | 7.8% | — |

- **The converse, as declared, does not hold in the teacher's answers.**
  Counted in the told rows, `has ⇐⇒ part-of` holds for 21% of its support
  and `part-of ⇐⇒ has` for 16%. Phi-4 lists what a cat has, but it seldom
  also lists the cat when asked what whiskers are part of. Forcing the model
  to read both ways makes it answer as a law says, not as Phi-4 said.
  Measured against Phi-4's own held-out answers, that costs 0.016 MRR.
- **Transitivity trades one-hop for two-hop.** Held out one hop falls from
  0.112 to 0.085. Two hops through told neighbours rise from 0.102 to 0.131.
  Transitivity also makes the model almost seed-independent (spread 0.002
  against 0.008). The chain gives each word a larger, more determined
  reading. It fits the questions Phi-4 did not answer less well, and the
  inferences better.
- **Both together** add the two costs.

## What this says about "laws must be followed"

The model follows the laws exactly: that is what the structure does. What
the measurement shows is whose answers the laws agree with. Inheritance
agrees with Phi-4 (docs/99: +0.012 held out on every seed). Transitivity
and the converse partly disagree with Phi-4's answers, at 16–25% agreement.
So a law declared in a persona file is a claim about the world. The quest
can test it, and has now tested it. Keep declared laws that the teacher's
answers support. For those it does not support, there are two choices:
keep them as the persona's own knowledge, where the model is meant to be
more consistent than the teacher, or ask Phi-4 the converse question
directly, as a quest kind of its own. The personas still declare them.
`learn_demo` and `persona_from_quest` use inheritance only, until a
teacher-agreeing converse is available.
