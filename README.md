# Phiano

*From **piano** (Italian: soft/loud) — a phase instrument for language.*

Phiano maps words onto a continuous phase manifold where semantic similarity
is measured by destructive interference between complex wave representations.
Words are keys, phasors are notes, sentences are chords, and training is
tuning — the model self-organizes like a piano that tunes itself.

## How It Works

Phiano represents each word as a **SpectralPhasor** — a complex number on
a 2π phase circle. When words co-occur in a sentence, their phases are
pulled toward the sentence's centroid phase using Kuramoto coupling.
Over many epochs, words that appear in similar contexts converge to
similar phases, creating a self-organizing semantic space.

The complex wave representation is:

```
Z = A · e^(i·(φ + n·α))
```

Where:
- **A** is amplitude (familiarity weight)
- **φ** is the primary phase angle
- **n** is the quantized energy sub-band level
- **α** is the fine-structure constant (~1/137)

Semantic similarity between words is measured by the energy delta
(destructive interference): `Δ = α · |Z₁ - Z₂|²`. Lower delta means
the words are more semantically similar.

## Recursive Learning Cycle

```
envision → apply → eval → iterate → scale
```

Each user input triggers this cycle:
1. **Envision** — detect unknown words, suggest related known words
2. **Apply** — train the facet on the input
3. **Eval** — score the input's coherence, novelty, and resonance
4. **Iterate** — repeat for each command
5. **Scale** — persist knowledge to disk on exit

## Quick Start

```sh
cargo run
```

```
phiano> learn "the cat sat on the mat"
phiano> learn "the dog sat on the rug"
phiano> synonym cat 5
phiano> eval "the cat sat on the mat"
phiano> stats
phiano> exit
```

### Showcase

```sh
cargo run < task\showcase.txt
```

Runs a full demo: composition, persona creation, comparison, style
attribution, impersonation, and interactive chat.

### Persona Chat — Hear Their Story, Chat in Their Style

```
phiano> persona from hemingway "The old man fished alone in the skiff. He had gone eighty-four days without taking a fish."
phiano> persona chat hemingway
```

Paste someone's text, get their persona, chat with them. The `persona from`
command auto-splits a block of text into sentences as examples — no need
to manually quote each sentence. Then `persona chat` lets you interact.
The persona introduces itself with derived traits, phase signature,
and fingerprint numbers. Type `bye` to end.

## Commands

### Core

| Command                  | Description |
|--------------------------|-------------|
| `learn "text"`           | Train on a sentence |
| `define <word>`          | Fetch & learn a word's definition |
| `eval "text"`            | Score text quality |
| `compose "prompt" [rounds] ["ex"]` | Recursive sector composition |
| `om eval "text"`         | Evaluate text in oscillator mode (sphere model) |
| `om wheel`               | Show the oscillator color wheel |
| `om sphere "text"`       | Show sphere projection for text |
| `om compare "text"`      | Compare transform vs oscillator models |
| `synonym <word> [n]`     | Find n nearest resonant words |
| `resonance "text" [n]`   | Find words resonating with text |
| `wave "text"`            | Show the sentence's complex wave |
| `save` / `load`          | Persist / load the facet |
| `stats`                  | Show facet + memory statistics |

### Persona

| Command                  | Description |
|--------------------------|-------------|
| `persona add <name> "ex1" "ex2" ...` | Create persona from examples |
| `persona from <name> "text block"` | Create persona from a text block (auto-splits sentences) |
| `persona list`           | List all personas |
| `persona show <name>`    | Show persona fingerprint |
| `persona compare <a> <b>` | Compare two personas |
| `persona impersonate <name> "prompt"` | Compose text as persona |
| `persona match "unknown text"` | Attribute text to closest persona |
| `persona chat <name>`    | Interactive chat with a persona |

### Drivers

| Command                  | Description |
|--------------------------|-------------|
| `ingest <file.txt>`      | Bulk ingest local definitions |
| `ingest-json <file.json>`| Bulk ingest JSON dictionary |
| `ingest-wiktionary <f>`  | Bulk ingest Wiktionary dump |
| `chunk <dict.json>`      | Split dictionary into chunks |
| `train [epochs]`         | Train from chunks in parallel |

Unrecognized input is treated as text to learn from.

## Persona System

### Fingerprint
A persona's fingerprint is a sector histogram showing where their text
clusters in phase space. Stop words are filtered out so content words
dominate. Word-level contributions are weighted by inverse amplitude
(rare distinctive words matter more).

### Style Attribution
`persona match` extracts a fingerprint from unknown text and computes
**likelihood** against all personas. Likelihood rewards texts that
concentrate in the persona's dominant sectors and penalizes texts in
sectors where the persona is weak. All fingerprints are re-extracted
at match time for fair comparison.

### Personality Traits
Traits are derived from the phase-space color distribution:
passionate (warm sectors), contemplative (cool sectors), balanced
(green sectors), dynamic (warm+cool mix), versatile/focused (entropy),
elaborate/concise (avg length). Not hardcoded — computed from geometry.

### Chat
`persona chat <name>` enters an interactive loop. The persona introduces
itself with traits, phase signature, and numbers. Each question is
answered via impersonation — the prompt vibrates through the persona's
fingerprint. Quality, fit, and sector metrics shown per turn.

## Oscillator Mode (om)

The oscillator model is an alternative to the transform model. Words are
**spinning oscillators on a 3D sphere** instead of static points on a 2D
circle. The sphere's surface is a color spectrum — hue from longitude,
brightness from latitude. Your viewing angle determines which colors
you see. The spectrum changes as you rotate.

| Feature | Transform | Oscillator |
|---------|-----------|------------|
| Geometry | 2D circle | 3D sphere |
| Word | Static phasor | Spinning oscillator |
| Coherence | Wave norm / N | Kuramoto order parameter |
| Similarity | Energy delta | Synchronization |
| Color diversity | — | Spectral entropy |
| Time | Static | Dynamic (rotates) |
| Viewing angle | Fixed | Variable |

See `docs/42_oscillator_mode.md` for the full depiction of the wheel/sphere.

## Architecture

```
src/
├── main.rs         Entry point
├── model.rs        Recursive learning agent
├── config.rs       Configuration constants
├── facet.rs        Core lexicon (Facet)
├── phasor.rs       Complex phasor (SpectralPhasor)
├── wave.rs         Wave operations & ray casting (Wave, c64)
├── oscillator.rs   Oscillator model: sphere, color wheel, sync (om)
├── trainer.rs      Kuramoto phase attraction (Trainer)
├── eval.rs         Semantic evaluation (Evaluator, Verdict)
├── envision.rs     Knowledge gap detection (Envision, Vision)
├── memory.rs       16-layer memory log (Memo, MemoryBand)
├── storage.rs      Binary persistence (Storage)
├── tokenizer.rs    Text normalization (Tokenizer)
├── chunker.rs      Dictionary chunking (ChunkStore)
├── compose/        Recursive sector composition
│   ├── mod.rs      Composition struct, sector colors
│   ├── flow.rs     RiverFlow: word selection by sector + ray_cast
│   ├── better.rs   Evaluation with prompt alignment
│   ├── worse.rs    Discard strategy
│   └── tune.rs     CompositionTuner: recursive refinement
├── persona/        Persona system
│   ├── mod.rs      Persona struct, similarity
│   ├── fingerprint.rs  Fingerprint, likelihood, traits
│   ├── impersonate.rs  Impersonator: biased composition
│   └── world.rs    PersonaWorld, MatchResult, comparisons
├── command/        REPL command handlers
│   ├── om.rs       Oscillator mode: eval, wheel, sphere, compare
├── drivers/        Source commands (ingest, train)
└── sources/        Dictionary data sources
```

See `src/spec.md` for the full module specification.
See `docs/` for 42 diagrammatic documents covering every aspect of the system.

## 16-Layer Memory

Interactions are classified into 16 layers across 4 bands:

| Band     | Layers  | Description |
|----------|---------|-------------|
| Surface  | 0-3     | Raw word recognition |
| Pattern  | 4-7     | Co-occurrence, bigram resonance |
| Semantic | 8-11    | Definition context, synonym clustering |
| Deep     | 12-15   | Abstract concept relationships |

## Dependencies

- `num-complex` — Complex number arithmetic
- `serde` + `bincode` — Binary serialization
- `rayon` — Parallel iteration for ray casting and chunk loading
- `rustyline` — REPL line editing
- `reqwest` — HTTP for API dictionary source

## License

MIT
