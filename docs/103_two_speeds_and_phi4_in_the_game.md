# 103 · Two speeds, the condition read from its count, and Phi-4 in the game

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

This doc follows docs/102 and paper 19 (Descriptive Logic I), and
supersedes their phinet numbers. The raw tables for both seeds are in
`103_language_game_seed7.md` and `103_language_game_seed8.md`.

## 1. The fault on seed 8, and the fix

In docs/102, the phinet learner learned the word *sepia* from one pointing
plus the word `colour` on seed 7 (100%), but not on seed 8 (0%). Per-aspect
debugging (`PHIANO_DEBUG=1`) showed the colour was right on seed 8 too
(100%). What was wrong was **act**: every order read as `undo`.

The act comes from the condition: what an order does when no word says
otherwise. The condition was read from the operator's function of the game
token. A game token relates to *every* answer and holds no post, and a
function reads a spread relation as one peaked answer. After one
retraining (the same data plus one pointing), that answer was `undo`.

**The fix: read the condition from its count, for every learner.** Words
with a post are still read from functions. The four main tests are
unchanged (100%). phinet improves everywhere else (seed 7 / seed 8):

| | before | after |
|---|---|---|
| sepia, 1 pointing + `colour` | 100 / 0% | **100 / 100%** |
| sepia, 3 pointings + `colour` | 100 / 52% | **100 / 100%** |
| 320 episodes, R = 32 | 62.7 / 53.8% | 77.9 / 90.2% |
| 640 episodes, R = 32 | 99.4 / 89.2% | 99.4 / 99.0% |
| 640 episodes, R = 8 | 85.8 / 87.7% | 94.1 / 98.1% |

In Phient's terms: a condition is a count and a word's post is a function.
The condition says what is likely, and the function says where a word
moves it.

## 2. Two speeds

`TwoSpeed { consolidate: N }` reads a relation from its count until it has
been told N times, and from the operator's function after that. The
operator trains on every relation all along, and only the reading
switches. N was fixed before any test was read, at 8 and 32.
`consolidate: -1` reads from the function only where the function already
names the same top answer as the count.

Exact match on all four in-game tests, by episodes seen (seed 7 / seed 8,
R = 32):

| learner | 80 | 160 | 320 | 640 |
|---|---|---|---|---|
| bag of words + softmax | 82.7 / 77.4% | 99.8 / 94.7% | 100 / 100% | 100 / 100% |
| Phical counts, movement | 50.8 / 55.4% | 69.8 / 70.4% | 96.6 / 92.7% | 98.7 / 97.4% |
| phinet functions | 43.0 / 52.6% | 54.5 / 65.1% | 77.9 / 90.2% | 99.4 / 99.0% |
| two speeds, N = 8 | 51.5 / 52.5% | 56.8 / 67.3% | 79.4 / 91.7% | 99.0 / 97.4% |
| two speeds, N = 32 | 50.8 / 55.4% | 69.8 / 70.4% | 87.0 / 92.7% | 98.4 / 97.4% |
| two speeds, until the function agrees | 45.0 / 55.3% | 58.0 / 64.6% | 86.8 / 91.6% | 98.6 / 97.3% |

The stranger (§32), new words for a game already played, child →
stranger:

| learner | 40 | 80 |
|---|---|---|
| Phical counts, movement | 47→67% / 51→73% | 65→81% / 68→77% |
| phinet functions | 51→48% / 48→52% | 60→75% / 59→73% |
| two speeds, N = 32 | 47→53% / 51→65% | 65→81% / 68→77% |

What this shows:

- **Two speeds give the envelope of the two, not more than either.** With
  N = 32 the learner reads like the counts while data is scarce and like
  the functions once it is not. Where the counts are ahead (80–320
  episodes), N = 32 is at or near the counts (at 320, 87.0% against the
  counts' 96.6% on seed 7). At 640 it is 1–2 points below the pure
  functions. The stranger at 80 is exactly the counts.
- **The functions already hold about nine relations in ten.** With 3,600
  episodes, the function names the same top answer as the count for 90.8%
  (seed 7) and 88.7% (seed 8) of the relations. Those relations could keep
  only their function and drop the table. That is the compression the 4%
  target needs (docs/97): the counts are the fast memory and the functions
  are what is kept.
- **Bag of words still learns the in-game tests fastest.** The movement
  learners are the ones that learn a stationed word from a single pointing
  (§29–30).

## 3. Phi-4 in the game (built; runs on the GPU machine)

- **phiecs `quest/builders.rs`.** The builders' game as a quest kind.
  `prompt(game, words, gestures)` states the set-up: the stones, the
  letters used as numerals, the places, and the reply form `act | what |
  colour | count | place`. It never says what a call means. `parse(reply)`
  reads the first line with five fields. Two tests.
- **phiano `language_game ask <episodes.tsv> <n> [--parallel k] <oracle…>`.**
  Asks Phi-4 about n training calls, k at a time. Each answer is saved as
  it arrives, and a stopped run resumes exactly, because each line carries
  its index. Tested end to end with a scripted stand-in teacher (300 calls,
  3 at a time, stopped and resumed).
- **`language_game teacher <episodes.tsv> [out.md]`.** Two reports:
  1. How often Phi-4 did what the rules say: orders, no-name words and
     reports, per aspect.
  2. Students trained on Phi-4's actions against students trained on the
     rule's actions for the same calls, both tested against the rules.
- **`scripts/phi4_builders.ps1 [-N 400] [-Parallel 3]`.** Runs both steps
  on the GPU machine.

This is the step from observing Phi-4's *relations* to observing Phi-4 *in
action*. No result is claimed until it has run.
