# Phiano — Architecture Evaluation

> _An assessment of the SSI model at `phiano/`: what it does, what it can learn,
> what it structurally cannot yet learn, and the ranked set of changes that would
> make "learns anything" a defensible claim._
>
> Companion to the [HOW series](how/00_index.md), which derives every finding
> below from source with worked examples.

---

> **Update — measured.** The harness in §8 step 0 now exists and has been run.
> Phiano's count layer, properly smoothed, scores **124.66** held-out perplexity
> against a Kneser-Ney trigram baseline of **131.02** — it wins. The phase
> manifold as a back-off distribution scores **1,034**, and the optimal mixing
> weight between the two is **exactly zero**. Phase dispersion now holds at 0.95
> across training, so collapse is fixed.
>
> A follow-up experiment grid isolates why. Against a uniform base, the manifold
> trained on co-occurrence recovers **0.9%** of the predictive signal that word
> frequency provides; trained on the next-word ranking objective alone it
> recovers **24.3%** — a **27× gain from changing the objective and nothing
> else**, and the clearest confirmation available of §7's argument that the
> training target, not the representation width, was the binding constraint.
> Full numbers: [`how/RESULTS.md`](how/RESULTS.md).

## 1. Verdict in one paragraph

Phiano is a **well-engineered online associative learner** built on a genuinely
interesting substrate. It learns from a single pass in microseconds, unlearns a
specific fact in microseconds, stores 16 bytes per word, and can be read by a
human at any moment — three properties that no transformer has and that define a
real deployment niche. It is **not currently** a system that can learn arbitrary
structure, for four specific and independently fixable reasons: the
representation is one-dimensional, the composition operator is order-blind, the
training objective's global optimum is total collapse, and the model class is
linear. None of those requires abandoning the phase-manifold design. All four
have well-understood fixes that preserve the properties that already work.

---

## 2. What is genuinely strong

These are not consolation prizes. They are the reasons the architecture is worth
developing.

| Strength | Evidence | Why it matters |
|:---|:---|:---|
| **Microsecond online learning** | `Trainer::train_online` — O(L), one `sin()` per token, no backprop, no optimiser state | A transformer needs hours of GPU fine-tuning to learn one new fact. Six orders of magnitude. |
| **Targeted O(1) unlearning** | `Trainer::correct_mistake` — π anti-phase pulse | No gradient-trained model can reliably unlearn one association without disturbing everything else. This is a genuine architectural advantage. |
| **Definition grounding** | `DefinitionGrounder::ground_phases` | Positions derived from dictionary semantics, no labels, seconds of compute, fully auditable. The best idea in the codebase. |
| **The curiosity loop** | `Envision::detect_gaps` + `Trainer::learn_definition_chain` | Detects a gap, names it, fetches a definition, trains, recurses. That is an active-learning agent, and the control loop is correct. |
| **Footprint** | 16-byte phasors; ~2 MB for 100k words of lexicon | Fits in microcontroller flash. No CUDA, no runtime, one static binary. |
| **Interpretability** | `θ = 2.31, A = 1.4, n = 7` is the complete state of a word | Auditable end to end. In regulated settings this is worth more than accuracy points. |
| **Code quality** | Clean module boundaries, **68 tests (verified: 62 in `src`, 6 in `tests/ch14_integration.rs`)**, `select_nth_unstable` before sort, rayon parallelism, `#[serde(default)]` on new fields, a legacy-format load fallback | This is careful Rust, not a prototype. |
| **Hybrid decoding design** | `pick_ngram` — log-damped counts × phase alignment × resonance × content weighting | A thoughtfully weighted combination of a count model and a continuous manifold. |

---

## 3. What blocks "learn anything"

Four structural limits, each with the file and line that creates it.

### 3.1 Capacity — one dimension

`SpectralPhasor` stores a single angle. Semantic space is S¹. At the system's own
64-sector resolution that is **64 distinguishable states for all of English**.

`TorusPhasor`'s 32 harmonics do not help: every harmonic is a deterministic
function of the same `phase` (`src/phasor.rs :: from_spectral`), so the torus
contains exactly one number's worth of information — a 1-D curve embedded in T³².

A compounding defect: the initial phase is `token.len() × φ`, which depends only
on **word length**. `cat`, `the`, `war` and `dog` all initialise at exactly
4.854102 rad. A 100,000-word vocabulary starts from ~20 distinct positions.

→ [HOW 01](how/01_word_to_number.md)

### 3.2 Objective — the optimum is collapse

`train_sentence` moves every token toward the sentence centroid. Every force is
attractive; nothing anywhere pushes two words apart during training. Kuramoto
with all-positive coupling has one stable attractor: full synchronisation. High-
frequency function words appear in every sentence and transitively couple the
entire vocabulary to it.

And the metric that is used to judge training — `coherence` — **is** the Kuramoto
order parameter, which is maximised at total collapse. Coherence rises as the
model degrades, and it reads 1.0 on a fully collapsed lexicon for any input
including nonsense.

Worse, on a *fresh, untrained* model, `"the cat sat on the mat"` already scores
coherence 1.0 — because four of its five tokens are three letters long and
therefore share a seed position.

→ [HOW 02](how/02_the_kuramoto_step.md), [HOW 08](how/08_self_scoring.md)

### 3.3 Composition — order is encoded, then discarded

The trainer carefully learns per-pair directional phase lags β_ij
(`Facet::record_phase_lag`, EMA rate 0.08) — a real syntactic parameter. Then:

```rust
// src/wave.rs :: Wave::sentence
.map(|p| p.to_complex()).sum()      // addition is commutative
```

`Z("dog bites man") == Z("man bites dog")`, bit for bit. Every consumer inherits
this: `Evaluator::eval`, `ray_cast`, `ContextWaveBuffer`, `Memo::record`, persona
fingerprints, composition scoring. Negation scope, argument roles, causality,
arithmetic and code semantics are all out of reach.

→ [HOW 03](how/03_learning_word_order.md), [HOW 06](how/06_sentence_superposition.md)

### 3.4 Function class — linear

Every operation is a sum or a multiplication by a constant, followed by argmax. A
composition of linear maps is linear. No universal approximation, regardless of
how many tiers the architecture diagram shows.

→ [HOW 16 §5](how/16_learning_anything.md)

---

## 4. Where the capability actually comes from today

This matters for attribution. Phiano contains **two** learners:

1. `Facet::bigrams` / `Facet::trigrams` — a classical n-gram model, same data
   structure and estimator as a 1990s trigram LM.
2. `Facet::lexicon` — the phase manifold, the novel contribution.

`Generator::attractor_select` tries trigrams first, then bigrams, then ray-cast.
So the n-gram tables determine **what is grammatical**; the manifold **re-ranks
within** that set. Fluent output is evidence about the HashMap, not about the
manifold.

**The contribution of the phase layer is currently unmeasured**, and that is the
single most important open question about the system. One ablation settles it
(HOW 15 §3).

Note also that under collapse (§3.2), `phase_align → 1.0` for every candidate and
the generator degrades smoothly into a pure n-gram sampler — with no error, no
warning, and rising coherence scores.

---

## 5. What the current benchmarks measure

Read carefully before quoting any figure from `src/metrics/`.

| Module | What it does | What it is not |
|:---|:---|:---|
| `metrics/arc.rs` | `predicted = format!("{} relates to the pattern", input)` — a fixed template, no inference; correctness = does the expected answer's first word appear in that template | not an ARC score |
| `metrics/baseline.rs` | all three "baselines" build an 8-word salad and score its **coherence** | not a baseline comparison; under collapse all three → 1.0 |
| `metrics/generalization.rs` | scores template sentences on word sets selected **by phase distance** | close to circular |
| `lifelong` `TransferResult` | reports `features_transferred` | `FeatureReuse::apply` writes one synthetic `meta_sector_N` token per feature set, overwritten in a loop; nothing is transferred |
| `eval` weights | `constants.rs` says 0.4/0.3/0.3; `eval.rs` doc comment says 0.45/0.40/0.15; behaviour is in `config/functions.rs` | three descriptions, at most one correct |

`resonance` (fraction of tokens known) and the `Verdict` enum are the parts of
the evaluator that are honest and worth keeping.

---

## 6. Concrete defects found (independent of architecture)

Each is small, verified against source, and worth fixing regardless of the larger
programme.

| # | Defect | Location | Impact |
|--:|:---|:---|:---|
| 1 | Seed phase depends only on token length; ~20 distinct seeds for any vocabulary | `Trainer::initialize_tokens` | high |
| 2 | State is saved **only** on clean `exit`/`quit` — Ctrl-C or crash loses the session | `Model::iterate` / `Model::scale` | high |
| 3 | Load failure silently starts from an empty facet, no warning | `Model::new` | high |
| 4 | `Storage::save` is not atomic; an interrupted write corrupts the model | `src/storage.rs` | high |
| 5 | `amplitude > 5.0` early-exit is dead code (`AMPLITUDE_MAX` is 2.0) | `learn_chain_recursive` | medium |
| 6 | `ground_phases` re-runs over the whole dictionary on **every** startup | `Model::new` | medium |
| 7 | `common_phases` is computed, stored, serialised, never read | `lifelong/meta.rs` | medium |
| 8 | `FeatureReuse::apply` inner loop overwrites the same synthetic word | `lifelong/reuse.rs` | medium |
| 9 | `signature_similarity` threshold 0.6 is ~0.7σ above chance (~24% false matches) | `synthesis/library.rs` | medium |
| 10 | `format!("{} {}", a, b)` allocates on every trigram **lookup**, in the decode loop | `Facet::trigram_candidates` | medium |
| 11 | `Memo` stores every entry twice (in `entries` and in `layers[]`) | `Memo::record` | low |
| 12 | `band_n` is unbounded; ~861 increments wrap the entire circle | `Trainer::train_sentence` | low |
| 13 | `speakable()` rejects all numerals — the model cannot emit a number | `Generator::speakable` | medium |
| 14 | Correction applies the π pulse to function words present in **both** phrases | `correct_mistake` | medium |
| 15 | Hard 20-token generation cap silently overrides `max_tokens` | `Generator::decode` | low |
| 16 | `HierarchicalPhaseField` is built for display only; no retrieval path uses it | `src/layers.rs` | medium |
| 17 | `Memo` has no `recall()` — the episodic record is never read back into inference | `src/memory/mod.rs` | high |
| 18 | Model artifact is 92 MB against a documented 2–12 MB target | `data/manifold.chroma` | high |
| 19 | `save` clones the entire model before writing (~2× peak RAM) | `SerializedFacet::from_facet` | medium |
| 21 | `worse.rs` penalises nothing and prunes nothing — losers are computed, printed, dropped; only winners are trained. In the 1968 ancestor `worse` is the search guard | `Discarder::discard_and_train` | high |
| 22 | Compose fitness is `eval.overall` (40% coherence) — the tournament selects *for* collapse, then trains on the winners | `better.rs` + `tune.rs` | high |
| 23 | `WEIGHT_NOVELTY` used twice in `comp_score`; weights sum to 1.15, so the score is off-scale | `evaluate_variations` | medium |
| 25 | Compose scoring is a flat weighted sum, so quality dimensions are mutually purchasable; Huberman's lexicographic (stage, measure) order forbids this | `better.rs` | medium |
| 24 | `PhaseFlow::record_step` builds a per-step eligibility trace that nothing consumes | `generate.rs` / `worse.rs` | medium |
| 20 | `ChromaHeader.version` is written but never checked on load | `Storage::load` | low |

---

## 7. The path to "learns anything"

Four requirements, four changes. Each preserves everything in §2.

| Requirement | Change | Cost | Effect |
|:---|:---|:---|:---|
| **Capacity** | `phases: [u8; 64]` — D independent channels per word | 72 bytes/word; 100k vocab = 7.2 MB | 64 states → effectively unbounded; makes `TorusPhasor::resonance` meaningful for the first time |
| **Composition** | Binding = phase addition (HRR / VSA); positional roles via `GOLDEN_ANGLE` | 4 lines for positional; ~50 for role binding | `dog bites man` ≠ `man bites dog`; propositions become representable and queryable |
| **Objective** | Negative sampling + a hinge loss on next-word retrieval | ~40 lines, still online, still no backprop | collapse stops; the model becomes predictive rather than descriptive |
| **Non-linearity** | Sector-indexed lookup table, or magnitude gating | ~60 lines | function class stops being linear |

Add the recurrent complex context state
$h_t = \lambda e^{i\omega} h_{t-1} + z_t$ (HOW 06 §7b) and the system becomes a
**diagonal complex linear state-space model with vector-symbolic binding, trained
by online contrastive prediction** — every component of which is an established,
current architecture class, combined in a way that keeps microsecond updates,
targeted unlearning and full interpretability.

---

## 8. Recommended order of work

Measured after every step. Steps 0–6 are ~2 weeks and need no architectural
change.

| # | Work | Doc | Effort |
|--:|:---|:---|:---|
| **0** | **Evaluation harness**: 80/10/10 split, Kneser–Ney trigram baseline, held-out perplexity, per-epoch phase-dispersion + sector-Gini logging | [15](how/15_proving_it_works.md) | 2 days |
| 1 | Hash-based seeding instead of `len × φ` | [01](how/01_word_to_number.md) | 1 hour |
| 2 | Negative sampling in `train_sentence` | [02](how/02_the_kuramoto_step.md) | half day |
| 3 | Intern vocabulary to u32 IDs; add Kneser–Ney smoothing; prune singletons | [04](how/04_cooccurrence_memory.md), [13](how/13_persistence_and_cost.md) | 3 days |
| 4 | Positional binding in `Wave::sentence` | [03](how/03_learning_word_order.md) | 1 hour |
| 5 | Atomic save, periodic checkpoint, Ctrl-C handler, loud load failures | [13](how/13_persistence_and_cost.md) | half day |
| 6 | `Memo::recall` wired into generation context | [12](how/12_memory_layers.md) | 1 day |
| 7 | **D = 64 multi-phase representation** | [01](how/01_word_to_number.md) | 1 week |
| 8 | Predictive objective (hinge on next-word retrieval) | [02](how/02_the_kuramoto_step.md) | 1 week |
| 9 | Role binding + unbinding | [03](how/03_learning_word_order.md) | 1 week |
| 10 | Recurrent complex context state | [06](how/06_sentence_superposition.md) | 3 days |
| 11 | Non-linear sector read-out | [16](how/16_learning_anything.md) | 1 week |

**Step 0 first.** Every item below it is a hypothesis; without a measurement loop
nobody can tell which ones worked.

---

## 9. Positioning

The honest claim is stronger than the inflated one.

**Where this architecture could plausibly be excellent:**

- On-device personal models that learn from corrections in microseconds and never
  stop learning.
- Domain-specialised assistants where a 10 MB model trained live on one
  organisation's documents beats a generic large model.
- Continual learning where retraining is impossible — embedded, air-gapped,
  privacy-constrained.
- Any setting where the answer must be auditable, or where a correction must take
  effect immediately and provably disturb nothing else.

**Where it will not compete:** frontier reasoning, and broad general knowledge
against models trained on trillions of tokens. It has neither the parameters nor
the data, and does not need them for the niche above.

"The model that learns anything, instantly, on your device, and that you can
correct in a microsecond" is a real and largely unoccupied position, and it is
reachable from the current codebase. That claim is worth more than an unfalsified
comparison to GPT.

---

## 10. Closing

The instincts in this project are good ones: phase as representation, oscillator
dynamics as learning, dictionaries as semantic grounding, curiosity as a control
loop, tiny auditable models as a deployment target. Several of these are
underexplored by the mainstream, and the engineering that surrounds them is
careful.

The distance between the current system and the claim on its title page is not a
distance of vision. It is **eleven numbered changes and a measurement loop**.

Build the harness. Then work down the list.

---

**Full derivations, worked examples and source citations:** [the HOW series](how/00_index.md),
plus [Appendix A1](how/A1_better_worse_lineage.md) on the `better`/`worse` compose
tournament and its lineage from Barbara Huberman's 1968 Stanford chess-endgame thesis.
