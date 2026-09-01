# MEASURED RESULTS — first harness run

> _Every number below comes from `cargo run --release --bin evaluate`, on
> `data/rust_book_corpus.txt`, 7,757 sentences, 80/10/10 deterministic split,
> vocabulary 6,016. Raw log: `data/evaluation.json`._

---

## 1. The headline

| Model | Held-out perplexity |
|:---|---:|
| **Phiano — counts + absolute discounting + unigram back-off** | **124.66** |
| Kneser-Ney trigram baseline | 131.02 |
| Phiano — phase manifold as the back-off distribution | 1,034 |

Two findings, and they point in opposite directions.

**Phiano's n-gram layer beats the Kneser-Ney baseline** — 124.66 vs 131.02, a 5%
improvement, once its tables are wrapped in absolute discounting with a proper
hierarchical back-off. Before this work the same tables were raw maximum
likelihood, an unseen n-gram had probability exactly zero, and measured
perplexity was **119,644**.

**The phase manifold does not currently contribute to prediction.** This is the
question [HOW 15](15_proving_it_works.md) was written to settle, and the answer
is unambiguous.

---

## 2. The mixing sweep — the decisive experiment

`P_base(c | ctx) = γ · P_phase(c | ctx) + (1 − γ) · P_unigram(c)`

Everything else — the counts, the discount, the interpolation — is held
identical. γ = 0 removes the manifold; γ = 1 makes it the sole back-off. So the
curve *is* the measurement of what the manifold contributes.

| γ | perplexity | γ | perplexity |
|---:|---:|---:|---:|
| **0.0** | **124.66** ← best | 0.6 | 160.04 |
| 0.1 | 127.95 | 0.7 | 172.77 |
| 0.2 | 132.24 | 0.8 | 191.98 |
| 0.3 | 137.28 | 0.9 | 228.53 |
| 0.4 | 143.29 | 1.0 | 1,034.12 |
| 0.5 | 150.66 | | |

**Monotonically increasing.** The optimal mixing weight is exactly zero. Every
unit of phase information added makes prediction worse.

This is not a tuning artefact. `P_phase` is a softmax over **mean phase
coherence across 16 independent channels** — the multi-channel similarity the
torus representation exists for, not a single angle — and the curve has no
minimum anywhere in the interior.

---

## 3. Is it the objective? — yes, and by a factor of 27

The obvious hypothesis: centroid attraction teaches the manifold *what
co-occurs*, while a language model is scored on *what follows*. `bin/experiment`
tests it across three axes — training regime × context construction × softmax
temperature — and adds the control that makes the answer quantitative.

### The control

Mixing phase against a **unigram** back-off asks it to beat word frequency, which
is a strong and trivially available opponent. Mixing it against a **uniform**
base asks only whether it beats knowing nothing. On a log scale, uniform is the
floor and unigram is a reference point, so the fraction of that gap the phase
distribution closes measures how much the representation actually knows.

| training regime | uniform base | phase base | unigram base | signal recovered |
|:---|---:|---:|---:|---:|
| co-occurrence + ranking | 193.58 | 192.77 | 124.66 | **0.9%** |
| **ranking objective only** | 193.58 | **173.92** | 124.66 | **24.3%** |

Training the manifold on next-word ranking alone, rather than alongside centroid
attraction, takes it from carrying essentially nothing (0.9%) to carrying a
quarter of the predictive signal that raw word frequency provides — a **27×
improvement from changing the objective and nothing else**.

That is the clearest confirmation available of the argument in
[HOW 02 §7](02_the_kuramoto_step.md) and [HOW 16 §4](16_learning_anything.md):
centroid attraction is a *descriptive* target whose optimum is collapse, and a
predictive target is what makes a representation encode anything worth having.
The two objectives pull the phases in different directions; running both is
worse than running the useful one.

### The recurrent context helps too

Comparing context constructions at γ = 1 (phase alone as the back-off):

| context | β = 1.0 | β = 0.25 |
|:---|---:|---:|
| two-word centroid | 359.32 | 199.98 |
| **recurrent state** `h_t = λ_k e^{iω_k} h_{t-1} + z_t` | **275.90** | **183.43** |

The recurrent state beats the two-word centroid by 23% at β = 1 and 8% at
β = 0.25. Carrying the whole prefix at per-channel timescales is worth real
perplexity — this is the one component holding information a trigram table
structurally cannot have.

### But γ\* is still 0

Across all 20 configurations of context × temperature, mixed against a unigram
back-off, the optimal mixing weight remains **exactly zero**. Phase at its best
(173.92) is still well behind unigram frequency (124.66).

Note also which temperature wins: β = 0.25, the *flattest* setting tested.
Sharpening the phase distribution monotonically hurts. A distribution that is
best when it is closest to uniform is a distribution that does not know much —
which is the same conclusion the recovery percentage reports, arrived at
independently.

## 4. Collapse is fixed

| epoch | valid ppl | coherence | **dispersion** | gini |
|---:|---:|---:|---:|---:|
| 0 | 1,173.00 | 0.575 | 0.959 | 0.092 |
| 1 | 1,509.08 | 0.650 | 0.959 | 0.090 |
| 2 | 1,731.88 | 0.551 | 0.944 | 0.102 |

Phase dispersion holds at **0.94–0.96** (1.0 = uniformly spread, 0.0 = every word
at one angle) and sector Gini stays near 0.09. The negative-sampling term is
doing its job: the manifold no longer synchronises toward a point.

The unit test `test_repulsion_prevents_collapse` asserts this directly, by
training the same corpus with and without repulsion and comparing dispersion.

Note also that **coherence moves independently of perplexity** — it rises from
0.575 to 0.650 between epochs 0 and 1 while held-out perplexity gets 29% worse.
That is [HOW 08](08_self_scoring.md)'s argument, now visible in data.

---

## 5. Overfitting

Held-out perplexity is **best at epoch 0** and rises monotonically thereafter,
while training perplexity falls (9.01 → 4.85). One pass over this corpus is the
right amount of training; the harness reports `best_epoch` so this is no longer
something anyone has to guess.

---

## 6. What this means

Read plainly:

- The **infrastructure claims hold**. Contrastive training prevents collapse.
  Identity seeding separates same-length words. Binding makes word order survive.
  The counts, properly smoothed, beat a standard baseline.
- The **representation carries real but insufficient signal**. It beats uniform;
  it loses to unigram frequency. At its best it recovers 24.3% of what counting
  words provides.
- The **objective was the binding constraint**, and it was measurable. Switching
  from co-occurrence to ranking moved the manifold from 0.9% to 24.3% — 27×,
  from one change.

Capacity was necessary and not sufficient. The channels are independent now
(`test_torus_reads_independent_channels` proves the torus is no longer a 1-D
curve), and what fills them usefully is the objective, not the width.

## 7. What to try next, in order

The three axes already swept say where the remaining slope is.

1. **Train on ranking only, for longer, and measure.** The 27× came from a few
   passes. `Harness::train_ranking_only` takes a pass count; sweep it. This is
   the single highest-slope direction identified so far.
2. **Wire the recurrent state into the trainer, not just the scorer.** It is
   worth 23% at scoring time while the trainer still uses a two-word window, so
   the representation is being taught against a weaker context than the one it
   is evaluated on.
3. **Widen `LM_CHANNELS` past 16** toward the full 64, now that there is a metric
   that will say whether the extra channels carry anything.
4. **Reconsider the count/phase split.** Phase is losing to unigram at the
   back-off position. It may be the wrong job for it — the manifold may be
   better used to *re-rank* the n-gram candidate set, where it only has to break
   ties among plausible continuations rather than model the whole vocabulary.

Every one of these is one command to evaluate, and the answer is a number.

---

## 8. Reproducing

```bash
cargo run --release --bin evaluate                        # per-epoch training curve
cargo run --release --bin experiment                      # the full experiment grid
cargo run --release --bin experiment -- <corpus> <epochs>
cargo test                                                # 94 tests
```

Output: `data/evaluation.json`, `data/experiment.json`.
