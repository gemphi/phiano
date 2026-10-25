# 183 · The coupled scribe — minting and filling in one run

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The rung

docs/181 ended on a split: token wandering MINTS new skeletons, mold substitution FILLS proven ones at scale, and each arm ran alone. This rung couples them: one scribe, one memory, each write drawn from the molds four times in five and from the token memory once in five — so a wandering discovery becomes a mold mid-run and the filling channel takes it to scale at once.

## Measured — and measured properly

A single run's counts swing with the seed (the token arm gave 72 new skeletons on one seed and 9 on another), so a swinging number proves nothing: every figure below is the mean of five seeded runs of 10,000 rounds, with its range. 3.5 seconds for all twenty runs.

| arm | ran | distinct kept (range) | new skeletons (range) |
|---|---|---|---|
| soup | 0.0% | 0 | 0 |
| token | 6.3% | 22 (9–44) | 21 (8–43) |
| mold | 76.6% | 2,576 (2,271–2,751) | 24 (21–31) |
| scribe (coupled) | 62.0% | 2,563 (2,318–3,205) | **37 (34–44)** |

The coupling is real: the scribe's skeleton range (34–44) does not overlap the mold's (21–31) — structural minting accelerated by half again — while program volume holds equal and the best single run (3,205) is the best of the table. Against the token specialist, the scribe mints as much or more with a hundred times the programs. The price is stated: fifteen points of raw run rate, the tax the wandering fifth pays in failed writes.

## Against the fixed condition

docs/182's bar was both counts accelerating past round 181's lines. Skeletons: MET, clearly. Programs: NOT accelerated — equal within range. PARTIAL, reported straight. The volume ceiling is the mold set's filling capacity, not the coupling; raising it means richer hole pools and molds minted from the quest's answers, which is where this loop meets the ollama rung.

## Status

2026-10-24. writer example at 153 lines, four arms, five seeds; workspace 33 test binaries, 0 failures. `cargo run --release -p phiano --example writer -- 10000`.
