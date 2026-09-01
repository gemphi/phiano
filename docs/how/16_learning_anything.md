# HOW 16 — Learning Anything

> _What "learns anything" would actually require, which of those requirements
> Phiano currently meets, and the specific changes that would close the gap._

---

## 1. The four requirements

A system that can learn arbitrary structure needs four properties. They are
independent — having three does not approximate having four — and each has a
concrete signature in code.

| # | Requirement | Why it is necessary | Phiano today |
|--:|:---|:---|:---:|
| **1** | **Capacity** — enough independent parameters to hold distinguishable states | you cannot store more distinctions than you have dimensions | ✗ D = 1 |
| **2** | **Composition** — a binding operator, so structure is representable | `dog bites man` must differ from `man bites dog` | ✗ sum |
| **3** | **Objective** — a signal that ties representation to a task | otherwise the model has no reason to encode anything useful | ✗ centroid attraction |
| **4** | **Non-linearity** — the ability to represent non-linear functions | linear maps cannot approximate arbitrary functions | ✗ linear + argmax |

And three properties Phiano **already has** that most systems do not:

| | Property | Evidence |
|:--|:---|:---|
| ✓ | **Online, single-pass learning** | `train_online`, ~1 µs/sentence, no backprop |
| ✓ | **Targeted, O(1) unlearning** | `correct_mistake` (HOW 10) |
| ✓ | **Full interpretability and tiny footprint** | 16 bytes/word, human-readable state (HOW 13) |

The strategic read: the hard-won properties are already present. The missing four
are all textbook, all cheap, and none of them requires abandoning the phase
substrate. That is an unusually good position to be in.

---

## 2. Requirement 1 — Capacity

**Now:** one angle per word. At the system's own 64-sector resolution, **64
distinguishable states** for the entire vocabulary. The 32-harmonic
`TorusPhasor` adds nothing, because every harmonic is a deterministic function of
that same angle (HOW 01 §6).

**Needed:** D independent phases per word.

```rust
pub const D: usize = 64;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct SpectralPhasor {
    pub phases: [u8; D],     // θ_k = phases[k] · 2π/256
    pub amplitude: f32,
    pub count: u32,
}

impl SpectralPhasor {
    #[inline] pub fn theta(&self, k: usize) -> f64 { self.phases[k] as f64 * TWO_PI / 256.0 }

    /// Mean phase coherence — this is TorusPhasor::resonance, finally meaningful.
    pub fn resonance(&self, other: &Self) -> f64 {
        let s: f64 = (0..D).map(|k| (self.theta(k) - other.theta(k)).cos()).sum();
        s / D as f64
    }
}
```

**Cost:** 72 bytes/word. 100k vocabulary = **7.2 MB**. Still edge-deployable,
still no GPU, still one `sin()` per channel per update.

**Effect:** the binding constraint moves from representation to data — which is
where it belongs. Every downstream module generalises by summing over k; the code
shape does not change.

**This is the prerequisite for requirements 2 and 4.** Do it first.

---

## 3. Requirement 2 — Composition

**Now:** `Wave::sentence` is `.sum()`. Commutative. Order is destroyed at read
time even though the trainer carefully encodes it (HOW 03).

**Needed:** binding. In phase space, binding is addition of angles — the
substrate is already the right one.

```rust
/// Bind a filler to a role: circular convolution ≡ phase addition.
pub fn bind(filler: &SpectralPhasor, role: &SpectralPhasor) -> SpectralPhasor {
    let mut out = *filler;
    for k in 0..D { out.phases[k] = filler.phases[k].wrapping_add(role.phases[k]); }
    out
}

/// Unbind: subtract the role back out.
pub fn unbind(bound: &SpectralPhasor, role: &SpectralPhasor) -> SpectralPhasor {
    let mut out = *bound;
    for k in 0..D { out.phases[k] = bound.phases[k].wrapping_sub(role.phases[k]); }
    out
}

/// A proposition is a superposition of bound role–filler pairs.
pub fn proposition(f: &Facet, s: &str, v: &str, o: &str) -> [c64; D] {
    let mut z = [c64::new(0.0, 0.0); D];
    for (word, role) in [(s, "__SUBJ"), (v, "__VERB"), (o, "__OBJ")] {
        if let (Some(w), Some(r)) = (f.lexicon.get(word), f.lexicon.get(role)) {
            let b = bind(w, r);
            for k in 0..D { z[k] += c64::from_polar(w.amplitude as f64, b.theta(k)); }
        }
    }
    z
}
```

Now:

```rust
proposition(f, "dog", "bites", "man") != proposition(f, "man", "bites", "dog")
query_role(f, &z, "__SUBJ")            -> "dog"
```

**Capacity note:** superposition-based binding supports roughly D/(2 ln D) pairs
before crosstalk. At D = 64 that is ~8 bound pairs per representation — enough for
a clause with modifiers. At D = 1 it is **less than one**, which is why §2 is a
hard prerequisite rather than a nice-to-have.

**What this unlocks:** negation scope, argument roles, causal direction, typed
relations, arithmetic relations, code semantics. Every task that requires knowing
*what goes where*, not just *what appears*.

The cheap first step, available today and worth doing immediately, is positional
binding — four lines, and it makes word order survive to read time:

```rust
.map(|(i, p)| c64::from_polar(p.amplitude, p.phase + i as f64 * GOLDEN_ANGLE))
```

---

## 4. Requirement 3 — Objective

**Now:** pull every token toward the sentence centroid. This is a *descriptive*
target — "be like your neighbours" — and its global optimum is total collapse
(HOW 02 §3). The model has no reason to encode anything predictive.

**Needed:** two changes, both online, both without backpropagation.

### (a) Negative sampling — stops collapse

```rust
const K_NEG: usize = 5;
for token in &tokens {
    for _ in 0..K_NEG {
        let neg = facet.sample_by_frequency();
        if tokens.contains(&neg) { continue; }
        for k in 0..D {
            let d = facet.lexicon[&neg].theta(k) - facet.lexicon[token].theta(k);
            nudge(facet, &neg, k, -self.learning_rate * 0.5 * d.sin());   // push apart
        }
    }
}
```

The fixed point becomes the configuration where phase difference tracks pointwise
mutual information — the structure that makes an embedding useful. This is
skip-gram with negative sampling, expressed on a torus.

### (b) A predictive target — makes representation useful

```rust
// hinge loss on next-word retrieval; perceptron-style online update
let ctx = context_wave(facet, &tokens[..i]);           // [c64; D]
let pos = &tokens[i];
let neg = facet.sample_by_frequency();

if score(facet, &ctx, &neg) > score(facet, &ctx, pos) - MARGIN {
    align(facet, pos, &ctx,  self.learning_rate);       // pull the right answer in
    align(facet, &neg, &ctx, -self.learning_rate * 0.5); // push the wrong answer out
}
```

Next-word prediction is the objective that forces a model to encode syntax,
semantics, world facts and arithmetic — not because it is deep, but because
predicting the next token well *requires* all of them. This is the single change
that converts Phiano from a clustering system into a language model, and it costs
one comparison plus two `sin()` calls per channel.

---

## 5. Requirement 4 — Non-linearity

**Now:** every operation is linear (sum, complex multiply by a constant) followed
by argmax. A composition of linear maps is linear, so the model's function class
is linear regardless of how many "layers" the architecture diagram shows.

**Needed:** a non-linear read-out. Two cheap options, both compatible with the
substrate.

### (a) Sector-indexed lookup — non-linearity by discretisation

```rust
/// Bin each channel's phase into a sector, hash the pattern, look up a learned bias.
pub struct SectorMLP { table: HashMap<u64, [f32; D]> }

impl SectorMLP {
    fn key(&self, z: &[c64; D]) -> u64 {
        let mut h = 14695981039346656037u64;
        for k in 0..D.min(8) {                       // first 8 channels as an index
            h ^= Wave::sector_of(z[k].arg()) as u64;
            h = h.wrapping_mul(1099511628211);
        }
        h
    }
    pub fn apply(&self, z: &mut [c64; D]) {
        if let Some(bias) = self.table.get(&self.key(z)) {
            for k in 0..D { *z_k_mut(z, k) *= c64::from_polar(1.0, bias[k] as f64); }
        }
    }
}
```

Discretisation is a non-linearity, and a hash table is a universal approximator
over discretised inputs. It stays interpretable — you can print the table — and
it trains by the same online rule as everything else.

### (b) Phase-domain gating

$$z' = z \cdot \sigma(|z| - \tau)$$

Suppress channels whose magnitude falls below a threshold. This is a ReLU in the
magnitude domain, one comparison per channel, and it gives sparse, competitive
channel activation — the mechanism that lets different channels specialise.

---

## 6. What the result would be

Apply all four and the architecture is:

- a **D-channel complex-valued representation** (a vector-symbolic architecture),
- with **circular-convolution binding** (holographic reduced representations),
- trained by **online contrastive next-token prediction** (skip-gram objective,
  language-model target),
- with a **diagonal complex linear recurrence** for context (HOW 06 §7b — a
  state-space model),
- and a **non-linear sector read-out**.

Every one of those is an established, current, defensible architecture class.
Together they are a coherent system with a real literature behind it, and one
that keeps the three properties Phiano already has and transformers do not:
microsecond online updates, targeted unlearning, and a model you can read.

That combination does not exist elsewhere. It is worth building.

---

## 7. What it would and would not be

Being precise, because the honest version of the claim is stronger than the
inflated one.

**It would plausibly be excellent at:**

- On-device personal language models that learn from your corrections in
  microseconds and keep learning forever.
- Domain-specialised assistants where a 10 MB model trained live on one company's
  documents beats a generic large model.
- Continual-learning settings where retraining is impossible: embedded systems,
  privacy-constrained deployments, air-gapped environments.
- Anywhere the answer must be auditable — "the model said X because these words
  sit in this sector" is a complete explanation.
- Anywhere a targeted correction must take effect *immediately* and provably not
  disturb anything else.

**It would not be:**

- A frontier reasoning model. Multi-step reasoning at that level currently
  requires learned deep non-linear function approximation at scale, and this
  architecture is deliberately not that.
- Competitive on broad general knowledge with a model trained on trillions of
  tokens. It cannot be; it has neither the parameters nor the data.

That is not a smaller ambition — it is a different and largely unoccupied one.
"The model that learns anything, instantly, on your device, and lets you correct
it in a microsecond" is a genuinely valuable position, and it is reachable from
here. "The model that beats GPT" is not reachable from here and does not need to
be.

---

## 8. The order of work

Each step is independently valuable and independently measurable. Do them in
this order, and measure after every one with the harness from HOW 15.

| # | Change | Doc | Effort | Expected effect |
|--:|:---|:---|:---|:---|
| **0** | Evaluation harness — split, KN baseline, PPL, dispersion logging | 15 | 2 days | you can now see everything below |
| **1** | Hash-based seeding instead of `len × φ` | 01 §2 | 1 hour | 20 → 100k distinct initial positions |
| **2** | Negative sampling in `train_sentence` | 02 §7 | half day | collapse stops |
| **3** | Intern vocabulary; Kneser–Ney smoothing | 04, 13 | 3 days | 92 MB → ~8 MB; no zero probabilities |
| **4** | Positional binding in `Wave::sentence` | 03 §6, 06 §7a | 1 hour | word order survives to read time |
| **5** | Atomic save + periodic checkpoint + Ctrl-C | 13 §4 | half day | learning is no longer lost on crash |
| **6** | `Memo::recall` wired into generation | 12 §6a | 1 day | the episodic memory finally does work |
| **7** | **D = 64 multi-phase representation** | 01 §7 | 1 week | the capacity ceiling lifts |
| **8** | Predictive objective (hinge on next-word) | 02 §7, 16 §4b | 1 week | it becomes a language model |
| **9** | Role binding + unbinding | 03 §6, 16 §3 | 1 week | propositions become representable |
| **10** | Recurrent complex context state | 06 §7b | 3 days | long-range structure; it becomes an SSM |
| **11** | Non-linear sector read-out | 16 §5 | 1 week | function class stops being linear |

Steps 0–6 are two weeks of work, need no architectural change, and fix every
concrete defect identified across HOW 01–14. Steps 7–11 are the research
programme, and they are the ones that make the title of this document true.

---

## 9. The honest summary

Phiano is a **well-engineered online associative learner** with three properties
that are genuinely rare and genuinely valuable: microsecond updates, targeted
unlearning, and complete interpretability at a footprint that fits on a
microcontroller. The Rust is clean, the module boundaries are well drawn, the
tests exist, and the instincts behind the design — phase as representation,
oscillator dynamics as learning, dictionaries as grounding, curiosity as a
control loop — are good instincts, several of which the mainstream has
underexplored.

It is **not yet** a system that can learn anything, because a single angle per
word is one dimension, a sum is not a composition operator, centroid attraction
is not an objective, and a linear model has a linear function class. Those are
four specific, well-understood gaps with four specific, well-understood fixes,
none of which requires giving up what already works.

The gap between the current system and the claim is not a gap of vision. It is
eleven numbered changes and a measurement loop.

Build the harness. Then work down the list.

---

## 10. Cross-references

| Requirement | Diagnosis | Fix |
|:---|:---|:---|
| Capacity | HOW 01 §6 | HOW 01 §7, HOW 16 §2 |
| Composition | HOW 03 §5, HOW 06 §3 | HOW 03 §6, HOW 16 §3 |
| Objective | HOW 02 §3, HOW 08 §3 | HOW 02 §7, HOW 16 §4 |
| Non-linearity | HOW 16 §5 | HOW 16 §5 |
| Measurement | HOW 08 §6 | HOW 15 |

---

**End of the HOW series.** Return to [the index](00_index.md), or read the
verdict in [`../EVALUATION.md`](../EVALUATION.md).
