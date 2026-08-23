# 32 — Complete Phiano Repository File Map

> _Comprehensive file, directory, and module index of the Phiano repository._

---

## 1. Project Directory Tree

```
phiano/
├── Cargo.toml                       # Dependencies & package configuration
├── Cargo.lock                       # Pinned dependency versions
├── README.md                        # Master project documentation
├── PLAN.md                          # Strategic roadmap & architectural milestones
├── .gitignore                       # Target, build, and weight exclusion rules
├── tsconfig.json                    # Root TypeScript monorepo config
│
├── data/                            # Training Corpora & Dictionaries
│   ├── README.md                    # Data index & schema documentation
│   ├── websters_dictionary.json     # Complete Webster's English Dictionary (30k+ words)
│   ├── rust_book_corpus.txt         # Normalized Rust Book training corpus
│   ├── curriculum.json              # Phased multi-epoch learning curriculum
│   ├── definitions.txt              # Seed definitions & anchor points
│   ├── searle_markers.json          # Chinese room experiment cognitive probes
│   ├── stop_words.txt               # High-frequency stop words
│   └── api_cache.txt                # Cached definition lookups
│
├── docs/                            # 44 Technical Architecture & Whitepaper Docs
│   ├── README.md                    # Documentation master index
│   ├── MASTER_CONNECTIONS.md        # System topology & connection matrix
│   ├── 01_piano_etymology.md        # Acoustic / piano language metaphor
│   ├── 02_architecture_overview.md   # System architecture & dataflow
│   ├── 03_phase_manifold.md         # Continuous 2π circle manifold math
│   ├── 04_spectral_phasor.md        # Complex coordinates Z = A · exp(i(φ + nα))
│   ├── 05_complex_wave.md           # Wave superposition & interference
│   ├── 06_kuramoto_coupling.md      # Kuramoto non-linear oscillator dynamics
│   ├── 07_ray_casting.md            # Fast nearest-neighbor ray casting
│   ├── 08_memory_layers.md          # 16-layer memory hierarchy
│   ├── 09_learning_cycle.md         # 5-stage recursive learning cycle
│   ├── 12_wave_superposition.md     # Sentence chord composition
│   ├── 13_energy_delta.md           # Destructive interference distance metric
│   ├── 14_fine_structure.md         # Sommerfeld constant α ≈ 1/137
│   ├── 16_eval_scoring.md           # Coherence, novelty & resonance scoring
│   ├── 38_persona_system.md         # Persona fingerprinting & voice extraction
│   ├── 42_oscillator_mode.md        # Sphere / wheel oscillator modes
│   ├── 43_phi_recursive_machine.md  # State machine architecture
│   ├── API_REFERENCE.md             # REST API endpoint documentation
│   ├── COGNITIVE_SPACES_OSCILLATOR_PAPER.md # Theoretical whitepaper
│   ├── papers/                      # Theoretical academic papers
│   └── rust_mastery/                # Rust systems engineering guides
│
├── scripts/                         # Command-Line Utilities & Demos
│   ├── README.md                    # Script catalog & usage
│   ├── demo.py                      # Interactive phase manifold demo
│   ├── cli_walkthrough.py           # Automated CLI feature tour
│   ├── ask_rust.py                  # Rust Book Q&A interface
│   ├── fetch_rust_book.py           # Corpus downloader & parser
│   ├── download_phi4.py             # Quantized GGUF model downloader
│   ├── download_phi4_vision.py      # Vision model weight downloader
│   └── train_and_compose_story.py   # Story composition script
│
├── specs/                           # Formal Mathematical Specifications
│   └── README.md                    # Formal manifold, Kuramoto & wave specs
│
├── src/                             # Core Rust Engine (28 Modules)
│   ├── lib.rs                       # Library entrypoint & module declarations
│   ├── main.rs                      # Binary entrypoint (CLI / REPL / Server)
│   ├── phasor.rs                    # SpectralPhasor complex number representation
│   ├── wave.rs                      # Wave superposition & ray-casting
│   ├── facet.rs                     # Facet continuous lexicon
│   ├── trainer/                     # Kuramoto phase attraction trainer
│   ├── generate.rs                  # Phase-guided sequence generator
│   ├── attention.rs                 # Harmonic attention & candidate re-ranking
│   ├── compose/                     # RiverFlow recursive story composer
│   ├── reasoning.rs                 # Phase-space pathfinding & convergence
│   ├── oscillator/                  # 3D spinning sphere oscillator field
│   ├── persona/                     # Persona fingerprinting & voice impersonation
│   ├── cognitive/                   # Degrees of freedom & intentionality markers
│   ├── layers.rs                    # 4-band hierarchical phase field
│   ├── memory/                      # 16-layer memory logging & retrieval
│   ├── envision.rs                  # Knowledge gap & unknown word detection
│   ├── eval.rs                      # Semantic coherence & novelty evaluation
│   ├── storage.rs                   # Bincode binary persistence
│   ├── tokenizer.rs                 # Text normalization & FNV-1a hashing
│   ├── chunker.rs                   # Parallel rayon chunk store
│   ├── curriculum.rs                # Curriculum training orchestrator
│   ├── synthetic.rs                 # Synthetic training data generator
│   ├── server/                      # Axum REST API endpoints
│   ├── command/                     # REPL command handlers & dispatcher
│   ├── drivers/                     # Data drivers (chunk, ingest, train)
│   ├── sources/                     # Webster's / Wiktionary dictionary sources
│   ├── config/                      # Mathematical constants & α parameters
│   └── bin/                         # Standalone training binaries
│
├── tasks/                           # Recipe Scripts & Demos
│   ├── README.md                    # Task index & run instructions
│   ├── showcase.txt                 # Full system showcase script
│   ├── chat_demo.txt                # Interactive chat session script
│   ├── story_demo.txt               # Narrative composition script
│   └── elon_demo.txt                # Persona impersonation demo script
│
├── tests/                           # Python & Integration Test Suite
│   ├── README.md                    # Test suite overview
│   ├── __init__.py                  # Test package initialization
│   ├── test_cognitive.py            # Cognitive space tests
│   ├── test_endpoints.py            # REST API endpoint tests
│   ├── test_generate_phase.py       # Generative sequence tests
│   ├── test_layers_phase.py         # 16-layer memory hierarchy tests
│   ├── test_phiano_rust.py          # Rust binary subprocess tests
│   ├── test_reasoning_phase.py      # Reasoning convergence tests
│   └── ... (16 test modules)
│
└── web/                             # React + Vite Interactive Dashboard
    ├── README.md                    # Frontend setup & dev guide
    ├── index.html                   # HTML entrypoint
    ├── vite.config.ts               # Vite configuration
    ├── package.json                 # Node dependencies
    └── src/                         # UI components, panels & canvas visualizers
```
