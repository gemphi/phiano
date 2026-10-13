# 112 · Learned posts in the builders' game: a small gain on Phi-4, a cost on the rules

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Trial.** On code, learned weights on the movement beat every fixed setting (docs/110). Here the same idea goes to the builders' game (`weighted.rs`).

- Each word's move, ln P(v | w, c) − ln P(v | c), is weighted by α[aspect, post strength]. The strength buckets, in nats, are <0.05, <0.25, <0.5, <1 and ≥1. The condition gets a weight β[aspect].
- There is no fixed τ: a weight near zero is a post not held.
- The weights are fitted by likelihood on four inner folds of the training set (counts from three quarters, answers from the fourth).

**First try, weights per (aspect, sightings).** Act 74.8%, but exact fell to 50.9% on the temperature-0 run. How often a word was seen does not tell which words should hold a post, so colour-words voted on count. That setting was dropped.

## Students of Phi-4 (5 folds by call, as docs/111)

| student | t0: exact | t0: act | sampled: exact | sampled: act |
|---|---|---|---|---|
| bag of words + softmax | **70.5%** | **77.6%** | **54.2%** | **67.5%** |
| counts, τ = 0.5 (D2) | 60.3% | 66.4% | 46.6% | 61.2% |
| counts, τ per aspect (docs/111) | 65.1% | 70.2% | 49.3% | 62.9% |
| **counts, learned posts** | 66.4% | 73.8% | 50.1% | 62.3% |

Learned weights for the temperature-0 run (last fold):

| aspect | β | <0.05 nats | <0.25 | <0.5 | <1 | ≥1 |
|---|---|---|---|---|---|---|
| act | 0.40 | 0.40 | 0.97 | 0.72 | 0.03 | 1.00 |
| what | 0.67 | 0.51 | 0.69 | 0.66 | 0.99 | 1.31 |
| colour | 1.92 | 0.86 | 0.21 | 0.38 | 0.31 | 2.82 |
| count | 1.59 | 1.00 | 0.07 | 0.33 | 0.30 | 1.68 |
| place | 1.87 | 0.46 | 0.34 | 0.02 | 0.13 | 2.20 |

The weights find, on their own, what docs/111 found with τ:

- **Colour, count and place are rules.** Only strong posts (≥ 1 nat) count, weighted 1.7–2.8; weak ones are near zero.
- **The act is a tendency.** Moves of 0.05–0.5 nats count about as much as strong ones.

The <0.05 bucket gets 0.4–1.0. Such words barely move anything, so their weight matters little.

## The rule game, same learner (`language_game`, seed 7)

| test | counts, movement (τ = 0.5) | counts, learned posts |
|---|---|---|
| seen / new combinations / report / no name | 100 / 100 / 100 / 100% | 100 / 100 / 100 / 100% |
| sepia: 1 pointing / 3 / 1 + `colour` / 3 + `colour` | 26 / 26 / 100 / 100% | **39** / 26 / 100 / 100% |
| training curve: 20 / 40 / 80 / 160 / 320 / 640 episodes | 19 / 38 / 51 / **70 / 97 / 99%** | 12 / 39 / **61** / 65 / 88 / 88% |
| stranger, 10 / 20 / 40 / 80 episodes | 12 / 29 / 67 / 81% | **22 / 35 / 70** / 80% |
| child, 10 / 20 / 40 / 80 episodes | **23 / 27 / 47** / 65% | 3 / 14 / 41 / **81%** |

## Reading

- **On Phi-4: a small, consistent gain.** Learned posts are the best counts student on both runs: +1.3 and +0.8 points exact over τ per aspect, and +3.6 on the act (temperature 0). Bag of words still leads, by 4 points exact. Weights on the aspect cannot tell *which* words go together, and that is where Phi-4's tendency lives (docs/111).
- **On the rules: a cost.** At 320–640 episodes, the learned posts reach 88% where the fixed post reaches 97–99%. With very few episodes (child, 10–20) they are far worse. A fixed τ is a prior that needs no data; weights need data and can overfit a clean rule. It is also possible the fold scheme is the cause here: with few calls, the inner folds hold out whole kinds of call. I have not checked this.
- **Verdict.** Keep D2 with a fixed post for rule-like games, and use learned posts only for a teacher known to teach by tendency. Better still, as next step: learn weights **per word** only where a word has been seen enough (the two speeds of paper 19), keeping τ = 0.5 as the prior. That is closer to bag of words without losing the one-pointing learning that bag of words cannot do (sepia 0%).

Run: `language_game graded <episodes.tsv> <out.md>` (it now includes the learned-posts row). The rule game with this learner: `language_game <out.md>`. Raw: `112_graded_*.md`, `112_rule_game_seed7.md`.
