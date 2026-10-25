# 188 · Conditions as channels — the phasor engine climbs

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The build

Relation widened from a byte to u16. Register::declare no longer refuses past MAX_KINDS: the first 64 are the base channels every word carries densely; every kind past them is an EXTENSION channel (docs/90 §2 made real in phiecs). Phasor is now a dense base of 64 Slots plus a sparse map of extension Slots opened lazily at a seeded angle of the word's own — a word pays for the conditions it touches, and nothing else. A Slot is theta, amplitude, evidence, spoken. Everything reads through it: couple, agreement, resonance, kinds (base and extension alike).

The arena's ECS reader tells every pair three ways and reads them deepest first, each level answering only when the one below is silent: the word BEFORE the last as the condition (channel "after x" — depth, docs/170), then the construction family ("does not"), then plain next. No channel is rationed.

## Measured — the same arena, same 13,735 positions, floor 0.04%

| phasor reader, step by step | top-1 | first → last slice |
|---|---|---|
| one next channel (docs/187) | 12.0% | 8.9 → 6.1% |
| + family as condition, 48 channels (docs/187) | 18.6% | 21.0 → 12.0% |
| + unbounded extension channels | 21.6% | 24.0 → 16.1% |
| + the word before as the first condition | **24.3%** | 27.5 → 22.3% |
| phifno Joint operator (batch, contrast line) | 16.6% | 19.5 → 16.1% |
| phiplm counts (contrast line) | 29.9% | 33.1 → 29.9% |
| phiplm concepts, counts under the family (contrast line) | 32.8% | 35.4 → 33.7% |

Doubled from the start of this line, every slice now above the batch-trained operator, the decline halved (−9.0 → −5.1 points). Against docs/187's fixed condition: passes the operator toward the counts — MET; stops declining — NOT MET, reported straight. The residual decline is the backoff itself: where the deepest channel is silent (a context never seen), the family and next channels answer, and those are the crowded ones.

## Next, stated not tuned

Read all three levels together rather than first-silent-wins: each level's candidates scored by agreement × amplitude and summed, so a crowded channel is outvoted by a confident deep one instead of silencing it. Then re-run the person (docs/186) on this reader, and the conversation on the machine.

## Status

2026-10-25. phiecs relation/mod.rs, phasor.rs (rewritten, one test renamed to its new truth), arena ecs.rs. Workspace 33 test binaries, 0 failures.
