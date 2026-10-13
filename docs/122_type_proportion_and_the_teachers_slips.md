# 122 · Proportion over types, and where the student beats its teacher

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

All runs here were made in the container from Phi-4's saved 253 answers and 72 glosses. Nothing new was asked of Phi-4.

## What changed since docs/121

1. **Proportion over types** (`types.rs`). As `max(i32)` is to `max(f64)`, so is `min(i32)` to `min(f64)`. The difference between Phi-4's two bodies for one name in two types is the edit the type makes (`.max()` → `max_by(partial_cmp)`, `.abs()` dropped for `u32`). The edit most names agree on is applied to the target's sibling in the source type. It counts 2 in the vote.
2. **The answer itself counts most.** A learned answer that differs from the new signature in type words only is the same task in another type; it now counts 5 against 1–2 for inferred routes. Before, several wrong inferences that agreed with each other could outvote one right sibling.
3. **Helpers are kept** (`analog.rs`). Phi-4's `lcm` answer defines `gcd` first; the answer is now the file's *last* top-level function, with the helpers placed after its body.
4. **Rendering of `/=`, `%=`, `<<`, `>>`** was broken (written `/ =`), which made every sibling of `reverse_digits` fail to compile. Fixed in `write.rs`.

## The two tests

| writer | new type of a known task | family never seen |
|---|---|---|
| docs/121 (type siblings first, routes vote, glosses) | 79% | 27% |
| **+ 1–4 above** | **85% (212 of 248)** | 27% (68) |
| **Phi-4 itself** | **90% (224)** | 90% |

Compiles: 98% and 89%.

## Student against teacher, task by task (new type of a known task)

| | student right | student wrong |
|---|---|---|
| **Phi-4 right** | 199 | 25 |
| **Phi-4 wrong** | **13** | 11 |

- **The student corrects its teacher in 13 tasks.** Phi-4's systematic slip is a `.map(|&x| x * 2)` dropped into `sum_above`, `evens`, `positives`, and a square in `sum_odd`. Where the student's other routes (another type's sibling, a proportion, a gloss) agreed on a body without the slip, the vote chose it. `sum_above` is right in all four types for the student and wrong in all four for Phi-4.
- **The student inherits the slip in 11**, where every sibling carries it (`sum_odd`, `keep_even`, `keep_positive`, `digit_sum`).
- **Of the 25 the teacher alone gets right, 18 are string tasks.** Each string family has one variant, so there is no sibling; these are the "family never seen" case inside the by-task test. The other 7: `fibonacci_i32`, `gcd_u64`, `max_abs_f64`, `doubled_f64`, `count_digits_u64`, `sum_even_u32/u64`.

## Reading

- **On a known task in a new type the student is at 85%, 5 points behind Phi-4**, and ahead of it on the slip Phi-4 makes most. The relation model is 12,300 tokens of counts and 253 told answers.
- **The remaining gap on this test is almost all single-variant families.** The next quest gives the string tasks siblings: a second parameter form or a `Vec<char>` variant of each, so that "the same task in another type" exists for them too.
- **On a family never seen, 27% stands** until Phi-4's expressions are in (`code_quest --exprs`, one unattended run of `scripts\phi4_code.ps1`).

Raw: `122_write_by_task.md`, `122_write_by_family.md`.
