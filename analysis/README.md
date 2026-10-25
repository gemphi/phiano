# Phiano Model Analysis (01–16)

> An independent, code-grounded evaluation of the Phiano phase-manifold oscillator model.
> Written 2026-08-31. Every claim is traced to actual source in [`src/`](../src/) or measured
> results in [`docs/`](../docs/). Where the repository's own documentation makes claims the
> code does not support, that is stated explicitly.

## Read Order

| # | File | Question Answered |
|---|------|-------------------|
| 01 | [01_what_phiano_can_achieve.md](01_what_phiano_can_achieve.md) | What is this system and what can it actually achieve today? |
| 02 | [02_mathematical_foundation.md](02_mathematical_foundation.md) | What is the mathematics of the phase manifold, and is it sound? |
| 03 | [03_learning_engine.md](03_learning_engine.md) | How does Phiano learn, mechanically, step by step? |
| 04 | [04_recursive_learning_cycle.md](04_recursive_learning_cycle.md) | How do envision → apply → eval → iterate → scale work in practice? |
| 05 | [05_memory_and_persistence.md](05_memory_and_persistence.md) | What does it remember, and how does persistence scale? |
| 06 | [06_phinum_engines_and_topology.md](06_phinum_engines_and_topology.md) | What do the 16/32/64 engines, I Ching, and spider-net actually compute? |
| 07 | [07_cognitive_layer_searle.md](07_cognitive_layer_searle.md) | What do the 16 Searle cognitive agents really do? |
| 08 | [08_generation_and_composition.md](08_generation_and_composition.md) | How good is generation, honestly? |
| 09 | [09_reasoning_and_program_synthesis.md](09_reasoning_and_program_synthesis.md) | What reasoning and program-synthesis power exists? |
| 10 | [10_lifelong_learning_power.md](10_lifelong_learning_power.md) | Why is online, no-retraining learning its genuine superpower? |
| 11 | [11_performance_footprint.md](11_performance_footprint.md) | What are the real measured performance and resource numbers? |
| 12 | [12_can_it_learn_anything.md](12_can_it_learn_anything.md) | Could this architecture ever learn *anything*? A capacity analysis. |
| 13 | [13_where_it_is_powerful.md](13_where_it_is_powerful.md) | Where is it genuinely powerful, near-term, in production terms? |
| 14 | [14_limitations_and_risks.md](14_limitations_and_risks.md) | What are the hard limits and honest gaps? |
| 15 | [15_vs_llms.md](15_vs_llms.md) | How does it truly compare with Phi-4 / GLM / transformers? |
| 16 | [16_roadmap_to_power.md](16_roadmap_to_power.md) | What concrete steps would make it formidable? |

## Verification Status (at time of writing)

- `cargo test --release`: **68/68 passing** (62 unit + 6 integration), 0.07 s.
- `cargo build --release`: clean; 4 minor warnings (unused import, dead code).
- Measured training benchmark (see file 11): 13,958 sentences, 6,993-word vocabulary,
  27 MB RAM, 1.11 s total training time, 488,052 phasor updates.

## One-Paragraph Verdict

Phiano is a **working, tested, genuinely novel online-learning engine** that trades the
transformer's statistical firepower for three things transformers cannot do: it learns in
~microseconds per sentence during the conversation itself, it never needs a retraining run,
and it runs in megabytes of RAM on a laptop CPU. Its lexical-semantic core — Kuramoto-style
phase attraction over a complex phasor manifold, plus learned pairwise phase lags — is
mathematically legitimate and sits in real research territory (vector symbolic
architectures, complex knowledge-graph embeddings, circular statistics). Its current
weaknesses are equally clear: single-dimension phases bound its representational capacity,
generation is an n-gram model with phase re-ranking capped at ~20 tokens, the cognitive and
topological layers are rule-based rather than learned, and its internal evaluation metrics
are partly self-referential. It is not yet a general intelligence — but as an
always-on, self-tuning, edge-deployable semantic memory it is real today, and file 12
shows the specific, non-mystical path by which this family of architecture could scale
toward learning arbitrary domains.
