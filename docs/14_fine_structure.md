# 14 - Fine-Structure Constant (α)

## The Constant

```
  α = 1 / 137

  ≈ 0.00729735256

  This is the physical fine-structure constant,
  the coupling constant of the electromagnetic interaction.

  In Phiano, it serves as the spectral coupling parameter:
    - Spacing between sub-bands (band_n levels)
    - Scaling factor for energy delta
```

## Role in Phase Calculation

```
  Effective phase = φ + n · α

  ┌──────────────────────────────────────────────┐
  │  n=1:  φ + 0.0073   (≈ φ)                   │
  │  n=2:  φ + 0.0146   (slight shift)          │
  │  n=3:  φ + 0.0219   (more shift)            │
  │  n=10: φ + 0.0730   (significant shift)     │
  │  n=50: φ + 0.365    (large shift)           │
  │  n=137: φ + 1.0     (full radian shift!)    │
  └──────────────────────────────────────────────┘

  Words at higher band_n are shifted further from
  their base phase, creating finer semantic distinctions.
```

## Why This Constant?

```
  1. It's dimensionless - no units to worry about
  2. It's small - sub-bands are close but distinguishable
  3. It's irrational - no exact resonances or periodicities
  4. It's fundamental - a universal constant of nature
  5. At n=137, the shift = 1 radian (a natural "octave")

  The choice is aesthetic/philosophical rather than
  physically motivated. It gives the system a natural
  progression scale where ~137 levels span a full radian.
```

## Energy Delta Scaling

```
  Δ = α · |Z₁ - Z₂|²

  Without α: Δ values would be O(1) for typical distances
  With α:    Δ values are O(0.001), giving fine granularity

  This makes the ranking more sensitive to small differences
  between similar words.
```
