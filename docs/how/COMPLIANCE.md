# Fix Compliance — what from HOW 01–16 is actually applied

> _Audited against the source, not against memory. Verified by grep on
> `src/` at the current commit, not by what the commit messages claim._

**52 of 88 proposed fixes are in. Two documents are entirely unapplied.**

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
| | Intern vocabulary to u32 IDs | **—** | still `HashMap<String, …>`; model still 92 MB |
| | Smoothing on Facet's own tables | **P** | in `PhianoLM` (scoring) only; `bigram_probability` is still raw MLE and returns 0.0 for unseen |
| | Prune singleton n-grams | **—** | |
| **05** | Two-phase (Jacobi) grounding | **A** | order-independent now |
| | Iterate to convergence | **A** | `GROUNDING_ROUNDS = 5` |
| | Skip function words | **A** | |
| | Dead `amplitude > 5.0` guard | **A** | now `>= AMPLITUDE_MAX * 0.9` |
| | Generalised `Groundable` trait | **—** | still dictionary-only |
| **06** | Positional binding | **A** | |
| | Recurrent context state | **A** | `h_t = λ_k e^{iω_k} h_{t-1} + z_t` |
| | Gate novelty on wave magnitude | **—** | `eval.rs` still takes `arg()` of a possibly-zero wave |
| | Multiplicative binding for compounds | **—** | |
| **07** | Separate phase from amplitude in scoring | **A** | |
| | Drop α from the ranking path | **A** | it was a no-op constant factor |
| | Multi-channel retrieval | **A** | `ray_cast_channels`, `ray_cast_word` |
| | Sector index (64× speedup) | **—** | retrieval is still O(V) per query |
| | Occupancy in the `stats` command | **P** | `sector_gini` exists; not wired into `stats` |
| **08** | Held-out perplexity | **A** | |
| | Dispersion logged beside coherence | **A** | |
| | Sector Gini | **A** | |
| | Kneser–Ney baseline to beat | **A** | |
| | Phase-layer ablation | **A** | |
| | Memory-based novelty in `Evaluator` | **P** | `Memo::novelty` written; `eval.rs` still uses centroid distance |
| | Fix the eval-weight disagreement | **A** | |
| | Retire the misleading metrics | **—** | `arc.rs` still predicts by `format!` template; `baseline.rs` still scores word salad |
| **09** | Semantic suggestions via the manifold | **—** | **entire doc unapplied** |
| | Gap ledger (track what was asked) | **—** | |
| | Escalate to sources before asking the user | **—** | |
| | Length prefilter, stop cloning every key | **—** | |
| **10** | Graded correction | **A** | `correct_graded` |
| | Spare words shared with the correction | **A** | |
| | Amplitude floor below initial | **—** | still `max(AMPLITUDE_INITIAL)`; "actively wrong" is indistinguishable from "never seen" |
| | Persisted correction log + replay | **—** | corrections do not survive a re-ingest |
| | Report the delta after correcting | **—** | |
| **11** | Allow numerals | **A** | |
| | Soft repetition penalty | **A** | |
| | Recurrent context feeds decode | **A** | via `context_phase()` |
| | Real sampling (softmax + RNG) | **—** | "temperature" still scales a fixed sinusoid of the step index |
| | Beam search | **—** | decode is greedy |
| | Clean at ingestion, drop `boilerplate` | **—** | |
| | Log decode degeneration | **—** | |
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
| | Intern vocabulary | **—** | the 92 MB is unfixed |
| | f32 phasors | **—** | |
| **14** | Order-invariant, chance-corrected matching | **—** | **entire doc unapplied** |
| | Reuse warm-starts from the component | **—** | reuse still only changes an iteration count |
| | Implement or delete `FeatureReuse::apply` | **—** | still writes one synthetic word per feature set |
| | `adapt` consumes the meta prior | **—** | `common_phases` still computed and never read |
| | Monitor on perplexity + dispersion | **—** | still watches `baselines.2` |
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
| **Metrics honesty** (08) | `arc.rs`, `baseline.rs`, `generalization.rs` | These still produce numbers that cannot support conclusions. Highest-value cleanup: they actively mislead. |
| **Lifelong** (14) | all 5 | Every scaffold is a stub that reports success it did not achieve. |
| **Envision** (09) | all 4 | The best control loop in the system, still using spelling similarity while a manifold sits unused beside it. |
| **Footprint** (04, 13) | interning, pruning, f32 | The 92 MB against a documented 2–12 MB target. Mechanical, large, contained. |
| **Generation** (11) | sampling, beam, degeneration logging | |
| **Non-linearity** (16) | readout | The fourth of the four requirements; the only one still open. |

---

## Note on the two unapplied documents

**HOW 09** and **HOW 14** received nothing. That was not a judgement — the first
pass prioritised the four structural requirements in HOW 16 plus the measurement
loop in HOW 15, and these two fell outside both. HOW 14 in particular is the one
place where the code *reports success it did not achieve*
(`TransferResult.features_transferred` counts feature sets that were never
transferred), which makes it the more urgent of the two.
