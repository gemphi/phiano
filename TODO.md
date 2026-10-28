# phiano — what is left to do

Paths are under `src/` unless they say otherwise. Diagrams: [ARCHITECTURE.md](ARCHITECTURE.md), one page per module in `phinet/docs/phiano/`.

## The arena (docs/203)

The readers that need nothing of phiano's (counts, concepts, the mirror, the operator, the learner) moved to `phient::lineup`; their items are in phient's TODO.md. `arena` keeps `Ecs`, which reads a persona's world.

- [ ] arena: put phient's learner behind `examples/talk.rs` and the workshop, so nothing that generates runs on counts.
- [x] arena: `Operator` was missing from the lineup table (now `phient::lineup`'s table).
- [ ] arena: `use crate::sys::Agent;` is repeated inside `Ecs::angle` and `Ecs::profile` (arena/ecs.rs).

## Duplicates moved out (2026-10-27)

- [x] scribe: moved to `phient::scribe` (it needs only phient and phiclm's keywords).
- [x] relational: `RelationalPhasor`/`RelationalFacet` moved to `phiecs::phasor::relational`, whole, as the experiment the spectral phasor's evidence was promoted from; `flow` and two examples read it there.
- [x] eval: `Verdict` renamed `Quality` (phiclm's `Verdict` is a program's compile-and-run result).
- [x] gemgum: moved to `phiecs::phasor::gemgum`, beside the phasor it reads as light; `compose` and `facet::spectral` call it there.
- [x] phasor: `SpectralPhasor` lives in `phiecs::phasor::spectral`, beside phiecs's `Phasor`; `crate::phasor` re-exports it, `config` re-exports its five constants, and `physics::Phical::effective_phase` reads `phiton::units::effective_phase`.
- [x] arena: the readers moved to `phient::lineup` (see above).
- [x] persona: phiano's `Persona` and `World` were not phiecs's (a style fingerprint and a set of them, against a persona file and the world of rows): renamed `Style` and `Styles`.
- [x] reasoning: `cognitive::ReasoningResult` and `reasoning::ReasoningChain` were one thing: now one `cognitive::Chain<S>` (`reasoning::ReasoningChain` is `Chain<ReasoningStep>`, kept in `cognitive` so `reasoning` stays out of the cycle); the cognitive core's steps are `Thought`, its engine `ChainOfThought`.
- [x] examples: the code reader's `Tally` and the learner example's `Hits` counted what phient's held-out score counts: both use `phient::study::Score`. The code writers' `Tally` was a scoreboard: renamed `Scoreboard`.
- [ ] examples/builders: `Tally` is phient's `relate::Tally` before it was general (docs/105–111), with game-scoped reading on top (games, open aspects, per-aspect teacher posts). Port those into phient's `Tally` and re-run the builders' measurements to show the recorded numbers hold, then retire the example's copy.

## Structure

- [ ] Break the dependency cycle: 4 of the 19 modules use each other, all app code (compose, generate, learn, persona; ARCHITECTURE.md, layer 2), counted on code, not doc links; command and drivers are a second, small one. It was 25 (counting doc links) before the phasor, facet, wave, storage, memory, eval, metrics and the training cluster moved out.
- [ ] The match-not-else-if rule is broken in bpe/pretokenize.rs:31, persona/fingerprint/traits.rs:31, eval/verdict.rs:12 and eval/verdict.rs:91.
- [ ] Whole-module `#![allow(dead_code)]` hides unused parts in metrics, reasoning, synthesis, wiki, curriculum, data, instruction, lifelong and physics.

## Dead or unwired modules

- [x] extension: nothing used `Extension`/`Slot` but its own tests; phiecs's `Phasor` holds extension channels and `world::SLOTS` the answer slots. Parked in `_temp/phinet-root/phiano-extension/`.
- [ ] attention: `attention/cross.rs` is never declared (no `mod cross;`), so it is not compiled; the only caller of `Attention::next_words` is the dead `Generator::attention_pick` (generate/steer.rs:13-25).
- [ ] seam: `seam::Seam` (was `seam::Model`, renamed off `model::Model`) has no user outside its test, and `continue_from` never reaches `compute` (seam.rs:26, :68).
- [ ] physics: `Topology` and its shapes are used nowhere outside physics; `spectral_convolution`, `temporal_project`, `curvature` and `sector_to_point` are never called (physics/mod.rs:51, physics/operators.rs:45, :67, physics/manifold.rs:87, :95).
- [ ] lifelong: `LifelongLearner`, `MetaLearner`, `FeatureReuse` and `ModelMonitor` have no callers; the warm-starts write channel 0 only (lifelong/mod.rs:1, :106, lifelong/meta.rs:84).
- [ ] synthesis: `ProgramOp::Map`, `Reduce` and `Compose` do nothing; `candidates` enumerates only Sort, Reverse and Identity; `ProgramSynthesizer` and `SynthesisHeuristic` have no callers (synthesis/program.rs:47, :51, synthesis/search.rs:14, :51).
- [ ] layers: `descend` and `candidates` have no callers; layer 0 is allocated and never filled (layers/phase/field.rs:10, :31, layers/phase/layer.rs:41).
- [ ] phinum: the Gen-0 model is kept only for the `riley` example, behind the `phinum` feature (lib.rs:44).

## Correctness

- [ ] metrics: `BenchmarkRunner::run_all` trains the live facet it measures, so running the benchmark changes the model (metrics/benchmark/runner.rs:64-65, metrics/arc.rs:112-114).
- [ ] metrics: shortcut detection runs on a fixed template, not on model output (metrics/benchmark/runner.rs:75-76).
- [ ] wiki: `&extract[..5000]` cuts at a byte index and panics inside a multi-byte character (wiki/extracts.rs:99, :146).
- [ ] server: the `flow` handler panics on a poisoned lock; the others return a status code (server/routes/flow.rs:30).
- [ ] server: handlers build a new `ChunkStore` per request, some with a hard-coded "data/chunks" (server/routes/chat.rs:95, server/routes/cognitive.rs:53); model does the same four times (model/run.rs:45, :48, :67, :84).
- [ ] persona: `match_text` and `match_text_with` disagree when the best score is ≤ 0 (persona/world/personas.rs:100 vs persona/world/matching.rs:40).
- [ ] phinet: `OperatorScorer` assumes 64-sample functions; a `Sides::Both` space would read the wrong words (phinet/lm.rs:53).
- [ ] oscillator: `from_phasor` uses the raw `band_n` counter as a frequency (it reaches 1.8M); `evaluate` scores the whole field, not the given text (oscillator/mod.rs:37, oscillator/eval.rs:24).
- [ ] curriculum: `bigrams_recorded` is overwritten each stage instead of added up (curriculum.rs:115).
- [ ] chunker: the progress line swaps its arguments and prints `[epoch 64/1]` (chunker.rs:135).
- [ ] helix: n-grams are not written, so a write then a read loses them; a malformed channel number becomes 0 (helix/read.rs:12, :22).
- [ ] bpe: `encode` silently drops pieces missing from the vocab; Phi-4's o200k digit split is not implemented (bpe/mod.rs:87, :13-16).
- [ ] envision: `detect_gaps` throws the definition away; eval and learn call it with a fresh ledger each time (envision/gaps.rs:52, command/eval.rs:28, command/learn.rs:79).
- [ ] memory: `consolidate` has no caller, so the log grows without limit; `record` still files by surface shape (memory/consolidate.rs:11, memory/memo.rs:18).
- [ ] correction: no undo exists though the doc promises one; `CORRECTION_JOURNAL_MAX` is unused (correction.rs:9, config/constants/novelty.rs:137).
- [ ] drivers: `ingest-phi4 [dir]` ignores its dir; the dialogue summary always prints the same layer line (drivers/ingest.rs:53-62, :90).
- [ ] lab: `dir_sentences` unwraps `file_stem`/`to_str` and panics on a non-UTF-8 path (lab.rs:50, :54).
- [ ] trainer: `train_corpus_with_descent` runs the centroid path, not the recommended ranking path (trainer/ngrams.rs:146, :109, :131).

## Stale docs

- [ ] reasoning: lists `program_analogy.rs` and `multi_path.rs`; the files are program.rs and paths.rs (reasoning/mod.rs:8, :15).
- [ ] cognitive: lists `speech_acts.rs` and `word_selection.rs`; agent numbers in `process` disagree with the agents' own docs (cognitive/mod.rs:7, :13, :79-94).
- [ ] kinds: the README names `RelationTypes`; the type is `KindTable`; the k=16/64 tables are provisional (kinds/README.md:12, :24, :190).
- [ ] sources: spec.md describes an `Ingester::ingest` that lives in `drivers::ingest` (sources/spec.md:26, :39).
- [ ] storage: says v3 files are written back as v4; save writes v6 (storage/types.rs:162).
- [ ] phasor: says evidence is 16-bit, packed four to a word; it is 32-bit (phasor/types.rs:135, :158).
- [ ] phinet: says the engine is the standalone `phinet` crate; the re-exports come from phistd and phical (phinet/mod.rs:5).
- [ ] sys: links to `sets` (the module is `set`); tells users to run `persona_quest`, a phiecs example (sys.rs:17, sys/race.rs:95, sys/being.rs:17).
- [ ] command: `benchmark` is dispatched but missing from `help`; two doc comments sit on the wrong items (command/dispatch.rs:68, :76-78, command/cognitive.rs:10).
- [ ] trainer, wave: docs sit on the wrong items (trainer/build.rs:158, wave/sentence.rs:121).
- [ ] phinum: cites "phidoc ◂ puijs", which are not dependencies (phinum/mod.rs:19).
- [ ] topdown: the sentence level does not move (1.01× chance); `Up::Bound` is kept though always worse (topdown.rs:76, :80).
- [x] The paradigm sentence "with a count" in README.md and personas/ (re-worded: the words move by the error of what was expected).
