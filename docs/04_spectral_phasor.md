# 04 — SpectralPhasor Structure

## 16-Byte Fixed-Width Representation

```
  ┌────────────────────────────────────────────────────┐
  │  SpectralPhasor  (16 bytes)                       │
  │                                                    │
  │  ┌──────────────┐  ┌──────────────┐  ┌──────────┐ │
  │  │  phase: f64  │  │ amplitude:f64│  │band_n:u32│ │
  │  │  8 bytes     │  │  8 bytes     │  │  4 bytes │ │
  │  │  [0, 2π)     │  │  [0, 2.0]    │  │  ≥ 1     │ │
  │  └──────────────┘  └──────────────┘  └──────────┘ │
  │   ↑                 ↑                 ↑            │
  │   Where on          How loud          Which octave  │
  │   the circle        (familiarity)     (sub-band)    │
  └────────────────────────────────────────────────────┘
```

## Complex Wave Conversion

```
  Z = A · e^(i·(φ + n·α))

  Decomposed:
    Z.re = A · cos(φ + n·α)    ← real part
    Z.im = A · sin(φ + n·α)    ← imaginary part

  Example (word "cat"):
    φ = 2.5 rad, A = 1.5, n = 3, α = 1/137

    effective_phase = 2.5 + 3/137 = 2.5219
    Z = 1.5 · (cos(2.5219) + i·sin(2.5219))
    Z = 1.5 · (-0.8137 + i·0.5813)
    Z = (-1.2206, 0.8719)
```

## Lifecycle

```
  ┌─────────────┐
  │  New word   │
  │  seen first │
  └──────┬──────┘
         │
         ▼
  ┌──────────────────────┐
  │ Initialize:          │
  │  φ = len(word)       │
  │    × 1.618 % 2π      │
  │  A = 1.0             │
  │  n = 1               │
  └──────────┬───────────┘
             │
             ▼
  ┌──────────────────────┐
  │ Each training pass:  │
  │  φ ← φ + lr·sin(Δφ) │
  │  A ← min(A+0.001, 2)│
  │  if |sin(Δφ)|<0.0005│
  │    n ← n + 1         │
  └──────────────────────┘
```
