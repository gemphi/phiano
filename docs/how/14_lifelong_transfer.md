# HOW 14 — Lifelong Transfer

> _The module that names the right goal. Every scaffold for continual learning is
> in place; most of them are currently empty._

---

## 1. The intent

`src/lifelong/` implements the Chollet Ch.14.5 programme: accumulate reusable
abstractions across tasks, adapt quickly via meta-learned priors, monitor for
drift.

```
LifelongLearner::learn_task()
  ├─▶ 1. query ComponentLibrary by phase signature
  │       └─▶ if matched: apply, mark_used, train 4 iterations
  ├─▶ 2. otherwise train 16 iterations
  └─▶ 3. evaluate coherence, return LearnResult
```

The module structure is right, the naming is right, and the ambition is correct.
This document is about the distance between the structure and the behaviour.

---

## 2. `learn_task` — the reuse decision

```rust
// src/lifelong/mod.rs
let reused_name = if let Some(comp) = self.library.find_reusable(facet, task) {
    let name = comp.name.clone(); self.library.mark_used(&name); Some(name)
} else { None };

let iterations = if reused_name.is_some() { 4 } else { 16 };
for _ in 0..iterations { trainer.train_sentence(facet, task); }
```

The shape is exactly right: **a recognised task should require less learning
than a novel one**, and 4 vs 16 encodes that. This is the core insight of
transfer learning, expressed in three lines.

What "reuse" actually does, though, is only *reduce the iteration count*. The
matched component's `Program` is never executed, never applied to the facet,
never consulted for its answer. `mark_used` increments a counter. So reuse is
currently a **speed-up flag**, not a transfer of knowledge.

**Fix — make the component do work:**

```rust
if let Some(comp) = self.library.find_reusable(facet, task) {
    // apply the stored program's phase pattern as a prior before training
    for (i, tok) in Tokenizer::tokenize(task).iter().enumerate() {
        if let (Some(target), Some(p)) = (comp.phase_signature.get(i), facet.lexicon.get_mut(tok)) {
            let d = wrap_signed(target - p.phase);
            p.phase = (p.phase + 0.3 * d).rem_euclid(TWO_PI);   // warm-start from the component
        }
    }
    self.library.mark_used(&comp.name);
}
```

Now a matched component genuinely transfers structure, and the reduced iteration
count is justified by the warm start rather than asserted.

---

## 3. `find_reusable` — the matching problem

```rust
// src/synthesis/library.rs
fn compute_phase_signature(facet: &Facet, text: &str) -> Vec<f64> {
    Tokenizer::tokenize(text).iter()
        .filter_map(|t| facet.lexicon.get(t).map(|p| p.phase)).collect()
}

fn signature_similarity(a: &[f64], b: &[f64]) -> f64 {
    let min_len = a.len().min(b.len());
    let mut total = 0.0;
    for i in 0..min_len {                       // ← positional, elementwise
        let mut diff = (a[i] - b[i]).abs();
        if diff > PI { diff = TWO_PI - diff; }
        total += 1.0 - diff / PI;
    }
    total / min_len as f64
}
```

The signature is a **positional list of phases**, and similarity compares
position i to position i. Two consequences:

**(a) Length and alignment sensitivity.** `"sort the list"` and
`"please sort the list"` have signatures offset by one, so every comparison is
between mismatched words. Similarity collapses even though the tasks are
identical.

**(b) The threshold is not what it looks like.**

```rust
best.and_then(|(c, sim)| if sim > 0.6 { Some(c) } else { None })
```

`1 − diff/π` where `diff` is uniform on [0, π] has expected value **0.5**. So two
*completely unrelated* signatures score ~0.5 on average, with standard deviation
≈ 0.29/√n. For a 4-token task, σ ≈ 0.144, and 0.6 is only 0.7σ above chance —
about a **24% false-match rate**. Under phase collapse (HOW 02), where all
phases converge, similarity → 1.0 and **everything matches everything**.

**Fix — order-invariant, chance-corrected matching:**

```rust
fn signature_similarity_v2(a: &[f64], b: &[f64]) -> f64 {
    // circular-histogram cosine: order-free, length-normalised
    let ha = phase_histogram(a, 64);      // 64-bin, L2-normalised
    let hb = phase_histogram(b, 64);
    let cos = ha.iter().zip(&hb).map(|(x, y)| x * y).sum::<f64>();
    ((cos - 0.5) * 2.0).max(0.0)          // rescale so chance ≈ 0.0, identical ≈ 1.0
}
// then threshold at 0.6 means "well above chance", which is what was intended
```

---

## 4. `FeatureReuse::apply` — the placeholder

```rust
// src/lifelong/reuse.rs
pub fn apply(facet: &mut Facet, features: &[FeatureSet]) {
    for fs in features {
        for &phase in &fs.phase_pattern {
            let word = format!("meta_{}", fs.label);     // e.g. "meta_sector_17"
            facet.get_or_init(&word);
            if let Some(p) = facet.lexicon.get_mut(&word) { p.phase = phase; }
        }
    }
}
```

Read the inner loop carefully. For each feature set it creates **one** synthetic
word — `meta_sector_17` — and then, in a loop over every phase in the pattern,
overwrites that same word's phase. The word ends at the **last** phase in the
vector; all the earlier ones are discarded.

Net effect of `transfer_knowledge`: the lexicon gains up to 64 synthetic tokens
named `meta_sector_0 … meta_sector_63`, each holding one arbitrary phase, and
these tokens then participate in training, generation and ray-casting as if they
were words. `TransferResult.features_transferred` counts the feature sets, so it
reports a large number regardless.

This is a stub that reports success. It should either be implemented or removed —
reporting transfer that did not happen is worse than reporting none.

**What real feature transfer looks like here:**

```rust
/// Transfer the *relational structure* of a source domain onto a target domain:
/// preserve angular relationships, translate the whole cluster.
pub fn apply_relational(facet: &mut Facet, src: &[String], dst: &[String], strength: f64) {
    let src_centroid = centroid_phase(facet, src);
    let dst_centroid = centroid_phase(facet, dst);
    let shift = wrap_signed(dst_centroid - src_centroid);

    for (s, d) in src.iter().zip(dst) {
        let target = match facet.lexicon.get(s) { Some(p) => p.phase + shift, None => continue };
        if let Some(p) = facet.lexicon.get_mut(d) {
            let diff = wrap_signed(target - p.phase);
            p.phase = (p.phase + strength * diff).rem_euclid(TWO_PI);
        }
    }
}
```

That is an actual analogy operation — "arrange the target domain the way the
source domain is arranged" — and it is the mechanism that would let a model
trained on Rust ownership transfer structure to, say, C++ RAII.

---

## 5. `MetaLearner` — what it extracts

```rust
// src/lifelong/meta.rs
all_phases.sort_by(...);
let common_phases = if all_phases.len() > 4 {
    let q = all_phases.len() / 4;
    all_phases[q..all_phases.len() - q].to_vec()     // interquartile slice
} else { all_phases };
```

`common_phases` is the **middle 50% of a sorted list of raw phase values across
all tasks**. Sorting phases numerically breaks the circle (0.01 and 6.27 are
adjacent on the manifold and maximally distant in the sorted list), and the
interquartile slice of a circular quantity is not a meaningful summary.

Then:

```rust
// MetaModel::adapt — common_phases is never read
let effective_lr     = (0.05 + avg_rate.abs() * 0.1).min(0.15);
let effective_epochs = (16.0 + avg_rate * 100.0).max(8.0) as usize;
```

`adapt` uses only `adaptation_rates`. `common_phases` — the entire output of the
"meta-learning" — is computed, stored, serialised, and never consulted.

That said, the *learning-rate adaptation itself is real*: tasks that produced
large coherence gains raise the LR and epoch count for the next task. That is a
legitimate, if simple, meta-learning signal, and it works. It just is not what
the module claims to be doing.

**Fix — a circular-mean prior that `adapt` actually uses:**

```rust
pub struct MetaModel {
    pub prior: Vec<(String, f64)>,   // word → circular-mean phase across tasks
    pub adaptation_rates: Vec<f64>,
    pub n_tasks: usize,
}

// circular mean, not numeric sort
fn circular_mean(phases: &[f64]) -> f64 {
    let (sx, sy): (f64, f64) = phases.iter().fold((0.0,0.0), |(x,y), &p| (x+p.cos(), y+p.sin()));
    sy.atan2(sx).rem_euclid(TWO_PI)
}

impl MetaModel {
    pub fn adapt(&self, facet: &mut Facet, trainer: &Trainer, task: &str) {
        for (word, prior_phase) in &self.prior {           // ← now used
            if let Some(p) = facet.lexicon.get_mut(word) {
                let d = wrap_signed(prior_phase - p.phase);
                p.phase = (p.phase + 0.25 * d).rem_euclid(TWO_PI);   // warm start
            }
        }
        // ... then the existing adaptive LR/epoch schedule
    }
}
```

---

## 6. `ModelMonitor` — the part that works

```rust
// src/lifelong/monitor.rs
pub fn check_drift(&self, facet: &Facet, recent: &[String]) -> Option<Alert> {
    let max_ood = recent.iter().map(|i| OodDetector::score(facet, i)).fold(0.0, f64::max);
    if max_ood > 0.7 { Some(Alert { alert_type: "distribution_drift", severity: max_ood, .. }) } else { None }
}

pub fn check_regression(&self, current: &BenchmarkReport) -> Option<Alert> {
    let delta = current.baselines.2 - self.history.latest()?.report.baselines.2;
    if delta < -0.05 { Some(Alert { alert_type: "performance_regression", .. }) } else { None }
}
```

This is well-built: threshold-based alerting, severity scores, persisted
`BenchmarkHistory` with a `trend()` accessor. The engineering is sound.

The one problem is what it monitors: `baselines.2` is the **phase baseline
coherence**, which per HOW 08 rises as the model collapses. So
`check_regression` will report health precisely while the model degrades, and it
will fire an alert if a genuine fix *reduces* collapse.

**Fix:** point it at held-out perplexity (HOW 15) once that exists, and add
phase dispersion as a second monitored quantity:

```rust
if phase_dispersion(facet) < 0.2 {
    alerts.push(Alert { alert_type: "manifold_collapse".into(),
        message: "phase dispersion below 0.2 — lexicon is synchronising".into(), severity: 0.9 });
}
```

That single alert would catch the system's most serious failure mode
automatically.

---

## 7. What this buys

- **The right decomposition.** `library / meta / reuse / monitor / history` is a
  correct factoring of the lifelong-learning problem, and having the seams in
  place means the implementations can be filled in independently.
- **`BenchmarkHistory` is genuinely useful** — timestamped, JSON-persisted,
  with a trend accessor. Once it is recording an honest metric it becomes the
  project's regression suite.
- **The adaptive LR/epoch schedule in `MetaModel::adapt` is real meta-learning**,
  even if small.
- **The reuse-reduces-iterations principle is correctly encoded**, and needs only
  the warm-start of §2 to become substantive.

---

## 8. The ceiling

Transfer requires **abstractions** to transfer. The system's abstractions are
`ComponentLibrary::Component` — a program plus a phase signature — and the
program is never executed, so what actually moves between tasks is a list of
angles.

With a single phase per word (HOW 01), the most a component can encode is "these
tokens sat at roughly these angles". That is not an abstraction; it is a snapshot.
Genuine transfer needs the abstraction to be a *relation* — the shape of the
arrangement, invariant to where it sits on the manifold — which is what
`apply_relational` in §4 gestures at and what multi-channel binding (HOW 03)
would make expressible.

---

## 9. How it generalises

In dependency order:

1. **Fix `signature_similarity`** (§3) — order-invariant and chance-corrected.
   Everything else in the module depends on matching being meaningful.
2. **Make reuse warm-start** (§2) — reuse should transfer structure, not just
   skip iterations.
3. **Implement or delete `FeatureReuse::apply`** (§4). If implemented, make it
   relational.
4. **Have `adapt` consume the prior** (§5), computed with a circular mean.
5. **Repoint the monitor at perplexity + dispersion** (§6).
6. **Then the module is doing what it says**, and `BenchmarkHistory` becomes the
   record that shows it.

---

## 10. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| Reuse only changes iteration count | `LifelongLearner::learn_task` |
| Signature is positional and length-sensitive | `signature_similarity` |
| 0.6 threshold is ~0.7σ above chance | E[1 − diff/π] = 0.5 for uniform diff |
| `apply` creates one word per feature set | `format!("meta_{}", fs.label)` inside the inner loop |
| `common_phases` is never read | grep `common_phases` — written in `learn`, unused in `adapt` |
| Monitor tracks `baselines.2` | `check_regression` |
| Phases are sorted numerically, not circularly | `all_phases.sort_by` in `MetaLearner::learn` |

---

**Next:** [HOW 15 — Proving It Works](15_proving_it_works.md).
