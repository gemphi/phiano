# 121 · Two tests of the student: a known task in a new type (79%), a family never seen (27%)

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Data.** Phi-4's 253 answers and its 72 one-token glosses (`data/phi4_code/`). All numbers use the audited checks of docs/119. Every run here was made in the container from the saved answers; nothing new was asked of Phi-4.

## Two tests

The goal is a student that re-creates Phi-4's coding from observation. That has two parts, and they need two tests.

1. **A known task in a new type** (`code_write --by-task`). Each of the 248 functions is held out alone. The student has seen its siblings (`sum(i32)` when asked `sum(i64)`) and every other family. This is what a user of the student meets once Phi-4 has been observed.
2. **A family never seen** (`code_write`, by family). All four type variants are held out together. This measures what the student can *infer* from other families.

| writer | new type of a known task | family never seen |
|---|---|---|
| 4-gram ⊕ movement, greedy | 2% | 0% |
| beam 8, first that compiles | 10% | 0% |
| family resemblance (a learned answer, words exchanged) | 76% | 6% |
| + proportion (as sum_even : sum_odd, so count_even : count_odd) | 78% | 25% |
| **+ Phi-4's glosses** | **79% (197 of 248)** | **27% (68 of 248)** |
| **Phi-4 itself** | **90% (224 of 248)** | 90% |

(Compiles: 98% and 89%. The row "with expressions" equals the glosses row here: Phi-4's expressions have not been asked yet.)

## What changed since docs/120

- **Type siblings come first** (`analog.rs`). A learned answer that differs from the new signature in type words only is the same task in another type: it is tried before a name sibling, and it counts 3 in the vote. Before this, `count_even(i64)` was written from `count_odd(i64)` instead of from `count_even(i32)`, because `count_odd(i64)` shared more tokens with the signature. By-task went from 54% to 78% on that change alone.
- **All routes vote** (`routes.rs`). A told expression counts 2, a proportion or a gloss 1, a type sibling 3, any other look-alike 0. The body most routes arrive at goes first; the first that compiles is kept.
- **Expression exchange** (`exprs.rs`, `code_quest --exprs`). The quest and the writer are built and tested with a hand-written file of 8 expressions (`even ⇒ x % 2 == 0`, `zero ⇒ x == 0`, `above ⇒ x > y`, …), which took the by-family score to 29%. That file was mine, so it is not in the table. The real one will come from Phi-4 the next time `scripts\phi4_code.ps1` runs.

## Reading

- **On the first test, the student is 11 points behind its teacher** (79% against 90%), and it is a relation model of 12,000 tokens of counts plus 253 told answers. Of the 51 it misses, the type sibling's body does not carry over in 20 (`f64` has no `%`, no `.abs()` on `u32`, `Option<f64>` has no `max()`), and the rest are Phi-4's own failures carried over or families with one variant only (the string tasks, where there is no sibling).
- **On the second test, the student is far behind** (27% against 90%). The three causes of docs/120 stand: no partner pair for a word, the teacher's slip propagated, a name that occurs nowhere else. A family that is a whole algorithm (`gcd`, `fibonacci`, `caesar`, `palindrome`) cannot be inferred from other families by any student; it has to be told. That is what the first test measures.
- **The division of labour holds.** The relation model keeps the game (types, shapes, which word changes what) and borrows word meanings from the teacher as told relations: glosses now, expressions next.

## What the user runs, and when

Nothing is needed from the GPU until the next round of asking. The next round is one unattended run of `scripts\phi4_code.ps1`: it asks Phi-4 the 72 expressions (short answers), then writes both tables. Everything else runs here.

Raw: `121_write_by_task.md`, `121_write_by_family.md`.
