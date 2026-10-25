# 97 · Laws as structure, and the staged student

In Phient's world, a word (w) is nothing but what it relates to, under what
conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor
Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64
relations, each a complex phasor on a closed circle with a count; meaning is a
side effect of pairs of words moving under a condition. Decided 2026-09-28:
**both sizes, staged**, and **laws are declared (words) or come from the
compiler (code)** — they are followed exactly, not rewarded.

## 1. The math: a law is how the answer is computed

A word is a function f_w ∈ ℂ⁶⁴ (more channels as declared, docs/90); a kind
k is an operator K_k; the answer to `w · k · ?` is the word whose function is
nearest ŷ = K_k(f_w).

Today a law is a penalty (`closure_epoch`): loss += α·(1 − cos(K_has(f_cat),
K_has(f_mammal))). The data can outvote it — docs/95 measured exactly that.

As structure, each declared law is an equation the forward pass obeys:

| law | equation (⊕ = phasor sum with counts) |
|---|---|
| inheritance (k inherits along is-a) | K_k(w) = own_k(w) ⊕ K_k(isa(w)) — recursive up the is-a chain |
| inverse (k′ = k⁻¹) | K_k′ is defined from K_k: v ∈ K_k(w) ⇔ w ∈ K_k′(v) |
| transitivity (is-a, part-of) | K_k(w) = own_k(w) ⊕ K_k(K_k(w)) |
| exclusion (unlike) | K_unlike(w) is scored against, never with, K_like(w) |

Only the `own` parts are learned. That is where the compression is: a cat
stores whiskers, not fur — fur is its kind's.

## 2. Code: the compiler's laws

| part | source | role |
|---|---|---|
| grammar | Rust's parser | hard: which token may come next (mask) |
| types, borrowing | `cargo check` | hard: the `worse` operator; code that does not compile is refuted |
| relations | the parser, exactly | channels: defines, calls, returns, type-of, implements, field-of |
| data | Rust book, gemphi's Rust, Phi-4's answers | what fills the `own` parts |

The condition c is the parser's state (inside `fn`, expecting an expression
of type T). Generation masks every token the grammar or types forbid, then
the operator chooses among the rest.

## 3. The two stages

**Stage A — the relation model (8/16/64M numbers).** Declared laws as
structure (§1), code laws from the compiler (§2), word functions + kind
operators in `phinet::relate`, readable and small. Learns from the quest and
from the corpora. Judged by agreement with Phi-4 on untold questions.

**Stage B — the distilled student (~1.4B, 10% of Phi-4).** Phi-4 is open
(14B, MIT licence on Hugging Face). Its next-token probabilities are the
signal — far richer than its chosen words — and its weights can be pruned and
factorised first (NVIDIA's Minitron derived 8B and 4B models from a trained
15B with up to 40× fewer training tokens than from scratch). phinet's
operator layers are the student's blocks; Stage A's relation model is its
readable layer, and new things are learned by adding rows and `own` parts
without retraining the rest. Needs Phi-4's weights (~28 GB) and, for 100M
tokens of distillation in reasonable time, a GPU.

## 4. Order

1. Inheritance and inverse as structure in `phinet::relate` (§1), measured
   against the soft closure on the same Phi-4 rows (docs/95) and the night's
   `all.tsv` (docs/96).
2. The declared-law file (which kinds inherit, which are inverses) — the
   user's list, versioned beside the data.
3. Rust relations from the parser; grammar mask; `cargo check` as `worse`.
4. Stage B: Phi-4 weights and a probability-dump tool (llama.cpp or
   transformers), then pruning and distillation into phinet blocks.
