# 125 · One hearing: the operator does not beat the counts there either; and the needle, set up

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. The question docs/124 §4 left

docs/124 said the operator's place might be the word heard once: the counts need `below` in three functions before they can tell it from `count`, and an operator inheriting along `has-word` might find `above : > :: below : <` from one hearing. That is now measured. `code_relations --held-out family` splits the held-out test rows by how many *other* told functions said the name's rarest word (0, 1, 2, 3 or more), and scores each stratum with every reading. Phi-4's 253 answers, 200 test rows (the `has-literal` kind now counted), R = 64, 60 epochs, two seeds:

| the name's rarest word was heard in … other functions | rows | prior | operator, inheriting | operator, two hops | counts, posts (D2) |
|---|---|---|---|---|---|
| 0 (seed 7 / 11) | 111 | 0.318 | 0.307 / 0.313 | 0.303 / 0.292 | **0.357** |
| 1 | 30 | 0.407 | 0.393 / 0.421 | 0.415 / 0.397 | **0.470** |
| 2 | 39 | 0.350 | 0.389 / 0.405 | 0.372 / 0.397 | **0.433** |
| 3 or more | 20 | 0.325 | 0.297 / 0.315 | 0.298 / 0.290 | **0.368** |

Whole board at R = 64 (seed 7): prior 0.338, operator own 0.326, inheriting 0.347, two hops 0.328, counts with posts 0.390 (hit@5 62.0% against the prior's 53.5%), counts τ = 0 0.375.

**Reading.** At one hearing the counts are 6 points above the prior and 5–8 above the inheriting operator; at two hearings 8 and 3–4. The operator is never the best line in any stratum, and at zero and three-plus hearings it is below the prior. So the hypothesis is refused: on these rows the epoch-trained operator does not learn a word from one hearing better than a count does, and the counts learn from one hearing already (the one-hearing stratum is the counts' best). What a word heard once gives is one post, and a post is enough to move the prior; what it cannot give is the separation of the word from its first company, and nothing in the operator supplies that either. The remedy stays what docs/124 §3 built: ask for more company (`--unknown` → `--uses`).

The operator's remaining job is therefore not learning from little; it is what the counts cannot represent at all — a relation between two words that were never told together, and the laws between kinds (docs/92–97). That is a different experiment and should be stated as one before it is run.

## 2. The needle, set up (paper 24 §7)

`language_game sepia-ask <dir> <n> --between m …` puts `m` other orders, answered as the game answers them, between the pointing at `sepia` and the order that uses it, so the teaching sits `m` exchanges back in Phi-4's transcript. `scripts\phi4_sepia.ps1` now runs the two `colour` teachings at 25, 100 and 400 exchanges after the plain run; `sepia-report` lists each distance as its own row beside the counts student. The student's row is the same at every distance because `Counts` has no order of telling; the teacher's rows are the measurement — how a large model's unedited transcript holds a word taught once, which is the thing a Context Language Model edits its transcript to keep.

`code_relations <dir> --held-out family --r 64 --epochs 60 --seed 7|11` · `language_game sepia-ask data\builders\sepia 50 --between 100 ollama run phi4-builder` · `scripts\phi4_sepia.ps1`
