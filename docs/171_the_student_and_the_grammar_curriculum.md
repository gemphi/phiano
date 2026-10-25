# 171 · The student: the grammar book as the concept-hierarchy curriculum

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The find

`docs/papers/MUCLecture_2022_5217521.pdf` is Raymond Murphy's *English Grammar in Use* (Cambridge, 5th ed.) — 145 units, the world's best-selling grammar book. It is already the thing Q2 (docs/170) asked for:

- **The table of contents is the top-down concept hierarchy.** Present and past → Present continuous → the construction *I am doing*. Present perfect, Future, Modals, Articles, Prepositions, Phrasal verbs — categories over units over constructions. This is the concept tree a word's meaning is a path through.
- **Each unit teaches one pattern by example.** Unit 1: *am/is/are + -ing*, with "She's driving", "I'm trying", "isn't raining". The bolded sentences are the evidence.
- **The exercises are the practice questions, with an answer Key.** The condition of satisfaction is built in: a unit is mastered iff its practice questions are answered. e.g. Unit 1, ex 1.4 #2: *"Let's go out now. ___ (it / rain) any more."* → Key: *It isn't raining.*

## The method (the user's: plan, assess, learn a little, confirm, move on)

1. **Plan** — the curriculum is the ToC. Each unit is a concept node with a path (`present-and-past/present-continuous`).
2. **Assess** — the condition of satisfaction per unit is its practice questions and their Key. Fixed before learning.
3. **Learn a little** — feed the unit's handful of example sentences under its concept scope; read the pattern off them (do not tell the rule).
4. **Confirm** — answer the practice question *from the concept*, check against the Key.
5. **Move on** — next unit. Master one, keep going (incremental, on the fly).

## The great start (run today)

`examples/learn_grammar.rs` runs two units of this exactly:

```text
Unit present-continuous: learned from 6 examples → rule: be + V-ing   (be+V-ing in 100% of them)
Unit present-simple:     learned from 6 examples → rule: V (base/-s)   (be+V-ing in 0%)

Practice (Unit 1 ex 1.4 #2): "Let's go out now. ___ (it / rain) any more."
  student answers: it is not raining  →  "it isn't raining"  (key: It isn't raining) ✓

Same verb 'work', resolved by CONCEPT:
  present-continuous → "is working"
  present-simple     → "work"
```

Six examples per unit was enough to pass the practice question — "learn a little, master, move on". And the last two lines are Q2 proven on a real curriculum: **the same word resolves to a different form by its concept** — meaning is the path, not the word.

## The code to utilise, by situation

- A unit's examples → rows: `phiplm::prose::paragraph` (sentences under a scope path = the concept hierarchy); the scope *is* the concept node.
- Learn them into a persona on the fly: `phiano::sys::Being::tell` (incremental, no retrain).
- Answer a practice question conditioned on the concept: the cast — `phiplm::relate::{ScopeTree, cast_under}` / `read_levels` (docs/126) — resolves a word under its scope path, which is exactly "read the form from the concept, not the word".
- Query/talk: `phiano::sys::lisp` operators (`(ask student rain under present-continuous)`), the circus.

## Status

Found and planned 2026-10-24. `learn_grammar` demonstrates the student on Units 1–2; the full curriculum is the next corpus to ingest (the 145 units as concept nodes), and the real reader for it is the cast over the concept scopes (Q2), not the by-hand pattern in the demo. The demo proves the shape; the cast does it at scale.
