# 176 · The arena — many models, one run

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## Why an arena

We have many models and were measuring them in separate runs — different streams, different floors, different luck — which cannot be compared. `phiplm::study::Arena` reads ONE stream and asks every model the same question at the same position **before any of them is told it**, so the table says which model is learning better rather than which run was easier. It is the one honest protocol of docs/172 (predict first, learn after) with several readers instead of one.

`Reader` is the whole contract: `name`, `guess(said)`, `tell(words)`. Each of Phient's models enters as one reader, in `phiano::arena`, one file each, no model's code mixed into another's:

| reader | the model it stands for |
|---|---|
| `count::Count` | phiplm's counts — the next word from the word before, nothing else; the line to beat |
| `concept::Concept` | phiplm's concepts — the construction family the words said show, backing off onto the counts (docs/174–175) |
| `ecs::Ecs` | phiecs — the words told as rows into a `World`, read by what the last word calls for |
| `mirror::Mirror` | phical's conjugate — the relation `next` as an all-pass filter on the circle, read by correlating `H·a_w` against every word (docs/153) |

## The run

3,000 sentences of real English prose, 4,510 words of vocabulary, **27,607 prequential positions**, floor 0.02%, 10 seconds.

| model | top-1 | first → last slice | lift | verdict |
|---|---|---|---|---|
| phiplm concepts (the family, backing off) | **32.7%** | 31.1% → 33.9% | +2.7 | learning |
| phiplm counts (the word before) | 30.3% | 29.4% → 32.2% | +2.8 | learning |
| phiecs (rows into a World) | 30.3% | 29.4% → 32.2% | +2.8 | learning |
| phical conjugate (the relation as a filter) | 0.4% | 0.7% → 0.4% | −0.3 | saturated |

What one run says that four runs could not:

**The concept hierarchy earns its keep — +2.4 points over the flat counts at the same budget, on the same positions, both still rising.** This is Q2's condition (docs/170) met at scale, and it flips the toy negative of docs/175, whose cause (families holding two sentences) is now confirmed rather than argued.

**phiecs is exactly the counts.** 30.3% and the same curve to the decimal: the `World`'s edges and weights, told the same rows, are the same model as a count table. That is a correctness result — the Row → World loop loses nothing — and a warning: phiecs adds no predictive power of its own at this reading, so it should be measured for what it is for (structure, aspects, quests), not as a rival reader.

**phical's filter is 20× the floor and nowhere near counting** (0.4% against 0.02%), on an online budget of two epochs per sixty-four lines over a sampled pool. It is learning something — a filter trained on a stream it never sees twice — and it is not competitive with counting when the vocabulary is open. Reported straight; the filter's own strength (reading a relation *backwards* with no inverse, docs/153) is not what top-1 next-word measures. phical's advantage is stated in the table too: it is given the vocabulary in advance because a word is a spectrum it owns a slot for, while the counting readers are given nothing.

**phifno is not in the table.** The Fourier neural operator is batch-trained over a table of functions; it cannot guess-then-learn position by position at this budget without a batch lane in the arena (train on the stream's first half, read the second). That lane is the next rung — a fabricated online number for it would be worth nothing.

## Also this round

Plural type names removed workspace-wide, 71 files: `Moments`→`Moment`, `Diagnostics`→`Diagnostic`, `BaselineScores`→`BaselineScore`, `Baselines`→`BaselineSet`, `EpochMetrics`→`EpochMetric`, `ArcResults`→`ArcResult`, `TrainingMetrics`→`TrainingMetric`, `NgramStats`→`NgramStat`, `KillerWords`→`KillerWord`, `PhicalOps`→`PhicalOp`, `Stats`→`Stat`, `Roles`→`RoleTable`, `LexicalRules`→`LexicalRule`, `SearleMarkers`→`SearleMarker`, `FelicityConditions`→`FelicityCondition`, `CandidateFeatures`→`CandidateFeature`, `GenWeights`→`GenWeight`, `Readings`→`ReadingSet`.

## Status

2026-10-24. `phiplm::study::Arena` + `Reader` (tested), `phiano::arena` with four readers, `examples/arena.rs`. Workspace: 31 test binaries ok, 0 failures, every example builds. Run it with `cargo run --release -p phiano --example arena -- <dir of .txt> [--lines 3000]`.
