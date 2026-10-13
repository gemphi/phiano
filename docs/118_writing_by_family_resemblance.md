# 118 · Writing what Phi-4 writes: compiling is easy, working is the gap

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

**Round 1 of `scripts/phi4_code.ps1`.** Phi-4 answered 100 Rust tasks in 35 families (`data/phi4_code/`).

## 1. Reading Phi-4's code (next token, whole families held out)

`code_reader --by-dir`, two splits:

| reader | Phi-4's code alone: top-1 | perplexity | identifiers top-1 | + our 690 files: top-1 | identifiers top-1 |
|---|---|---|---|---|---|
| 4-gram | 42.1 / 48.6% | 29.0 / 14.1 | 26.0 / 32.0% | 42.3 / 49.5% | 9.2 / 16.2% |
| **4-gram ⊕ movement, learned weights** | **45.2 / 53.0%** | **13.6 / 9.0** | **30.7 / 35.2%** | **45.3 / 56.9%** | **17.1 / 27.9%** |

The training is only about 4,700 tokens of Phi-4's code. Movement with learned weights beats the 4-gram on every measure, as it did on our own code (docs/110).

Adding our own Rust helps the reading of keywords and punctuation (split 1: 53.0 → 56.9%). It hurts the names: identifiers fall from 30.7 to 17.1% and from 35.2 to 27.9%. Our vocabulary of phasors and facets crowds out Phi-4's `v`, `acc` and `count`.

## 2. Writing what Phi-4 writes (`code_write --answered`)

Given each signature, the student writes the body; each family is held out in turn.

- **Measures.** Does the body parse? Does it compile (`rustc`)? Does it **work**, that is, do the family's asserts pass (`checks.rs`, compiled and run)?
- **The teacher.** Phi-4's own answers go through the same checks.
- **The checks were validated.** 153 reference implementations of the round-2 families pass 153 of 153.

Trained on Phi-4's answers to the other families only:

| writer | parses | compiles | **works** |
|---|---|---|---|
| 4-gram, greedy | 55% | 0% | 0% |
| 4-gram ⊕ movement, greedy | 46% | 11% | 0% |
| + the signature as a condition (relations and copy), greedy | 34% | 7% | 0% |
| the same, beam 8, first that compiles | 71% | 29% | 3% |
| **family resemblance** (a learned answer, words exchanged), else the beam | 64% | 64% | **8%** |
| **Phi-4 (the teacher)** | – | – | **95% (90 of 95)** |

With our own Rust added to the training as well, the beam compiles 61% but works 3%; family resemblance works 8%.

**What was added on the way** (`code_write/`):

- **Open vocabulary.** A word of the signature never seen in training gets an id of its own, so it can be said again.
- **Small number literals kept** (`0`, `1`, `2`). Parsing rose from 8% to 55–60% with this.
- **The signature as a condition on the whole body** (`sig.rs`). Every word of the signature relates to body tokens, and a copy relation decides, after each token, how likely the next one repeats a signature word.
- **A beam** that keeps the first body that compiles.
- **Family resemblance** (`analog.rs`, PI §§65–67). A learned answer whose signature has the same shape is said again, with the differing words exchanged (`product` ↔ `sum`, `i32` ↔ `i64`). Answers that say their own name again in the body come first. That is the relation "said in the signature ⇒ said again", carried by a whole answer.

**What works is telling.**

- Family resemblance got `sum` right from `product`: `v.iter().product()` becomes `v.iter().sum()`. It got `min` from `max`, and `max` from `min`.
- It got `product` wrong. Phi-4's own `sum` is `v.iter().fold(0, |acc, &x| acc + x)`, which names no operation. So the exchange carries `+` where `*` is needed.
- **The gap is word meaning.** That `product` means `*`, `even` means `% 2 == 0` and `positive` means `> 0` is a relation of the *word*, not of the code around it. A student who has seen each name in one family only cannot learn it from code statistics.

## 3. Round 2: give the student neighbours

The task list now has **253 tasks in 83 families** (`code_quest/tasks.rs`). The 48 new families resemble the first ones in name and shape: `sum_even`, `count_odd`, `count_below`, `all_even`, `any_negative`, `keep_positive`, `max_index`, `product_to` (= factorial), `square`, `cube`, `is_anagram`, and more.

With 83 families, a held-out name such as `even` or `positive` is seen in other families too (`count_even`, `sum_even`, `keep_even`, `all_even`). The test is whether family resemblance and the signature relations can then carry the meaning of a name from one family to another, the way §67's fibres overlap.

`scripts\phi4_code.ps1` resumes: the 100 answered tasks are kept, and the 153 new ones are asked.

Raw: `118_write_phi4_round1.md`, `118_write_phi4_round1_with_ours.md`, `data/phi4_code/read_*.md`.
