# 31 - Golden Ratio Initialization

## Seed Phase Formula

```
  φ_seed = (len(word) × golden_ratio) % 2π

  golden_ratio = 1.61803398875...

  ┌──────────────────────────────────────────────────┐
  │  Word      │ len │ len×φ    │ % 2π   │ φ_seed   │
  ├────────────┼─────┼─────────┼────────┼──────────┤
  │ "a"        │  1  │ 1.618   │ 1.618  │ 1.618    │
  │ "cat"      │  3  │ 4.854   │ 4.854  │ 4.854    │
  │ "house"    │  5  │ 8.090   │ 1.807  │ 1.807    │
  │ "computer" │  8  │ 12.944  │ 0.377  │ 0.377    │
  │ "algorithm"│  9  │ 14.562  │ 1.995  │ 1.995    │
  │ "quantum"  │  7  │ 11.326  │ 4.899  │ 4.899    │
  └──────────────────────────────────────────────────┘
```

## Why the Golden Ratio?

```
  The golden ratio φ = 1.618... is the most irrational number.

  ┌──────────────────────────────────────────────────┐
  │  Properties:                                      │
  │                                                  │
  │  1. Never produces exact resonances              │
  │     (unlike rational multipliers)                │
  │                                                  │
  │  2. Maximally spreads initial phases             │
  │     Words of different lengths get               │
  │     well-separated starting positions            │
  │                                                  │
  │  3. Deterministic                                │
  │     Same word always gets same initial phase     │
  │     (reproducible training)                      │
  │                                                  │
  │  4. Length-correlated                             │
  │     Words of similar length start nearby         │
  │     (slight morphological prior)                 │
  └──────────────────────────────────────────────────┘

  Distribution of initial phases for words of length 1-15:

  φ ∈ [0, 2π)
  │
  │  len=1  ●
  │  len=2    ●
  │  len=3      ●
  │  len=4        ●
  │  len=5          ●
  │  len=6            ●
  │  len=7              ●
  │  len=8                ●
  │  len=9                  ●
  │  len=10                   ●
  │  ...
  │
  ┼────────────────────────────────  2π

  The golden ratio ensures these points never exactly repeat,
  giving each word length a unique starting position.
```

## After Training

```
  Initial positions are quickly overwritten by training.
  After 50 epochs, a word's phase is determined almost
  entirely by its co-occurrence patterns, not its length.

  The golden ratio initialization only matters for:
    - Words that appear in very few definitions
    - The first few epochs of training
    - Reproducibility across runs
```
