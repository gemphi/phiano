# Phiano Chapter 14 Implementation Plan: 64 Tasks

## Overview

This plan implements the key concepts from *Deep Learning with Python* Ch 14 into Phiano's phase-oscillator architecture. It bridges geometric deep learning (value-centric analogy) with program-centric reasoning, improves generalization metrics, and ensures the web UI uses puijs design tokens consistently.

**Guiding principles:**
- Files under 200 lines where possible
- No redundant code — reuse existing facet/wave/attention infrastructure
- puijs brand tokens for all web UI styling
- Each task is independently testable

---

## Phase A: Foundation — Data Splits, Validation, Baselines (14.1) — Tasks 1–10

### Task 1: Create `src/data/splits.rs` — train/val/test splitter
- Implement `DataSplits` struct that partitions sentences into train (80%), validation (10%), test (10%)
- Uses deterministic hashing (not random) for reproducibility
- Exposes `train()`, `val()`, `test()` iterators over `Vec<String>`

### Task 2: Create `src/data/mod.rs` — data module root
- Declare `pub mod splits;`
- Re-export `DataSplits`

### Task 3: Create `src/data/preprocess.rs` — vectorization & preprocessing
- `fn preprocess_text(text: &str) -> Vec<String>` — tokenize, lowercase, strip punctuation
- `fn vectorize(facet: &Facet, tokens: &[String]) -> Vec<f64>` — convert tokens to phase-amplitude pairs
- Reuse existing `Tokenizer` internally

### Task 4: Create `src/metrics/baseline.rs` — baseline scoring
- `fn random_baseline(facet: &Facet, prompt: &str) -> f64` — score from random word selection
- `fn frequency_baseline(facet: &Facet, prompt: &str) -> f64` — score from most frequent words
- `fn phase_baseline(facet: &Facet, prompt: &str) -> f64` — score from nearest-phase words (no n-gram)
- These give lower bounds to compare against the full model

### Task 5: Create `src/metrics/mod.rs` — metrics module root
- Declare `pub mod baseline; pub mod generalization; pub mod adversarial; pub mod arc;`
- Re-export key types

### Task 6: Create `src/metrics/eval_split.rs` — validation-aware evaluation
- `fn eval_on_split(facet: &Facet, evaluator: &Evaluator, split: &DataSplits, split_name: &str) -> EvalSummary`
- `EvalSummary { mean_coherence, mean_novelty, mean_resonance, n_samples }`
- Ensures model is only tested on the test split once, at the end

### Task 7: Create `src/metrics/capacity.rs` — capacity tuning
- `fn tune_capacity(facet: &mut Facet, trainer: &Trainer, val_sentences: &[String]) -> CapacityConfig`
- Sweeps over: learning rate, epochs, sector resolution
- Returns best config by validation coherence

### Task 8: Create `src/metrics/regularization.rs` — regularization controls
- `fn apply_amplitude_decay(facet: &mut Facet, decay_rate: f64)` — prevent amplitude explosion
- `fn apply_phase_jitter(facet: &mut Facet, jitter: f64)` — prevent phase collapse
- `fn apply_band_regularization(facet: &mut Facet, max_band: u32)` — cap band_n growth

### Task 9: Add `src/data/mod.rs` to `lib.rs`
- Add `pub mod data;` to the module declarations

### Task 10: Add `src/metrics/mod.rs` to `lib.rs`
- Add `pub mod metrics;` to the module declarations

---

## Phase B: Limitations — Generalization & Robustness (14.2) — Tasks 11–18

### Task 11: Create `src/metrics/generalization.rs` — local vs extreme generalization
- `fn local_generalization_score(facet: &Facet, train_words: &[String], test_words: &[String]) -> f64`
  - Measures how well phases learned from train_words predict test_words that are nearby in phase space
- `fn extreme_generalization_score(facet: &Facet, train_words: &[String], novel_words: &[String]) -> f64`
  - Measures performance on words far from training distribution
- `fn generalization_gap(local: f64, extreme: f64) -> f64` — the gap

### Task 12: Create `src/metrics/adversarial.rs` — adversarial robustness
- `fn phase_perturbation_test(facet: &Facet, word: &str, delta: f64) -> bool`
  - Perturbs a word's phase by `delta` and checks if evaluation changes drastically
- `fn adversarial_sensitivity(facet: &Facet, prompt: &str, n_perturbations: usize) -> f64`
  - Average sensitivity across n random perturbations
- `fn brittleness_score(facet: &Facet, prompt: &str) -> f64`
  - High = brittle (small perturbation → large output change)

### Task 13: Create `src/metrics/ood_detection.rs` — out-of-distribution detection
- `fn is_ood(facet: &Facet, prompt: &str, threshold: f64) -> bool`
  - Computes distance from prompt's context wave to the facet centroid
  - If distance > threshold, flag as OOD
- `fn ood_score(facet: &Facet, prompt: &str) -> f64`
  - Continuous score [0, 1] where 1 = maximally out-of-distribution

### Task 14: Create `src/metrics/distribution_shift.rs` — training distribution tracking
- `struct DistributionTracker` that maintains a running mean phase + amplitude
- `fn update(&mut self, facet: &Facet, text: &str)` — update tracker with new training input
- `fn shift_score(&self, facet: &Facet, text: &str) -> f64` — how far is this input from the running distribution?

### Task 15: Create `src/reasoning/counterfactual.rs` — hypothetical reasoning
- `fn counterfactual(facet: &Facet, premise: &str, counterfactual: &str) -> String`
  - Swaps a key word's phase to the counterfactual word's phase, re-evaluates
  - Returns what changes in the output
- This addresses Ch 14.2's point about reasoning about hypothetical situations

### Task 16: Create `src/reasoning/mod.rs` — reasoning module root
- Declare `pub mod counterfactual; pub mod pathfinding; pub mod program_synthesis; pub mod analogy;`
- Re-export `ReasoningEngine` from existing `reasoning.rs` (rename to `pathfinding.rs`)

### Task 17: Rename `src/reasoning.rs` → `src/reasoning/pathfinding.rs`
- Move existing `ReasoningEngine`, `ReasoningChain`, `ReasoningStep` into the new module
- Update `lib.rs` to `pub mod reasoning;` (remove old `pub mod reasoning;`)

### Task 18: Add tests for generalization and adversarial metrics
- Test `local_generalization_score` with known facet
- Test `adversarial_sensitivity` with small perturbation
- Test `ood_score` with in-distribution vs out-of-distribution words

---

## Phase C: Greater Generality — ARC-Style Benchmarks (14.3) — Tasks 19–26

### Task 19: Create `src/metrics/arc.rs` — ARC-style evaluation
- `struct ArcTask { id, input_pairs: Vec<(String, String)>, test_input: String, expected: String }`
- `fn load_arc_tasks(path: &str) -> Vec<ArcTask>` — load from JSON
- `fn evaluate_arc(facet: &mut Facet, trainer: &Trainer, tasks: &[ArcTask]) -> ArcResults`
  - For each task: train on input_pairs, then test on test_input
  - Measures: can the model infer a rule from few examples?

### Task 20: Create `src/metrics/shortcut_detection.rs` — shortcut rule detection
- `fn detect_shortcuts(facet: &Facet, prompt: &str, response: &str) -> Vec<ShortcutWarning>`
  - Checks if the model is exploiting surface features (word length, frequency) instead of semantic phase
- `struct ShortcutWarning { shortcut_type: String, description: String, severity: f64 }`

### Task 21: Create `src/metrics/adaptation.rs` — adaptation efficiency
- `fn adaptation_efficiency(facet: &mut Facet, trainer: &Trainer, task: &str, max_examples: usize) -> f64`
  - Measures: how many examples does the model need to reach 80% coherence?
  - Returns examples_needed / max_examples (lower = more efficient = more intelligent)

### Task 22: Create `src/metrics/novelty_benchmark.rs` — novel task benchmark
- `fn novel_task_score(facet: &mut Facet, trainer: &Trainer, description: &str) -> f64`
  - Presents a task the model has never seen
  - Measures coherence and resonance of the first response
  - High score = adapts well to novel situations

### Task 23: Create `data/arc_tasks.json` — sample ARC tasks for phiano
- 10 simple analogy tasks: "X is to Y as Z is to ?"
- 5 pattern completion tasks: "A, B, C, D, E, ?"
- 5 transformation tasks: "reverse: hello → olleh, reverse: world → ?"

### Task 24: Create `src/metrics/benchmark_runner.rs` — benchmark harness
- `struct BenchmarkRunner` that runs all metrics (baseline, generalization, ARC, adaptation)
- `fn run_all(facet: &mut Facet, trainer: &Trainer) -> BenchmarkReport`
- `BenchmarkReport` serializable to JSON for tracking over time

### Task 25: Create `src/metrics/report.rs` — benchmark report formatting
- `fn format_report(report: &BenchmarkReport) -> String` — human-readable
- `fn compare_reports(old: &BenchmarkReport, new: &BenchmarkReport) -> String` — diff
- Implements `Display` for `BenchmarkReport`

### Task 26: Add `benchmark` subcommand to CLI
- Add `src/command/benchmark.rs` — runs `BenchmarkRunner::run_all` and prints report
- Register in `src/command/mod.rs`

---

## Phase D: Missing Ingredients — Analogy & Hybrid Reasoning (14.4) — Tasks 27–36

### Task 27: Create `src/reasoning/analogy.rs` — value-centric analogy
- `fn value_centric_analogy(facet: &Facet, source: &str, target: &str) -> AnalogyResult`
  - Compares words by continuous phase similarity (this is what deep learning does well)
  - Returns: similarity score, nearest neighbors, phase distance
- `fn find_analogies(facet: &Facet, word: &str, n: usize) -> Vec<(String, f64)>`
  - Find top-n words that are analogous to the given word in phase space

### Task 28: Create `src/reasoning/program_analogy.rs` — program-centric analogy
- `fn program_centric_analogy(facet: &Facet, source_pattern: &str, target_pattern: &str) -> AnalogyResult`
  - Compares structural relationships (bigram patterns, sector transitions)
  - Detects shared structural form: "A relates to B" ≈ "C relates to D"
- `fn extract_structure(facet: &Facet, sentence: &str) -> Vec<PhaseRelation>`
  - Extracts phase relations (angle, distance) between consecutive words

### Task 29: Create `src/reasoning/analogy.rs` types
- `struct AnalogyResult { source: String, target: String, value_score: f64, program_score: f64, combined: f64 }`
- `struct PhaseRelation { from_phase: f64, to_phase: f64, delta: f64, sector_transition: u16 }`
- `fn combine_analogy(value: f64, program: f64) -> f64` — weighted combination

### Task 30: Create `src/reasoning/hybrid.rs` — hybrid reasoning engine
- `struct HybridReasoner` that combines:
  - Phase-space pathfinding (existing `ReasoningEngine`)
  - Program-centric analogy (new)
  - Value-centric analogy (new)
- `fn solve_hybrid(&self, facet: &Facet, problem: &str) -> HybridResult`
  - Tries geometric path first, falls back to structural analogy, combines both

### Task 31: Create `src/reasoning/sorting.rs` — sorting as a reasoning test
- `fn sort_by_phase(facet: &Facet, words: &[String]) -> Vec<String>` — sort words by phase angle
- `fn sort_test(facet: &Facet) -> bool` — test if the model can sort arbitrary-length lists
- This is Ch 14.4's example: a program generalizes to any list size, a neural net doesn't

### Task 32: Create `src/reasoning/planning.rs` — multi-step planning
- `struct Plan { steps: Vec<PlanStep>, goal: String }`
- `fn plan(facet: &Facet, goal: &str, max_steps: usize) -> Plan`
  - Uses phase-space traversal to plan a sequence of sub-goals
  - Each step is a sub-goal that moves the context wave toward the goal phase

### Task 33: Create `src/reasoning/abstraction.rs` — abstraction extraction
- `fn extract_abstraction(facet: &Facet, examples: &[String]) -> Abstraction`
  - Given multiple examples, find the common phase pattern
  - Returns: centroid phase, common sector transitions, shared structural elements
- `struct Abstraction { centroid_phase: f64, common_relations: Vec<PhaseRelation>, member_words: Vec<String> }`

### Task 34: Integrate analogy into cognitive core
- Update `src/cognitive/reasoning.rs` to use `HybridReasoner` instead of simple `reason_chain`
- The cognitive core now combines geometric and structural analogy in each reasoning step

### Task 35: Create tests for analogy modules
- Test `value_centric_analogy` with known similar words
- Test `program_centric_analogy` with structural patterns
- Test `sort_by_phase` with unsorted word list
- Test `HybridReasoner::solve_hybrid`

### Task 36: Update `src/reasoning/mod.rs` with all submodules
- Declare all new reasoning submodules
- Re-export `HybridReasoner`, `AnalogyResult`, `Plan`

---

## Phase E: Future — Program Synthesis & Lifelong Learning (14.5) — Tasks 37–48

### Task 37: Create `src/synthesis/program.rs` — program synthesis core
- `struct Program { operations: Vec<ProgramOp>, phase_template: Vec<f64> }`
- `enum ProgramOp { Map(String), Filter(String), Reduce(String, String), Compose, Sort, Reverse }`
- `fn synthesize(facet: &Facet, examples: &[(String, String)]) -> Option<Program>`
  - Searches over possible programs to find one matching input-output examples
  - Uses phase patterns as the specification language

### Task 38: Create `src/synthesis/search.rs` — program search space
- `fn candidate_programs(depth: usize) -> Vec<Program>` — enumerate programs up to depth
- `fn evaluate_program(prog: &Program, facet: &Facet, examples: &[(String, String)]) -> f64`
  - Score: how well does this program match the examples?
- Uses beam search to limit combinatorial explosion

### Task 39: Create `src/synthesis/heuristic.rs` — learned heuristics for search
- `fn phase_heuristic(facet: &Facet, examples: &[(String, String)]) -> Vec<f64>`
  - Uses facet phases to guide which program structures are likely
  - E.g., if input and output have similar phases → likely Map, not Reverse
- This is Ch 14.5's idea: deep learning guides program synthesis search

### Task 40: Create `src/synthesis/mod.rs` — synthesis module root
- Declare `pub mod program; pub mod search; pub mod heuristic; pub mod library;`
- Re-export `Program`, `synthesize`

### Task 41: Create `src/synthesis/library.rs` — reusable component library
- `struct ComponentLibrary { components: Vec<Component> }`
- `struct Component { name: String, program: Program, phase_signature: Vec<f64>, reuse_count: usize }`
- `fn register(&mut self, name: &str, program: Program, facet: &Facet)`
- `fn find_reusable(&self, facet: &Facet, task: &str) -> Option<&Component>`
  - Finds a component with matching phase signature

### Task 42: Create `src/lifelong/mod.rs` — lifelong learning coordinator
- `struct LifelongLearner { library: ComponentLibrary, metrics: BenchmarkHistory }`
- `fn learn_task(&mut self, facet: &mut Facet, trainer: &Trainer, task: &str) -> LearnResult`
  - 1. Check library for reusable components
  - 2. If found, adapt and apply
  - 3. If not, learn from scratch, then register new component
- `fn transfer_knowledge(&mut self, source: &str, target: &str) -> TransferResult`

### Task 43: Create `src/lifelong/meta.rs` — meta-learning
- `fn meta_learn(facet: &mut Facet, trainer: &Trainer, tasks: &[String]) -> MetaModel`
  - Trains on multiple tasks, extracts common patterns
- `struct MetaModel { common_phases: Vec<f64>, adaptation_rates: Vec<f64> }`
- `fn adapt(&self, facet: &mut Facet, trainer: &Trainer, new_task: &str)`
  - Uses meta-learned patterns to speed up learning on a new task

### Task 44: Create `src/lifelong/history.rs` — benchmark history tracking
- `struct BenchmarkHistory { entries: Vec<BenchmarkEntry> }`
- `struct BenchmarkEntry { timestamp: String, report: BenchmarkReport }`
- `fn record(&mut self, report: BenchmarkReport)`
- `fn trend(&self) -> Vec<f64>` — coherence trend over time
- Serializes to `data/benchmark_history.json`

### Task 45: Create `src/lifelong/reuse.rs` — feature & architecture reuse
- `fn extract_features(facet: &Facet) -> Vec<FeatureSet>`
  - Extracts reusable phase patterns from the facet
- `fn apply_features(facet: &mut Facet, features: &[FeatureSet])`
  - Applies pre-learned features to a new facet (transfer learning)
- `struct FeatureSet { phase_pattern: Vec<f64>, sector_distribution: Vec<u16>, label: String }`

### Task 46: Add `src/synthesis/mod.rs` and `src/lifelong/mod.rs` to `lib.rs`
- Add `pub mod synthesis;` and `pub mod lifelong;`

### Task 47: Create integration test for program synthesis
- Test `synthesize` with simple examples (sorting, reversal)
- Test `ComponentLibrary::find_reusable` after registering a component
- Test `LifelongLearner::learn_task` with two sequential tasks

### Task 48: Create `src/lifelong/monitor.rs` — deployment monitoring
- `struct ModelMonitor { history: BenchmarkHistory, alerts: Vec<Alert> }`
- `fn check_drift(&self, facet: &Facet, recent_inputs: &[String]) -> Option<Alert>`
  - Detects if recent inputs are drifting from training distribution
- `fn check_regression(&self, current: &BenchmarkReport) -> Option<Alert>`
  - Detects if performance has regressed from a previous benchmark

---

## Phase F: Reasoning Model Improvements — Tasks 49–56

### Task 49: Rewrite `ReasoningEngine::solve` with convergence diagnostics
- Current: just finds nearest resonant words (too simple)
- New: track phase trajectory, detect oscillation vs convergence vs divergence
- Add `enum ConvergenceMode { Converged, Oscillating, Diverging, Stuck }`
- Add `fn diagnose(chain: &ReasoningChain) -> ConvergenceMode`

### Task 50: Add multi-path reasoning to `pathfinding.rs`
- `fn solve_multi_path(facet: &Facet, problem: &str, n_paths: usize) -> Vec<ReasoningChain>`
  - Explores n different reasoning paths from different starting sectors
  - Returns all paths, sorted by final coherence
- `fn best_path(paths: &[ReasoningChain]) -> &ReasoningChain`

### Task 51: Add reasoning depth control
- `fn solve_with_depth(facet: &Facet, problem: &str, effort: EffortLevel) -> ReasoningChain`
- `enum EffortLevel { Instant, Quick, Standard, Deep, Exhaustive }`
  - Instant: 1 step, no context update
  - Quick: 4 steps
  - Standard: 16 steps (current)
  - Deep: 32 steps with multi-path
  - Exhaustive: 64 steps with multi-path + program synthesis

### Task 52: Add reasoning trace visualization data
- Add `phase_trajectory: Vec<f64>` to `ReasoningStep` (phase at each step)
- Add `amplitude_trajectory: Vec<f64>` (amplitude at each step)
- Add `sector_visits: Vec<u16>` (which sectors were visited)
- This data feeds the web UI's reasoning visualization

### Task 53: Add reasoning confidence scoring
- `fn confidence(chain: &ReasoningChain) -> f64`
  - Combines: convergence (0.3), coherence (0.3), path length penalty (0.2), novelty (0.2)
- Add `confidence: f64` to `ReasoningChain`

### Task 54: Add reasoning comparison mode
- `fn compare_reasoning(facet: &Facet, problem: &str) -> ReasoningComparison`
  - Runs: phase pathfinding, cognitive chain, hybrid reasoner
  - Returns all three results side by side with scores
- `struct ReasoningComparison { pathfinding: ReasoningChain, cognitive: ReasoningResult, hybrid: HybridResult }`

### Task 55: Add reasoning step templating
- Instead of "Step N: resonate with WORD", generate actual sentences
- `fn template_step(step: &ReasoningStep, facet: &Facet) -> String`
  - Uses existing `instruction::templated_output` patterns
  - Produces readable intermediate reasoning steps

### Task 56: Add reasoning API endpoint
- Add `POST /api/reason` to `src/server/routes_chat.rs` or new `routes_reasoning.rs`
- Accepts: `{ prompt, effort_level, multi_path }`
- Returns: `{ chain, confidence, convergence_mode, trajectory }`

---

## Phase G: Web UI — puijs Brand Consistency — Tasks 57–62

### Task 57: Audit `web/src/styles/variables.css` for puijs compliance
- Current: uses custom `--bg-primary`, `--color-primary` etc. (not puijs tokens)
- Action: replace with `--phi-color-*`, `--phi-space-*`, `--phi-radius-*`, `--phi-shadow-*`
- Keep custom gradients as puijs extensions, not replacements

### Task 58: Update `web/src/styles/globals.css` to use puijs tokens
- Replace all hardcoded colors with `var(--phi-color-*)`
- Replace all spacing with `var(--phi-space-*)`
- Replace all radii with `var(--phi-radius-*)`
- Replace all shadows with `var(--phi-shadow-*)`
- Import puijs tokens at top: `@import '@phiace/puijs/src/tokens/tokens.scss';` or inline them

### Task 59: Update `web/src/components/Header.tsx` for puijs brand
- Replace inline styles with puijs token references
- Use `--phi-color-primary` for brand accent
- Use `--phi-shadow-*` for elevation
- Use `--phi-transition-*` for animations

### Task 60: Update `web/src/components/Sidebar.tsx` for puijs brand
- Use `--phi-color-background-secondary` for sidebar bg
- Use `--phi-color-border-subtle` for dividers
- Use `--phi-radius-md` for nav items
- Use `--phi-color-primary` for active state

### Task 61: Update `web/src/components/ChatPanel.tsx` for puijs brand
- Replace custom `--color-primary` with `--phi-color-primary`
- Replace `--bg-card` with `--phi-color-background-card`
- Replace `--shadow-md` with `--phi-shadow-2`
- Use puijs `--phi-font-mono` for code blocks in chat

### Task 62: Update remaining web components for puijs consistency
- `EvalPanel.tsx`, `StatsPanel.tsx`, `LearnPanel.tsx`, `OscillatorPanel.tsx`
- `DictionaryPanel.tsx`, `DocsPanel.tsx`, `InfinityPanel.tsx`
- `Phi4StudioPanel.tsx`, `SymbolsPanel.tsx`, `VersusPanel.tsx`, `PhaseTopologyPanel.tsx`
- All should use `--phi-*` tokens, no hardcoded colors

---

## Phase H: Integration, Testing & Documentation — Tasks 63–64

### Task 63: Add integration tests for Ch 14 concepts
- Create `tests/ch14_integration.rs`
- Test: train → validate → test pipeline (14.1)
- Test: adversarial robustness threshold (14.2)
- Test: ARC task evaluation (14.3)
- Test: hybrid reasoning (value + program analogy) (14.4)
- Test: program synthesis on simple examples (14.5)
- Test: lifelong learning transfer (14.5)

### Task 64: Update `PLAN.md` and `README.md` with Ch 14 implementation status
- Update PLAN.md: mark Phases 1-6 as done, add Ch 14 section
- Update README.md: document new modules (metrics, synthesis, lifelong, reasoning)
- Document the benchmark runner CLI command
- Document the ARC evaluation format

---

## Summary

| Phase | Theme | Tasks | Key Files |
|-------|-------|-------|-----------|
| A | Data splits, validation, baselines (14.1) | 1–10 | `src/data/`, `src/metrics/` |
| B | Generalization & robustness (14.2) | 11–18 | `src/metrics/`, `src/reasoning/` |
| C | ARC benchmarks, adaptation (14.3) | 19–26 | `src/metrics/arc.rs`, `data/arc_tasks.json` |
| D | Analogy & hybrid reasoning (14.4) | 27–36 | `src/reasoning/analogy.rs`, `hybrid.rs` |
| E | Program synthesis & lifelong learning (14.5) | 37–48 | `src/synthesis/`, `src/lifelong/` |
| F | Reasoning model improvements | 49–56 | `src/reasoning/pathfinding.rs` |
| G | puijs web UI brand consistency | 57–62 | `web/src/styles/`, `web/src/components/` |
| H | Integration & documentation | 63–64 | `tests/`, `PLAN.md`, `README.md` |
