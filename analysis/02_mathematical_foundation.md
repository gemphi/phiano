# 02 — Mathematical Foundation: The Phase Manifold

> Files examined: [`src/phasor.rs`](../src/phasor.rs), [`src/wave.rs`](../src/wave.rs),
> [`src/config/constants.rs`](../src/config/constants.rs), [`src/facet/mod.rs`](../src/facet/mod.rs),
> [`docs/03_phase_manifold.md`](../docs/03_phase_manifold.md), [`docs/04_spectral_phasor.md`](../docs/04_spectral_phasor.md).

---

## 1. The Core Representation

Every vocabulary token is a [`SpectralPhasor`](../src/phasor.rs) — three numbers:

```text
Z = A · e^(i(φ + n·α))

φ      ∈ [0, 2π)   primary phase angle          (f64)
A      ∈ [1.0, 2.0] familiarity amplitude        (f64)
n      ∈ ℤ≥0        fine-structure sub-band level (u32)
α      = 1/137      Sommerfeld constant           (constants.rs:18)
```

- `to_complex()` realizes `Z` as a `c64` ([phasor.rs:39–42](../src/phasor.rs)).
- `TorusPhasor` expands one phasor into 32 harmonic phases:
  `harmonics[k] = φ·φ_golden^k + k·α mod 2π` ([phasor.rs:91–101](../src/phasor.rs)), with
  resonance = mean cosine of per-harmonic phase differences ([phasor.rs:104–110](../src/phasor.rs)).

**Assessment.** This is a *complex-valued lexical embedding* with three notable design
choices: (a) the representation is **circular**, so "similarity" is naturally
rotation-aware; (b) the band term `n·α` gives each word a second, quantized radial-ish
degree of freedom; (c) initialization is **deterministic** — `φ₀ = len(word)·φ_golden mod 2π`
([trainer/mod.rs:147–154](../src/trainer/mod.rs)) — so two fresh instances of the system
learn identical manifolds from identical data. Determinism is an underrated engineering
virtue: it makes the whole model reproducible, diff-able, and regression-testable.

## 2. Semantic Distance = Destructive Interference

Similarity between two words (or a word and a sentence wave) is the squared complex
distance scaled by α ([wave.rs:68–117](../src/wave.rs)):

```text
Δ(a, b) = α · ‖Z_a − Z_b‖²  =  α(A_a² + A_b² − 2A_aA_b·cos(θ_a − θ_b))
```

The cosine term is the only semantic channel: **two words are similar iff their effective
phases are near-equal (mod 2π), weighted by amplitudes.** The sector system
([wave.rs:119–176](../src/wave.rs)) discretizes the circle into 64 (configurable to 1024)
sectors; antonyms are defined as diametrically opposite sectors (`sector + N/2`),
a clean if crude binary-opposition model.

## 3. Where This Mathematics Sits in the Literature

This is not crank physics dressed as code — it lands in legitimate, active research
families, which is precisely why it deserves serious analysis:

| Phiano construct | Nearest established relative |
|---|---|
| Words as phases on a circle; similarity = phase agreement | **Complex/circular embeddings** — ComplEx (Trouillon et al., 2016) and **RotatE** (Sun et al., 2019) score knowledge-graph relations as rotations/phases in ℂ; these systems handle millions of triples |
| Phase attraction from co-occurrence | **Random indexing / vector symbolic architectures** (Kanerva's HDC; Sahlgren's random indexing): context accumulation over random base vectors, online, no gradients |
| Sentence = superposition wave | **Holographic Reduced Representations** (Plate, 1995): binding via circular convolution; superposition as additive memory |
| Kuramoto coupling as the learning law | **Kuramoto–Sakaguchi dynamics** (Kuramoto 1984; Strogatz 2000): well-studied synchronization physics; order parameter r is a standard coherence measure |
| Circular mean centroid | **Circular statistics** (Mardia & Jupp): the amplitude-weighted `atan2(ΣA·sinφ, ΣA·cosφ)` in [`compute_centroid_phase`](../src/trainer/mod.rs) is the textbook von-Mises-style mean direction |

The original synthesis is the packaging: an *online* Hebbian-style circular embedding
system where **the only learned dynamical parameters are pairwise phase lags β_ij**
(file 03, §4), driven by synchronization physics rather than SGD.

## 4. The Constants: Physics Metaphor or Load-Bearing?

[`constants.rs`](../src/config/constants.rs) encodes α = 1/137, φ (golden ratio), π, and
2^n sector resolutions. Honest audit of which constants do real work:

| Constant | Role | Load-bearing? |
|---|---|---|
| `LEARNING_RATE = 0.05` | Phase pull strength | **Yes** — the actual step size of all learning |
| `CONVERGENCE_THRESHOLD = 0.0005` | When `band_n` increments | **Yes** — controls the anti-collapse mechanism |
| `SYNTACTIC_LAG_BETA = π/16` | Default β_ij before learning | **Yes** — initial syntax coupling |
| `SYNTAX_LAG_LEARN_RATE = 0.08` | EMA rate for learned β_ij | **Yes** — the only learned-parameter rate |
| `AMPLITUDE_*` (1.0→2.0, +0.001) | Familiarity growth | Yes — usage weighting |
| α = 1/137 | Sub-band spacing `φ + nα` | Partially — it sets band spacing to a tiny irrational-ish step; any small irrational would behave equivalently. Physical numerology, but harmless and deterministic |
| φ (golden ratio) | Seed phases, harmonic multipliers, jitter | Partially — golden-ratio seeding gives good circle coverage (van der Corput-like distribution); the specific value is not sacred |
| 64 hexagrams / King Wen map | Bucket labels | No — pure labeling (file 06) |

**Verdict:** the physics vocabulary (fine structure, light quanta, color) is metaphor,
but the *dynamical* constants are a real, tunable learning system. `capacity.rs` even
grid-searches learning rates [0.01–0.12] × epochs [16–64] ([src/metrics/capacity.rs](../src/metrics/capacity.rs)).

## 5. Information Capacity of the Representation (Preview)

File 12 quantifies this fully, but the essential arithmetic belongs here:

- One `SpectralPhasor` = 16 bytes (f64 phase, f64 amplitude, u32 band).
- The **semantic channel is essentially 1-dimensional** (phase on a circle) + 1 quantized
  band + 1 familiarity scalar.
- With V words on one circle, mean angular separation is 2π/V. At V = 100,000 that is
  ~63 µrad — far below the learning system's own noise floor (step size 0.05 rad,
  convergence threshold 5·10⁻⁴ rad). **Beyond a few thousand words per band, phase
  neighborhoods are collisions, not semantics.**
- The 32-harmonic torus expansion multiplies distinguishable structure, but the
  `resonance()` metric averages over harmonics (a lossy projection), and harmonics are
  *deterministic functions of φ* — they add measurement richness, not independent
  storage dimensions.

This is the central mathematical fact of the whole project: **the manifold is sound but
narrow.** Everything that follows in this analysis — what works, what caps out, what the
roadmap must fix — flows from it.

## 6. Soundness Scorecard

| Property | Status |
|---|---|
| Well-defined operations (wrap, superpose, distance, sectors) | Sound; unit-tested in [phasor.rs:113–151](../src/phasor.rs) |
| Rotation invariance / periodicity handled correctly | Yes — `rem_euclid(2π)` throughout |
| Deterministic reproducibility | Yes — golden-ratio seeding, no RNG anywhere in the core |
| Metric properties | Δ is a squared metric on ℂ (triangle-inequality holds via norm); sector mapping loses info by design |
| Capacity | **Bounded — the binding constraint** (§5, file 12) |
| Theoretical grounding | Legitimate relatives: RotatE/ComplEx, VSA/HRR, random indexing, Kuramoto theory |
