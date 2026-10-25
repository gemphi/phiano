# 170 · The questions, and their conditions of satisfaction

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

This doc fixes, in advance, the questions we are answering and what would satisfy each (Searle: an intention is understood only through its condition of satisfaction, docs/169). Without these fixed first, every measurement just moves the bar — which is the mistake the earlier runs of `learn_bcpl` kept making.

## The correction that reframes all of it

**A word is the leaf, not the unit. The unit is the concept, and concepts are a top-down hierarchy.** Meaning is not in a word and not in the last two words (flat leaves — "shitty rows"). It is the **path** from the field down to the word: computer science → compilers → operators → this token. Teaching a little works *because* a new word is placed under a concept node it inherits context from — you do not need to learn everything, only enough to place the next thing and keep placing on the fly. This is the scope tree and the cast (docs/126): a word is drawn into its path of scopes, and the path is the meaning. **Operators, not types** — we compute over these paths with the Lisp (docs/165), we do not declare class hierarchies.

Depth is real and measured: on BCPL the construct nesting reaches **max depth 328, mean 29** (`bcpl::learn`), so the buildings are genuinely deep. The lesson from the depth sweep: raw deep-path context alone over-specialises (accuracy falls past depth 8) — the path must be read as a *hierarchy of concepts that back off*, outer level conditioning inner, which is exactly `read_levels` / the cast, not a single deep n-gram.

## Q1 — Is structure learnable at all, from examples, online?

The counting reader, fed function by function, predicting before it learns each.

**Condition of satisfaction (fixed):** next-token *role* accuracy on the grammatical hierarchy must exceed twice the 1-in-6 floor (>33%), online.

**Answer: MET.** `bcpl::learn` reaches ~46% next-role top-1 over 1.05M online positions, floor 17% (2.7×). Exact identifiers stay unknowable (the open stratum, docs/157) — correctly, because a never-seen name cannot be divined; their *role* is learned. Grammar roles and keywords come straight from Crick 2004 (App. C/D). This answers "can it learn structure" — yes — and nothing more; it is not the product.

## Q2 — Does meaning live in the concept hierarchy, top-down?

Predict a token not from flat neighbours but from its **path of concepts** (field → subfield → construct → token), outer levels conditioning inner.

**Condition of satisfaction (fixed):** prediction using the scope/concept path must beat prediction from the flat window at the same budget — i.e. the hierarchy must *add* information a flat n-gram does not. Measured by `read_levels` / the cast (docs/126), not a single deep n-gram.

**Answer: OPEN — this is the next build.** The machinery exists (`ScopeTree`, `cast_under`, `read_levels`); the task is to run it over the deep BCPL/concept paths and show the lift over flat. The depth is there to use (328 levels); the single deep n-gram over-specialises, which is the evidence that the *backing-off hierarchy* is the right reader, not a flat one.

## Q3 — Can it learn a PERSON on the fly, from a little of their corpus?

The friend's demo, and the product: feed a person's words; it picks up their word-choices and preferences from a little; it keeps learning on the fly; you talk to it (the circus) and query it (the Lisp operators).

**Condition of satisfaction (fixed):** after a *small* slice of a person's corpus, the persona's predicted word-choices must match that person's later choices above the general-frequency baseline — and must keep improving as more is fed, with **no retraining** (each new text is `tell`ed, counts update in place).

**Answer: MECHANISM EXISTS, DEMO PENDING.** `Being::tell` is incremental and in place; `Knowing::learn_new` appends without re-reading; the circus talks; the Lisp asks. What is owed is the measured demo: teach a Being a person's corpus in chunks, show word-choice pickup after one chunk, show it rising chunk over chunk, no retrain. On the user's machine the corpus is quested from a real person via ollama; in-sandbox it is any text slice.

## The through-line

Q1 proved structure is learnable. Q2 is the real claim — meaning is the concept path, top-down, backing off — and is the next build on existing machinery. Q3 is the product — on-the-fly person learning — whose mechanism is already in place and needs its measured demo. BCPL was the grandfather that proved the structure is small and learnable (docs/166, 2% open stratum); it is not the destination.

## Status

Questions and conditions of satisfaction fixed 2026-10-24. `bcpl` modularised: `code::bcpl::{role, learn}` (the verb on the noun), `read_bodies` the one shared reader, `learn_bcpl` a thin runner. Q1 met and parked; Q2 and Q3 are the work, in that order.
