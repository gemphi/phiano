# 116 · Phi-4 taught a word by pointing: it learns the word, and loses the game

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Run.** `scripts\phi4_sepia.ps1`, 2026-10-01/02, GTX 1660 Ti. Phi-4 played at temperature 0, with the roles stated (the setting of docs/115: act 99.5%).

- **The teaching.** Before each order the prompt tells one or three pointings:
  - *A pointed at a group: 6 slabs at p4, of a colour you have no word for, and said "sepia"* (or "colour sepia").
  - The reply form also allows "a word A taught you by pointing".
- **The orders.** 50 orders that use `sepia` per teaching, the same orders as our students' sepia test (§§28–30, paper 19).
- **Raw files.** `data/builders/sepia/`.

## Result

| teaching | replies read | exact | colour = `sepia` | act | stone | count | place | our counts student: exact / colour |
|---|---|---|---|---|---|---|---|---|
| 1 pointing | 37 of 50 | 29.7% | 64.9% | 54.1% | 83.8% | 81.1% | 83.8% | 32% / 100% |
| 3 pointings | 50 of 50 | 34.0% | **90.0%** | 48.0% | 100% | 84.0% | 94.0% | 32% / 100% |
| 1 pointing + `colour` | 44 of 50 | 27.3% | 63.6% | 59.1% | 86.4% | 86.4% | 79.5% | **100% / 100%** |
| 3 pointings + `colour` | 50 of 50 | 16.0% | **90.0%** | **24.0%** | 100% | 76.0% | 82.0% | **100% / 100%** |

- Bag of words gets 0% on sepia in every teaching (paper 19).
- The student column is our counts student (D2, movement), trained on the rule game plus the same pointings, and asked the same 50 orders.

**What went wrong instead of `bring`.** In the 1-pointing run, 12 of 37 acts were `record` and 5 were `colour` or `color`, an act that is not in the game. In the 3 pointings + `colour` run, most acts were `record`, for example `record | slab | sepia | 6 | p2`. Without a teaching, the same prompt gave `record` for 2 calls in 380 (docs/115).

## Reading

- **Phi-4 learns the word, and fast.** One pointing puts `sepia` in the colour slot in about 64% of the orders it answered. Three pointings put it there in 90%. Our counts student gets the colour right in every order, but only because in these 50 orders the colour is the aspect that decides it. Without `colour`, its exact match is 32%: the pointing at slabs also leaves `sepia` related to the stone, the count and the place (§28: a pointing can be taken as naming a shape, a number, a place).
- **The grammar word helps our student and does nothing for Phi-4.** For the counts student, `colour` sets the post (D2) under the colour aspect alone, and exact goes from 32% to 100% (§§29–30). For Phi-4, `colour` changes nothing for 1 pointing (64.9% → 63.6% colour) or for 3 (90% → 90%). Phi-4 already took the word as a colour word: the prompt says the group's colour is one there is no word for. That told it what §29's grammar word tells our student.
- **Phi-4 loses the game.** Being taught a word moves it into another game: naming and noting, instead of fetching. Its act falls from 99.5% (docs/115) to 24–59%, and the more teaching, the worse. With 3 pointings + `colour`, three answers in four are `record`.
  - For our student the pointing is its own game (`ostension`). The act of the order game is not read across games unless the two games agree on it (D3, docs/104), and they do not. So its act stays `bring` (100%).
  - Phi-4 has no such fence. The scene of the teaching leaks into the scene of the order.
- **Exact match:** our student 32% → 100% (with `colour`), Phi-4 16–34%. On this test the relation model with local game conditions plays the game better than the large model it set out to copy. The large model holds the new word better.

## What this means for the 4% model

- **What Phi-4 gives the model.** A new word's first relation (the colour) from one or three showings, without being told which aspect. That is worth distilling. On its own, our student needs `colour` to get there.
- **What the model keeps for itself.** The condition: which game an utterance belongs to (D3, D8). Phi-4's practice shows that the game is not a property of the words. The teacher's practice can lose it, so the student must keep it.
- **Trial: distillation, stage B (`sepia-report`, last column).** The test runs in 5 folds over the 50 orders.
  - In each fold, the student looks at Phi-4's answers to the orders of the other folds.
  - If Phi-4 put the taught word in the colour slot in most of them, the student is told what §29's grammar word would tell it: this is a colour-word.
  - The student keeps its own act, stone, count and place.

  The result is **100% exact in all four teachings**, including the two without `colour`, where the student alone got 32%. Phi-4 alone got 16–34%.

  The gate opened in every fold, because Phi-4's readings were 64–90% `sepia`. So the 100% is that of the student with `colour`. What Phi-4 contributed is the grammar word, read off its practice instead of said by A. This is distillation of one relation, not of a skill. But it is the division of labour that the numbers above ask for: the word's aspect from the large model, the game from the relation model.

`sepia-report` raw: `116_sepia.md`. Phi-4's acts, per teaching:

| teaching | bring | record | colour/color | unread |
|---|---|---|---|---|
| 1 pointing | 20 | 12 | 5 | 13 |
| 1 pointing + `colour` | 26 | 14 | 4 | 6 |
| 3 pointings | 24 | 26 | – | – |
| 3 pointings + `colour` | 12 | 38 | – | – |
