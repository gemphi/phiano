# 29 - FNV-1a Hash

## Algorithm

```
  FNV-1a (Fowler-Noll-Vo) 64-bit hash

  Constants:
    offset_basis = 14695981039346656037  (2^64 - 1 + 2^40 + ...)
    prime        = 1099511628211

  Algorithm:
    hash = offset_basis
    for each byte in input:
      hash = hash XOR byte
      hash = hash × prime  (wrapping multiplication)

  ┌──────────────────────────────────────────────┐
  │  Input: "cat"                                │
  │                                              │
  │  hash = 14695981039346656037                 │
  │                                              │
  │  byte 'c' (99):                              │
  │    hash = hash ^ 99                           │
  │    hash = hash × 1099511628211 (wrapping)    │
  │                                              │
  │  byte 'a' (97):                              │
  │    hash = hash ^ 97                           │
  │    hash = hash × 1099511628211 (wrapping)    │
  │                                              │
  │  byte 't' (116):                             │
  │    hash = hash ^ 116                          │
  │    hash = hash × 1099511628211 (wrapping)    │
  │                                              │
  │  Result: unique 64-bit identifier            │
  └──────────────────────────────────────────────┘
```

## Properties

```
  ✓ Deterministic - same input always gives same hash
  ✓ Fast - single pass, no allocations
  ✓ Good distribution - minimal collisions for short strings
  ✓ Fixed-size output - u64, 8 bytes
  ✓ No dependencies - pure arithmetic

  Used for: ContextWaveEntry.text_hash
  Purpose: deduplication and quick text identity comparison
```

## In Memory Records

```
  ContextWaveEntry {
      timestamp_ms: 1700000000000,
      superposition_wave: (-1.94, 2.93),
      text_hash: 0xA3F2B1C8D4E5F6A7,  ← FNV-1a of input text
      layer: 5,
  }

  Two entries with the same text_hash are the same input.
  Enables: "have I seen this exact text before?"
  without storing the full text string.
```
