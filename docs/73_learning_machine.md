# 73 - The Learning Machine, and What It Says Adds Value

> _Every hand-tuned number in the generator's scorer replaced by a weight fit to held-out text; the fitted weights are the value audit._

`cargo run --release --bin learn` (fit on the validation split, evaluate on the test split; Rust Book), then `cargo run --release --bin generate_eval` (60 held-out prompts, Kneser-Ney judge). One seed.

---

## 1. What was hard-coded

`Generator::score_candidates` scored an n-gram candidate as

```
ln(1 + min(count, 24)) · (0.35 + 0.25·phase_align + 0.40·resonance) · content · repeat · operator
        content = 0.55 for a function word, 1.35 otherwise;  repeat = 0.15 if emitted in the last 12 tokens
```

Six constants, none measured. `src/learn` replaces the scorer with a **log-linear model** `score = Σ_f w_f x_f` over eight features and fits `w` by full-batch Adam on the negative log-likelihood of the *true* next word among the n-gram's top-64 candidates (`Tuner`, convex, early-stopped, finite-difference-free since the gradient is closed-form; a planted-weight recovery test guards it). One feature definition (`learn::features`) serves both the tuner offline and the generator online, so what is fit is what is used. Weights persist in `data/learned.json` (`LEARNED_FILE`); `Generator::new` loads them when present and falls back to the hand scorer otherwise.

Two of the eight features are new, and they are the paradigm's own: **`relational`** — `Facet::relational_score(ctx, cand)`, the mean of `on(c_j, cand, k_j)` over the context words `c_j` whose pair with the candidate carries evidence, each pair read on the channel its condition selects (offset or marker) — and **`relational_pairs`**, `ln(1 + pairs)`. This is the Difference Engine reading: every context word is a wheel, the condition between it and the candidate's position is the gear that selects the channel, and the candidate's target wheel on that channel is read against the context word's source wheel. Nothing is averaged into a context vector first; every pair keeps its own channel.

## 2. The fit

```
scorer                     held-out NLL   perplexity among the candidates
counts only                    2.1328              8.44
hand-tuned (as shipped)        2.5206             12.44
learned                        1.9855              7.28        (best at epoch 31 of 300)
```

The hand scorer was *worse than counting*. The learned scorer is 14 % better than counting on the test split.

```
feature             weight     verdict
ln_count            +1.509     the n-gram count, as expected
phase_align         +0.020     the manifold's single phase, cos(φ_w − φ_target): nothing
resonance           +0.026     the flow's collective phase: nothing
function_word       +0.301     function words are *more* likely, not less (the hand scorer penalised them ×0.4)
repeat              +0.551     real text repeats function words; fit, but see §3
operator_cos        +0.612     the directed operator's cosine: adds value on held-out text
relational          +0.702     the pairwise directed reading: adds the most after the count
relational_pairs    +0.504     and more evidenced pairs is better
```

This is the value audit the user asked for, done by the data rather than by hand. The two features that carry the paradigm — the relational reading and the operator — earn their weights. The two that carry the *old* manifold — a single phase per word compared to a target phase, and the collective phase of a flow — are at zero, on held-out text, with the n-gram count in the model. They stay as features so the fit can keep saying so; the hand scorer's confidence in them (0.25 and 0.40 of its non-count mass) was misplaced.

## 3. Generation with the learned scorer

```
arm                                        KN ppl    eval    distinct   tokens
hand scorer, unguided                       13.6    0.730     0.966     20.0
hand scorer, directed operator w = 0.5      13.7    0.729     0.957     20.0
learned, no operator                        12.1    0.735     0.884     19.4
learned + operator                          12.9    0.733     0.851     19.3
learned + operator, 64 candidates           13.1    0.735     0.856     19.1
learned, 8 relational proposals            998.2    0.723     0.996     17.1
```

The learned scorer beats the hand scorer on the corpus-likeness judge (12.1 vs 13.6) and on the engine's own evaluator (0.735 vs 0.730) with no operator at all — the relational features do the work. One decode-time correction was needed: the fitted `repeat` weight (+0.55) is right about text and wrong for a low-temperature sampler, which loops on it ("the type of the type of…", distinct 0.66). `GenWeights::for_decoding` swaps that one weight for a stated *sampling constraint* (`REPEAT_DECODE_PENALTY = ln 0.15`); distinct recovers to 0.88. This is the one place a hand constant remains in the learned path, and it is labelled as a decoder constraint, not a model parameter.

**The proposal problem, closed.** docs/72's prior-corrected proposals over-corrected into the rare tail. Relational proposals (`Generator::relational_propose`: the top-k words by `relational_score` with at least two evidenced pairs) fix *that*: the proposed words are real relational neighbours, not tail noise (distinct 0.996 — they are not repeats). What they cannot fix is the sampler: a proposed word enters the trigram tables cold, the next step has no continuation from it, and the judge — itself a trigram model — reads the break as a 70× perplexity jump. Proposals of any kind are incompatible with a trigram-continuation generator and a trigram judge, and `RELATIONAL_PROPOSALS` ships at 0. The relational *score* ships at +0.70 inside every step. That is the fix: score relationally, do not propose from outside the sampler's reach.

## 4. What is hard-coded now, and what is not

| was | now |
|---|---|
| 0.35 / 0.25 / 0.40 / 0.55 / 1.35 / 0.15 in the scorer | eight fitted weights, `data/learned.json`, refit by `bin/learn` |
| operator weight 0.5 (`OPERATOR_WEIGHT`) | the `operator_cos` weight, fitted (+0.61); the hand path keeps 0.5 |
| γ, β in the LM mixture | β = 8 still fixed; γ is swept per run (docs/65–68); fitting both with `Tuner` is a two-feature job not yet done |
| `MARKER_CHANNELS_MAX = 24`, `RELATION_RATE`, `NEG_RATE`, `RELATION_WINDOW` | training hyperparameters; measured once each in docs/64, not learned |
| `PersonalityMapper` thresholds 0.4 / 0.4 / 0.3, `Impersonator` 0.4 / 0.6, `Evaluator` 0.4 / 0.3 / 0.3 | still hand-set; the same `Tuner` can fit them against a labelled objective the moment one exists |
| `REPEAT_DECODE_PENALTY` | a decoder constraint, stated as such |

## 5. Changes in code

| where | what |
|---|---|
| `src/learn/mod.rs` | `GenWeights` (hand / counts_only / for_decoding / load / save), `Position`, `Tuner` (nll, grad, fit), planted-weight test |
| `src/learn/features.rs` | `CandidateFeatures::build`, `FEATURE_NAMES` |
| `src/facet/mod.rs` | `Facet::relational_score` |
| `src/generate.rs` | `weights`, `ctx`, `with_weights`, learned branch of `score_candidates`, `relational_propose`, `RELATIONAL_WINDOW`, `RELATIONAL_PROPOSALS` |
| `src/bin/learn.rs`, `src/bin/generate_eval.rs` | the fit and the eight arms |
| `src/config/constants.rs` | `LEARNED_FILE` |
| `data/learned.json` | the fitted weights |
