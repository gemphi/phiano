# 173 · One Fourier address

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## The complaint, and the fix

The same Fourier functionality was handled in several places: the harmonics in phifno, the Fourier neural operator in phical, the heat flow (exact in Fourier modes) in phical, the FFT kernels in phistd — four crates for one subject. Code of different modules must not mix: every Fourier thing now has the one address, **phifno**.

Moved into phifno (the whole directories, imports rewritten to phistd, `phifno::sphere` references now `crate::sphere`):

- `operator/` — `Fno`, `FnoConfig`, `Tie` (circle, sphere, anisotropic), `Layer`, modes/masks, `Module`/`Sequential`, `FrequencySampler`, versioned save with optimiser state, training (`train_epoch_softmax`, `rank`, `zeta_weights`), the graph tie. It already imported phifno's harmonics for `Tie::Sphere` — the operator was living below the representation it reads.
- `heat.rs` — Laplace–Beltrami heat flow on the circle, exact in Fourier modes; band-limited on the sphere.
- `fft` — a re-export module (`phifno::fft::{fft_inplace, fft2_inplace}`): the kernels' home of record stays `phistd::tensor`, because the tensor autograd is built on them and phistd sits below phifno; phifno is where you look.

phical keeps what is phical's: the exact geometry of the relation circle — `conjugate` (the mirror, an all-pass `e^{iθ_k}`, not a transform), `joint`, `relate` (laws, closure, inheritance), `liskov`, `q64`. It re-exports `pub use phifno::{heat, operator};` so every existing `phical::operator::…` and `phical::heat::…` path still reads, and its doc table marks both as living in phifno. phifno's dependencies gained serde and bincode (the operator's save format); nothing else changed hands.

Not moved, after checking each: `phiano::phasor::spectrum` (a phasor's harmonic read-out — representation, not a transform), `phiton::spectrum` (phase → light wavelength — domain mapping), `phistd::hexagram` (uses the one `fft_inplace`, which is reuse, not duplication), `phiplm::operator` (`RelationModel`, `FnoWave` — prose reading built ON the operator, composition not duplication).

The layering now matches the reference stack it was named for: phistd is torch (engine: tensors, tape, the raw FFT), **phifno is torch-harmonics AND neuraloperator (everything Fourier)**, phical is the exact relation geometry, phiplm reads language with them.

Also: `phistd`'s example `bench_table.rs` → `table.rs` (its rename to `bench.rs` collided with phiano's bench example).

## Status

2026-10-24. Workspace builds, 31 test binaries all ok, 0 failures. Synced to the machine with the moved files deleted at their old paths.
