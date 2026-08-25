# 24 - Band_n Sub-Levels

## Purpose: Preventing Phase Collapse

```
  Without band_n promotion:

  Epoch 1:    ● ●  ●     ●        (words at various phases)
              ●     ●  ●

  Epoch 50:             ●        (ALL words converged to
              ●         ●         same phase - useless!)
              ●         ●

  With band_n promotion:

  When |sin(Δφ)| < 0.0005 (word already aligned):
    φ barely changes
    n += 1  ← shifts effective phase by α

  Epoch 50:   ● ●  ●     ●        (words aligned but
              ●     ●  ●          at different n levels,
              ●     ●  ●          so effective phases differ)
```

## Effective Phase Space

```
  2D space: (φ, n)

  φ ∈ [0, 2π)    - continuous
  n ∈ {1, 2, 3, ...} - discrete

  effective_phase = φ + n·α

  ┌──────────────────────────────────────────────┐
  │  n=1  │  n=2  │  n=3  │  n=4  │  n=5  │ ...│
  │       │       │       │       │       │     │
  │  ●●●  │   ●●  │  ●●●  │   ●   │  ●●   │     │
  │  ●●   │  ●●●  │   ●   │  ●●   │   ●   │     │
  │       │       │       │       │       │     │
  └──────────────────────────────────────────────┘
       ↑       ↑       ↑       ↑       ↑
     shift=0  shift=α  shift=2α shift=3α shift=4α

  Words at the same φ but different n are distinguishable.
  This creates a 2D semantic space from a 1D circle.
```

## Band Distribution After Training

```
  Typical distribution after 50 epochs on Webster's:

  n=1:  ████████████████████  40%  (rare words, seen few times)
  n=2:  ███████████████       25%  (uncommon words)
  n=3:  ██████████            15%  (moderate frequency)
  n=4:  ██████                 8%  (common words)
  n=5:  ████                   5%  (very common)
  n=6+: ███                    7%  (the, a, is, of, etc.)

  Higher n = more training exposure = more refined position.
```
