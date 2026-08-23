# 11 — Facet (Core Lexicon)

## Structure

```
  Facet {
      lexicon: HashMap<String, SpectralPhasor>
  }

  ┌──────────────────────────────────────────────┐
  │  lexicon                                     │
  │                                              │
  │  "apple"  → SpectralPhasor { φ=1.2, A=1.5, n=3 } │
  │  "banana" → SpectralPhasor { φ=3.8, A=1.2, n=2 } │
  │  "cat"    → SpectralPhasor { φ=2.5, A=1.8, n=5 } │
  │  "dog"    → SpectralPhasor { φ=2.6, A=1.7, n=4 } │
  │  ...                                        │
  │  (30,000+ entries after training)           │
  └──────────────────────────────────────────────┘
```

## Key Methods

```
  vocabulary_size()     → N (number of words)
  contains_word(w)      → bool
  get_phasor(w)         → Option<&SpectralPhasor>
  average_amplitude()   → f64 (mean familiarity)
  dominant_band()       → u32 (most common n level)
  centroid()            → c64 (center of semantic space)

  ┌─────────────────────────────────────────────┐
  │  Centroid = Σ all phasors as complex        │
  │                                             │
  │  Z_centroid = Σ Aᵢ · e^(i·(φᵢ + nᵢ·α))    │
  │                                             │
  │  Represents the "average meaning"           │
  │  of everything the model knows.             │
  └─────────────────────────────────────────────┘
```

## Serialization

```
  Facet ──bincode──► .chroma file
    ├── ChromaHeader { version, vocab_size, alpha }
    └── HashMap<String, SpectralPhasor>

  File size: ~5.7 MB for 30k words
  Load time: <100ms
```
