# 190 · Answer slots — the phasor engine takes the arena

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The build

docs/90 §2's answer slots, in phiecs: a kind owns SLOTS = 16 extension channels (kind#0 … kind#15); when from ⇒ to is told, besides the plain channel, from speaks and to stands one turn past it on the slot the answer's own id scatters to (phistd::mix64 of the word id). Reading from under the kind: for each slot the word has SPOKEN on, the target is one turn past it there, and a candidate is scored on its own slot only; the plain channel answers only when the word has spoken on no slot. So two answers to one word no longer average unless they share a slot — one angle per word per channel stopped being the mean of everything. Relation widened to u32 and the Register indexed by a HashMap so tens of thousands of condition channels cost O(1) to declare and find.

## Arena — the phasors lead (same 13,735 positions, floor 0.04%)

| model | top-1 | first → last |
|---|---|---|
| **phiecs phasors: word before → family → next, with slots** | **32.9%** | **36.6** → 32.3% |
| phiplm concepts on counts | 32.8% | 35.4 → 33.7% |
| phiplm counts | 29.9% | 33.1 → 29.9% |
| phifno Joint operator (batch) | 16.6% | 19.5 → 16.1% |
| phical conjugate | 0.7% | — |

The line across four rounds: 12.0 (one channel) → 18.6 (family) → 21.6 (unbounded channels) → 24.3 (word before) → **32.9 (slots)**. Nothing in it is a count; every step was one structural change, measured. Against docs/189's condition: closes on the counts — EXCEEDED, it passed them; the decline stops — NOT yet (−4.3, from −5.1). The first slice, 36.6%, is the highest any reader has scored here.

## The person — still behind, stated

| corpus | counts | concepts | phasors with slots |
|---|---|---|---|
| Crick (5 chunks) | 12.3 → 20.6% | 12.1 → 21.0% | 9.7 → 15.8% (overall 13.5) |
| Murphy (10 chunks) | 16.7 → 14.3% | 19.5 → 18.2% | 14.9 → ~13% |

Rising, learning the person with nothing retrained — and below the counts, where on the glosses it is above them. The reason is the corpus, not the reader: glosses repeat their contexts, prose does not, so prose falls more often from the deep channel to the crowded ones. The voice says it: the compiler → "the compiler is the to use the flex and bison" — the construction held (Crick did use flex and bison), the open stratum after it wandered.

## Status and next

2026-10-25. phiecs relation (u32, indexed), world/relate (slots). 33 test binaries, 0 failures. Next, with its condition: the decline and the prose gap are the same thing — the fallback channels; the quest on the machine supplies the contexts prose lacks (uses of what the person is unsure of), and the person's continuation must then hold past the first construction.
