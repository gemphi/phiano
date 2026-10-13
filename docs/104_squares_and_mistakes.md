# 104 · The squares game (§48, §64) and a teacher who makes mistakes (§54)

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

The write-up is paper 20, `docs/papers/20_DESCRIPTIVE_LOGIC_2_NAMES_SAMPLES_SIMPLES_RULES.md` (and `.pdf`), which reads Philosophical Investigations §§33–64. Raw tables:

- `104_squares_game_seed{7,8}.md`
- `104_mistakes_seed{7,8}.md` (trial 9)
- `104_mistakes_trial8_seed{7,8}.md` (before the fix)

## Squares game

`bin/squares_game` (under a second). It is a 3 × 3 grid read row by row:

- R, G, W, B each name one square;
- U, V, X, Y each name two squares side by side (§64).

The Phient reader learns two relations for each word by EM over the ways
a sentence can lie on the grid: *width* (1–3) and *colour at offset*. It
is 100% on all four tests, both seeds, from 50 sentences:

- plain names;
- one composite at a trained place;
- one composite at a new place;
- two or more composites.

It finds width 2 for exactly the four composites. The positional softmax
(`word@index`) reaches 50–64% on plain sentences and 0–17% on the other
tests at 1,000 sentences. The naming table is 100% on plain sentences and
0–6% on everything else.

The reader is told the form of the game (reading order, adjacent squares).
A learned-alignment sequence model would be the fair modern comparison.
It has not been built.

## Mistakes

`language_game mistakes out.md --seed 7` takes about 35 minutes. One
aspect of a share *p* of training episodes is replaced by a wrong value.
The mistakes are either silent, or seen corrected (played in a game
`corrected`).

- **Trial 8.** Silent mistakes up to 40% cost Phical counts nothing.
  Corrected mistakes broke D3: game agreement was judged over all games,
  and the error-filled `corrected` game vetoed carrying words from orders
  into reports. Counts fell to 77.8% on seed 8 at 10%.
- **Trial 9.** Agreement is judged between the two games concerned only.
  Counts: 100% in every column, both seeds. phinet: 95.9–100%. Bag of
  words: 100% on silent mistakes, but 88.7–97.5% on corrected mistakes at
  20–40%.

Code: `compose.rs` (`agree_between`, `carries`) and `learners.rs` (`games`,
`open`). Trial 9 changes nothing when a language has only the order and
report games. A full seed-7 rerun reproduces docs/103's tables, with one
exception: the "until the function agrees" rows, which moved by up to 4.9
points at small data. They had depended on hash-map order whenever a
function's two top answers tied (as had lookup's ties, by 0.4 points).
Ties now break by name, and two runs of the same code are identical
(`104_rerun_seed7.md`).
