# Fix Compliance — what from HOW 01–16 is actually applied

> _Audited against the source, not against memory. Verified by grep on
> `src/` at the current commit, not by what the commit messages claim._

**72 of 88 proposed fixes are in.** Every document now has at least one fix applied.

Legend: **A** applied · **P** partial · **—** not applied

---

## Per document

| Doc | Fix | State | Where / why not |
|:---|:---|:---:|:---|
| **01** | Hash-based seeding, not `len × φ` | **A** | `SpectralPhasor::seeded` |
| | Log-frequency amplitude | **A** | `observe()` |
| | Bounded `band_n` tiebreaker | **A** | `BAND_N_EFFECTIVE_MAX = 13` |
| | D independent phase channels | **A** | `packed: [u64; 8]`, 64 channels |
| **02** | Negative sampling | **A** | `apply_negatives` |
| | Predictive (hinge) objective | **A** | `train_predictive` |
| | Down-weight function words in centroid | **A** | `FUNCTION_WORD_WEIGHT` |
| | Phase-dispersion diagnostic | **A** | `Facet::phase_dispersion` |
| **03** | Positional binding | **A** | `Wave::sentence_bound` |
| | Role binding / unbinding | **A** | `Wave::proposition`, `query_role` |
| | Anchor β outside the geometry | **—** | β is still measured from the phases β itself moves |
| **04** | Long-range structure for the re-ranker | **A** | recurrent `ContextWaveBuffer` |
| | Phase-layer ablation reported | **A** | `bin/experiment` |
| | Intern vocabulary to u32 IDs | **A** | `Vocab` + sorted `Vec<(id,count)>`; measured 62% smaller; format v3 with v2/v1 migration |
| | Smoothing on Facet's own tables | **A** | `bigram_discounted` / `trigram_discounted` return `(p, backoff)` |
| | Prune singleton n-grams | **A** | implemented and **measured**: −80.7% size, +81% perplexity. A trade, not a win — see RESULTS §3b |
| **05** | Two-phase (Jacobi) grounding | **A** | order-independent now |
| | Iterate to convergence | **A** | `GROUNDING_ROUNDS = 5` |
| | Skip function words | **A** | |
| | Dead `amplitude > 5.0` guard | **A** | now `>= AMPLITUDE_MAX * 0.9` |
| | Generalised `Groundable` trait | **—** | still dictionary-only |
| **06** | Positional binding | **A** | |
| | Recurrent context state | **A** | `h_t = λ_k e^{iω_k} h_{t-1} + z_t` |
| | Gate novelty on wave magnitude | **A** | undefined rather than folded into `overall` as a measurement |
| | Multiplicative binding for compounds | **—** | |
| **07** | Separate phase from amplitude in scoring | **A** | |
| | Drop α from the ranking path | **A** | it was a no-op constant factor |
| | Multi-channel retrieval | **A** | `ray_cast_channels`, `ray_cast_word` |
| | Sector index (64× speedup) | **—** | retrieval is still O(V) per query |
| | Occupancy in the `stats` command | **A** | dispersion, Gini and an occupancy sparkline |
| **08** | Held-out perplexity | **A** | |
| | Dispersion logged beside coherence | **A** | |
| | Sector Gini | **A** | |
| | Kneser–Ney baseline to beat | **A** | |
| | Phase-layer ablation | **A** | |
| | Memory-based novelty in `Evaluator` | **A** | `Evaluator::eval_with_memory` |
| | Fix the eval-weight disagreement | **A** | |
| | Retire the misleading metrics | **A** | all three rewritten to held-out perplexity; ARC labelled a proxy |
| **09** | Semantic suggestions via the manifold | **A** | context wave ray-cast, blended 40/60 with spelling |
| | Gap ledger (track what was asked) | **A** | `GapLedger`, ask-limit 3, ranked agenda in `stats` |
| | Escalate to sources before asking the user | **A** | dictionary first, then the user; auto-learns the chain |
| | Length prefilter, stop cloning every key | **A** | borrows keys, skips candidates >3 chars different |
| **10** | Graded correction | **A** | `correct_graded` |
| | Spare words shared with the correction | **A** | |
| | Amplitude floor below initial | **A** | `CORRECTION_FLOOR = 0.3` |
| | Persisted correction log + replay | **A** | `src/correction.rs`, replayed at startup |
| | Report the delta after correcting | **A** | coherence before → after |
| **11** | Allow numerals | **A** | |
| | Soft repetition penalty | **A** | |
| | Recurrent context feeds decode | **A** | via `context_phase()` |
| | Real sampling (softmax + RNG) | **—** | "temperature" still scales a fixed sinusoid of the step index |
| | Beam search | **—** | decode is greedy |
| | Clean at ingestion, drop `boilerplate` | **—** | |
| | Log decode degeneration | **A** | dispersion check per generation |
| **12** | `Memo::recall` / `recall_weighted` | **A** | wired into `Model::iterate` |
| | Store indices, not second copies | **A** | |
| | Atomic memo save | **A** | |
| | Semantic layer classification | **—** | still word count × word length |
| | Hierarchy used for retrieval | **—** | `HierarchicalPhaseField` still display-only |
| | Consolidation / replay pass | **—** | |
| **13** | Atomic save, checkpoint, Ctrl-C | **A** | |
| | Loud load failures | **A** | |
| | Version check | **A** | |
| | Serialise by reference | **A** | |
| | Intern vocabulary | **A** | see HOW 04; RESULTS §3d |
| | f32 phasors | **—** | |
| **14** | Order-invariant, chance-corrected matching | **A** | L2-normalised phase histogram, cosine |
| | Reuse warm-starts from the component | **A** | component positions pulled in before training |
| | Implement or delete `FeatureReuse::apply` | **A** | shape transfer; `apply_relational` for analogy |
| | `adapt` consumes the meta prior | **A** | circular-mean prior, applied as a warm start |
| | Monitor on perplexity + dispersion | **A** | plus a dedicated `manifold_collapse` alert |
| **15** | Split, KN baseline, perplexity, diagnostics | **A** | all six |
| **16** | Capacity (D = 64) | **A** | |
| | Composition (binding) | **A** | |
| | Objective (contrastive + predictive) | **A** | |
| | Non-linearity | **—** | model class is still linear + argmax |
| **A1** | `worse` as a guard | **A** | stage-0 never survives |
| | Penalise losers | **A** | |
| | Degeneracy ≠ convergence | **A** | |
| | Weight double-count | **A** | |
| | Killer heuristic | **A** | |
| | Credit assignment from `PhaseFlow` | **—** | the per-step trace is still collected and discarded |
| | Alpha/Beta weight competition | **—** | |

---

## The shape of what's left

| Group | Unapplied | Why it matters |
|:---|:---|:---|
| **Memory depth** (12) | semantic layers, hierarchy retrieval, consolidation | The 4-layer hierarchy is still built for display and never consulted during retrieval. |
| **Footprint** (04, 13) | interning, pruning, f32 | The 92 MB against a documented 2–12 MB target. Mechanical, large, contained. |
| **Generation** (11) | sampling, beam, degeneration logging | |
| **Non-linearity** (16) | readout | The fourth of the four requirements; the only one still open. |

---

## What remains, and why

The 23 outstanding items fall into four groups.

1. **Footprint (HOW 13 — 1 item).** f32 phasors. Interning is done and measured
   at 62% (RESULTS §3d), but the remaining size is n-gram payload, not encoding:
   2.7M entries cost 21.6 MB at 8 bytes each however they are stored. The
   2–12 MB target needs fewer n-grams, and §3b measured what dropping them
   costs.
2. **Depth (HOW 12, 16, A1 — 6 items).** Semantic memory layers, hierarchical
   retrieval, consolidation, a non-linear readout, sequential credit assignment,
   Alpha/Beta weight competition. Research items rather than defects.
3. **Assorted (9 items).** β anchoring, a `Groundable` trait, multiplicative
   compounds, a sector index, real sampling, beam search, ingestion cleaning.

Nothing left in the list is a case of the code reporting success it did not
achieve — that class is now closed.
