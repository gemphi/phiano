# 92 · Kind operators: the first run

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. docs/91's first
suggestion, run: a word is a learned function on the circle (64 complex
samples), a kind is a token in the same table, and **one shared Fourier
operator** maps `unit(word + kind)` to the answer's function
(`bin/kind_operator`, phinet `Joint`, hinge against a sampled decoy, a fifth
of the rows held out by hash). The question: does a shared operator answer
relations it was never told, where the quest's angles could not?

## 1. The bar

A held-out row can score above chance with no inference at all: the
synthetic quest's `used-for` always names one of five tools, so ranking the
five first scores H(5)/5 = 0.457 without knowing which. So every run prints
the **prior** — answers ranked by how often the kind named them in
training — and **within kind**, the rank among the kind's own answers only.
Uniform chance within kind: is-a (32 heads) 0.127, used-for (5 tools) 0.457,
part-of/near/unlike (≈256 words) ≈ 0.024.

## 2. Results

Synthetic quest (1,280 rows, 251 held out, pool 293, chance 0.021), 30 epochs:

| reading | told MRR | held out MRR | is-a | part-of | near | used-for | unlike |
|---|---|---|---|---|---|---|---|
| prior | — | 0.089 | 0.048 | 0.004 | 0.004 | 0.370 | 0.004 |
| operator, circle (33,360 params) | 0.596 | **0.110** | 0.057 | 0.016 | 0.031 | 0.404 | 0.027 |
| operator, pointwise (no channel mixing) | 0.582 | 0.110 | 0.051 | 0.010 | 0.024 | 0.397 | 0.047 |
| operator, anisotropic sphere (19,024) | 0.585 | 0.110 | 0.052 | 0.011 | 0.019 | 0.397 | 0.053 |
| operator, two hops through told neighbours | — | 0.105 | **0.089** | 0.008 | 0.007 | 0.392 | 0.015 |
| quest facet (angles), docs/83 | 0.594 | 0.011 one hop | 0.095 two hops | | | | |

Full Phi-4 quest (501 rows, 112 held out, pool 378, chance 0.017), 60 epochs:

| reading | told MRR | told hit@5 | held out MRR | held out within kind |
|---|---|---|---|---|
| prior | — | — | 0.052 | 0.083 |
| operator, circle | **0.810** | 96.1% | 0.055 | 0.094 |
| operator, pointwise | 0.798 | 96.4% | 0.050 | 0.099 |
| operator, anisotropic | 0.822 | 96.7% | 0.043 | 0.089 |
| quest facet (angles), docs/89 | 0.762 | 92.4% | 0.005 | — |

## 3. What it says

1. **The operator learns each kind's answer type, not the answer.** Held-out
   MRR is 5× chance (0.110 vs 0.021; 0.055 vs 0.017) and ten to twenty times
   the angles' held-out reading — but it is the prior's number (0.089,
   0.052), and within each kind it is at or below uniform (is-a 0.059 vs
   0.127; used-for 0.404 vs 0.457). Nothing specific to the held-out word is
   inferred.
2. **The Fourier mixing is not what works.** Pointwise (no information moves
   between channels) and the sphere tie score the same as the circle on
   every held-out column — docs/91 §2.6's control, answered.
3. **Told answers are held better than by angles** — Phi-4 0.810 against
   0.762 at a comparable size (33k operator + 25k table numbers against 50k
   angles), and several answers per word no longer have to share one angle.
4. **Composition carries a little**: two hops through told neighbours lift
   is-a from 0.057 to 0.089 — the same as the angles' two hops (0.095) —
   because in the synthetic structure a word's `near` sibling has the same
   head. Still below uniform within kind.

## 4. Next

Inference has to come from a *law* the operators obey, not from sharing
weights: `is-a ∘ near = is-a` holds on every synthetic family, and a closure
loss (docs/91 §2.3, `07` §12) can make the one-hop operator absorb it. The
law itself should be found in the data, not written in: for each pair of
kinds, the share of told triples `(w k₁ v), (w k₂ z), (v k₂ z)` that agree;
closure is applied where that share is high. Then held-out one hop is read
again against the same prior.
