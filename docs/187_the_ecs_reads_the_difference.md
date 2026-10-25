# 187 · Nothing learns from counts — the ECS reads the difference

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The correction

The user's words: nothing ever learns from counts; counts are extract. What is to be measured is the DIFFERENCE under what CONDITIONS, updating all the phasors and the existing engines all the time, structured by operators like CPL. A word is a shape formed by the words it associates with — a force that binds — but that is not meaning; meaning is in the why. The arena's readers and the person demo had drifted back to count tables, and the ECS itself had the defect in plain sight: relate_by MOVED the phasors and calls_for read the edge table. The movement was written and never read. docs/176's phiecs IS the counts was that defect, reported as a result.

## The fix, in phiecs

World::reads(word, kind) — the read FROM THE PHASORS: the words standing nearest one TURN past the word on that kind's channel, each scored by agreement there times amplitude. A relation is a turn (TURN = an eighth of the circle): from ⇒ to puts to one turn past from, so the relation is a rotation applied — an operator, as in CPL — not a pair remembered. Coupling is directed and sequential (from moves toward one turn before to; to toward one turn past where from now stands), so after a single tell the pair agrees exactly. Phasor::couple steps by max(rate, 1/(n+1)): the first hearing goes all the way, later hearings average, and the floor keeps it moving for ever — update all the phasors, all the time. A word is read from under a kind only where it has SPOKEN there (source evidence kept apart from target evidence): being pointed at gives an angle, not a voice — a Being no longer fabricates about a word it only heard named. Kinds declared symmetric (near, with, like, …) turn by zero: the two sides of a smile stand at the same angle. Agent::ask and phikit's Watch::follows now read the phasors; the edge table remains the record only.

## Measured, straight (arena, 1,500 sentences, 13,735 scored positions, floor 0.04%)

| reader | top-1 | first → last |
|---|---|---|
| phiplm concepts (counts under the family) | 32.8% | 35.4 → 33.7% |
| phiplm counts | 29.9% | 33.1 → 29.9% |
| phiecs phasors, one next channel | 12.0% | 8.9 → 6.1% |
| phiecs phasors, read UNDER THE CONDITION (48 family channels) | **18.6%** | 21.0 → 12.0% |
| phifno Joint operator (batch) | 16.6% | 19.5 → 16.1% |
| phical conjugate | 0.7% | — |

The phasor engine learns — 300× the floor on one channel, 465× under conditions, and conditioning alone lifted it by half — and it reads the difference, not a tally. It also sits below the count readers and DECLINES as more is fed: one angle per word per channel is the running mean of everything the word was ever followed by, and crowds. Forty-eight family channels relieve it and do not remove it. This is not a reason to return to counts; it is the capacity question stated exactly: the condition needs its own channel, and 64 channels is the base, not the bound.

## Next, fixed

Conditions as EXTENSION channels — docs/90's channels past the base 64, recovered into the build in round 177, exist for precisely this: a word pays for the conditions it touches, unbounded. Couple under (kind, family) on an extension channel instead of competing for 48; the user's fractal 16–64 kinds per word scaling to 1024 is this mechanism. Condition of satisfaction: the phasor reader's curve stops declining and its top-1 rises past the batch operator's 16.6% toward the counts, same arena, same positions.

## Status

2026-10-25. phiecs relation/phasor/world changed; Agent::ask, phikit Watch, arena Ecs read phasors; tests updated to the turn (three count-era assertions corrected, each noted in place). Workspace 33 test binaries, 0 failures.
