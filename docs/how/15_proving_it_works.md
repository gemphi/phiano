# HOW 15 — Proving It Works

> _An architecture that cannot be falsified cannot be defended. This document is
> the experiment that would settle whether the phase manifold earns its place._

---

## 1. The question

Phiano contains two learners:

- a **trigram model** (`Facet::bigrams`, `Facet::trigrams`) — classical, known
  performance characteristics
- a **phase manifold** (`Facet::lexicon`) — the novel contribution

The trigram model alone produces fluent-looking output. So no sample of generated
text, however good, is evidence about the manifold. There is exactly one question
worth answering:

> **Does the phase manifold reduce held-out perplexity relative to a
> well-smoothed trigram baseline over the same data?**

If yes, the design is validated and the number is publishable. If no, the
manifold is not yet contributing and the fixes in HOW 01/02/03 are where the
effort belongs. Either answer is progress. Not knowing is the only bad state.

---

## 2. Why perplexity, specifically

Perplexity is

$$\text{PPL} = \exp\!\left(-\frac{1}{N}\sum_{i=1}^{N}\log P(w_i \mid w_{<i})\right)$$

and it has the three properties the current metrics lack:

| property | perplexity | coherence |
|:---|:---|:---|
| computed on data never trained on | yes | no |
| can get worse | yes | not under the current trainer |
| comparable across models | yes | no |
| gameable by collapse | no — collapse destroys the predictive distribution | yes — collapse maximises it |

Perplexity punishes exactly the failure this architecture is prone to. That makes
it the right instrument, not merely a conventional one.

---

## 3. The protocol

### Step 1 — a real split

```rust
// src/metrics/eval_split.rs — extend
pub struct Split { pub train: Vec<String>, pub valid: Vec<String>, pub test: Vec<String> }

pub fn split_corpus(sentences: Vec<String>, seed: u64) -> Split {
    let mut s = sentences; deterministic_shuffle(&mut s, seed);
    let n = s.len();
    Split {
        train: s[..n*80/100].to_vec(),
        valid: s[n*80/100..n*90/100].to_vec(),
        test:  s[n*90/100..].to_vec(),
    }
}
```

Rules, without exception: the trainer never sees `valid` or `test`;
`ground_phases` never sees them either (a definition containing a test sentence
is leakage); `test` is read once, at the end, when the design is frozen.

Source: `data/rust_book_corpus.txt` plus `data/dialogues/conversations.json`.

### Step 2 — the baseline you must beat

```rust
// src/metrics/kn_baseline.rs  (new)
pub struct KneserNey { d: f64, uni: HashMap<u32, f64>, bi: ..., tri: ... }

impl KneserNey {
    pub fn train(corpus: &[String]) -> Self { /* count, then discount D = 0.75 */ }
    pub fn log_prob(&self, ctx: (u32, u32), w: u32) -> f64 { /* interpolated KN */ }
    pub fn perplexity(&self, held_out: &[String]) -> f64 { /* exp(-mean log prob) */ }
}
```

Kneser–Ney trigram on the same 80% split. Roughly 120 lines. This is the number
Phiano must beat; without it, "the model works" has no referent.

### Step 3 — make Phiano emit probabilities

Currently `decode` returns a word, not a distribution. Perplexity needs
`P(w | context)`. Expose the scoring function as a normalised distribution:

```rust
// src/generate.rs
pub fn next_distribution(&self, facet: &Facet, ctx: &ContextWaveBuffer,
                         prev: Option<&str>, last: Option<&str>) -> HashMap<String, f64> {
    let target = ctx.context_phase() + SYNTACTIC_MOMENTUM_DEFAULT;
    let mut scores: HashMap<String, f64> = HashMap::new();

    // same terms as pick_ngram, but kept as scores over the full candidate set
    for (w, c) in candidate_set(facet, prev, last) {
        let capped   = (c as f64).min(24.0).ln_1p();
        let align    = facet.lexicon.get(&w).map(|p| (p.phase - target).cos().max(0.0)).unwrap_or(0.0);
        scores.insert(w, capped * (0.35 + 0.25 * align));
    }
    // interpolate with a uniform floor so nothing is exactly zero
    softmax_with_floor(scores, 1e-8)
}
```

The uniform floor matters: without it a single unseen token makes perplexity
infinite and the whole measurement uninformative.

### Step 4 — the four-way comparison

| # | system | what it isolates |
|--:|:---|:---|
| 1 | Kneser–Ney trigram | the baseline |
| 2 | Phiano, phase re-ranking **disabled** | Phiano's own n-gram layer |
| 3 | Phiano, full | the hybrid |
| 4 | Phiano, n-grams **disabled** (ray-cast only) | the manifold alone |

The diagnostic reads directly off the table:

- **3 < 1** → the architecture beats the baseline. Report it.
- **3 ≈ 2** → the manifold contributes nothing; it is a decorative re-ranker.
- **2 < 1** → Phiano's n-gram implementation is worse than a smoothed one; add KN
  smoothing (HOW 04 §5) before anything else.
- **4 ≫ 1** → the manifold alone is near-useless, which is the expected result at
  D = 1 and the strongest argument for HOW 01's multi-phase change.

### Step 5 — collapse instrumentation, every epoch

```rust
#[derive(Serialize)]
pub struct EpochMetrics {
    pub epoch: usize,
    pub train_ppl: f64,
    pub valid_ppl: f64,          // ← the number that matters
    pub coherence: f64,          // keep, but never alone
    pub phase_dispersion: f64,   // 1 - |Σe^{iθ}|/N   (HOW 02 §3)
    pub sector_gini: f64,        // occupancy inequality (HOW 07 §6)
    pub vocab_size: usize,
    pub mean_amplitude: f64,
}
```

The signature to watch for:

```
epoch  valid_ppl   coherence   dispersion
   1      412.3       0.31        0.94
   5      388.1       0.52        0.71
  10      401.7       0.68        0.48     ← ppl turns up, dispersion falling
  20      467.2       0.81        0.29
  40      612.9       0.93        0.11     ← collapse, and coherence says "excellent"
```

If the run looks like that, the model has an early-stopping point around epoch 5
and a structural problem after it. Both are actionable, and neither is visible
from coherence alone.

---

## 4. Five falsifiable claims

State them, test them, publish whichever survive.

| # | claim | test | falsified if |
|--:|:---|:---|:---|
| 1 | The manifold improves prediction | §3 step 4 | PPL(3) ≥ PPL(2) |
| 2 | Learning is genuinely online | train sentence-at-a-time; measure PPL after each | no monotone improvement over the first 1,000 sentences |
| 3 | Correction is targeted | correct one association; measure PPL delta on a control set | control-set PPL rises materially |
| 4 | The model is small | measure the artifact | > 20 MB at 100k vocabulary |
| 5 | Learning is fast | measure µs/sentence | > 100 µs for a 10-token sentence |

Claims 4 and 5 will almost certainly pass once HOW 13's interning is done, and
they are the project's strongest marketing. Claims 1–3 are the research content.
Claim 3 in particular is a *differentiator* — no transformer can pass it — and it
is cheap to test.

---

## 5. The test harness

```rust
// tests/evaluation.rs
#[test] #[ignore]   // cargo test --ignored --release
fn full_evaluation() {
    let split = split_corpus(load_corpus(), 42);

    let kn = KneserNey::train(&split.train);
    let kn_ppl = kn.perplexity(&split.valid);

    let mut facet = Facet::new();
    let trainer = Trainer::new(LEARNING_RATE);
    let mut log = Vec::new();
    for epoch in 0..50 {
        for s in &split.train { trainer.train_sentence(&mut facet, s); }
        log.push(EpochMetrics {
            epoch,
            train_ppl: phiano_perplexity(&facet, &split.train),
            valid_ppl: phiano_perplexity(&facet, &split.valid),
            coherence: mean_coherence(&facet, &split.valid),
            phase_dispersion: phase_dispersion(&facet),
            sector_gini: sector_gini(&facet),
            vocab_size: facet.vocabulary_size(),
            mean_amplitude: facet.average_amplitude(),
        });
    }

    std::fs::write("data/evaluation.json", serde_json::to_string_pretty(&log).unwrap()).unwrap();
    println!("KN trigram valid PPL: {:.2}", kn_ppl);
    println!("Phiano best valid PPL: {:.2}", log.iter().map(|m| m.valid_ppl).fold(f64::MAX, f64::min));
}
```

One command, one JSON file, and every claim in this document becomes a number.

---

## 6. What to stop reporting

Until they are fixed, these produce figures that cannot support conclusions:

| metric | why |
|:---|:---|
| `coherence` alone | maximised by collapse (HOW 08 §3) |
| ARC score from `metrics/arc.rs` | prediction is a fixed template string (HOW 08 §6) |
| `metrics/baseline.rs` comparisons | all three score word-salad coherence |
| `generalization.rs` local/extreme | selects test items by the quantity it measures |
| `TransferResult.features_transferred` | counts feature sets; `apply` transfers nothing (HOW 14 §4) |

None of these needs deleting — each needs repointing at a held-out signal. The
harness above supplies one.

---

## 7. Why this is worth doing first

Every improvement proposed in HOW 01–14 is a hypothesis. Multi-phase
representation *should* help. Negative sampling *should* prevent collapse.
Binding *should* enable composition. Without a measurement loop, those remain
opinions, and the codebase accumulates changes whose effects nobody can see.

With a 200-line harness, each becomes a one-line experiment:

```
baseline (KN trigram)                 PPL 340
current Phiano                        PPL 512    ← worse; the manifold is costing you
+ negative sampling (HOW 02)          PPL 380
+ D=64 phases (HOW 01)                PPL 295    ← now beating the baseline
+ positional binding (HOW 03)         PPL 268
+ recurrent context (HOW 06)          PPL 241
```

That table is the project's roadmap and its evidence at the same time. It is also
the difference between a system that is claimed to learn anything and one that is
demonstrated to learn better than the obvious alternative.

Build the harness first. Everything else gets easier.

---

## 8. Checklist for this document

| Item | Status to establish |
|:---|:---|
| 80/10/10 split with no leakage | to build — `src/metrics/eval_split.rs` |
| Kneser–Ney trigram baseline | to build — ~120 lines |
| `next_distribution` on the generator | to build — refactor of `pick_ngram` |
| Four-way ablation | to run |
| Per-epoch dispersion + Gini logging | to add |
| Deprecate collapse-gameable metrics | to do |

---

**Next:** [HOW 16 — Learning Anything](16_learning_anything.md) — the capstone.
