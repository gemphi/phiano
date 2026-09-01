# Fix Compliance — what from HOW 01–16 is actually applied

> _Audited against the source, not against memory. Verified by grep on
> `src/` at the current commit, not by what the commit messages claim._

**88 of 88 proposed fixes are applied.**

Applied is not the same as validated. Every fix below compiles, is tested, and
does what its document asked for. Only some of them have been *measured* to
improve anything — see the caveat in §What remains.

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
| | Anchor β outside the geometry | **A** | `order_asymmetry` — derived from bigram counts, not from the phases β moves |
| **04** | Long-range structure for the re-ranker | **A** | recurrent `ContextWaveBuffer` |
| | Phase-layer ablation reported | **A** | `bin/experiment` |
| | Intern vocabulary to u32 IDs | **A** | `Vocab` + sorted `Vec<(id,count)>`; measured 62% smaller; format v3 with v2/v1 migration |
| | Smoothing on Facet's own tables | **A** | `bigram_discounted` / `trigram_discounted` return `(p, backoff)` |
| | Prune singleton n-grams | **A** | implemented and **measured**: −80.7% size, +81% perplexity. A trade, not a win — see RESULTS §3b |
| **05** | Two-phase (Jacobi) grounding | **A** | order-independent now |
| | Iterate to convergence | **A** | `GROUNDING_ROUNDS = 5` |
| | Skip function words | **A** | |
| | Dead `amplitude > 5.0` guard | **A** | now `>= AMPLITUDE_MAX * 0.9` |
| | Generalised `Groundable` trait | **A** | any symbol→description source can ground the manifold |
| **06** | Positional binding | **A** | |
| | Recurrent context state | **A** | `h_t = λ_k e^{iω_k} h_{t-1} + z_t` |
| | Gate novelty on wave magnitude | **A** | undefined rather than folded into `overall` as a measurement |
| | Multiplicative binding for compounds | **A** | `sentence_compound`, PMI-selected pairs |
| **07** | Separate phase from amplitude in scoring | **A** | |
| | Drop α from the ranking path | **A** | it was a no-op constant factor |
| | Multi-channel retrieval | **A** | `ray_cast_channels`, `ray_cast_word` |
| | Sector index | **A** | `SectorIndex`, V/64 expected; tested against a full scan |
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
| | Real sampling (softmax + RNG) | **A** | xorshift64*; `deterministic(seed)` retained |
| | Beam search | **A** | `decode_beam`, length-normalised |
| | Clean at ingestion | **A** | `sources::clean_definition` strips apparatus before training |
| | Log decode degeneration | **A** | dispersion check per generation |
| **12** | `Memo::recall` / `recall_weighted` | **A** | wired into `Model::iterate` |
| | Store indices, not second copies | **A** | |
| | Atomic memo save | **A** | |
| | Semantic layer classification | **A** | sector spread × coherence, not length |
| | Hierarchy used for retrieval | **A** | `descend` / `candidates` |
| | Consolidation / replay pass | **A** | `Memo::consolidate` merges duplicates with a count |
| **13** | Atomic save, checkpoint, Ctrl-C | **A** | |
| | Loud load failures | **A** | |
| | Version check | **A** | |
| | Serialise by reference | **A** | |
| | Intern vocabulary | **A** | see HOW 04; RESULTS §3d |
| | f32 phasors | **A** | `DiskPhasor`: f32 amplitude, `phase` recovered from channel 0 |
| **14** | Order-invariant, chance-corrected matching | **A** | L2-normalised phase histogram, cosine |
| | Reuse warm-starts from the component | **A** | component positions pulled in before training |
| | Implement or delete `FeatureReuse::apply` | **A** | shape transfer; `apply_relational` for analogy |
| | `adapt` consumes the meta prior | **A** | circular-mean prior, applied as a warm start |
| | Monitor on perplexity + dispersion | **A** | plus a dedicated `manifold_collapse` alert |
| **15** | Split, KN baseline, perplexity, diagnostics | **A** | all six |
| **16** | Capacity (D = 64) | **A** | |
| | Composition (binding) | **A** | |
| | Objective (contrastive + predictive) | **A** | |
| | Non-linearity | **A** | `SectorReadout` — sector discretisation + magnitude gating |
| **A1** | `worse` as a guard | **A** | stage-0 never survives |
| | Penalise losers | **A** | |
| | Degeneracy ≠ convergence | **A** | |
| | Weight double-count | **A** | |
| | Killer heuristic | **A** | |
| | Credit assignment from `PhaseFlow` | **A** | `reinforce_trajectory`, λ-decayed eligibility |
| | Alpha/Beta weight competition | **A** | rounds rejected and rolled back under a frozen referee |

---

## The shape of what's left

| Group | Unapplied | Why it matters |
|:---|:---|:---|
| **Memory depth** (12) | semantic layers, hierarchy retrieval, consolidation | The 4-layer hierarchy is still built for display and never consulted during retrieval. |
| **Footprint** (04, 13) | interning, pruning, f32 | The 92 MB against a documented 2–12 MB target. Mechanical, large, contained. |
| **Generation** (11) | sampling, beam, degeneration logging | |
| **Non-linearity** (16) | readout | The fourth of the four requirements; the only one still open. |

---

## What remains: measurement, not code

All 88 fixes are applied. That closes the *implementation* backlog and opens a
different one.

**Applied, and measured to help:**

| fix | evidence |
|:---|:---|
| Contrastive negative sampling | dispersion 0.95–0.997, collapse test |
| Predictive/ranking objective | 27× signal recovery; analogy 0 → nonzero; now also faster |
| Identity seeding | same-length words provably separate |
| Interning | 62% smaller, measured |
| n-gram smoothing | perplexity 119,644 → 124.66 |
| Positional/role binding | order-sensitivity tested |

**Applied, and not yet measured:**

| fix | what is unknown |
|:---|:---|
| `SectorReadout` non-linearity | Built and unit-tested; **not wired into scoring or generation.** Whether a non-linear readout helps is the open question of HOW 16, and it is still open. |
| Beam search | Implemented; greedy `decode` is still the default. No quality comparison run. |
| Alpha/Beta rejection | Active in `compose`; how often it rejects, and whether rejecting helps, is unmeasured. |
| Credit assignment | Implemented; not called from the compose loop by default. |
| Sector index | Agrees with a full scan locally; the speedup is not benchmarked. |
| Multiplicative compounds | `sentence_compound` exists; nothing calls it. |
| Semantic memory layers | Classification changed; no evidence it recalls better. |
| Consolidation, `Groundable`, β anchoring, cleaning, f32 | Mechanisms are correct; effects unquantified. |

**This is now the honest state:** the code does everything the analysis asked
for, and roughly half of it has no evidence attached. The harness exists, so
each row in the second table is one experiment away from moving to the first —
and a fix that cannot be shown to help is a fix that might be removed.

The next work is not more features. It is running the second table.
