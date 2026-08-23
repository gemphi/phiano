# 23 — Amplitude & Familiarity

## Amplitude Growth

```
  Each training exposure:
    A = min(A + 0.001, 2.0)

  ┌──────────────────────────────────────────────┐
  │  Amplitude vs. Training Exposures             │
  │                                              │
  │  2.0 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─    │  ← cap
  │                     ╱                        │
  │                   ╱                          │
  │                 ╱                            │
  │  1.5 ─ ─ ─ ─ ╱ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─    │
  │             ╱                                │
  │           ╱                                  │
  │         ╱                                    │
  │  1.0 ─ ╱ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─    │  ← initial
  │       │                                      │
  │       0    500   1000  1500  2000  exposures │
  └──────────────────────────────────────────────┘

  1.0 → 2.0 takes 1000 exposures (0.001 per exposure)
  After cap, amplitude stays at 2.0 (diminishing returns)
```

## Effect on Wave Superposition

```
  High-amplitude words dominate the centroid:

  Z_sentence = Σ Aᵢ · e^(i·(φᵢ + nᵢ·α))

  ┌─────────────────────────────────────────────┐
  │  "the" (A=2.0) + "rare" (A=1.0)            │
  │                                             │
  │  Z = 2.0·e^(i·θ_the) + 1.0·e^(i·θ_rare)   │
  │                                             │
  │  "the" contributes 2× more to the centroid  │
  │  → training pulls rare words toward "the"   │
  │  → common words define the semantic frame   │
  └─────────────────────────────────────────────┘

  This is desirable: common words (the, is, a)
  create a stable frame of reference, while rare
  words get positioned relative to them.
```

## Amplitude in Search

```
  Ray cast: Δ = α · |Z_target - Z_word|²

  High-amplitude words have larger |Z|,
  so they tend to have larger Δ from most targets.

  This means:
    - Rare words (low A) cluster more tightly
    - Common words (high A) are more spread out
    - Synonyms of common words have larger deltas
    - Synonyms of rare words have smaller deltas
```
