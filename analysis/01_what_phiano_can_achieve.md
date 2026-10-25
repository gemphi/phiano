# 01 — What Phiano Is and What It Can Achieve

> Evaluation target: the Phiano/Phinum model at [`C:\Users\phiac\Workspace\gemphi\phiano`](../).
> Method: direct reading of [`src/`](../src/), execution of the test suite, and reconciliation
> of the repository's documentation claims against the implemented code.

---

## 1. What Phiano Is (In One Paragraph)

Phiano is a **Rust language engine that represents every word as a complex phasor**
`Z = A·e^(i(φ + nα))` on a continuous 2π phase circle, and **learns by pulling the phases
of words that co-occur toward a shared centroid** — a Kuramoto-oscillator-inspired,
backpropagation-free, online learning rule. Sentences become complex superposition waves;
semantic similarity becomes destructive-interference distance; vocabulary familiarity
becomes amplitude. Around this core, the project layers a 16-band episodic memory
([`src/memory/mod.rs`](../src/memory/mod.rs)), a rule-based Searle speech-act cognitive
core ([`src/cognitive/`](../src/cognitive)), hash-based multi-resolution "Phinum" classifiers
([`src/phinum/`](../src/phinum)), phase-space reasoning engines ([`src/reasoning/`](../src/reasoning)),
an n-gram-first generator with phase steering ([`src/generate.rs`](../src/generate.rs)),
a 31-endpoint Axum HTTP API ([`src/server/api.rs`](../src/server/api.rs)), and an unusually
complete evaluation harness ([`src/metrics/`](../src/metrics)).

## 2. What It Verifiably Achieves Today

| Capability | Status | Evidence |
|---|---|---|
| Online learning, mid-conversation, no retraining | **Working** | [`Trainer::train_sentence`](../src/trainer/mod.rs) called per chat turn from [`routes_chat.rs`](../src/server/routes_chat.rs); 488k updates in 1.11 s measured |
| Persistent knowledge across sessions | **Working** | bincode `manifold.chroma` + `memory.chroma` save/load ([`src/storage.rs`](../src/storage.rs), [`Model::scale`](../src/model.rs)) |
| Recursive dictionary self-study (definition chains) | **Working** | [`learn_definition_chain`](../src/trainer/mod.rs) walks unknown words → definitions → unknown words, depth-bounded |
| In-chat self-correction (anti-phase pulse) | **Working** | [`correct_mistake`](../src/trainer/mod.rs): π-radian repulsion + 0.8 amplitude decay, then retrain |
| REST API + SSE streaming + web dashboard | **Working** | 31 endpoints in [`src/server/api.rs`](../src/server/api.rs); React dashboard in [`web/`](../web) |
| Distilling external vocabularies (Phi-4 tiktoken, BPE merges, Webster's, Wikipedia) | **Working** | [`src/sources/`](../src/sources): `phi4.rs`, `wiktionary.rs`, `wiki_bulk.rs` |
| Local generation without any GPU or LLM weights | **Working, limited quality** | [`Generator::decode`](../src/generate.rs): trigram → bigram → torus ray-cast; caps at 20 tokens |
| Benchmark harness (OOD, adversarial, adaptation, capacity, shifts) | **Working** | 15 modules in [`src/metrics/`](../src/metrics) |
| 68/68 tests passing | **Verified** | `cargo test --release` run 2026-08-31 |

## 3. What It Does NOT Achieve Today (Despite Documentation Claims)

These are stated here bluntly because files 06–09 and 14 expand on each:

1. **It does not run the Phi-4 GGUF models.** `models/` contains
   `phi4-mm-Q4_K_M.gguf` etc., but no code path loads or infers them. [`src/sources/phi4.rs`](../src/sources/phi4.rs)
   only ingests `vocab.json`/`merges.txt` text assets. Chat is fully local.
2. **It does not solve real ARC tasks.** [`data/arc_tasks.json`](../data/arc_tasks.json)
   contains 20 hand-written toy tasks; [`src/metrics/arc.rs`](../src/metrics/arc.rs) marks
   success as `coherence > 0.5` plus a first-token string match. No grid transformation
   or rule induction is performed.
3. **The 16 cognitive agents are keyword-rule systems, not learned minds.** Speech-act
   classification is substring matching over [`data/searle_markers.json`](../data/searle_markers.json);
   synthesis output is fixed sentence templates ([`src/cognitive/word_selection.rs`](../src/cognitive/word_selection.rs)).
4. **The "16-layer memory" is a length-binned episodic log.** [`classify_layer`](../src/memory/mod.rs)
   sorts text into layers by word count and average word length — and the log is not
   recalled into generation.
5. **Coherence 1.0000 results are partly tautological.** Training aligns phases;
   coherence measures phase alignment (see file 14, §3).

## 4. The Three Structural Advantages

Stripped of metaphor, Phiano's architecture buys three things a frozen transformer
cannot have at any parameter count:

1. **Learning is a state update, not a training run.** A new fact ("my daughter Maya is
   allergic to peanuts") is absorbed by phase-attraction of ~5 phasors in microseconds.
   No gradient, no replay buffer, no checkpoint. This is the system's core intellectual
   property.
2. **The context window is O(1).** [`ContextWaveBuffer`](../src/generate.rs) keeps two
   floats (Σx, Σy) with 0.5 decay per turn — an unbounded conversation costs the same
   memory as a two-word one. Transformers pay KV-cache memory linear in history.
3. **New knowledge never displaces old vocabulary.** A new word gets a new phasor
   ([`initialize_tokens`](../src/trainer/mod.rs)); existing phasors are nudged, not
   overwritten. There is no catastrophic-forgetting cliff known from SGD. (Associations
   do drift softly — quantified honestly in file 10, §3.)

## 5. Achievement Rating Summary

| Dimension | Rating (1–5) | One-Line Justification |
|---|---|---|
| Novelty of core mechanism | 5 | Online circular-statistics learning + learned pairwise phase lags is genuinely original in this packaging |
| Engineering quality | 4 | Idiomatic Rust, rayon parallelism, 68 green tests, clean module separation |
| Learning capability (lexical) | 4 | Fast, stable, persistent, self-correcting at the word/association level |
| Learning capability (structural) | 2 | Syntax is counted n-gram tables; no mechanism learns grammar rules |
| Generation quality | 2 | ≤20-token, n-gram-dominated output; composition is templated |
| Cognitive depth | 2 | Rule-based agents; intentionality is classification + templates |
| Evaluation honesty | 2 | Rich harness, but headline numbers are self-referential |
| Production readiness (as semantic memory) | 3 | Server, persistence, API exist; 92 MB model and no auth/HA story |
| Potential as research program | 4 | File 12: sits in legitimate VSA/complex-embedding territory with a credible scaling path |

## 6. Bottom Line

Phiano can achieve — today, on a laptop CPU, with no GPU — **a self-tuning, persistent,
conversational semantic memory** that ingests dictionaries, Wikipedia, and reference-model
vocabularies; answers with locally generated short text; classifies speech acts; measures
its own coherence; and improves with every sentence it reads. That is a real, uncommon
capability profile.

It cannot yet achieve general reasoning or long-form generation, and its evaluative,
cognitive, and topological superstructure currently runs far ahead of what its
one-dimensional phase core mathematically supports. The rest of this analysis explains
both halves of that sentence precisely — and file 16 specifies the shortest credible
path from here to a system that would be genuinely formidable.
