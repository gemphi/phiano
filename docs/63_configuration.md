# Configuration Guide

All tunables live in `src/config/constants.rs`. Change a constant, rebuild. There is no YAML/env overlay yet — the constants **are** the config.

## Learning

| Constant | Default | Range | Effect |
|----------|---------|-------|--------|
| `LEARNING_RATE` | `0.05` | 0.01–0.15 | How fast a word's phase θ drifts toward its sentence centroid. Higher = faster, less stable. |
| `AMPLITUDE_INCREMENT` | `0.001` | — | Familiarity growth per training hit. |
| `AMPLITUDE_MAX` | `2.0` | — | Ceiling on familiarity. |
| `AMPLITUDE_INITIAL` | `1.0` | — | New-word starting amplitude. |
| `CONVERGENCE_THRESHOLD` | `0.0005` | — | When `|sin(Δθ)|` is below this, `band_n` increments (prevents collapse). |
| `INGEST_EPOCHS` | `64` | — | Default epochs for bulk ingest. |

## Syntax / generation

| Constant | Default | Effect |
|----------|---------|--------|
| `SYNTACTIC_LAG_BETA` | `π/16` | Default directional lag β when no pair has been learned. |
| `SYNTAX_LAG_LEARN_RATE` | `0.08` | EMA rate for βᵢⱼ from observed word order. |
| `SYNTACTIC_MOMENTUM_DEFAULT` | `0.15` | Initial phase velocity during generation. |
| `PHASE_REPULSION` | `π` | Anti-phase pulse for `!correct` / `correct_mistake`. |
| `TORUS_DECODE_POOL` | `48` | Candidate pool for torus ray-cast. |
| `TORUS_HARMONICS_COUNT` | `32` | Harmonic bands on the torus (T³²). |
| `DEFAULT_CONTEXT_WINDOW` | `4096` | Ring-buffer capacity of `ContextWaveBuffer`. |
| `DEFAULT_REASONING_STEPS` | `5` | Reasoning-chain iteration cap. |

## Evaluation

| Constant | Default | Effect |
|----------|---------|--------|
| `NOVELTY_SCALE` | `0.3` | `novelty = 1 − exp(−distance × scale)` |
| `EVAL_WEIGHT_COHERENCE` | `0.4` | Weight in overall score |
| `EVAL_WEIGHT_NOVELTY` | `0.3` | |
| `EVAL_WEIGHT_RESONANCE` | `0.3` | Fraction of known tokens |

## Paths

| Constant | Default |
|----------|---------|
| `CHROMA_FILE` | `data/manifold.chroma` |
| `MEMORY_FILE` | `data/memory.chroma` |
| `CHUNK_STORE_DIR` | `data/chunks` |
| `DEFINITIONS_FILE` | `data/definitions.txt` |

## Tuning recipes

- **More fluent, less novel** — raise `LEARNING_RATE` to `0.10`, lower generation temperature (`/api/generate` `temperature: 0.05`).
- **More McKenna / recursive** — keep `LEARNING_RATE` at `0.05`, raise temperature to `0.3`, leave `SYNTACTIC_MOMENTUM_DEFAULT` at `0.15`.
- **Faster bootstrap** — `cargo run --release --bin bootstrap_facet -- data/rust_book_corpus.txt 1000`.
- **Surgical correction** — `PHASE_REPULSION` is π; do not change unless you want partial (not anti-) phase push.

Constants are re-exported from `phiano::config`. The PUI does not hot-reload them — restart the binary after edits.
