# 70 - The Open Issues, Closed or Measured

> _Everything Paper 1 §9 and docs/69 §5 listed as open, taken in order._

`cargo run --release --bin phinet [corpus] 20000 4` on `data/rust_book_corpus.txt` and `data/prose_corpus.txt` (WikiText-2), one seed. All other numbers from `cargo test --release`.

---

## 1. The operator on the two-sided lexicon — which side should the reader see?

Format v5 gave every channel a source side and a target side (docs/69). The LM harness and every operator number in docs/65–68 encoded source sides only. `FunctionSpace::from_facet_sides` now offers four readings: **source** (as before), **target**, **both** (128 samples, the circle walked twice), and **directed** — contexts built from source sides, the table a word is ranked against holding target sides, which is what a relation *is* under the split.

Next-word ranking, held-out, MRR ÷ chance:

```
                           Rust Book (pool 778)              WikiText-2 (pool 2,240)
reading            identity  pointwise  operator     identity  pointwise  operator
one side (v4)          —         —         —            6.7x     18.9x      27.9x   (docs/65)
source                2.0x      4.1x     13.1x           4.7x     12.2x      26.5x
target                3.2x      3.2x     12.4x           5.5x      8.7x      29.0x
both (128)            2.6x      3.7x      9.8x           5.5x     10.7x      27.9x
directed             4.5x      5.0x     13.6x          15.5x     18.2x      31.9x
```

Three things:

* **The directed reading is the right one.** With no operator at all, ranking the target sides against a source-side context reads 15.5× chance on WikiText-2 — more than double the best identity reading the one-sided lexicon ever had (6.7×) and more than three times the source-only reading of the same two-sided lexicon (4.7×). The split did not just fix the collapse; it made the lexicon's own algebra predictive.
* **The operator gains less than the identity does.** 27.9× → 31.9× on WikiText-2, 13.1× → 13.6× on the Rust Book. The operator was already doing, approximately, what the directed reading does exactly: turning a context into "the side a next word should present". Given the right sides it has less left to learn.
* **Reading the source side of a two-sided lexicon is worse than reading a one-sided one** (4.7× vs 6.7× identity on WikiText-2). A one-sided phase was doing both jobs badly; a source side does one job, and asking it to do the other is a category error. Any code that compares a context to a word's source side should now be read as a bug.

Walking the circle twice (128 samples) buys nothing and doubles the parameters. The reader in `command/generate` uses `Sides::Directed`.

## 2. The operator is in `generate.rs`

`OperatorGuide` (`src/generate.rs`): loads a `.fno`, encodes the loaded facet with a chosen `Sides`, refuses a lexicon without relation evidence or an operator whose space does not match, keeps the prompt-plus-emitted history, and caches one operator application per history length. `Generator::with_operator` attaches it; `score_candidates` multiplies each candidate's score by `(1 − w) + w·(1 + cos)/2` with `OPERATOR_WEIGHT = 0.5`, so the operator can halve a candidate and never zero it. `command generate` picks up `data/phinet_generate.fno` when present (`OPERATOR_FILE`). Beam search resets the guide on the prompt only; per-hypothesis histories are the next step. Unit-tested for the bound and the history; **not yet measured** for generation quality — that needs an evidenced manifold, and the shipped `manifold.chroma` is v1.

## 3. `band_n` is a counter; the quantum number is its clamp

The trainer increments `band_n` on every converged channel-0 step, so on the shipped manifold it reaches 1,845,381. `SpectralPhasor::effective_phase` already clamped it to `BAND_N_EFFECTIVE_MAX = 13` (one sector of fine-structure shift); Phical and Phiton did not, and `Phiton::phase_to_wavelength` pushed every such word to 750 nm. `Phical::quantum(band_n) = min(band_n, 13)` now sits under every Phical and Phiton formula (`effective_phase`, `phase_to_wavelength`, `color_space_time_distance`, `oscillation_energy`, `ColorSpaceTimePoint::from_phase`, `bind_to_node`). Test: a count of 13 and a count of 1.8 million are the same point.

## 4. Evidence counts past 65,535 — format v6

*the* saturated its offset+1 count after two passes. Evidence lanes are 32-bit (`EVIDENCE_WORDS = 32`, `EVIDENCE_MAX = u32::MAX`); v4 and v5 files read back through `widen_evidence`, losslessly (tested). `FORMAT_VERSION = 6`.

## 5. `cosine_table` on all cores — done in phinet (docs/69 §4)

86 → 51 ms per step on two cores at `[20] × [6,221] × 64`. The WikiText-2 run at the mixture objective is still the open experiment; it is now cheaper per core, not free, and the directed reading above is the lexicon it should use.

## 6. Stale `phiano/src/phinet/{tensor,operator}.rs`

Never compiled (not in `mod.rs`). The two files are now three-line stubs saying so and asking to be `git rm`'d; the tools here cannot delete on the target machine.

## 7. The Q64 half-step, the standing-wave node, `phi-cal` — done in docs/69

## 8. Beam search and the operator

Open, and small: `Hypothesis` needs its own history for the guide. Listed rather than hidden.

---

## What changed in the code

| where | what |
|---|---|
| `src/phinet/space.rs` | `Sides`, `encode_side`, `encode_both`, `FunctionSpace::from_facet_sides`, `table()`, `space()` |
| `src/bin/phinet.rs` | the four readings, space-generic config |
| `src/generate.rs` | `OperatorGuide`, `OPERATOR_WEIGHT`, `Generator::with_operator`, operator factor in `score_candidates` |
| `src/command/generate.rs`, `src/config/constants.rs` | `OPERATOR_FILE`, directed guide when present |
| `src/phical/mod.rs`, `src/phiton/mod.rs`, `src/phical/operators.rs` | `Phical::quantum` under every formula |
| `src/phasor.rs`, `src/storage.rs` | 32-bit evidence, `widen_evidence`, `DiskPhasorV6`, v6 read/write |
