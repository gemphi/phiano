# 107 · Phi-4 in the builders' game: what it did, and what a student learns from it

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Setup.** On 2026-09-30, `scripts/phi4_builders.ps1` put Phi-4 (Ollama, on the GPU machine) into the builders' game.

- It was asked what it does on 400 calls taken from the training episodes.
- The prompt stated the set-up: the stones, the letters used as numerals, the places, and the reply form. It never stated the rules.
- The raw episodes are in `data/builders/phi4_seed7.tsv`, and the first report in `data/builders/phi4_seed7.md`.
- This doc is `language_game follow data/builders/phi4_seed7.tsv`, with the parser as it stands after the fixes below. The raw table is `107_follow_seed7.md`.

## 1. Did Phi-4 work as a teacher?

**It answered.** 369 of 400 replies (92%) could be read as five aspects. Most of the unread ones were a markdown table rule (`--- | --- | …`) or explanation text; the parser now skips table rules. The replies also contained terminal escape codes from `ollama run`, which the parser now renders.

**It knows the words.** Once its notation is read in the game's terms:

| reading | exact | act | what | colour | count | place |
|---|---|---|---|---|---|---|
| Phi-4's own notation | 28.7% | 63.1% | 94.9% | 66.7% | 76.4% | 69.9% |
| its notation in the game's terms | **50.7%** | 63.1% | **94.9%** | **96.2%** | **89.7%** | **89.7%** |

The notation reading changes only these, and nothing else is corrected:

- `-` for "no colour said" is read as `any`, `-` for "no numeral" as `1`, and `-` for "no place" as `here`.
- The letters a–j in the count are read as numbers, which the prompt declared them to be.

**It confuses the two games.** Its act is right only 63% of the time.

- In 85 orders it answered `record`, which is the reporter's act.
- In 25 reports it answered `bring`, which is the builder's act.

The prompt set the game only by one sentence of scene ("A calls out to you, you are B" / "B reports to A, you are A"), and Phi-4 did not keep the roles apart.

Its other divergences are small:

- `yellow block` written in the *what* slot (9 calls);
- 9 counted as 10 (9 calls);
- `here` read as p1 (8 calls).

**It does not answer the same call the same way.** 60 calls were asked more than once. Of the 265 pairs of answers to the same call, only 95 (35.8%) were identical, even in the game's terms. The likely cause is sampling: `ollama run phi4` samples rather than always choosing the likeliest answer. Ollama's documented default temperature is 0.8, unless a model sets its own.

## 2. Can a student learn to act as Phi-4 does?

The test uses 5 folds by call, so a repeated call is never on both sides. Each learner is trained on Phi-4's actions (game's terms) in four folds and predicts Phi-4's action in the fifth.

| predictor of Phi-4's action | exact | act | what | colour | count | place |
|---|---|---|---|---|---|---|
| our rules (the game as we wrote it) | 50.7% | 63.1% | 94.9% | 96.2% | 89.7% | 89.7% |
| bag of words + softmax | **54.2%** | 67.5% | 93.5% | 91.6% | 82.9% | 90.8% |
| Phical counts, movement | 46.6% | 61.2% | 87.8% | 91.6% | 81.3% | 88.6% |
| Phient two-speed (32) | 46.9% | 61.2% | 88.1% | 91.6% | 81.3% | 88.6% |
| Phient phinet, R = 32 | 45.0% | 60.4% | 87.8% | 91.1% | 77.0% | 86.2% |
| naming table | 46.1% | 55.3% | 94.9% | 92.1% | 84.3% | 90.8% |
| lookup | 0.5% | 53.1% | 25.7% | 49.3% | 47.4% | 53.1% |

The students learn Phi-4's practice from only about 300 of its answers. On its act they do as well as our rules, or better: bag of words gets 67.5% against the rules' 63.1%. But no predictor can beat a teacher that gives different answers to the same call. With identical answers to only 36% of repeated calls, exact prediction of any single answer is capped near that level plus what is easy.

## 3. Verdict, and what was changed

- **Phi-4 helps as a source of relations.** The night quest's 3,630 relations taught the relation model held-out relations at twice the prior (docs/99, docs/101). They also showed family resemblance and cluster names in its categories (docs/105).
- **Phi-4 works in the game on the words**: 95% stone, 96% colour, 90% count, 90% place.
- **It does not yet work as a steady teacher of action.** The roles are confused (act 63%), and the same call gets different answers.
- **Fixed for the next run:**
  - `scripts/phi4_builders.ps1` now plays through a model `phi4-builder`: phi4 at temperature 0 with a fixed seed, created by the script. The same call should then get the same answer.
  - The first run's file is kept. The new run writes `phi4_t0_seed7.tsv` and a `_follow.md` report.
  - Pass `-Sampled` to repeat the first run's setting.
- **Parser:** it now renders terminal escapes, skips markdown table rules, and reads letter numerals in the count. The log keeps line breaks (`⏎`) instead of flattening them.
- **Not yet changed: the prompt's role sentence.** This is kept, so that temperature is the one thing that differs between the two runs.

## 4. How to see it learn

`scripts/demo_learning.ps1` runs the real crates on your machine:

1. the squares game, in under a second: the reader finds the two-square words from 50 sentences and gets every test right;
2. the builders' game, in about 3 minutes;
3. the Phi-4 relation model answering questions;
4. this Phi-4 analysis, in about 3 minutes.

Reports go to `data/demo_learning`.
