# HOW 06 — Sentence Superposition

> _Many phasors become one wave. The wave is small, fast, and elegant — and it
> answers "which words?" while refusing to answer "in what arrangement?"_

---

## 1. The mechanism

$$Z_{\text{sentence}} = \sum_{w \in \text{tokens}} A_w e^{i(\theta_w + n_w\alpha)}$$

```rust
// src/wave.rs
pub fn sentence(facet: &Facet, words: &[String]) -> c64 {
    words.iter().filter_map(|w| facet.lexicon.get(w))
         .map(|p| p.to_complex()).sum()
}
pub fn text(facet: &Facet, text: &str) -> c64 {
    Self::sentence(facet, &Tokenizer::tokenize(text))
}
```

Two derived quantities carry almost all downstream meaning:

- $|Z|$ — **magnitude**: how aligned the words are. Max = ΣA (all in phase),
  min = 0 (perfectly cancelling).
- $\arg Z$ — **direction**: where the sentence sits on the manifold.

---

## 2. Worked example — alignment vs cancellation

Three words, all amplitude 1.0.

### Case A — aligned

| word | θ |
|:--|--:|
| `rust` | 1.00 |
| `memory` | 1.10 |
| `safety` | 1.20 |

$\sum\cos = 0.54030 + 0.45360 + 0.36236 = 1.35626$
$\sum\sin = 0.84147 + 0.89121 + 0.93204 = 2.66472$

$|Z| = 2.99000$, $\arg Z = 1.10001$

Normalised coherence: $|Z| / N = 0.99667$. Near-perfect — the words agree.

### Case B — opposed

| word | θ |
|:--|--:|
| `rust` | 1.00 |
| `banana` | 3.09 |
| `entropy` | 5.18 |

These are ~2.09 rad = 120° apart, i.e. the three cube roots of unity.

$\sum\cos = 0.54030 - 0.99917 + 0.44016 = -0.01871$
$\sum\sin = 0.84147 + 0.04070 - 0.89797 = -0.01580$

$|Z| = 0.02449$, coherence $= 0.00816$. Near-total destructive interference.

This is the physics working exactly as intended: semantically scattered words
cancel, semantically clustered words reinforce. As a **coherence detector** the
mechanism is sound and cheap — one complex add per token, no matrix multiply.

---

## 3. Worked example — the information that vanishes

Sum is commutative, so:

```
Wave::text(facet, "dog bites man")   ==   Wave::text(facet, "man bites dog")
Wave::text(facet, "not safe")        ==   Wave::text(facet, "safe not")
Wave::text(facet, "a implies b")     ==   Wave::text(facet, "b implies a")
```

Bit-for-bit identical `c64` values. Everything reading `Wave::text` therefore
cannot distinguish them:

| Consumer | Source | Consequence |
|:---|:---|:---|
| `Evaluator::eval` | `src/eval.rs` | identical coherence/novelty/verdict |
| `Memo::record` | `src/model.rs` | stored as the same wave |
| `ContextWaveBuffer` | `src/generate.rs` | conversation state is a bag |
| `Wave::ray_cast` | `src/wave.rs` | identical retrieval |
| persona fingerprints | `src/persona/` | identical style signature |
| composition scoring | `src/compose/` | identical alignment score |

There is a second, subtler loss. Summing **N** words into **one** complex number
is a compression from N×(1 angle) to 1 angle. Two very different sentences with
the same centroid are indistinguishable:

- `hot cold` (θ = 0 and θ = π, amplitude 1 each) → Z = 0
- `left right` (θ = π/2 and θ = 3π/2) → Z = 0
- the empty sentence → Z = 0

All three score coherence 0.0 and are, to the model, the same object.

---

## 4. The magnitude/direction confound

`Evaluator` reads both quantities off the same number:

```rust
coherence = (wave.norm() / known as f64).clamp(0.0, 1.0);   // magnitude
novelty   = f(angular distance between wave.arg() and centroid.arg());  // direction
```

But magnitude and direction of a sum are not independent. As $|Z| \to 0$,
$\arg Z$ becomes numerically meaningless — the direction of a near-zero vector is
dominated by floating-point noise. So exactly when coherence is lowest, novelty
is least reliable, and the two are then combined into `overall` as if they were
independent measurements:

```rust
overall = 0.4·coherence + 0.3·novelty + 0.3·resonance
```

**Fix:** gate novelty on magnitude.

```rust
let novelty = if wave.norm() < 0.1 * known as f64 {
    f64::NAN   // undefined, not zero — do not fold noise into the score
} else { ... };
```

---

## 5. What this buys

- **O(N) sentence encoding**, no attention matrix, no O(N²) anything. A
  4,096-token context is 4,096 complex adds — microseconds.
- **Incremental and reversible.** `Z + z_new` extends a context; `Z − z_old`
  retracts it. The `ContextWaveBuffer` exploits this to maintain running state in
  constant memory.
- **A real coherence signal**, demonstrated in §2 — this genuinely measures
  semantic agreement among known words.
- **Two f64s of state** for an unbounded context window. Nothing in the
  transformer family comes close on memory.

---

## 6. The ceiling

Summation is a **linear, permutation-invariant, rank-1 aggregation**. Each of the
three words is a separate limit:

- **Linear** — no interaction terms. The representation of `hot dog` is
  determined by `hot` and `dog` independently, so idioms, compounds and
  metaphors have no representational home.
- **Permutation-invariant** — no syntax, as shown in §3.
- **Rank-1** — the whole sentence is one point, so it cannot represent two
  simultaneous facts distinctly (superposing three bound pairs already
  saturates a 1-D channel).

---

## 7. How it generalises

Three changes, in increasing order of ambition, each independently useful.

### (a) Positional binding — restores order, four lines

```rust
pub fn sentence_ordered(facet: &Facet, words: &[String]) -> c64 {
    words.iter().enumerate()
        .filter_map(|(i, w)| facet.lexicon.get(w).map(|p| (i, p)))
        .map(|(i, p)| c64::from_polar(p.amplitude, p.phase + i as f64 * GOLDEN_ANGLE))
        .sum()
}
```

`GOLDEN_ANGLE` = 2π/φ² ≈ 2.39996 is already in `config/constants.rs`, and its
irrationality is exactly the property you want: no two positions collide, ever.

Immediate effect: `dog bites man` ≠ `man bites dog`.

### (b) Recurrent state — restores recency and long-range structure

Replace the decayed sum in `ContextWaveBuffer` with a rotation-and-decay
recurrence:

$$h_t = \lambda\, e^{i\omega} h_{t-1} + z_t, \qquad \lambda \in (0,1)$$

```rust
pub fn push_token(&mut self, facet: &Facet, token: &str) {
    let rot = c64::from_polar(self.lambda, self.omega);   // λ·e^{iω}
    self.h = self.h * rot;
    if let Some(p) = facet.lexicon.get(token) { self.h += p.to_complex(); }
}
```

This is not an ad-hoc tweak — it is a **diagonal complex linear recurrence**, the
core of modern state-space models (S4/S5, LRU, Mamba's linear component). Those
architectures are competitive with attention on long-range benchmarks precisely
because a complex diagonal recurrence with $|\lambda| < 1$ and a rotation gives
each channel a characteristic timescale and frequency.

Phiano is already a complex-valued system with per-word phases. It is one
`self.h * rot` away from being a legitimate linear SSM, which is a real, current,
defensible position in the literature — and a far stronger claim than "context
buffer".

With D channels (HOW 01), give each channel its own $\lambda_k, \omega_k$
log-spaced across timescales, and the context state genuinely carries multi-scale
history.

### (c) Multiplicative interaction — restores composition

For a two-word compound, bind rather than add:

$$z_{\text{compound}} = z_a \cdot z_b = A_aA_b\,e^{i(\theta_a+\theta_b)}$$

`hot dog` becomes a point determined by both words *jointly*, distinct from
`hot` + `dog`. Detect compounds via high pointwise mutual information in the
existing bigram table — the data to do this is already collected.

---

## 8. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| Sentence wave is a plain sum | `Wave::sentence` |
| Unknown words are silently dropped | `filter_map` in `Wave::sentence` |
| Coherence = \|Z\|/N | `src/eval.rs` |
| Novelty uses `arg` of a possibly-tiny wave | `src/eval.rs`, novelty branch |
| Context buffer decays by 0.5 per turn | `CONTEXT_DECAY_BASE` in `src/generate.rs` |
| `GOLDEN_ANGLE` already defined | `src/config/constants.rs` |

---

**Next:** [HOW 07 — Ray Casting](07_ray_casting.md).
