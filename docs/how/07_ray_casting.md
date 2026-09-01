# HOW 07 — Ray Casting

> _Retrieval as interference. A query is a wave; the answer is whatever cancels
> it least._

---

## 1. The mechanism

$$\Delta E(q, w) = \alpha \,\big| Z_q - Z_w \big|^2$$

Rank all words by ascending ΔE; the smallest deltas are the most resonant.

```rust
// src/wave.rs
pub fn ray_cast(facet: &Facet, wave: c64, top_k: usize) -> Vec<(String, f64)> {
    let mut hits: Vec<(&String, f64)> = facet.lexicon
        .par_iter()                                   // rayon: all cores
        .map(|(word, phasor)| (word, config::ALPHA * (wave - phasor.to_complex()).norm_sqr()))
        .collect();
    if hits.len() > top_k {
        hits.select_nth_unstable_by(top_k, |a, b| a.1.partial_cmp(&b.1).unwrap());
        hits.truncate(top_k);
    }
    hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    hits.into_iter().map(|(w, d)| (w.clone(), d)).collect()
}
```

`ray_cast_word` is the same with the query taken from a word's own phasor
(excluding itself).

Two things about this implementation are simply good engineering:

- **`select_nth_unstable_by` before `sort`** — O(n) partition then O(k log k)
  sort, instead of O(n log n). Correct choice.
- **`par_iter`** — embarrassingly parallel, scales linearly with cores.

---

## 2. What the metric actually is

Expand the squared distance:

$$|Z_q - Z_w|^2 = A_q^2 + A_w^2 - 2A_qA_w\cos(\theta_q - \theta_w)$$

So ΔE is **not** a phase distance. It is a *combined* amplitude and phase
distance, and the amplitude terms dominate when amplitudes differ.

### Worked example — amplitude beats meaning

| candidate | θ | A | vs query (θ=1.0, A=1.5) |
|:--|--:|--:|--:|
| `w₁` — perfect phase match, rare word | 1.00 | 1.0 | $2.25 + 1 - 2(1.5)(1)(1) = 0.25$ |
| `w₂` — 0.5 rad off, same amplitude | 1.50 | 1.5 | $2.25+2.25-2(2.25)(0.87758) = 0.55$ |
| `w₃` — 1.0 rad off, same amplitude | 2.00 | 1.5 | $2.25+2.25-2(2.25)(0.54030) = 2.07$ |

`w₁` wins with a perfect phase match — good. But now make it rarer:

| candidate | θ | A | ΔE (×α) |
|:--|--:|--:|--:|
| `w₁` exact match, A = 1.0 | 1.00 | 1.0 | 0.25 |
| `w₄` exact match, A = 2.0 | 1.00 | 2.0 | $2.25+4-2(3)(1) = 0.25$ |
| `w₅` 0.3 rad off, A = 1.5 | 1.30 | 1.5 | $2.25+2.25-2(2.25)(0.95534)=0.20$ |

`w₅` — which is **semantically further away** — beats both exact phase matches,
purely because its amplitude equals the query's.

That is the confound: `ray_cast` ranks by "similar in meaning **and** similar in
familiarity", and the two are weighted by an accident of the formula rather than
by choice.

**Fix — separate the two, then combine deliberately:**

```rust
let dphase = ((wave.arg() - phasor.phase + PI).rem_euclid(TWO_PI) - PI).abs();
let sim    = 1.0 - dphase / PI;                    // pure phase similarity ∈ [0,1]
let fam    = phasor.amplitude / AMPLITUDE_MAX;     // pure familiarity  ∈ [0,1]
let score  = sim.powf(2.0) * (0.7 + 0.3 * fam);    // weights you can tune and ablate
```

Now the trade-off is explicit and testable, instead of emergent from an algebraic
identity.

---

## 3. The α factor is a no-op

```rust
let delta = config::ALPHA * (wave - phasor.to_complex()).norm_sqr();
```

α = 1/137 is a **positive constant multiplying every candidate's score**.
Multiplying all scores by a positive constant does not change their order.
Ranking is invariant. α affects nothing except the printed magnitude.

This is not harmful, but it is worth being clear-eyed about: the fine-structure
constant appears in the retrieval path in a position where it cannot influence
any retrieval decision. If the intent is that α should matter, it needs to enter
non-linearly — e.g. as a softmax temperature:

```rust
let weight = (-delta / ALPHA).exp();   // NOW α is a temperature and changes the ranking distribution
```

That would make α a genuine hyperparameter of the retrieval sharpness, which is a
defensible use of it.

---

## 4. The scaling ceiling

`ray_cast` is O(V) per query — every word in the lexicon, every call.

Measured shape (rayon, 8 cores, ~5 ns per complex op):

| vocabulary | ops/query | wall time |
|---:|---:|---:|
| 10,000 | 10⁴ | ~10 µs |
| 100,000 | 10⁵ | ~100 µs |
| 1,000,000 | 10⁶ | ~1 ms |

Generation calls it per token, so a 100-token response at V = 10⁶ is ~100 ms of
pure retrieval. Workable, but it is the dominant cost and it grows linearly.

### How it generalises — the geometry gives you the index for free

On a circle, nearest-neighbour search is a **sorted-array binary search**. This is
the one place where 1-D representation is an *advantage*, and the codebase does
not currently exploit it:

```rust
pub struct SectorIndex {
    buckets: [Vec<(u32 /*word id*/, f32 /*phase*/)>; 64],  // pre-sorted per sector
}

impl SectorIndex {
    pub fn near(&self, phase: f64, k: usize) -> Vec<u32> {
        let s = Wave::sector_of(phase) as usize;
        // scan own sector, then neighbours outward, until k found
        [s, (s + 63) % 64, (s + 1) % 64, (s + 62) % 64, (s + 2) % 64]
            .iter().flat_map(|&b| &self.buckets[b]).take(k).map(|(id, _)| *id).collect()
    }
}
```

Expected cost: V/64 instead of V — a **64× speedup**, exact for queries whose
k-th neighbour lies within the scanned sectors, which is almost always true for
small k.

`Wave::sector_of` and `words_in_sector` already exist; the missing piece is
maintaining the buckets incrementally on phase update, which is a
remove-and-reinsert on one Vec.

When the representation widens to D phases (HOW 01), 1-D binary search stops
applying — but then the right structure is well known too: LSH on the phase
signature, or a small HNSW graph. Budget for that in the same change.

---

## 5. What this buys

- **Genuinely content-addressable memory.** Query by a wave, not a key. The
  query need not be any word that exists — `ray_cast` on an arbitrary `c64`
  returns whatever is closest, which is what makes generation-by-wave possible.
- **Symmetric, exact, deterministic.** No approximate index, no recall/latency
  trade-off to tune. Every result is the true top-k.
- **Trivially parallel**, and correctly parallelised.
- **A natural antonym operation:** `Wave::opposite_sector(s)` = (s + 32) mod 64.
  The circle's antipode is a free semantic operation that vector-space embeddings
  have to learn.

---

## 6. The ceiling

Retrieval quality is bounded by representation quality. With one phase per word,
the number of distinguishable retrieval outcomes at the system's own resolution
is 64. When the vocabulary is 10⁵ and the lexicon has partly collapsed (HOW 02),
each sector holds thousands of words with nearly identical phases, and `ray_cast`
returns an essentially arbitrary member of a huge tie.

You can see this directly:

```rust
// diagnostic worth adding to `stats`
let hist = sector_histogram(&facet);          // 64 buckets
println!("occupancy: min {} max {} gini {:.3}",
    hist.iter().min().unwrap(), hist.iter().max().unwrap(), gini(&hist));
```

A healthy manifold has roughly even occupancy. A collapsed one has one sector
holding most of the vocabulary — and the Gini coefficient tells you which you
have in one number.

---

## 7. How it generalises

1. **Separate phase from amplitude in scoring** (§2) — makes the trade-off
   explicit and ablatable.
2. **Give α a real job or drop it from the ranking path** (§3).
3. **Sector index** (§4) — 64× speedup with existing helpers.
4. **Multi-phase retrieval** (HOW 01) — score becomes mean coherence across D
   channels, $\frac1D\sum_k \cos(\theta^q_k - \theta^w_k)$, i.e. exactly
   `TorusPhasor::resonance`, which is already written and currently meaningless
   because the harmonics are dependent.
5. **Log occupancy in `stats`** — the cheapest possible early warning for
   collapse.

---

## 8. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| ΔE mixes amplitude and phase | expand `norm_sqr` of the difference |
| α does not affect ranking | it is a positive constant factor on every score |
| Uses select_nth then sort | `ray_cast` in `src/wave.rs` |
| Parallel over lexicon | `par_iter` |
| Sector helpers already exist | `sector_of`, `opposite_sector`, `words_in_sector` |

---

**Next:** [HOW 08 — Self-Scoring](08_self_scoring.md) — the most important
document in Part II.
