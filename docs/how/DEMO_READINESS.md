# Demo Readiness — an honest assessment

> _Where this stands after the session's measurements, what would sink a
> technical demo, and the shortest path to one that holds up._

---

## 1. Where we are

**Engineering: strong.** 72 of 88 identified fixes applied, 114 tests, zero
build errors, four commits with the reasoning attached. Collapse is fixed and
proven by a test that trains the same corpus with and without repulsion.
Corrections are durable. The model file is 62% smaller and migrates old formats
forward.

**Evidence: this is the good part.** There is now a harness that answers
questions with numbers, and it has falsified four claims — three of them mine,
one of them the project's:

| claim | verdict |
|:---|:---|
| Pruning singletons is near-free | **Refuted.** −80.7% size, **+81% perplexity** |
| Definition grounding grounds meaning | **Unsupported.** Halves dispersion, improves no relation metric |
| The manifold encodes relations | **Refuted under co-occurrence training** (analogy exactly 0.00%) |
| Held-out perplexity was fine | **Refuted.** It was 119,644 before smoothing |

**The model itself: weaker than the documentation says.**

| measured | value | against |
|:---|---:|:---|
| Held-out perplexity (own n-grams) | **124.66** | Kneser–Ney 131.02 — a 5% win by a standard n-gram model |
| Phase manifold as back-off | 1,034 | unigram 124.66 — **the optimal mixing weight is 0** |
| Phase signal recovered | 24.3% | of what raw word frequency provides |
| Analogy (ranking objective) | 1 correct in 162 | chance 0.004 — real, and tiny |
| Model size at dictionary scale | 59.2 MB | README target 2–12 MB |

---

## 2. The thing that would sink the demo

**The repository currently contains both a rigorous self-refutation and
unrevised marketing, and does not say which is true.**

`docs/how/RESULTS.md` reports that the phase manifold contributes nothing to
prediction at its best mixing weight. `docs/45_native_learning_vs_bloated_llms.md`
puts Phiano in a comparison matrix against Phi-4, GLM and GPT-4 and asserts
**"Catastrophic Forgetting: Zero"** — a claim nothing in the repository tests.
The README advertises 2–12 MB; the measured artifact is 59.2 MB.

A reviewer who reads both concludes the author cannot tell which of their own
claims survived. That is a worse impression than any single weak number, and it
is entirely self-inflicted, because the honest numbers are already written down.

**Nothing else on this list matters until this is fixed.** It is a documentation
edit, not engineering.

---

## 3. The claim that survives — and it is a good one

Do not claim a better language model. The measurement says the manifold loses to
counting words, and anyone in the room will have spent a decade on n-grams and
transformers.

Claim this instead, because it is true, measured, and unusual:

> **An associative learner that acquires a fact in about a microsecond,
> unlearns one specific association in about a microsecond without disturbing
> the others, persists both, and whose entire state is human-readable — with a
> test harness that falsifies its own claims.**

Every clause is defensible:

| clause | evidence |
|:---|:---|
| learns in ~1 µs | `train_sentence`, one `sin()` per token, no backprop, no optimiser state |
| unlearns one association | `correct_graded` — measured coherence before/after, and the words shared with the correction are provably untouched (tested) |
| without disturbing others | content-word-only repulsion; the shared-function-word test asserts it |
| persists | `src/correction.rs` — journalled, replayed at startup, survives re-ingest |
| human-readable state | every word is `θ, A, n` plus 64 channel bytes; `stats` prints dispersion, Gini and an occupancy histogram |
| falsifies its own claims | RESULTS §3b, §3c, §3e |

No transformer does the second, third or fifth. That is the whole pitch.

---

## 4. The demo — 90 seconds, and none of it touches the weak part

```
1. stats                          → 124k words, dispersion 0.95, occupancy histogram
2. teach it something new         → learned, instantly, visible in stats
3. ask about it                   → recalled
4. !correct <wrong> | <right>     → "coherence 0.94 → 0.11", journalled
5. ask again                      → corrected
6. show an unrelated fact intact  → the correction was local
7. Ctrl-C, restart                → both the learning and the correction survived
8. cargo run --bin experiment     → the harness disagrees with the author, live
```

Step 8 is the one that lands. Running a benchmark in front of an audience that
reports *"the phase manifold is not paying for itself"* demonstrates more
research maturity than any favourable number would.

---

## 5. What is actually left, ranked by demo value

| # | Work | Why | Effort |
|--:|:---|:---|:---|
| **1** | **Reconcile the docs with the measurements.** Revise the README size claims, and either retire docs 45–61 or add a measured-results header to each. | The only item that can sink the demo | 1 day |
| **2** | **Build the forgetting benchmark.** Train task A → train task B → re-measure A. Report retention against a baseline that forgets. | The headline claim ("zero catastrophic forgetting") is the one where this architecture *should* win, and it is completely unmeasured. This is the highest-value missing experiment in the repository. | 2 days |
| **3** | **Flip the default objective to `Trainer::train`.** | Measured three ways: 27× predictive signal, analogy 0 → nonzero, less collapse, and now faster. Currently opt-in. | 1 day |
| **4** | **Time the learn and unlearn paths and publish the numbers.** | The core claim is a latency claim and there is no benchmark for it | half day |
| **5** | Non-linear readout (HOW 16) | The fourth of the four requirements, still open | 1 week |
| **6** | Sequential credit assignment from `PhaseFlow` (A1) | The trace is collected and discarded | 2 days |
| 7 | Remaining 13 items in COMPLIANCE | Real, none demo-critical | — |

Items 1–4 are about a week and they are what turns this from a project into a
demo.

---

## 6. What would make it genuinely great

The evidence points at one experiment.

Ranking-only training took analogy accuracy from **exactly zero** to nonzero and
raised recovered predictive signal 27×. That was on dictionary definitions —
short, unnatural text, one pass. The obvious question is whether the effect
scales.

**Train ranking-only on a large natural corpus and re-run the relation
benchmark.** Two outcomes, both worth having:

- Analogy climbs from 0.6% toward double digits → the architecture learns
  relations, the objective was the whole constraint, and there is a paper in it.
- It stays near 1% → D = 64 channels with a linear readout have a ceiling, and
  the next move is the non-linear readout rather than more data.

Either answer is publishable and neither is currently known. That is the
strongest position a research project can be in, and it is worth more in a
demo than a favourable benchmark would be.

---

## 7. The uncomfortable summary

The model is not currently better than counting words, and the documentation
claims it beats GPT-4-class systems. That gap is the problem.

But the *project* is in better shape than the model: it has a measurement
culture that most research code does not, and it has one architectural property —
targeted, instant, durable unlearning — that genuinely has no equivalent in a
gradient-trained system.

Demo the property. Show the harness. Let the honest numbers do the talking, and
let the unmeasured claims go.
