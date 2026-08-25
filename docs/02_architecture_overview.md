# 02 - Phiano Architecture Overview

> _High-level system topology, recursive learning cycle, and module interaction pipeline._

---

## 1. Complete Architecture Topology

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'background': 'transparent', 'mainBkg': 'transparent', 'nodeBorder': '#3b82f6', 'clusterBkg': 'transparent', 'clusterBorder': '#475569', 'lineColor': '#60a5fa', 'textColor': '#ffffff', 'primaryTextColor': '#ffffff', 'nodeTextColor': '#ffffff', 'edgeLabelBackground': '#0f172a'}}}%%
graph TD
    subgraph "Clients & Frontends"
        REPL["CLI REPL<br/>(rustyline)"]
        REST["REST API Server<br/>(Axum)"]
        WebDash["Web Dashboard<br/>(React + Vite)"]
    end

    subgraph "Core Dispatcher & Coordination"
        Dispatcher["Command Dispatcher<br/>(src/command/mod.rs)"]
    end

    subgraph "5-Phase Recursive Learning Cycle"
        Envision["1. Envision<br/>(Gap Detection)"]
        Apply["2. Apply / Train<br/>(Kuramoto Coupling)"]
        Eval["3. Eval<br/>(Coherence Scoring)"]
        Iterate["4. Iterate<br/>(16-Layer Memo)"]
        Scale["5. Scale<br/>(Bincode Storage)"]
    end

    subgraph "Mathematical Substrate"
        Facet["Facet Lexicon<br/>(HashMap<String, SpectralPhasor>)"]
        Phasor["SpectralPhasor<br/>Z = A · exp(i(φ + nα))"]
        Wave["Wave & Ray-Casting<br/>(c64 ops)"]
    end

    subgraph "Generative & Cognitive Engines"
        Generator["Phase Sequence Generator<br/>(src/generate.rs)"]
        Composer["RiverFlow Composer<br/>(src/compose/mod.rs)"]
        Reasoning["Reasoning Engine<br/>(src/reasoning.rs)"]
        Oscillator["Oscillator Field & Sphere<br/>(src/oscillator/mod.rs)"]
        Persona["Persona Fingerprinting<br/>(src/persona/mod.rs)"]
    end

    REPL --> Dispatcher
    REST --> Dispatcher
    WebDash --> REST

    Dispatcher --> Envision
    Envision --> Apply
    Apply --> Eval
    Eval --> Iterate
    Iterate --> Scale

    Apply <--> Facet
    Facet <--> Phasor
    Phasor <--> Wave

    Facet --> Generator
    Facet --> Composer
    Facet --> Reasoning
    Facet --> Oscillator
    Facet --> Persona
```

---

## 2. The 28 Core Modules

Phiano's core engine is structured into 28 specialized, zero-overhead Rust modules in [`src/`](file:///c:/Users/phiac/Workspace/gemphi/phiano/src/):

| Category | Modules | Purpose |
|:---|:---|:---|
| **Phase Mathematics** | `phasor.rs`, `wave.rs`, `config/` | Complex numbers, $2\pi$ circle coordinates, fine-structure quantization |
| **Lexicon & Training** | `facet.rs`, `trainer/`, `tokenizer.rs`, `chunker.rs`, `curriculum.rs` | Kuramoto coupling, vocabulary self-tuning, parallel chunking |
| **Cognitive Memory** | `memory/`, `layers.rs`, `storage.rs`, `envision.rs`, `eval.rs` | 16-layer memory, gap detection, coherence evaluation, bincode persistence |
| **Generative Systems** | `generate.rs`, `compose/`, `attention.rs`, `synthetic.rs` | Context wave superposition, RiverFlow narrative generation, attention |
| **Reasoning & Oscillators**| `reasoning.rs`, `oscillator/`, `cognitive/`, `persona/` | Phase pathfinding, 3D sphere projection, persona fingerprinting |
| **Servers & Sources** | `server/`, `sources/`, `command/`, `drivers/`, `wiki_bulk.rs` | Axum HTTP server, Webster's dictionary ingester, CLI REPL commands |

---

## 3. Related Documentation

- For the full cross-module connection matrix, see [`MASTER_CONNECTIONS.md`](./MASTER_CONNECTIONS.md).
- For complete file map, see [`32_file_map.md`](./32_file_map.md).
- For theoretical foundation, see [`03_phase_manifold.md`](./03_phase_manifold.md).
