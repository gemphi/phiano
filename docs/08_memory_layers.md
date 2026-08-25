# 08 - Memory: 16-Layer System

## Layer Classification

```
  Input text → classify_layer()

  ┌─────────────────────────────────────────────────────┐
  │  Signal 1: Word Count → Band                        │
  │                                                     │
  │  1-3 words    → Surface  (layers 0-3)              │
  │  4-8 words    → Pattern  (layers 4-7)              │
  │  9-16 words   → Semantic (layers 8-11)             │
  │  17+ words    → Deep     (layers 12-15)            │
  └─────────────────────────────────────────────────────┘

  ┌─────────────────────────────────────────────────────┐
  │  Signal 2: Avg Word Length → Sub-layer              │
  │                                                     │
  │  1-4 chars    → sub-layer 0                        │
  │  5-6 chars    → sub-layer 1                        │
  │  7-8 chars    → sub-layer 2                        │
  │  9+ chars     → sub-layer 3                        │
  └─────────────────────────────────────────────────────┘

  Final layer = band.base_layer() + sub_layer  (clamped to 15)
```

## 4×4 Grid

```
  ┌─────────┬─────────┬─────────┬─────────┐
  │  L0     │  L1     │  L2     │  L3     │  SURFACE
  │ 1-3w    │ 1-3w    │ 1-3w    │ 1-3w    │  (word recognition)
  │ short   │ medium  │ long    │ v.long  │
  │ words   │ words   │ words   │ words   │
  ├─────────┼─────────┼─────────┼─────────┤
  │  L4     │  L5     │  L6     │  L7     │  PATTERN
  │ 4-8w    │ 4-8w    │ 4-8w    │ 4-8w    │  (co-occurrence)
  │ short   │ medium  │ long    │ v.long  │
  ├─────────┼─────────┼─────────┼─────────┤
  │  L8     │  L9     │  L10    │  L11    │  SEMANTIC
  │ 9-16w   │ 9-16w   │ 9-16w   │ 9-16w   │  (definition context)
  │ short   │ medium  │ long    │ v.long  │
  ├─────────┼─────────┼─────────┼─────────┤
  │  L12    │  L13    │  L14    │  L15    │  DEEP
  │ 17+w    │ 17+w    │ 17+w    │ 17+w    │  (abstract concepts)
  │ short   │ medium  │ long    │ v.long  │
  └─────────┴─────────┴─────────┴─────────┘
```

## Entry Structure

```
  ContextWaveEntry {
      timestamp_ms: u64        ← when recorded
      superposition_wave: (f64, f64)  ← (re, im) of sentence wave
      text_hash: u64           ← FNV-1a hash of input text
      layer: usize             ← 0-15
  }

  Storage:
    entries: Vec<ContextWaveEntry>       ← flat log (append-only)
    layers: [Vec<ContextWaveEntry>; 16]  ← per-layer buckets
```
