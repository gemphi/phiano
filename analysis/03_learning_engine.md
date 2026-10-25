# 03 — The Learning Engine: How Phiano Actually Learns

> Files examined: [`src/trainer/mod.rs`](../src/trainer/mod.rs), [`src/facet/mod.rs`](../src/facet/mod.rs),
> [`src/cognitive/grounding.rs`](../src/cognitive/grounding.rs), [`src/oscillator/train.rs`](../src/oscillator/train.rs),
> [`docs/06_kuramoto_coupling.md`](../docs/06_kuramoto_coupling.md).

This file is the mechanical answer to *"how would it learn?"* — every learning rule in
the codebase, stated exactly as implemented, then classified by what kind of learning it is.

---

## 1. The Primary Rule: `train_sentence` (Kuramoto Phase Attraction)

[`Trainer::train_sentence`](../src/trainer/mod.rs) processes one sentence in five moves:

**Step 1 — Initialize unseen tokens** deterministically:
`φ₀ = len(token)·φ_golden mod 2π`, amplitude 1.0, band 1.

**Step 2 — Record sequential structure** (three tables, [`Facet`](../src/facet/mod.rs)):
- bigram counts `w_a → w_b`
- trigram counts `w_a w_b → w_c`
- **phase lags**: `β_ij ← 0.92·β_ij + 0.08·(φ_j − φ_i)` (EMA, rate 0.08,
  [facet/mod.rs:195–208](../src/facet/mod.rs)) — the observed angular displacement from
  word i to word j in running text.

**Step 3 — Compute the sentence's semantic centroid**: the amplitude-weighted circular
mean `Θ = atan2(Σ A·sinφ, Σ A·cosφ)` ([trainer/mod.rs:157–168](../src/trainer/mod.rs)).

**Step 4 — Pull every token's phase** toward two forces
([trainer/mod.rs:82–122](../src/trainer/mod.rs)):

```text
semantic_force = sin(Θ − φ_i)                        # pull to sentence centroid
syntax_force   = mean over neighbors j of:
                   sin(φ_prev − φ_i + β_prev)          # lag-coupled predecessor
                   sin(φ_next − φ_i − β_next)          # lag-coupled successor

combined = 0.7·semantic + 0.3·syntax   (syntax weight only when neighbors exist)
φ_i ← (φ_i + 0.05·combined) mod 2π
```

**Step 5 — Reinforce**: amplitude `A ← min(A + 0.001, 2.0)`; if
`|semantic_force| < 0.0005` the word is already phase-locked with the sentence, so
`band_n += 1` — a novel anti-collapse device that moves *converged* words to a new
sub-band instead of letting them collapse into identical phases.

### What kind of learning is this, formally?

It is **online, single-pass, co-occurrence-driven clustering on the circle** — a
Hebbian-cousin rule ("words that appear together phase-align together") in the same
family as random indexing and topic co-occurrence accumulation, but expressed in
circular statistics. It is *not*:
- supervised (no labels),
- gradient-based (no loss function, no backprop),
- Bayesian (no posterior),
- deep (no composition of learned layers).

What it *is*: **O(L) per sentence (L = tokens), zero retraining, zero replay, fully
invertible state (the phasors are the model), and provably bounded updates**
(step ≤ learning rate 0.05, always). Convergence behavior is inherited from
Kuramoto theory: phases of repeatedly co-occurring words relax to a synchronized
cluster; the order parameter r (used as "coherence", file 08 §5) rises accordingly.

## 2. Secondary Learning Rules (All Working, All Online)

| Rule | Where | What it does |
|---|---|---|
| **Definition grounding** | [`DefinitionGrounder::ground_phases`](../src/cognitive/grounding.rs) | Re-seeds a word's phase to the circular centroid of its dictionary definition's words (half-way nudge, `0.5·Δθ`). Converts "cat = small domestic feline…" into phase placement |
| **Recursive dictionary self-study** | [`learn_definition_chain`](../src/trainer/mod.rs) | BFS over the chunk dictionary: unknown word → train on definition → recurse into *its* unknown words (depth 3 default, amplitude > 5.0 stops expansion). The system literally looks words up and teaches itself |
| **Negative feedback / self-correction** | [`correct_mistake`](../src/trainer/mod.rs) | On "no, X means Y": X's tokens get a **π-radian anti-phase pulse** + amplitude × 0.8, then Y is trained. Instant unlearning of an association — impossible in a frozen transformer without fine-tuning |
| **Multi-epoch with warmup + early stop** | [`train_multi_epoch`](../src/trainer/mod.rs) | LR ramps `(e+1)/warmup` for first epochs; stops when max phase shift < 5·10⁻⁴. Mirrors standard warmup schedules, self-applied per text |
| **Pairwise oscillator training** | [`OscillatorTrainer::train_epoch`](../src/oscillator/train.rs) | All-pairs `Δφ_i += sin(φ_j − φ_i)·lr` with amplitude weight-decay 0.001 — a purer Kuramoto pass used by the oscillator field view |
| **Persona / fingerprint statistics** | [`src/persona/`](../src/persona) | Sector histograms over a speaker's text; similarity/difference vectors between speakers |

## 3. What Is Learned vs. What Is Fixed (The Crucial Table)

| Component | Learned? | Mechanism |
|---|---|---|
| Word phases φ | **Yes** | Kuramoto pull, per sentence |
| Familiarity A | **Yes** | +0.001 per occurrence, cap 2.0 |
| Sub-band n | **Yes** (ratchet) | +1 on convergence |
| Pairwise phase lags β_ij | **Yes** | EMA 0.08 over observed orderings — *the only learned parameters of the dynamics themselves* |
| Bigram/trigram tables | **Yes** | Counting |
| Attention weights | **No** | Fixed 8-head phase-sector formula, no learnable Q/K/V (file 07 §5) |
| 0.7/0.3 semantic-syntax mix | **No** | Hard-coded constant |
| Phase-kick 0.35, momentum 0.15/0.85 | **No** | Hard-coded decode constants |
| Speech-act classifiers | **No** | Keyword lists in JSON |
| POS tagger | **No** | Word lists + -ing/-ed/-s suffix rules |
| Composer templates | **No** | Hard-coded English frames |

**Reading:** Phiano is a *fixed dynamical system with adaptive state* — like a reservoir
computer whose reservoir state is the entire knowledge store. This is exactly why it
cannot diverge or hallucinate training instability, and exactly why its expressivity
ceiling is set by its fixed coupling structure (file 12 §5 develops the consequence:
to "learn anything," some of the fixed constants must themselves become learned — the
β_ij EMA already points the way).

## 4. The Most Original Mechanism: Learned β_ij Phase Lags

Standard n-gram models store *counts*; Phiano additionally stores **the circular
displacement each transition induces**: after seeing "the cat sat", the model has not
only P(sat | the cat) but *the angle from `the→cat` and `cat→sat`*. Generation then
uses `facet.phase_lag(prev, word)` to kick the decoding phase
([generate.rs:320–340](../src/generate.rs)) — syntax becomes a **dynamical steering
signal** rather than only a lookup table. To my knowledge this packaging —
transition-conditioned phase coupling learned by EMA — is the project's most defensible
novelty claim, and it is testable against n-gram baselines (file 16, task 4).

## 5. Learning Properties Scorecard

| Property | Verdict | Note |
|---|---|---|
| Online / incremental | **Yes** — sub-millisecond per sentence | The system's defining property |
| Stable / bounded | **Yes** | Updates ≤ lr·1; no exploding state; amplitude capped |
| No catastrophic vocabulary forgetting | **Yes** | New words never overwrite old phasors |
| Soft associative drift | **Yes (by design, but unmeasured)** | Old associations decay as words appear in new contexts — see file 10 §3 |
| Self-supervised | **Yes** | Raw text only; no labels needed |
| Supervised / RL capability | **No** | No loss, no credit assignment across steps |
| Meta-learning (learning the learner) | **Seeds only** | β_ij EMA + capacity grid search; all macro-constants fixed |
| Theoretical convergence guarantees | **Partial** | Inherits Kuramoto lock-in heuristics; no formal proof in repo |

## 6. Bottom Line

Phiano's learning is **real, fast, stable, and persistent** — a legitimate member of the
online-embedding family with one genuinely novel organ (learned pairwise phase lags).
Its limits are equally structural: nothing in the current rules learns *structure*
(grammar, composition, inference); structure is either counted (n-grams) or hard-coded
(rules/templates). File 04 shows how the learning cycle wraps this engine; file 12 asks
what it would take for this engine family to learn anything at all.
