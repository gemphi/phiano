# Phiano — Source Module Specification

## Architecture Overview

Phiano is a chromatic resonance agent that learns language by mapping words
onto a continuous phase manifold. Semantic similarity is measured by
destructive interference between complex wave representations.

The system operates in a recursive learning cycle:

```
envision → apply → eval → iterate → scale
```

Each user input triggers this cycle: the model envisions what it doesn't
know, applies training, evaluates understanding, iterates on gaps, and
scales by persisting knowledge.

## Module Layout

```
src/
├── main.rs        — Entry point, REPL initialization
├── model.rs       — Recursive learning agent (Model struct)
├── config.rs      — Central configuration constants
├── facet.rs       — Core lexicon (Facet struct)
├── phasor.rs      — Complex phasor representation (SpectralPhasor)
├── wave.rs        — Wave operations and ray casting (Wave, c64)
├── trainer.rs     — Kuramoto phase attraction learning (Trainer)
├── eval.rs        — Semantic evaluation (Evaluator, Eval, Verdict)
├── envision.rs    — Knowledge gap detection (Envision, Vision)
├── memory.rs      — 16-layer memory log (Memo, MemoryBand)
├── storage.rs     — Binary persistence (Storage, SerializedFacet)
├── tokenizer.rs   — Text normalization and tokenization (Tokenizer)
├── chunker.rs     — Dictionary chunking for parallel ingestion (ChunkStore)
├── command/       — REPL command handlers
│   ├── mod.rs     — Dispatcher, Context, Command enum, Parser
│   ├── help.rs    — Help command
│   ├── learn.rs   — Learn command (online training)
│   ├── define.rs  — Define command (word definition lookup)
│   ├── eval.rs    — Eval command (text evaluation)
│   ├── synonym.rs — Synonym command (ray cast word search)
│   ├── resonance.rs — Resonance command (ray cast wave search)
│   ├── wave.rs    — Wave command (display wave representation)
│   ├── ingest.rs  — Ingest commands (local, JSON, Wiktionary)
│   ├── chunk.rs   — Chunk command (split dictionaries)
│   ├── train.rs   — Train command (parallel chunk training)
│   ├── save.rs    — Save/Load commands
│   └── stats.rs   — Stats command (facet + memory statistics)
└── sources/       — Dictionary data sources
    ├── mod.rs     — DictionarySource trait, Ingester
    ├── api.rs     — API dictionary source
    ├── json.rs    — JSON dictionary source
    ├── local.rs   — Local file dictionary source
    └── wiktionary.rs — Wiktionary dump source
```

## Core Types

### Facet (`facet.rs`)
The core lexicon — a `HashMap<String, SpectralPhasor>` mapping words to
their complex phasor representations. Provides vocabulary lookup,
amplitude statistics, and centroid computation.

### SpectralPhasor (`phasor.rs`)
A 16-byte fixed-width complex phasor with:
- `phase: f64` — primary angle on [0, 2*pi)
- `amplitude: f64` — intensity/familiarity weight
- `band_n: u32` — quantized energy sub-band level

Complex representation: `Z = A * e^(i*(phi + n*alpha))`
where alpha is the fine-structure constant (~1/137).

### Wave (`wave.rs`)
Static methods for wave operations:
- `Wave::sentence(facet, words)` — superposition wave for known words
- `Wave::text(facet, text)` — wave for raw text (tokenizes first)
- `Wave::ray_cast_word(facet, word, k)` — find k nearest words by energy delta
- `Wave::ray_cast(facet, wave, k)` — find k words resonating with a wave

Type alias: `c64 = Complex64`

### Trainer (`trainer.rs`)
Unsupervised learning via Kuramoto phase attraction:
- `train_sentence` — train on one sentence (core algorithm)
- `train_batch` — train on multiple sentences for multiple epochs
- `train_online` — single-pass training for interactive REPL
- `train_definition` — train on a word-definition pair

### Memo (`memory.rs`)
16-layer memory log with 4 bands (Surface, Pattern, Semantic, Deep).
Each interaction is classified by word count and average word length.

### Evaluator (`eval.rs`)
Scores text on three dimensions:
- **Resonance**: fraction of known tokens (0.0-1.0)
- **Coherence**: wave alignment normalized by token count (0.0-1.0)
- **Novelty**: distance from facet centroid (0.0-1.0)
- **Overall**: weighted combination (45% coherence, 40% resonance, 15% novelty)

Produces a `Verdict` enum with 9 qualitative categories.

### Envision (`envision.rs`)
Detects knowledge gaps by finding unknown words and suggesting similar
known words using prefix overlap and bigram Jaccard similarity.

## Key Enums

### `Command` (`command/mod.rs`)
Represents all recognized REPL commands. Parsed from strings with
`Command::from_str()`. Used by the dispatcher for routing.

### `Verdict` (`eval.rs`)
Qualitative assessment of text quality. 9 variants from `Empty` to
`CoherentNovel`. Implements `Display` for human-readable output.

### `MemoryBand` (`memory.rs`)
4-variant enum representing depth bands (Surface, Pattern, Semantic, Deep).
Each band covers 4 layers. Implements `Display` for stats output.

## Data Flow

```
User Input
    │
    ▼
Model::iterate()
    │
    ├─► Dispatcher::dispatch() ──► Command handler
    │                                   │
    │                                   ├─► Trainer (trains Facet)
    │                                   ├─► Evaluator (scores text)
    │                                   └─► Envision (detects gaps)
    │
    └─► Model::envision() ──► Envision::detect_gaps()
    │
    ▼
Model::scale() (on exit)
    ├─► Storage::save() — persists Facet
    └─► Memo::save_to_file() — persists memory
```

## File Formats

### `.chroma` files (bincode serialized)
- `ChromaHeader` — version, vocabulary_size, fine_structure_alpha
- `HashMap<String, SpectralPhasor>` — the lexicon

### Memory file (bincode serialized)
- `Vec<ContextWaveEntry>` — flat log
- `[Vec<ContextWaveEntry>; 16]` — layered entries

### Chunk files (JSON)
```
data/chunks/<letter>/<letter>.json
```
Each file contains a `HashMap<String, String>` of word→definition pairs.
