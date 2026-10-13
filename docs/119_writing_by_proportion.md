# 119 · Writing by proportion, and checks that cannot be fooled

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Round 2.** Phi-4 answered all 253 tasks (83 families). The student writes each of the 248 function tasks with its whole family held out, from Phi-4's answers to the other families only.

## 1. First, the checks were wrong

The user spotted it. Family resemblance "passed" `count_negative` with `x % 2 != 0` and `count_even` with `x % 2 != 0`. The inputs made odd count equal negative count, and even count equal odd count.

**The audit** (`code_write --audit`, `audit.rs`):

- Every learned answer of the same signature shape is said again under each task's names and run against the task's check. A pass by another family's body is a coincidence in the check.
- **First audit: 200 coincidences.** Pairs included `is_odd` / `is_prime`, `gcd` / `abs_diff`, `fibonacci` / `triangle`, `count_even` / `count_odd`, `count_negative` / `count_odd`, `mean` / `second_largest`, `first_even` / `second_largest`, `sum` / `sum_abs`, `all_positive` / `is_sorted`, and others.
- **The fix.** Each check got inputs that tell the look-alikes apart, plus an assert with negatives for the signed types.
- **After: 38 left, all of them right.** For example, `product`'s body `v.iter().product()` with the name exchanged *is* `v.iter().sum()`. Likewise `min` ↔ `max`, and `product` → `v.iter().is_sorted()`.
- **Validated both ways.** The 153 reference implementations still pass 153 of 153, and Phi-4 still passes 224 of 248 (90%).

**Every number below uses the fixed checks.** The family-resemblance row of docs/118 fell from 11% to 6% under them.

## 2. Writing by proportion (`proportion.rs`)

*As `sum_even` is to `sum_odd`, so is `count_even` to `count_odd`.*

- **A word's meaning in code is the change it makes.** For two learned answers whose names differ in one word (`even` → `odd`), the token-level difference of their bodies is that word pair's edit (`== 0` → `!= 0`).
- **The edit is reused.** A new name that differs from a learned name in the same word gets the learned body with the edit made, and the names exchanged.
- **The first candidate that compiles is kept.** Otherwise the writer falls back to family resemblance, then to the beam.

| writer (each family held out; Phi-4's other answers only) | compiles | **works** |
|---|---|---|
| 4-gram, greedy | 0% | 0% |
| 4-gram ⊕ movement, greedy | 9% | 0% |
| + signature as condition (relations, copy) | 30% | 0% |
| beam 8, first that compiles | 61% | 1% |
| family resemblance (whole answer, words exchanged) | 87% | 6% |
| **proportion, else family resemblance, else beam** | **89%** | **25% (63 of 248)** |
| **Phi-4 (the teacher)** | – | **90% (224 of 248)** |

**What proportion got right** (variants of each family that work): `triangle` 8, `sum` 8, `sorted` 6, `min` 5, and 4 each for `sum_odd`, `sum_above`, `max`, `is_odd`, `is_even`, `count_even`, `count_odd`, `any_even`, `all_even` and `count_above`. Then 3 each for `any_negative` and `all_positive`, and 2 each for `count_positive` and `count_negative`.

The same two cases the user flagged, now:

```rust
fn count_negative(v: &[i32]) -> usize { v.iter().filter(|&&x| x < 0).count() }   // positive → negative
fn count_even(v: &[u32]) -> usize { v.iter().filter(|&&x| x % 2 == 0).count() }  // odd → even
```

## Reading

- **The meaning of a word, learned as the change it makes, moves from family to family.** `even`, `odd`, `positive`, `negative`, `above`, `below`, `max`, `min`, `sum` and `product` carry their meaning into names never seen. This is family resemblance (PI §§65–67) made exact: the fibres are the edits, and they overlap across families.
- **Phi-4 is still far ahead: 90% against 25%.** What the student cannot do yet:
  - names whose words occur in no other family (`caesar`, `palindrome`, `lcm`);
  - shapes that differ (two parameters against one);
  - two words changing at once (`count_above` → `sum_below`).
- **Next, in order of likely gain:**
  1. chain two edits;
  2. allow a different parameter shape when the extra parameter is used as one token;
  3. ask Phi-4 for the missing pairs. The quest can be steered to the word pairs the student lacks: distillation by the student's own questions.

Raw: `119_write_phi4_round2.md`, `119_check_audit.md`.
