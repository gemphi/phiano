# 22 - Trainer Algorithm Deep Dive

## train_sentence() Step by Step

```
  Input: facet, "the cat sat on the mat"

  ┌─────────────────────────────────────────────────────┐
  │  STEP 1: Tokenize                                   │
  │  tokens = ["the", "cat", "sat", "on", "the", "mat"]│
  └──────────────────────┬──────────────────────────────┘
                         │
                         ▼
  ┌─────────────────────────────────────────────────────┐
  │  STEP 2: Initialize unseen tokens                   │
  │  for each token not in lexicon:                     │
  │    φ = len(token) × golden_ratio % 2π              │
  │    A = 1.0, n = 1                                   │
  │                                                     │
  │  "the" → already exists (skip)                      │
  │  "cat" → already exists (skip)                      │
  │  "sat" → already exists (skip)                      │
  │  "on"  → already exists (skip)                      │
  │  "mat" → already exists (skip)                      │
  └──────────────────────┬──────────────────────────────┘
                         │
                         ▼
  ┌─────────────────────────────────────────────────────┐
  │  STEP 3: Compute centroid phase                     │
  │                                                     │
  │  sum_x = Σ φᵢ·cos(φᵢ)·Aᵢ  (weighted by amplitude) │
  │  sum_y = Σ φᵢ·sin(φᵢ)·Aᵢ                          │
  │  φ_centroid = atan2(sum_y, sum_x)                  │
  │                                                     │
  │  All 6 tokens contribute (deduplicated by HashMap)  │
  │  → 5 unique words: the, cat, sat, on, mat           │
  └──────────────────────┬──────────────────────────────┘
                         │
                         ▼
  ┌─────────────────────────────────────────────────────┐
  │  STEP 4: Phase relaxation                           │
  │  for each token:                                    │
  │    phase_error = sin(φ_centroid - φᵢ)              │
  │    φᵢ += lr × phase_error                          │
  │    φᵢ = φᵢ.rem_euclid(2π)                          │
  │                                                     │
  │    if |phase_error| < 0.0005:                       │
  │      nᵢ += 1  (promote to higher band)             │
  │                                                     │
  │    Aᵢ = min(Aᵢ + 0.001, 2.0)  (familiarity)       │
  └──────────────────────┬──────────────────────────────┘
                         │
                         ▼
  ┌─────────────────────────────────────────────────────┐
  │  STEP 5: Return count of updated tokens             │
  │  return 6                                           │
  └─────────────────────────────────────────────────────┘
```

## Why sin() for Phase Error?

```
  sin(Δφ) properties:
    Δφ = 0    → sin = 0     (no change needed, already aligned)
    Δφ = π/2  → sin = 1     (maximum pull toward centroid)
    Δφ = π    → sin = 0     (opposite - no pull! ambiguous)
    Δφ = -π/2 → sin = -1    (maximum pull in opposite direction)

  This creates a smooth attraction that:
    ✓ Is zero when already aligned
    ✓ Is strongest at 90° offset
    ✓ Is zero at 180° (antipodal - needs multiple passes)
    ✓ Always pulls in the shortest direction
```
