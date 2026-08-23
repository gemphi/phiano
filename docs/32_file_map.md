# 32 — Complete File Map

## Project Structure

```
  phiano/
  │
  ├── Cargo.toml              ← dependencies & metadata
  ├── Cargo.lock              ← pinned dependency versions
  ├── README.md               ← project overview & quick start
  │
  ├── data/
  │   ├── websters_dictionary.json   (22 MB, 30k+ words)
  │   ├── definitions.txt            (5 KB, local defs)
  │   ├── manifold.chroma            (5.7 MB, trained facet)
  │   ├── memory.chroma              (136 B, memory log)
  │   └── chunks/                    (letter-split JSON files)
  │       ├── a/a.json
  │       ├── b/b.json
  │       └── ...
  │
  ├── docs/                           ← 32 diagrammatic docs
  │   ├── 01_piano_etymology.md
  │   ├── 02_architecture_overview.md
  │   ├── 03_phase_manifold.md
  │   ├── 04_spectral_phasor.md
  │   ├── 05_complex_wave.md
  │   ├── 06_kuramoto_coupling.md
  │   ├── 07_ray_casting.md
  │   ├── 08_memory_layers.md
  │   ├── 09_learning_cycle.md
  │   ├── 10_tokenizer.md
  │   ├── 11_facet.md
  │   ├── 12_wave_superposition.md
  │   ├── 13_energy_delta.md
  │   ├── 14_fine_structure.md
  │   ├── 15_command_dispatch.md
  │   ├── 16_eval_scoring.md
  │   ├── 17_envision.md
  │   ├── 18_storage.md
  │   ├── 19_sources.md
  │   ├── 20_chunking.md
  │   ├── 21_context_handlers.md
  │   ├── 22_trainer_deep_dive.md
  │   ├── 23_amplitude.md
  │   ├── 24_band_levels.md
  │   ├── 25_centroid_novelty.md
  │   ├── 26_repl.md
  │   ├── 27_verdict.md
  │   ├── 28_memory_band.md
  │   ├── 29_fnv_hash.md
  │   ├── 30_training_pipeline.md
  │   ├── 31_golden_ratio.md
  │   └── 32_file_map.md           ← this file
  │
  └── src/
      ├── main.rs              ← entry point
      ├── model.rs             ← recursive learning agent
      ├── config.rs            ← constants
      ├── facet.rs             ← core lexicon
      ├── phasor.rs            ← complex phasor
      ├── wave.rs              ← wave ops & c64 alias
      ├── trainer.rs           ← Kuramoto learning
      ├── eval.rs              ← evaluation (Verdict enum)
      ├── envision.rs          ← gap detection
      ├── memory.rs            ← 16-layer memo (MemoryBand enum)
      ├── storage.rs           ← binary persistence
      ├── tokenizer.rs         ← text processing
      ├── chunker.rs           ← parallel chunking
      ├── spec.md              ← source module spec
      │
      ├── command/
      │   ├── mod.rs           ← Dispatcher, Context, Command enum
      │   ├── help.rs          ← help command
      │   ├── learn.rs         ← learn command
      │   ├── define.rs        ← define command
      │   ├── eval.rs          ← eval command
      │   ├── synonym.rs       ← synonym command
      │   ├── resonance.rs     ← resonance command
      │   ├── wave.rs          ← wave display command
      │   ├── ingest.rs        ← ingestion commands
      │   ├── chunk.rs         ← chunk command
      │   ├── train.rs         ← parallel train command
      │   ├── save.rs          ← save/load commands
      │   ├── stats.rs         ← statistics command
      │   └── spec.md          ← command module spec
      │
      └── sources/
          ├── mod.rs           ← DictionarySource trait, Ingester
          ├── api.rs           ← API source
          ├── json.rs          ← JSON source
          ├── local.rs         ← local file source
          ├── wiktionary.rs    ← Wiktionary source
          └── spec.md          ← sources module spec
```

## Key Types Summary

```
  ┌────────────────┬──────────────────┬──────────────────────┐
  │ Type           │ Module           │ Purpose              │
  ├────────────────┼──────────────────┼──────────────────────┤
  │ Facet          │ facet.rs         │ Word→Phasor lexicon  │
  │ SpectralPhasor │ phasor.rs        │ 16-byte phasor       │
  │ c64            │ wave.rs          │ Complex64 alias      │
  │ Wave           │ wave.rs          │ Wave operations      │
  │ Trainer        │ trainer.rs       │ Kuramoto learning    │
  │ Evaluator      │ eval.rs          │ Text scoring         │
  │ Verdict        │ eval.rs          │ Qualitative enum     │
  │ Envision       │ envision.rs      │ Gap detection        │
  │ Vision         │ envision.rs      │ Gap report           │
  │ Memo           │ memory.rs        │ 16-layer memory      │
  │ MemoryBand     │ memory.rs        │ Depth band enum      │
  │ Storage        │ storage.rs       │ Persistence facade   │
  │ Tokenizer      │ tokenizer.rs     │ Text processing      │
  │ ChunkStore     │ chunker.rs       │ Parallel chunking    │
  │ Command        │ command/mod.rs   │ Command enum         │
  │ Context        │ command/mod.rs   │ Shared handler ctx   │
  │ Dispatcher     │ command/mod.rs   │ Command router       │
  │ DictionarySource│ sources/mod.rs  │ Source trait         │
  └────────────────┴──────────────────┴──────────────────────┘
```
