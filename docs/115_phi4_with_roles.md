# 115 · Phi-4 told the roles: from tendency to rule

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Run.** On 2026-10-01, `scripts\phi4_builders.ps1 -Roles` was run on a GTX 1660 Ti. It asked the same 400 calls at temperature 0. The prompt was the same as before, except that the scene now also says who fetches and who takes note (`builders::prompt_with(.., roles = true)`). Raw: `data/builders/phi4_t0_seed7_roles*`.

## Three runs side by side

| run | replies read | exact (game's terms) | act | same answer to the same call | bag of words predicts Phi-4 | counts (D2) predicts Phi-4 |
|---|---|---|---|---|---|---|
| sampled | 369 of 400 | 50.7% | 63.1% | 35.8% | 54.2% | 46.6% |
| temperature 0 | 393 of 400 | 66.4% | 74.3% | 88.9% | 70.5% | 60.3% |
| **temperature 0 + roles** | 380 of 400 | **88.2%** | **99.5%** | **100% (276 of 276)** | 89.7% | 85.3% |

In the game's terms, the other aspects are:

- stone 97.4%;
- colour 100%;
- count 93.7%;
- place 97.1%.

**The role confusion was the prompt's.** Told the roles, Phi-4 confuses the act in 2 calls out of 380, down from 101. It answers every repeated call the same way.

**What remains is notation, not practice:**

- 13 calls count `i` (9) as 10;
- 10 calls write the colour into the stone (`yellow block`);
- 11 calls give a place `here` where the rule has none.

## The students, on a teacher who now keeps rules (5 folds by call)

| student of Phi-4 | exact | exact, calls with a known word | act |
|---|---|---|---|
| our rules (the game as we wrote it) | 88.2% | – | 99.5% |
| bag of words | **89.7%** | **92.4%** | 96.6% |
| counts, D2 (τ = 0.5) | 85.3% | 87.8% | 96.6% |
| counts, τ per aspect (docs/111) | 89.5% | 92.1% | 96.6% |
| counts, weights per (aspect, post strength) (docs/112) | 89.5% | 92.1% | 96.1% |
| counts, two-speed posts, 8 sightings (docs/113) | 84.5% | 87.0% | 96.6% |
| Phient phinet, R = 32 | 83.2% | – | 96.6% |

The students' act is 96.6%, not 99.5%, only because `help`, `away` and `no` are each a single call that is never heard in training (docs/111).

## Reading

- **Phi-4 now works as a teacher.** It answers steadily (100%), keeps to the game (88% exact, and 99.5% on the act), and its students predict it within 0.2 points of the best: counts with τ per aspect reach 89.5%, against bag of words at 89.7%.
- **Which student is best depends on the teacher, as Descriptive Logic predicts:**
  - On the **rule-keeping** teacher, the per-aspect post (τ chosen in training) and the learned post weights tie with bag of words. The two-speed posts are slightly below D2 (84.5% against 85.3%): with no tendency to find, the slow speed only adds noise.
  - On the teacher of **tendencies** (temperature 0, no roles), the two-speed posts were best (docs/113).

  The post threshold is part of the game, not one constant (§88): chosen in training per aspect, it suits both teachers.
- **Recommended student for Phi-4 runs from now on:** counts with τ per aspect, chosen in the training folds. It matches bag of words here, and on the tendency teacher it was 5.4 points behind (65.1% against 70.5%). It keeps D2's one-pointing learning. The two-speed posts stay the choice when the teacher's practice is graded.
- **Next:** Phi-4 as a teacher of new words (sepia by pointing, §28–30), now that it keeps the roles. Then distil, using the night quest's relations together with the game.
