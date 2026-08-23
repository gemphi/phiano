# 12 — Wave Superposition

## Sentence Wave Construction

```
  Input: "the cat sat"
  Tokens: ["the", "cat", "sat"]

  ┌─────────────────────────────────────────────┐
  │  Step 1: Look up each word's phasor         │
  │                                             │
  │  "the" → (A=1.0, φ=1.2, n=2)               │
  │  "cat" → (A=1.8, φ=2.5, n=5)               │
  │  "sat" → (A=1.3, φ=2.7, n=3)               │
  └──────────────────┬──────────────────────────┘
                     │
                     ▼
  ┌─────────────────────────────────────────────┐
  │  Step 2: Convert each to complex            │
  │                                             │
  │  Z_the = 1.0·e^(i·(1.2+2α)) = (0.36, 0.93) │
  │  Z_cat = 1.8·e^(i·(2.5+5α)) = (-1.4, 1.1)  │
  │  Z_sat = 1.3·e^(i·(2.7+3α)) = (-0.9, 0.9)  │
  └──────────────────┬──────────────────────────┘
                     │
                     ▼
  ┌─────────────────────────────────────────────┐
  │  Step 3: Sum all complex values             │
  │                                             │
  │  Z = Z_the + Z_cat + Z_sat                  │
  │    = (0.36-1.4-0.9, 0.93+1.1+0.9)          │
  │    = (-1.94, 2.93)                          │
  │                                             │
  │  |Z| = 3.51  (amplitude)                    │
  │  arg(Z) = 2.15 rad  (phase)                 │
  └─────────────────────────────────────────────┘
```

## Text Wave (with tokenization)

```
  Wave::text(facet, "The Cat Sat!")
    │
    ├─► Tokenizer::tokenize() → ["the", "cat", "sat"]
    │
    └─► Wave::sentence(facet, tokens)
          │
          └─► filter_map: skip unknown words
                │
                └─► sum complex values
```

## Coherence from Wave

```
  coherence = |Z_sentence| / N_known

  High coherence: words reinforce each other
    |Z| ≈ N → coherence ≈ 1.0

  Low coherence: words cancel each other
    |Z| << N → coherence << 1.0

  Example:
    "the cat sat" → |Z|=3.51, N=3 → coherence=1.17 → clamped to 1.0
    "cat quantum banana" → |Z|=0.8, N=3 → coherence=0.27
```
