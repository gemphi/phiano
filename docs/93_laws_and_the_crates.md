# 93 · Laws, closure, and which crate does what

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. docs/92 found that one
shared operator learns each kind's answer *type* but not the answer. This note
adds the laws the kinds obey, counted in the told rows, and closure training
that makes the operator obey them — and moves the work into the crates it
belongs to.

## 1. Which crate does what

| crate | job here | what it holds |
|---|---|---|
| **phiecs** | asks | the quest; `relations.tsv`; `phiecs::quest::parse_tsv`, now the one parser for that table (phiano depends on phiecs; phiecs depends on nothing) |
| **phinet** | learns, knowing no words | `phinet::relate`: `Triple`, `example`, `pools`; `find_laws`; `closure_pairs`, `closure_epoch`; `score`, `prior`, `two_hop`, `chance` (3 tests) |
| **phiano** | names | `phiano::relate::RelationModel`: words and kinds to table rows, a word's first function taken from the quest facet's own angles (`+facet`), `answers(word, kind)` by name, save/load `.relate` (2 tests) |
| **phical** (`phi-cal`) | exact geometry of the circle and sphere | not used by this step; it stays the geometry child of phinet (docs/85 move 2) |

`bin/kind_operator` is now a thin program over the three: phiecs reads, phinet
trains, phiano names and saves.

## 2. Laws

A law `k₁ ⇒ k₂`: when `w k₁ v`, `w` and `v` share a `k₂` answer. Counted on
the told rows only, over triples where both have a told `k₂`. Synthetic quest:

| law | holds | of |
|---|---|---|
| near ⇒ is-a | **100%** | 140 |
| part-of ⇒ used-for | 22.1% | 136 |
| part-of ⇒ is-a | 2.9% | 137 |
| every other pair | 0% | 145–180 |

The one law in the hidden structure (a word's next sibling has its head) is
found exactly, and nothing else passes. Closure uses laws at ≥ 80%.

## 3. Results

Synthetic quest, 30 epochs, 251 of 1,280 rows held out, pool 293 (chance
0.021). `is-a` within-kind uniform chance 0.127.

| arm | told MRR | held out MRR | held-out is-a (pool / within) | law hops is-a |
|---|---|---|---|---|
| prior | — | 0.089 | 0.048 / 0.048 | — |
| operator | 0.601 | 0.110 | 0.055 / 0.057 | 0.254 / 0.255 |
| operator + closure (213 pairs) | 0.678 | **0.172** | **0.339 / 0.487** | **0.604 / 0.670** |
| operator + closure + facet start | **0.807** | **0.193** | 0.367 / 0.464 | 0.617 / 0.669 |

**Inference, measured.** With closure the one-hop operator reads a held-out
`w is-a ?` it was never told at 0.339 against a prior of 0.048 — within the
32 heads, 0.487 against uniform 0.127. Reading through the law's neighbours
reaches 0.604. Kinds with no law (part-of, near, unlike, used-for) stay at
the prior, as they should: nothing in the told rows implies them. Starting
words from the quest facet's angles (the Kuramoto result as the operator's
first guess) lifts what is told from 0.678 to 0.807.

Full Phi-4 quest (501 rows, 112 held out): **no law reaches 80%** — the best
are `is-a ⇒ has` 27.3% of 11 and `is-a ⇒ is-a` 12.5% of 16 — so closure has
nothing to train and held out stays at the prior (0.049–0.056 against 0.052).
Told: 0.807, 0.822 from the facet start. 64 questions give 11–19 testable
triples per kind pair; laws need a quest that asks each neighbourhood more
than once.

## 4. Inverse laws (added 2026-09-28)

`phinet::relate::find_inverses` counts `w k v ⇒ v k' w` over told rows where
`v` was asked `k'`; `implied` lists the untold rows a kept inverse gives, and
`kind_operator`'s `+inverse` trains them with the told ones. Synthetic:
`unlike ⇐⇒ unlike` 100% of 154, every other pair ≤ 1.1%. Held-out `unlike`
goes from 0.057 to **0.556** (prior 0.004) and held out overall from 0.172 to
0.280. Plainly: here the inference is the counted law's own reading — the 43
implied rows are the answers — and the operator's part is to hold them with
everything else; nothing is written in, and a law that is not in the data
(the other 19 pairs) implies nothing.

## 5. Next

A bigger Phi-4 quest shaped for laws: ask every answer back (depth, not
breadth), so `w k₁ v` and `v k₂ ?` are both told; then the same table.
Laws of other shapes (inverse: `w k v ⇒ v k' w`; chain: `k₁ ∘ k₂ = k₃`) are
the next counts.
