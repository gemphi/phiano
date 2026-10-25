# 191 · The teacher — the quest loop, complete and measured null

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The build

examples/teacher.rs: the person learned in chunks on the phasors (docs/186, docs/190); after each chunk the learner names the constructions it is UNSURE of (shown once, not yet a family — Knowing::unsure, docs/174), asks a teacher for uses of exactly those (family_prompt), and the answers are told into the World like anything else. Two arms, same chunks, same positions, scored before each chunk is told: alone, and taught. The teacher never sees a scored sentence. The teacher is any phiecs Oracle: --oracle ollama:<model> uses Spawned::ollama; without it the RESERVE — the last fifth of the corpus, held out of the stream — answers the same prompts with the person's own sentences. One code path, two teachers.

## Measured

| corpus | asked | uses told | phasors alone | phasors taught | counts (contrast) | lift |
|---|---|---|---|---|---|---|
| Crick, 6 chunks, 30 asks/chunk | 180 | 10 | 13.1% | 13.0% | 16.7% | −0.1 |
| Murphy, 8 chunks, 60 asks/chunk | 480 | 52 | 14.3% | 14.4% | 17.7% | +0.0 |

NO EFFECT, reported straight. The cause is the dose, not the loop: a reserve that is a fifth of a book rarely contains the exact pair asked about, so the teacher answered one ask in ten on Murphy and one in eighteen on Crick, and 52 sentences against 140,000 streamed words is 0.04% of the data. The mechanism is complete — ask, answer, tell, measure — and the sandbox has no teacher worth the name. The verdict line now says NO EFFECT for lifts inside ±0.5 instead of rounding a hair into MET.

## The run that answers the question

On the machine, with ollama, every ask returns three uses (1,440 for Murphy's 480 asks, more with --ask raised), generated for the construction rather than searched for:

    cargo run --release -p phiano --example teacher -- crates/phiano/data/person/murphy.txt --chunks 8 --ask 60 --oracle ollama:phi4

Condition unchanged: taught beats alone by at least half a point on the same unseen positions, and the continuation holds past the first construction. If it does not, the quest is not the fix for the prose gap, and that is worth knowing as much as the other way.

## Status

2026-10-25. teacher.rs (132 lines); Oracle re-exported path phiecs::quest::{Oracle, Spawned}. Workspace 33 test binaries, 0 failures.
