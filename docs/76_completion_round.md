# 76 · The completion round: what was open, what is now built, what the numbers say

> In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models — Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle (sphere) with a count of how often it has been played out; meaning is a side effect of pairs of words moving under a condition.

## Phinet (engine)

| was open (docs/75, phinet what-is-missing) | now | measured |
|---|---|---|
| cross-entropy over the table | `Tape::cross_entropy`, `Fno::train_epoch_softmax` | FD test; torch parity 0.0; learns the shift (test) |
| AdamW, optimiser state saved | `Adam::with_decay`, `AdamState`, `to_bytes_with` | torch AdamW parity 0.0 |
| version byte in `to_bytes` | format 2 (`PHINET` + version), v1 still loads; format 3 = f32 storage (`to_bytes_compact`, half size, < 1e-10 drift) | tests |
| slow `apply` | slice loops, chunked `apply` | 916 → 88 µs per function; hinge epoch 3.0 → 1.8 s |
| `Module` trait | `Module`, `Sequential` | test: gradient reaches both operators |
| resolution-free `R` | `Fno::at_resolution` | P32 → P64/P128 cosine 0.799 vs native 0.805 |
| **the phasor ranges** | `phinet::phasor`: P16–P128 × Q16–Q64, `Range::ALL` (12), `resample`, `quantise`, `sector_of`, `table`; Phiano `phical::ranges`, Phical Q64 ≡ `Quantum::Q64` (test) | loss 0.0064/0.0016/0.0004 for Q16/Q32/Q64 at every resolution; quanta nest 100% |
| 2-D torus | `Tape::fft2` / `ifft2`, `fft2_inplace` | Parseval, tone-in-one-bin, FD gradient, torch parity 1e-16 |
| lr schedule | `Schedule::{Constant, Cosine, Step}` on `Adam` | tests; saved in state |
| rustdoc published | `npm run rustdoc` → `/rustdoc/phinet/` | — |

29 tests, 62 torch parity checks, all pass. Still open in the engine: a real batched complex GEMM (the remaining ~10× on `apply`), f32 *compute*, PhiDoc search.

## Phiano

**Decoding from the operator** — `Decode::{Trigram, Operator{k}, Union{k}}` on `Generator`, `OperatorGuide::nearest`, `bin/decode_eval` (Austen, 40 prompts × 30 tokens, k = 32):

| arm | KN ppl | evaluator | distinct | novel bigrams |
|---|---|---|---|---|
| trigram sampler | 13.6 | 0.679 | 0.867 | 0.0% |
| operator nearest-k | 97.2 | 0.682 | 0.802 | 0.2% |
| union | 86.2 | 0.678 | 0.818 | 0.2% |

Her own sentences: 187.5. The sampler ceiling is gone — the composer now writes from the function space — and the new ceiling is the operator's ranking quality (16.1× chance). The text is no longer stitched and not yet hers.

**Persona sectors centred** on the lexicon mean (`Fingerprint::lexicon_mean`): attribution 64% → 70%; Austen–Chesterton similarity 0.991 → 0.980; dominant sectors still 44–48 — open.

**Killer heuristic** — not implemented, on purpose: the beam scores every candidate, and the killer heuristic only saves work under alpha-beta pruning, which this beam does not do. It would change nothing measurable.

## What remains, honestly

1. The operator's own ranking (raise it, and `decode_eval` will show it in the text).
2. A sector that separates authors by more than a few percent.
3. The three missing Austen novels.
4. Engine: batched GEMM; f32 compute; docs search.
