# 111 · Phi-4 at temperature 0, and a teacher who teaches by tendency

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. The steady run

`scripts/phi4_builders.ps1` asked Phi-4 the same 400 calls again, this time through `phi4-builder` (temperature 0, fixed seed). The prompt was unchanged, so temperature is the only thing that differs from docs/107.

| run | replies read | exact (game's terms) | act | same answer to the same call | best student of Phi-4 |
|---|---|---|---|---|---|
| sampled (docs/107) | 369 of 400 | 50.7% | 63.1% | 35.8% (95 of 265) | bag of words 54.2% |
| **temperature 0** | **393 of 400** | **66.4%** | **74.3%** | **88.9% (295 of 332)** | bag of words **70.5%** |

In the game's terms, the other aspects are:

- stone 98.2%;
- colour 100%;
- count 94.1%;
- place 96.4%.

**What the fix did:**

- Phi-4 now goes on the same way nine times in ten.
- It follows our rules on two calls in three.
- Its unread replies fell from 31 to 7.

It is not fully deterministic even at temperature 0: 37 pairs still differ. The likely cause is three parallel requests batched on the GPU, which is known to change floating-point sums. That cause is not verified.

**What it did not fix: the roles.** Of the orders, 76 are still answered `record` (the reporter's act), and 25 reports are answered `bring`. The confusion is not random; it follows the words (orders only, rule = `bring`):

| word in the call | answered `record` |
|---|---|
| call begins with `slab` | 9 of 9 |
| `pillar` anywhere | 26 of 58 (45%) |
| `slab` anywhere | 28 of 60 (47%) |
| `block` / `beam` anywhere | 14% / 15% |
| `this` anywhere | 8% |
| any colour-word (`red`, `blue`, `yellow`) | **0 of 136** |

So Phi-4 has a practice here, not a slip. Which stone is named leans it one way, and a colour-word settles it the other. No single word decides. It is a **tendency**.

## 2. Why the counts student lagged, and a trial

In docs/107 and again here, the counts student followed Phi-4 less well than bag of words (60.3% against 70.5%), mostly on the act (66.4% against 77.6%).

**Hypothesis.** A word must move an aspect by τ = 0.5 nats to hold a post (D2). A tendency of 47% against a base of about 26% moves the act by only about 0.1 nats, so the counts student never hears `slab` or `pillar` on the act. Bag of words weighs every word and does hear them.

**Trial:** `language_game graded <episodes.tsv> <out.md>`. The same five folds by call as `follow` are used.

1. Sweep one τ for all aspects.
2. Then choose τ **per aspect** inside the training folds (4 inner folds; ties go to the larger τ), never on the test fold.

**Temperature-0 run** (393 calls, 362 of them with a word heard in training):

| student of Phi-4 | exact | exact, calls with a known word | act | what | colour | count | place |
|---|---|---|---|---|---|---|---|
| bag of words + softmax | **70.5%** | **76.5%** | **77.6%** | **97.7%** | 92.1% | 85.2% | 95.7% |
| counts, τ = 0 | 44.0% | 47.8% | 72.5% | 89.1% | 82.2% | 67.4% | 84.0% |
| counts, τ = 0.1 | 46.1% | 50.0% | 72.8% | 88.8% | 85.0% | 67.2% | 86.0% |
| counts, τ = 0.5 (D2 as written) | 60.3% | 65.5% | 66.4% | 89.1% | 91.6% | 85.2% | 93.9% |
| counts, τ = 1 | 54.5% | 59.1% | 66.4% | 78.1% | 92.1% | **87.3%** | 95.7% |
| **counts, τ per aspect** | 65.1% | 70.7% | 70.2% | 88.8% | 92.1% | **87.3%** | 95.7% |

**Sampled run** (369 calls, 344 known):

| student of Phi-4 | exact | exact, calls with a known word | act |
|---|---|---|---|
| bag of words | **54.2%** | **58.1%** | **67.5%** |
| counts, τ = 0.5 | 46.6% | 49.7% | 61.2% |
| **counts, τ per aspect** | 49.3% | 52.6% | 62.9% |

The τ chosen per aspect, fold by fold, lists act / what / colour / count / place:

- temperature 0: 0 / 0.5 / 1 / 1 / 1, then 0.25 / 0.5 / 1 / 1 / 1, 0 / 0.25 / 1 / 1 / 1, 0 / 0 / 1 / 1 / 1, 0.05 / 0 / 1 / 1 / 1;
- sampled: the same shape, with act at 0.02–0.25 and colour, count and place at 0.5–1.

## 3. Reading

- **The hypothesis holds in part.** A low τ lets the counts student hear the tendency: its act rises from 66.4% to 72.8%. But a low τ for every aspect lets noise vote on colour, count and place, and exact falls to 46%.
- **Exactness depends on the aspect, and the data say which.** Chosen per aspect, τ goes low for the act (a tendency) and to 1 for colour, count and place (rules). That lifts exact from 60.3% to 65.1% (temperature 0) and from 46.6% to 49.3% (sampled), and narrows the gap to bag of words by half (temperature 0: 10.2 → 5.4 points) and by a third (sampled: 7.6 → 4.9). This is D13 made finer: a working exactness per aspect of the game, not one τ for the language (§88: what counts as exact depends on the purpose).
- **Bag of words still leads, by about 5 points exact and 7 on the act.** Movement multiplies each word's evidence as if the words were independent. A tendency spread over correlated words (stone and colour together) is exactly where a model that weighs words against each other is better. This is the old naive Bayes against logistic regression result, seen in our own terms. The fix is the one docs/110 found for code: learned weights on the movement. It has not been done here yet.
- **Unseen calls are nobody's.** `help`, `away` and `no` are each a single call, so all their askings fall in one fold. No student gets any of them right (checked on the temperature-0 run: 31 of 31 wrong for every student). The gap in *what* (97.7% against 88.8%) is almost entirely these calls: which default each student falls back to when it has heard nothing.

## 4. Next

1. Run `scripts\phi4_builders.ps1 -Roles`, which tells Phi-4 who fetches and who takes note. The script now also writes `_graded.md`. If the role confusion is the prompt's fault, the act should go from 74% towards 95%.
2. Give the builders' movement the learned weights of docs/110, a weight per (aspect, sightings), and measure it against bag of words on the same folds.
