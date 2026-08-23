# 03 — The Phase Manifold

## 2π Phase Circle

```
              Im (imaginary)
               │
          π/2  │    ● "apple"
               │         ● "fruit"
               │
          ─────┼────────────── Re (real)
               │         0°
               │
               │    ● "computer"
         -π/2  │
               │

  Each word = a point on the unit circle × amplitude
  Position = (A·cos(φ), A·sin(φ))
```

## Multi-Band Structure

```
  Band n=1 (fundamental):
    φ ∈ [0, 2π)     ← primary semantic axis

  Band n=2 (first overtone):
    φ + α            ← fine-structure shift
    Creates a second "octave" of meaning

  Band n=3 (second overtone):
    φ + 2α           ← another layer

  ...

  Effective phase = φ + n·α
  where α = 1/137 (fine-structure constant)

  ┌──────────────────────────────────────┐
  │  n=1  ●─────●─────●─────●           │  fundamental
  │  n=2   ●─────●─────●─────●          │  shifted by α
  │  n=3    ●─────●─────●─────●         │  shifted by 2α
  │  n=4     ●─────●─────●─────●        │  shifted by 3α
  └──────────────────────────────────────┘
           ↑     ↑     ↑     ↑
         words that co-occur climb to higher n
         (band_n += 1 when phase_error < 0.0005)
```

## Why a Circle?

```
  Linear space:    A ──── B ──── C
                   │      │      │
                   │  AB  │  BC  │   distances are additive
                   │      │      │
                   AB + BC = AC   (transitive)

  Phase space:     A ●         ● B
                    \         /
                     \  AB   /
                      \     /
                       \   / BC
                        \ /
                         ● C

  AB + BC ≠ AC   (not transitive!)
  Words can be close to B but far from each other.
  This captures ANTONYMY: "hot" ↔ "cold" both near "temperature"
  but far from each other on the circle.
```
