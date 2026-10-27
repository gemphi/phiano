# 202 · The crowd — three readouts, three negatives, and what they prove

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

docs/200 left one blocker: the 2,000 most-told words — the closed stratum, where intentions live (docs/201) — are one type under the plain channel. This round tried to type them from their own phasors, three ways, each measured on the 300 most-told words of the English guide (`examples/crowd.rs`, `Ecs::profile`). Condition fixed in advance: the crowd parts into groups whose members read as one class.

| readout | what it reads | result |
|---|---|---|
| one angle under `next>` (docs/200) | where the word is reached | 2,062 words in one sector: the mean |
| sixteen angles, one per answer slot of `next`, quantised to 4 sectors | which slots it speaks on and where | 236 profiles for 300 words, 217 singletons; the largest group `i you was 3 not 5 she 7 b 8 if say didn't` — nearly every word carries the same glyphs `2123033332132322`, differing in one slot: the slot angles crowd too |
| the DIFFERENCE from the crowd's mean on each slot (ahead / behind / with / silent, dead band 0.25 and 0.5 turns) | where it stands against the others, per slot | singletons almost throughout; at 0.5 the largest group is `that if 9 12` |

**NOT MET, three times.** Not tuned further.

## What the negatives prove

A content word's phasor is a type on its own because it is reached from a few places: `accident`, `airport`, `street` sit together (docs/200) with nobody naming the class. A closed-stratum word's phasor is NOT a type on its own, under any readout, because it is reached from everywhere — and that is not a defect of the readout. `that` is one thing after `know` and another before `book`; `to` is one thing after `want` and another before `the`. These words have a type PER CONDITION, never standalone, which is also what grammar has always said of them: function words are defined by position, not by substance.

So the information the intention rung needs is not in a word's own phasor; it is in the PAIR — the type of a word given the type of what came before it — which is exactly what the `after <word>` channels hold and what the arena already reads deepest-first (docs/170). The template of a sentence over the closed stratum must therefore be a sequence of TRANSITIONS (type | previous type), not of unigram types; and the crowd's 1,024 templates are shapes of transitions.

## Status

2026-10-27. `examples/crowd.rs`, `Ecs::profile`, `phiecs::world::{SLOTS, TURN}` exported. Default read unchanged; nothing switched on. Next: the template as transitions — each position typed by (its type, the previous word's type) read off the deep channel — and the intention purity re-measured against the 87.8% baseline.
