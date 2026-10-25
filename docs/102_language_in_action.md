# 102 · Language in action: the builders' game

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

The write-up is the paper
`docs/papers/19_DESCRIPTIVE_LOGIC_1_LANGUAGE_IN_ACTION.md` (and `.pdf`). It
reads Philosophical Investigations §§1–32 in Phient's terms and measures
them. The raw tables are in `102_language_game_seed7.md` and
`102_language_game_seed8.md`.

`cargo run --release --bin language_game -- out.md --seed 7 [--r 32] [--epochs 30]`
takes about 20 minutes on one core. Set `PHIANO_QUICK=1` to skip the sepia,
curve and stranger trials, and `PHIANO_DEBUG=1` to see per-aspect accuracy
and the first mistakes.

## In one table

Seed 7 / seed 8, exact match on all five aspects of the action:

| learner | new combinations §8 | report §21 | no name §27 | sepia, 1 pointing + `colour` §29–30 |
|---|---|---|---|---|
| lookup | 0 / 0% | 1 / 1% | 100 / 100% | 0 / 0% |
| naming table (§1) | 100 / 100% | 0 / 0% | 0 / 0% | 0 / 0% |
| bag of words + softmax | 100 / 100% | 100 / 100% | 100 / 100% | 0 / 0% |
| Phical counts, movement | 100 / 100% | 100 / 100% | 100 / 100% | **100 / 100%** |
| Phient (phinet R=32), movement | 100 / 100% | 100 / 100% | 100 / 100% | 100 / 100% (after docs/103; was 100 / 0%) |

The descriptive logic works: posts, games that agree only where they agree,
and composition by movement. Where the methods differ:

- The bag of words needs fewer episodes.
- Counts learn a stationed word from one pointing.
- The phinet functions need more data than the counts.

That led to two-speed learning, where a relation is a count first and a
function later. It is built and measured in docs/103, which supersedes this
doc's phinet numbers.
