# HOW 03 — Learning Word Order

> _A circle has no preferred direction. Language does. This is the mechanism that
> tries to bridge the two — and the place where the bridge is dropped._

---

## 1. The mechanism

Plain Kuramoto is symmetric: word A pulls B exactly as B pulls A. Word order
would be invisible. The **Sakaguchi phase lag** breaks the symmetry by inserting
a directional offset β into the coupling:

$$\theta_i \leftarrow \theta_i + \eta\Big[\sin(\theta_{i-1} - \theta_i + \beta_{i-1,i}) + \sin(\theta_{i+1} - \theta_i - \beta_{i,i+1})\Big]$$

Note the signs: the **preceding** neighbour couples with `+β`, the **following**
neighbour with `−β`. The result is that words want to sit a fixed angular
distance *ahead* of their predecessors. Order becomes rotation.

**In the source:** `src/trainer/mod.rs :: train_sentence` (the `beta_prev` /
`beta_next` arrays), and `src/facet/mod.rs :: record_phase_lag` / `phase_lag`.

---

## 2. β is learned, not fixed

This is the part of the design that is genuinely good and under-advertised. β is
not a constant — each ordered pair gets its own, updated by exponential moving
average:

```rust
// src/facet/mod.rs :: record_phase_lag
let entry = self.phase_lags.entry(a).or_default()
                .entry(b).or_insert(SYNTACTIC_LAG_BETA);   // π/16 = 0.19635
let rate = SYNTAX_LAG_LEARN_RATE;                          // 0.08
*entry = (*entry * (1.0 - rate) + observed * rate).rem_euclid(TWO_PI);
```

with the observation taken directly from the current geometry:

```rust
// src/trainer/mod.rs
let observed_lag = (p1.phase - p0.phase).rem_euclid(TWO_PI);
facet.record_phase_lag(&window[0], &window[1], observed_lag);
```

So `phase_lags["the"]["cat"]` converges to the typical angular gap the model
observes between `the` and `cat`. It is a learned, directed, per-pair quantity —
a genuine syntactic parameter.

---

## 3. Worked example — β learning on a repeated bigram

Train `"the cat"` five times. Suppose after the first sentence
θ_the = 4.8442, θ_cat = 4.8442 (from HOW 02), so observed = 0.0.

β starts at the default π/16 = 0.196350. EMA at rate 0.08:

| pass | observed lag | β after update |
|---:|---:|---:|
| 0 | — | 0.196350 |
| 1 | 0.0000 | 0.180642 |
| 2 | 0.0091 | 0.166393 |
| 3 | 0.0168 | 0.153452 |
| 4 | 0.0233 | 0.141688 |
| 5 | 0.0287 | 0.130983 |

β is decaying toward the observed gap, as designed. The EMA time constant is
1/0.08 = 12.5 observations, so β for a pair is meaningfully learned after roughly
a dozen co-occurrences.

### The circularity to be aware of

`observed` is measured from the very phases that the β term is currently pushing
around. β chases the phases; the phases are moved by β. There is no external
signal anchoring the loop. In the example above β is converging on ~0, i.e. "no
lag", which is the collapse fixed point of HOW 02 expressed in the syntax layer.

A non-circular observation would anchor β to something outside the geometry —
position distance, or a POS-pair prior:

```rust
// anchor the observation to the linguistic fact, not the current geometry
let observed = SYNTACTIC_LAG_BETA * (position_gap as f64);
```

---

## 4. Worked example — the asymmetry in action

Give three tokens genuinely distinct phases (post-fix seeding) and one round of
syntactic-only coupling, β = π/16 = 0.19635, η = 0.05:

| token | θ |
|:--|--:|
| `dogs` (i=0) | 1.000 |
| `chase` (i=1) | 2.000 |
| `cats` (i=2) | 3.000 |

**`chase` (i=1), two neighbours:**

- from `dogs`: sin(1.000 − 2.000 + 0.19635) = sin(−0.80365) = −0.71963
- from `cats`: sin(3.000 − 2.000 − 0.19635) = sin(0.80365) = +0.71963
- mean = **0.0** → `chase` does not move.

**`dogs` (i=0), one neighbour (`chase` follows it):**

- sin(2.000 − 1.000 − 0.19635) = sin(0.80365) = +0.71963 → pushed **forward** by
  η·0.71963·0.3 = +0.01079 (after the 0.3 syntactic weight).

**`cats` (i=2), one neighbour (`chase` precedes it):**

- sin(2.000 − 3.000 + 0.19635) = sin(−0.80365) = −0.71963 → pushed **backward** by
  −0.01079.

Read the result: the sentence is being **compressed toward its middle**, with the
first word rotating forward and the last rotating back. When β exactly matches
the inter-word gap, the forces cancel and the chain is at equilibrium — a phase
*chain* with fixed spacing, ordered by position. That is the intended structure,
and the maths does deliver it.

So: **the training signal genuinely contains word order.** That is real, and it is
better than a bag-of-words trainer.

---

## 5. The ceiling — where order is dropped

Here is the problem, and it is the most consequential single line in the codebase:

```rust
// src/wave.rs :: Wave::sentence
words.iter().filter_map(|w| facet.lexicon.get(w))
     .map(|p| p.to_complex())
     .sum()          // ← addition is commutative
```

Sentence representation is a **plain sum**. Addition is commutative. Therefore:

$$Z(\texttt{"dog bites man"}) = Z(\texttt{"man bites dog"})$$

exactly, bit for bit.

Everything downstream of `Wave::sentence` inherits this: `Evaluator::eval`,
`Wave::ray_cast`, `ContextWaveBuffer`, `Memo::record`, persona fingerprints,
composition scoring. The β machinery carefully encodes order **into the training
signal**, and then the representation function throws it away **at read time**.

Concretely, the following pairs are indistinguishable to the current model:

| A | B |
|:---|:---|
| `dog bites man` | `man bites dog` |
| `the code compiles then it runs` | `it runs then the code compiles` |
| `x = y + 1` | `1 + y = x` |
| `not safe` | `safe not` |

Any task requiring propositional structure — negation scope, argument roles,
causality, arithmetic, code semantics — is out of reach while the composition
operator is `sum`.

---

## 6. How it generalises — binding

The fix is well established and, unusually, it is *native to this substrate*.
Vector Symbolic Architectures / Holographic Reduced Representations bind roles to
fillers using **circular convolution**, which in the phase domain is simply
**phase addition**:

$$\text{bind}(r, f) = A_r A_f \, e^{i(\theta_r + \theta_f)} \qquad \text{unbind}(b, r) = e^{i(\theta_b - \theta_r)}$$

Binding is a multiply. Unbinding is a divide. Both are one operation on a complex
number. This is exactly what a phasor system is good at, and it is the single
strongest argument that the phase-manifold choice was a good one.

### Implementation — order-sensitive sentence wave

```rust
// src/wave.rs — replace Wave::sentence
pub fn sentence_bound(facet: &Facet, words: &[String]) -> c64 {
    words.iter().enumerate()
        .filter_map(|(i, w)| facet.lexicon.get(w).map(|p| (i, p)))
        .map(|(i, p)| {
            // position role vector: golden-angle rotation per slot
            let role = (i as f64) * crate::config::GOLDEN_ANGLE;
            c64::from_polar(p.amplitude, p.phase + role)      // ← bind
        })
        .sum()
}
```

`GOLDEN_ANGLE` is already defined in `src/config/constants.rs`. This is a
four-line change and it makes `dog bites man` ≠ `man bites dog` immediately.

### Implementation — role binding for propositions

Position roles give order. Named roles give structure:

```rust
pub fn bind_proposition(facet: &Facet, subj: &str, verb: &str, obj: &str) -> c64 {
    let r = |name: &str| facet.lexicon.get(name).map(|p| p.phase).unwrap_or(0.0);
    let w = |name: &str| facet.lexicon.get(name).copied();

    let mut z = Wave::zero();
    if let Some(p) = w(subj) { z += c64::from_polar(p.amplitude, p.phase + r("__SUBJ")); }
    if let Some(p) = w(verb) { z += c64::from_polar(p.amplitude, p.phase + r("__VERB")); }
    if let Some(p) = w(obj)  { z += c64::from_polar(p.amplitude, p.phase + r("__OBJ"));  }
    z
}

// query: "who was the subject?" → unbind and ray-cast
pub fn query_role(facet: &Facet, z: c64, role: &str, k: usize) -> Vec<(String, f64)> {
    let r = facet.lexicon.get(role).map(|p| p.phase).unwrap_or(0.0);
    Wave::ray_cast(facet, z * c64::from_polar(1.0, -r), k)   // ← unbind
}
```

Now `bind_proposition(f, "dog", "bites", "man")` and
`bind_proposition(f, "man", "bites", "dog")` are different complex numbers, and
`query_role(f, z, "__SUBJ", 1)` recovers the right filler.

### The one caveat

Superposition-based binding has a capacity limit: with D independent phase
channels you can superpose roughly D/(2 ln D) bound pairs before crosstalk
swamps retrieval. At D = 1 (today), that number is **less than one** — which is
another way of saying binding is impossible at the current representation width.

**So HOW 01's multi-phase change is a hard prerequisite for HOW 03's binding
change.** They are one project, not two. Do 01 first.

---

## 7. What this buys, honestly

- The **learned per-pair β** is a real syntactic parameter, cheap to store
  (`HashMap<String, HashMap<String, f64>>`), and it is more than most
  bag-of-words systems carry.
- The **asymmetric coupling maths is correct** — the worked example in §4 shows
  the intended phase-chain equilibrium forms.
- With `sentence_bound` (§6) the order information is no longer discarded, and
  the β machinery starts paying for itself downstream instead of only at training
  time.

---

## 8. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| β is per-pair and EMA-learned | `Facet::record_phase_lag`, rate = `SYNTAX_LAG_LEARN_RATE` = 0.08 |
| Default β = π/16 | `SYNTACTIC_LAG_BETA` in `src/config/constants.rs` |
| Preceding uses +β, following uses −β | `train_sentence`, `beta_prev` / `beta_next` |
| Sentence wave is order-invariant | `Wave::sentence` — `.sum()` |
| β observation is measured from current phases | `observed_lag` in `train_sentence` |

---

**Next:** [HOW 04 — Co-occurrence Memory](04_cooccurrence_memory.md) — where the
fluency actually comes from.
