# 69 - Two Sides to Every Channel

> _The collapse `bin/trace` found, the fix, and what the connected crates gained._

`cargo run --release --bin trace [corpus] [passes]` and `--bin promoted` on `data/rust_book_corpus.txt`, one seed.

---

## 1. The defect

`docs/64` reported offset-channel dispersion 0.85–0.93 and called the channels healthy. By frequency tier they are not:

```
words by frequency     offset+1 (2 passes)   offset+1 (4 passes)   'of' (2)   'of' (4)
top 100                      0.003                 0.003              0.495      0.315
100–500                      0.026                 0.016              0.826      0.670
500–2,000                    0.723                 0.609              0.947      0.895
2,000–6,165                  0.890                 0.770              0.978      0.980
```

The hundred most frequent words sit at one angle on every offset channel and the collapse spreads down the ranking with passes. The same-channel repulsion keeps the tail spread and the average honest; it never touches the head, because the negative pool caps any word at eight copies and only the negative moves.

Drawing negatives by `count^0.75` (uncapped) and pushing both words: top-100 0.003 → 0.004. Not the fix.

**Why.** One phase per word per channel makes "a then b" and "b then a" the same constraint. Attraction on every adjacent pair is then a Kuramoto system on the connected graph of co-occurrence, and its fixed point is total synchronisation; the frequent words, adjacent to everything, get there first. Repulsion of any strength only slows it.

## 2. The fix: a source side and a target side

`SpectralPhasor` now carries `tgt: [u64; 8]` next to `packed`: the angle a word presents when it is the word *related to*, as `packed` is the angle it presents when it is the word *doing the relating*. `couple_relations` moves the earlier word's source side toward the later word's target side and nothing else; `on(a, b, k)` is `cos(src_a[k] − tgt_b[k])` and is directed; `similarity_with` compares source with source and target with target. Format v5; a v4 file reads back with both sides equal, which is the state a new phasor starts in. Word2vec's input/output vectors, on the circle.

With the split and the `count^0.75` negatives together:

```
words by frequency     offset+1 src (2)   tgt (2)   src (4)   tgt (4)   'of' src (2)   'of' src (4)
top 100                     0.391          0.562     0.383     0.541        0.896          0.743
100–500                     0.329          0.917     0.275     0.873        0.963          0.874
500–2,000                   0.787          0.940     0.634     0.906        0.938          0.921
2,000–6,165                 0.966          0.963     0.935     0.905        0.948          0.939
```

The split alone (capped pool) gave 0.082 / 0.072 at the top; the negatives alone gave 0.004. Both together hold the head at 0.4–0.6 across passes. The frequent words' *source* sides remain the most similar tier — they genuinely share successors — and that is now a fact about the corpus rather than an artefact of the rule.

## 3. What it cost and what it bought (`bin/promoted`)

```
                                           before (one side)    after (two sides)
control, dense resonance (relations off)        44.5%                 44.5%
promoted, dense resonance                       37.1%                 37.0%
promoted, evidence-weighted, balanced           51.8%                 51.8%
promoted, evidence-weighted, raw                31.4%                 36.8%
promoted, best single relation                  22.6%                 27.0%
composition, family channel, vs null             4.1x                  3.4x
composition, pair's channel, vs null             0.9x                  2.1x
    antonym / comparative, family channel     1.9x / 2.6x            9.7x / 8.2x
the / a similarity                             +0.772                +0.400
borrow / ownership                             -0.195                -0.157
```

The analogy `a:b :: c:?` is now read directed — `tgt_d = src_c + (tgt_b − src_a)` — which is what a relation *is* under the split. The pair's own channel goes from null to 2.1×; the family channel gives up 0.7× overall and gains 4–5× on antonyms and comparatives. `the/a` at +0.40 instead of +0.77 is the collapse leaving. Balanced similarity does not move.

Not re-measured after the split: the operator numbers in `docs/65–68` (the LM harness encodes source sides only). They are still the numbers for a one-sided lexicon.

## 4. The connected crates

| crate | what was missing | what is there now |
|---|---|---|
| `phinet` | `cosine_table` on one core (7 min/epoch mixture) | rayon over rows, forward and backward: 86 → 51 ms per step on two cores at `[20]×[6221]×64` |
| `phinet` | an exact per-channel algebra | `Tape::mobius(x, abcd)`: `w = (az+b)/(cz+d)`, gradients in `z` and all four coefficients finite-difference checked; identity and rotation (= bind) tested |
| `phinet` | a spectral prior | `zeta_weights(modes, s)` with `Fno::scale_modes` |
| `phiace/crates/phical` | empty (`lib.rs` 0 bytes) | `phi-cal`: `sphere` (stereo/unstereo, evidence as latitude, Fibonacci lattice), `mobius` (matrix group), `q64` (juncture with bound), `winding`, `heat` (circle heat flow, overlap after diffusion); 13 tests |
| `phiano` | `is_standing_wave_node` constant-true | nodes are golden-angle lattice points; tested not-everywhere |
| `phiano` | `Hexagram::phase_angle` without the half-step | `2π(q+½)/64`; a test proves it is the same map as `phi_cal::q64::lift` |
| `phiano` | persona distance blind to sector adjacency | `Fingerprint::overlap_after(other, t)` (heat flow), `Fingerprint::q64()` |
| `phiano` | — | depends on `phi-cal`; `phiano::phical::exact` re-exports it |

One claim corrected: the resonance loss under double quantisation is bounded per channel by π/32 ≈ 0.098 (the cosine is 1-Lipschitz), not by `1 − cos(π/32)`; the 64-channel mean loss on random pairs measures under 0.01. The test states the true bound.

## 5. Still open

* Operator numbers on the two-sided lexicon (and whether the LM reader should see both sides).
* `evid` saturates at 65,535 and *the* hits it.
* WikiText-2 at the mixture objective — the parallel table makes it a tenth cheaper per core, not free.
* Stale `phiano/src/phinet/{tensor,operator}.rs` on disk.
