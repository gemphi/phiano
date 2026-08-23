# 02 — Architecture Overview

## System Diagram

```
  ┌─────────────────────────────────────────────────────────┐
  │                       PHIANO                            │
  │                                                         │
  │  ┌─────────┐    ┌─────────┐    ┌─────────┐            │
  │  │  REPL   │───►│ Command │───►│ Trainer │            │
  │  │ (rusty) │    │Dispatch │    │(Kuramoto)│            │
  │  └────┬────┘    └────┬────┘    └────┬────┘            │
  │       │              │              │                   │
  │       │         ┌────┴────┐    ┌────▼────┐            │
  │       │         │ Context │    │  Facet  │            │
  │       │         │ (shared)│    │(lexicon)│            │
  │       │         └────┬────┘    └────┬────┘            │
  │       │              │              │                   │
  │       │         ┌────▼────┐    ┌────▼────┐            │
  │       │         │  Memo   │    │  Wave   │            │
  │       │         │(16-layer│    │(c64 ops)│            │
  │       │         │ memory) │    └─────────┘            │
  │       │         └─────────┘                            │
  │       │                                                │
  │  ┌────▼──────────────────────────────────┐            │
  │  │         Recursive Learning Cycle       │            │
  │  │  envision → apply → eval → iterate    │            │
  │  │                          → scale      │            │
  │  └───────────────────────────────────────┘            │
  │                                                         │
  │  ┌─────────┐  ┌─────────┐  ┌──────────┐              │
  │  │ Storage │  │ Sources │  │ Chunker  │              │
  │  │(bincode)│  │(4 types)│  │(parallel)│              │
  │  └─────────┘  └─────────┘  └──────────┘              │
  └─────────────────────────────────────────────────────────┘
```

## Module Dependency Graph

```
  main.rs
    └── model.rs
          ├── facet.rs ◄── phasor.rs
          │     └── wave.rs ◄── tokenizer.rs
          ├── trainer.rs ◄── tokenizer.rs, phasor.rs
          ├── memory.rs
          ├── storage.rs ◄── facet.rs, phasor.rs
          ├── envision.rs ◄── tokenizer.rs, facet.rs
          ├── eval.rs ◄── tokenizer.rs, wave.rs, facet.rs
          ├── chunker.rs ◄── facet.rs, trainer.rs
          ├── command/
          │     ├── mod.rs (Dispatcher, Context, Command enum)
          │     ├── learn.rs, define.rs, eval.rs
          │     ├── synonym.rs, resonance.rs, wave.rs
          │     ├── ingest.rs, chunk.rs, train.rs
          │     ├── save.rs, stats.rs, help.rs
          │     └── (each uses Context → facet, trainer, memo)
          └── sources/
                ├── mod.rs (DictionarySource trait)
                ├── api.rs, json.rs
                ├── local.rs, wiktionary.rs
```
