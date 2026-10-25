# 189 · Two measurements on the phasor engine, both kept

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. Summing the levels: NOT MET, reverted

docs/188 proposed reading the three condition levels together — each candidate's agreement × amplitude summed across the word-before channel, the family channel and next — so a confident deep channel would outvote a crowded one. Measured on the same arena: 20.7% against deepest-first's 24.3%, and the decline back to −7.8 points. The crowded shallow channel casts many small positive votes and swamps the deep channel's one confident answer. Reverted to deepest-first; the result and the reason are kept in the reader's own comment.

## 2. The person on phasors

examples/person.rs now carries the phiecs reader beside the two count lines, and its answers are the phasors' answers. Crick's dissertation, five chunks, the same positions for all three:

| reader | first → last chunk | overall |
|---|---|---|
| counts | 12.3 → 20.6% | 17.3% |
| concepts on counts | 12.1 → 21.0% | 17.2% |
| phasors, read under the condition | 6.3 → 10.2% | 8.6% |

The phasor reader learns the person and keeps rising with nothing retrained (+3.9 points; the condition of docs/186 is met on its own terms), at half the counts' top-1. Its voice says so too: the front end → "the front end for bcpl …" holds, because the word-before channel has seen that context; past it the continuation degrades to function-word noise where the shallow channels answer. The engine knows the person's constructions and loses the open stratum.

## What these two say together

The model is right in kind and short of capacity in one specific place: ONE ANGLE per word per channel is the circular mean of every successor that word ever had on that channel. Conditions multiply the channels (docs/188, +100%); they cannot give a word two answers on one channel. docs/90 §2 already names the mechanism: ANSWER SLOTS — a kind owning n extension channels with each answer living on the slot its hash picks, so a word is read on a slot only where it was trained on it and cat · is-a no longer ties mammal with allergies. That is the next rung, with its condition: on the arena, the phasor reader's decline stops and its top-1 closes on the counts; on the person, the continuation holds past the first construction.

## Status

2026-10-25. arena ecs.rs reverted to deepest-first with the summing result recorded; person.rs on phasors; workspace 33 test binaries, 0 failures.
