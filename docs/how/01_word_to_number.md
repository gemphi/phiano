# HOW 01 — Word → Phasor

> _A word enters the system as text and leaves as three numbers. Everything the
> model will ever know about that word must fit in those three numbers._

---

## 1. The mechanism

Every word in the lexicon is one `SpectralPhasor`:

```rust
pub struct SpectralPhasor {
    pub phase: f64,      // θ ∈ [0, 2π)   — where the word sits on the circle
    pub amplitude: f64,  // A ∈ [1.0, 2.0] — how familiar the word is
    pub band_n: u32,     // n ∈ ℕ          — fine-structure sub-band index
}
```

Its wave form is

$$Z = A \cdot e^{i(\theta + n\alpha)}, \qquad \alpha = \tfrac{1}{137}$$

**In the source:** `src/phasor.rs`, `SpectralPhasor::to_complex()`.

The lexicon is a flat map:

```rust
pub lexicon: HashMap<String, SpectralPhasor>   // src/facet/mod.rs
```

That is the entire learned state of the semantic layer. There is no matrix, no
tensor, no per-word vector. One angle, one magnitude, one integer.

---

## 2. Birth: how a new word gets its first position

```rust
// src/trainer/mod.rs :: initialize_tokens
let seed_phase = (token.len() as f64 * PHI).rem_euclid(TWO_PI);
SpectralPhasor::new(seed_phase, AMPLITUDE_INITIAL, BAND_N_INITIAL)
```

with `PHI = 1.6180339887498948`, `AMPLITUDE_INITIAL = 1.0`, `BAND_N_INITIAL = 1`.

### Worked example — the seed table

The seed depends on **one input only: the character length of the token**.

| length | θ_seed (rad) | sector (of 64) | example words |
|---:|---:|---:|:---|
| 1 | 1.618034 | 16 | `a`, `i`, `o` |
| 2 | 3.236068 | 32 | `on`, `is`, `to`, `of` |
| 3 | 4.854102 | 49 | `the`, `cat`, `sat`, `mat`, `dog`, `war` |
| 4 | 0.188951 | 1 | `love`, `hate`, `bomb`, `tree` |
| 5 | 1.806985 | 18 | `peace`, `crime`, `apple` |
| 6 | 3.425019 | 34 | `memory`, `murder`, `garden` |
| 7 | 5.043053 | 51 | `justice`, `poverty`, `physics` |
| 8 | 0.377901 | 3 | `language`, `violence` |
| 9 | 1.995935 | 20 | `dimension`, `structure` |
| 10 | 3.613969 | 36 | `philosophy`, `arithmetic` |

Read that table twice. **`cat`, `the`, `sat`, `mat`, `dog` and `war` all begin at
exactly the same point on the manifold** — 4.854102 rad, sector 49. Not nearby.
Identical to the last bit.

Across a realistic vocabulary, word length ranges roughly 1–20. So a lexicon of
100,000 words is initialised into **about 20 distinct starting positions**, with
the largest bucket (5–8 letter words) holding tens of thousands of exact
collisions.

This is not a rounding artefact. It is what `token.len() as f64 * PHI` computes.

### Why φ was chosen, and why it does not help here

The golden ratio is genuinely the right multiplier for spreading points on a
circle: φ is the "most irrational" number, so the sequence `k·φ mod 2π` fills the
circle with maximal uniformity — this is why sunflower seeds and phyllotaxis use
it, and the code is right to reach for it.

The problem is the index. `k` here is **word length**, which takes ~20 values.
Golden-angle spreading over 20 values gives you 20 well-spread points, which is
exactly what the table shows, and exactly 20 points is not enough to distinguish
100,000 words.

**The one-line fix** is to index by identity rather than length:

```rust
// FNV-1a is already implemented in src/memory/mod.rs :: fnv1a_hash
let h = Memo::fnv1a_hash(token);
let seed_phase = ((h as f64 / u64::MAX as f64) * TWO_PI + (h % 997) as f64 * GOLDEN_ANGLE)
    .rem_euclid(TWO_PI);
```

Same determinism, same zero storage, same golden-angle spreading — but now
100,000 distinct seeds instead of 20. This is the cheapest high-value change in
the entire codebase.

> Note: `DefinitionGrounder::ground_phases` (HOW 05) partially repairs this for
> words that have dictionary definitions, by moving each word halfway toward its
> definition's centroid. Words with no definition entry — proper nouns, jargon,
> code identifiers, anything learned live in chat — keep the collided seed
> forever.

---

## 3. Growth: amplitude as familiarity

```rust
phasor.amplitude = (phasor.amplitude + AMPLITUDE_INCREMENT).min(AMPLITUDE_MAX);
// AMPLITUDE_INCREMENT = 0.001, AMPLITUDE_MAX = 2.0, AMPLITUDE_INITIAL = 1.0
```

Amplitude is a **saturating counter**. It rises 0.001 per training touch, from
1.0, capped at 2.0.

### Worked example — the saturation horizon

A word reaches the ceiling after

$$\frac{2.0 - 1.0}{0.001} = 1000 \text{ training touches}$$

In the Rust Book corpus (`data/rust_book/`, ~180 files), the token `the` appears
tens of thousands of times. It hits amplitude 2.0 within the first few hundred
sentences and then **stops carrying information for the rest of training**.

Meanwhile a word appearing 40 times sits at amplitude 1.04.

So amplitude is informative in the range [1, 1000] occurrences and constant above
it. Since word frequency is Zipfian, that means amplitude saturates for exactly
the words the model sees most and is most confident about. The dynamic range is
spent in the wrong place.

**How it generalises:** make it log-frequency, which is the shape Zipf asks for:

```rust
phasor.count += 1;
phasor.amplitude = 1.0 + (phasor.count as f64).ln() / 14.0;  // ln(1e6)/14 ≈ 1.0
```

Now `the` at 10⁶ occurrences → 1.99, a word at 40 → 1.26, a word at 2 → 1.05.
Ordering is preserved across six orders of magnitude instead of three.

---

## 4. `band_n` and the fine-structure term

`band_n` increments whenever a word's phase has essentially stopped moving:

```rust
// src/trainer/mod.rs
match semantic_force.abs() < CONVERGENCE_THRESHOLD {   // 0.0005
    true => phasor.band_n += 1,
    false => {}
}
```

and it enters the wave through `θ_eff = θ + n·α`.

### Worked example — how much does band_n actually move a word?

α = 1/137 = 0.0072993 rad. A sector at 64-sector resolution is
2π/64 = 0.0981748 rad.

$$\frac{\text{sector width}}{\alpha} = \frac{0.0981748}{0.0072993} = 13.45$$

So it takes **14 band increments to move a word by one sector**, and

$$\frac{2\pi}{\alpha} = 860.8$$

band increments to wrap the entire circle. A word that converges early and keeps
getting touched will silently walk all the way around the manifold and land
somewhere unrelated after ~861 increments — and `band_n` is a `u32` with no wrap
guard, so nothing stops it.

This is worth being precise about, because the fine-structure constant is doing
two jobs at once in the design narrative:

- **Stated job:** a physically-motivated sub-band quantisation, like electron
  energy levels.
- **Actual job:** an anti-collapse tiebreaker that stops converged words from
  becoming numerically identical.

The second job is real and needed (see HOW 02 on collapse). But α is an odd
choice of size for it: too small to separate words into distinguishable sectors,
too large to be safely unbounded. A tiebreaker wants to be *sub-resolution and
bounded*:

```rust
let jitter = (phasor.band_n.min(13) as f64) * ALPHA;  // stays inside one sector
```

That keeps the physics aesthetic, keeps the anti-collapse function, and removes
the unbounded drift.

---

## 5. What this buys

Real, and worth stating plainly:

- **16 bytes of numeric state per word.** `f64 + f64 + u32` = 20 bytes, 24 with
  alignment; the design target of 16 is reachable with `f32 + f32 + u32`. A
  100k-word lexicon is ~2 MB of phasors. That is a genuinely small model.
- **O(1) update, no backpropagation, no optimiser state.** Learning a new word is
  a hash insert plus a `sin()`.
- **Full interpretability.** `θ = 2.31, A = 1.4, n = 7` is a complete, readable
  description of what the model knows about a word. No LLM offers this.
- **No catastrophic forgetting by gradient interference.** Updating one word does
  not overwrite a shared weight matrix.

These are not small properties. They are the reason the architecture is worth
fixing rather than replacing.

---

## 6. The ceiling

A single angle is a **one-dimensional** representation. The state space of a word
is S¹ — the circle.

The consequence is measurable. In a d-dimensional real embedding, the number of
directions that are pairwise near-orthogonal grows exponentially in d
(Johnson–Lindenstrauss): you can pack ~e^(εd) roughly-distinguishable concepts.
On a circle, d = 1, and the number of distinguishable positions at the system's
own 64-sector resolution is **64**.

Sixty-four cells for every concept in English.

This is the single hardest ceiling in the system, and it is not softened by the
64 hexagrams, the 32-harmonic torus, or the 16 memory layers, because:

```rust
// src/phasor.rs :: TorusPhasor::from_spectral
harmonics[k] = (phasor.phase * PHI.powi(k as i32 % 4) + (k as f64 * ALPHA)).rem_euclid(TWO_PI);
```

Every one of the 32 harmonics is a **deterministic function of the same single
`phase`**. There are no independent parameters. `TorusPhasor` is a 1-dimensional
curve drawn inside T³², not a 32-dimensional representation. It cannot store
32 dimensions of information because it contains exactly one number's worth.

---

## 7. How it generalises

The fix is direct, cheap, and preserves every property in §5.

**Give each word D independent phases instead of one.**

```rust
pub struct SpectralPhasor {
    pub phases: [u8; 64],   // 64 independent angles, θ_k = phases[k] * 2π/256
    pub amplitude: f32,
    pub count: u32,
}
```

- Storage: 64 + 4 + 4 = **72 bytes/word**. 100k words = 7.2 MB. Still an
  edge-deployable model, still no GPU.
- Capacity: from 64 distinguishable positions to 256⁶⁴ — i.e. the constraint
  stops being representational capacity and starts being data, which is where you
  want the constraint to live.
- Similarity becomes mean phase coherence across the D channels:
  $\text{sim}(u,v) = \frac{1}{D}\sum_k \cos(\theta^u_k - \theta^v_k)$,
  which is exactly the `TorusPhasor::resonance` function **already written** in
  `src/phasor.rs` — it just needs harmonics that carry independent information
  rather than restating `phase`.
- Every downstream module — ray casting, sector histograms, Kuramoto training,
  persona fingerprints — generalises by summing over k. The code shape does not
  change.

This one change turns `TorusPhasor` from a decorative identity into the actual
representation, and it is the prerequisite for everything in HOW 16.

---

## 8. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| Seed depends only on token length | `src/trainer/mod.rs :: initialize_tokens` |
| ~20 distinct seeds for any vocabulary | run the seed table above for L = 1..20 |
| Amplitude saturates at 1000 touches | `AMPLITUDE_INCREMENT`, `AMPLITUDE_MAX` in `src/config/constants.rs` |
| 14 band increments per sector | 2π/64 ÷ (1/137) = 13.45 |
| Harmonics carry no extra information | `TorusPhasor::from_spectral` — all terms are functions of `phase` |

---

**Next:** [HOW 02 — The Kuramoto Step](02_the_kuramoto_step.md) — what happens to
those numbers when a sentence arrives.
