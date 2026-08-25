# 05 - Complex Wave Representation (c64)

## The c64 Type

```
  c64 = Complex64 = num_complex::Complex<f64>

  ┌─────────────────────────┐
  │  Z = re + i·im          │
  │                         │
  │  re: f64  (real)        │
  │  im: f64  (imaginary)   │
  └─────────────────────────┘

  Polar form:
    Z = A · e^(i·θ)
    A = |Z| = √(re² + im²)     (amplitude/norm)
    θ = atan2(im, re)           (phase angle)
```

## Wave Superposition

```
  Sentence: "the cat sat"

  Word waves:
    "the" → Z₁ = A₁·e^(i·θ₁) = (re₁, im₁)
    "cat" → Z₂ = A₂·e^(i·θ₂) = (re₂, im₂)
    "sat" → Z₃ = A₃·e^(i·θ₃) = (re₃, im₃)

  Superposition (sum):
    Z = Z₁ + Z₂ + Z₃
      = (re₁+re₂+re₃, im₁+im₂+im₃)

  Visualized:
    Im
     │        Z₂
     │       ╱
     │      ╱
     │     ╱    Z = Z₁+Z₂+Z₃
     │    ╱    ╱
     │ Z₁╱    ╱
     │  ╱    ╱
     │ ╱    ╱
     │╱    ╱
     ┼──────────────── Re
     │  ╱
     │ ╱  Z₃
     │╱
     │
```

## Energy Delta (Destructive Interference)

```
  Δ = α · |Z₁ - Z₂|²

  Small Δ → words are semantically similar
  Large Δ → words are semantically different

  Example:
    Z_cat = (1.2, 0.8)
    Z_dog = (1.1, 0.7)
    Δ = α · |(0.1, 0.1)|² = α · 0.02 = 0.000146  ← very similar!

    Z_cat = (1.2, 0.8)
    Z_quantum = (-0.5, 1.8)
    Δ = α · |(1.7, -1.0)|² = α · 3.89 = 0.0284   ← very different!
```
