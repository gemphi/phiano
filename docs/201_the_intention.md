# 201 · The intention — the ladder above the sentence, measured, and where it ends

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

The ruling: the hierarchy does not end at the sentence — word → sentence → paragraph → chapter → INTENTION → the compiler. The English guide hands over ground truth for the top rung for free: a unit's prose EXPLAINS (16,728 sentences), an exercise line INSTRUCTS (893), the Key ANSWERS (1,437). So an intention discovered from the types alone can be checked against what the text was for (`examples/intent.rs`). Condition fixed before the run: the discovered classes beat the majority-label baseline by ≥5 points of purity.

## Measured

| signature of a sentence | K | signatures | purity | baseline |
|---|---|---|---|---|
| (open, open, close, short) | 8 | 106 | 87.8% | 87.8% |
| (three opening types, short) | 16 | 192 | 87.8% | 87.8% |

**No effect, twice.** The commonest signature, `(6, 6, 6)`, holds 7,985 explain, 836 answer and 376 instruct together — because `complete`, `write`, `put`, `choose` occur often enough in a grammar book to be crowd words, and the crowd is one type under the plain channel (docs/200). An intention lives in the closed stratum, and the closed stratum is exactly what the plain channel cannot type. Not tuned further: the fix is the one already named — type the crowd under its deep channels — not another signature.

| level | alphabet | distinct shapes | recurrence |
|---|---|---|---|
| sentence (docs/200) | 16 types | 5,915 | top-1,024 cover 76% |
| unit, as a raw string of its sentences' type-sequences | type strings | 143 of 143 | none |

The chapter rung tells its own lesson: a unit as a raw string of type-sequences never recurs, so the paragraph level needs its OWN alphabet — the 1,024 kept sentence templates as its symbols — before it can be compressed. Each level's alphabet is the level below's kept templates. That is the hierarchy, and it is BCPL's: tokens are the alphabet of the tree, tree kinds the alphabet of OCODE, OCODE the alphabet of the machine — one ladder (`phiclm::manifest`), climbed a rung at a time.

## Where the ladder ends: the compiler

Each intention is a kind on the ladder — explain = `s_ass` (telling), instruct = `s_rtap` (a routine call: doing), answer = `s_resultis` — and every instruction becomes a program: its template counted into a vector, summed, judged. **893 of 893 instructions compiled and ran** (372 steps each, mean), every result equal to the sentence's length. The pipe from a sentence's shape to a judged program is real and closed; what it carries today is only the shape. What it must carry next is the intention's own arguments — which grammar, which sentences — read off the deep channels once the crowd is typed.

## Status

2026-10-27. `examples/intent.rs` (ground-truth labels from the guide's layout, the signature purity, the unit shapes, the instruction → program → judge pipe). Default read unchanged. Next, in this order: (1) type the crowd under the `after <word>` channels (the deep read the arena already walks); (2) re-measure intention purity; (3) the paragraph alphabet = the kept sentence templates.
