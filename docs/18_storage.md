# 18 - Storage & Persistence

## Binary Format (.chroma)

```
  ┌──────────────────────────────────────────────────┐
  │  File: data/manifold.chroma                      │
  │  Format: bincode (binary serialization)          │
  │  Size: ~5.7 MB for 30k words                     │
  │                                                  │
  │  ┌────────────────────────────────────────────┐  │
  │  │  SerializedFacet                            │  │
  │  │                                             │  │
  │  │  ┌──────────────────────────────────────┐  │  │
  │  │  │  ChromaHeader                         │  │  │
  │  │  │  ├── version: u32 = 1                │  │  │
  │  │  │  ├── vocabulary_size: usize           │  │  │
  │  │  │  └── fine_structure_alpha: f64        │  │  │
  │  │  └──────────────────────────────────────┘  │  │
  │  │                                             │  │
  │  │  ┌──────────────────────────────────────┐  │  │
  │  │  │  lexicon: HashMap<String,            │  │  │
  │  │  │                   SpectralPhasor>    │  │  │
  │  │  │                                      │  │  │
  │  │  │  "apple" → {1.2, 1.5, 3}            │  │  │
  │  │  │  "cat"   → {2.5, 1.8, 5}            │  │  │
  │  │  │  ...                                 │  │  │
  │  │  └──────────────────────────────────────┘  │  │
  │  └────────────────────────────────────────────┘  │
  └──────────────────────────────────────────────────┘
```

## Memory File

```
  File: data/memory.chroma
  Format: bincode

  ┌────────────────────────────────────────────┐
  │  Memo                                       │
  │  ├── entries: Vec<ContextWaveEntry>        │
  │  │   (flat append-only log)                │
  │  │                                         │
  │  │   [0] { ts, wave, hash, layer=3 }       │
  │  │   [1] { ts, wave, hash, layer=7 }       │
  │  │   [2] { ts, wave, hash, layer=0 }       │
  │  │   ...                                   │
  │  │                                         │
  │  └── layers: [Vec<ContextWaveEntry>; 16]  │
  │      (per-layer buckets)                   │
  │                                            │
  │      [0] → [entry, entry, ...]            │
  │      [1] → [entry, ...]                   │
  │      ...                                  │
  │      [15] → [entry, ...]                  │
  └────────────────────────────────────────────┘
```

## Save/Load Flow

```
  SAVE (on exit or "save" command):
    Facet ──► SerializedFacet::from_facet()
         ──► bincode::serialize_into()
         ──► File::create("data/manifold.chroma")

    Memo  ──► bincode::serialize_into()
         ──► File::create("data/memory.chroma")

  LOAD (on startup or "load" command):
    File::open("data/manifold.chroma")
         ──► bincode::deserialize_from()
         ──► SerializedFacet::into_facet()
         ──► Facet
```
