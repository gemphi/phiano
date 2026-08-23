# 06 — Kuramoto Phase Attraction

## The Kuramoto Model

```
  Standard Kuramoto:
    dφᵢ/dt = ωᵢ + (K/N) · Σⱼ sin(φⱼ - φᵢ)

  Phiano's simplified version:
    φᵢ ← φᵢ + lr · sin(φ_centroid - φᵢ)

  Where:
    φᵢ         = phase of word i
    φ_centroid = amplitude-weighted centroid of sentence
    lr         = learning rate (0.05)
```

## Training Step Visualization

```
  Before training:                After training:

    Im                              Im
     │                               │
     │  ● "cat" (φ=2.5)             │      ● "cat" (φ=2.48)
     │                               │       ╱
     │         ● "sat" (φ=2.7)      │      ╱
     │                               │     ● "sat" (φ=2.52)
     │                               │     ╱
     │  ● "the" (φ=1.2)             │    ╱
     │                               │ ● "the" (φ=1.35)
     ┼──────────── Re                ┼──────────── Re

  Centroid φ_c ≈ 2.3

  Each word's phase is pulled toward φ_c:
    "cat":  2.5 → 2.5 + 0.05·sin(2.3-2.5) = 2.5 - 0.01 = 2.49
    "sat":  2.7 → 2.7 + 0.05·sin(2.3-2.7) = 2.7 - 0.02 = 2.68
    "the":  1.2 → 1.2 + 0.05·sin(2.3-1.2) = 1.2 + 0.04 = 1.24
```

## Convergence Over Epochs

```
  Epoch 1:    ●  ●    ●         (scattered)
              ●       ●

  Epoch 10:   ● ●●  ●           (clustering)
                ●

  Epoch 50:     ●●●●            (converged)
                ●

  Words that co-occur converge to similar phases.
  Words that never co-occur stay at their initial positions.
  This is self-organizing — no labels, no gradients.
```

## Band_n Promotion

```
  When |sin(φ_c - φᵢ)| < 0.0005 (word already aligned):

    φ barely changes, but n += 1

  This shifts the word to a higher octave:
    effective_phase = φ + (n+1)·α

  Purpose: prevents phase collapse.
  Without this, all words would converge to exactly
  the same phase and become indistinguishable.
  Higher n = more refined semantic distinction.
```
