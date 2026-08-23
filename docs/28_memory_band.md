# 28 — MemoryBand Enum

## Four Bands

```
  ┌─────────────────────────────────────────────────────┐
  │  MemoryBand (4 variants)                            │
  │                                                     │
  │  Surface  (layers 0-3)   base_layer() = 0          │
  │  Pattern  (layers 4-7)   base_layer() = 4          │
  │  Semantic (layers 8-11)  base_layer() = 8          │
  │  Deep     (layers 12-15) base_layer() = 12         │
  └─────────────────────────────────────────────────────┘
```

## Band → Layer Mapping

```
  from_layer(layer) matches layer / 4:

  layer 0-3  → Surface   (0/4=0)
  layer 4-7  → Pattern   (4/4=1)
  layer 8-11 → Semantic  (8/4=2)
  layer 12-15→ Deep      (12/4=3)

  ┌─────┬─────┬─────┬─────┐
  │ L0  │ L1  │ L2  │ L3  │ ← Surface
  ├─────┼─────┼─────┼─────┤
  │ L4  │ L5  │ L6  │ L7  │ ← Pattern
  ├─────┼─────┼─────┼─────┤
  │ L8  │ L9  │ L10 │ L11 │ ← Semantic
  ├─────┼─────┼─────┼─────┤
  │ L12 │ L13 │ L14 │ L15 │ ← Deep
  └─────┴─────┴─────┴─────┘
```

## Display in Stats

```
  stats command output:

  Memory layers:
    L 0 (surface): 42 entries
    L 3 (surface): 15 entries
    L 5 (pattern): 28 entries
    L 9 (semantic): 12 entries
    L14 (deep): 3 entries

  Uses MemoryBand::from_layer(layer) → Display trait
  → "surface", "pattern", "semantic", "deep"
```

## Band Count Aggregation

```
  band_count(MemoryBand::Surface) =
    layer_count(0) + layer_count(1) +
    layer_count(2) + layer_count(3)

  Useful for high-level memory statistics
  without iterating individual layers.
```
