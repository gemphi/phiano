# 42 — Oscillator Mode: The Sphere Model

## Overview

The oscillator model (`om`) is the alternative to the transform model.
Where the transform model maps words to static complex numbers on a 2D
circle, the oscillator model maps words to **living oscillators on a 3D
sphere**. The sphere's surface is a color spectrum — hue from longitude,
brightness from latitude. Your viewing angle determines which colors you
see. The spectrum **changes** as you rotate around the sphere.

```text
    TRANSFORM MODEL                    OSCILLATOR MODEL

    2D Phase Circle                    3D Phase Sphere

         Im                                 N pole (θ=+π/2)
          │                                  ╱─────╲
     Z₂  ╱                                 ╱         ╲
        ╱                                ╱  EQUATOR    ╲  ← θ=0
       ╱  Z = Z₁+Z₂+Z₃                  ╲             ╱
      ╱                                    ╲         ╱
     ╱                                       ╲─────╱
    Z₁                                          S pole (θ=-π/2)
    ──┼──────── Re
                                                   ↑
    Static. Points don't move.              Dynamic. Oscillators spin.
    Similarity = |Z₁-Z₂|²                   Similarity = synchronization
```

## The Sphere

Each word becomes an oscillator with four parameters:

```
  ┌──────────────────────────────────────────────────────────┐
  │  Oscillator                                              │
  │                                                          │
  │  longitude (φ)  ──→  hue (color)         [0, 2π)        │
  │  latitude  (θ)  ──→  brightness          [-π/2, +π/2]   │
  │  frequency (ω)  ──→  rotation speed       rad/time       │
  │  amplitude (A)  ──→  intensity            [0, ∞)        │
  └──────────────────────────────────────────────────────────┘
```

### Mapping from Phasor to Oscillator

The existing `SpectralPhasor` maps onto the sphere:

```
  phasor.phase    ──→  oscillator.longitude
  phasor.amplitude ──→ oscillator.amplitude
  phasor.band_n   ──→  oscillator.frequency = band_n × α × 1000
  derived         ──→  oscillator.latitude = (amplitude - 1) × π/4
```

The latitude is derived from amplitude:
- amplitude = 1.0 → equator (latitude = 0)
- amplitude = 0.0 → south pole (latitude = -π/2)
- amplitude = 2.0 → north pole (latitude = +π/2)

This means highly familiar words (high amplitude) cluster near the north
pole, rare words near the south pole, and the bulk of the vocabulary
spreads across the equator.

### The Color Spectrum

The sphere's surface is painted with 16 colors mapped from longitude:

```text
         0°                    90°                   180°                   270°
         │                      │                      │                      │
         ▼                      ▼                      ▼                      ▼
    ┌─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
    │ crimson │ orange  │ yellow  │  green  │  blue   │ indigo  │ magenta │  rose   │
    │   red   │  amber  │  lime   │ emerald │  teal   │ violet  │         │         │
    │ scarlet │  gold   │         │         │         │         │         │         │
    └─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘
         │                      │                      │                      │
         │      WARM SECTOR      │      COOL SECTOR     │     WARM SECTOR      │
         │   (passionate, loud)  │  (contemplative,     │  (passionate, loud)  │
         │                      │   quiet, deep)        │                      │
```

As oscillators spin (φ_visible(t) = φ + ω·t), their colors shift. A word
that appears crimson at t=0 may appear orange at t=1, depending on its
frequency. The sphere is **alive** — it breathes color.

## Viewing Angle

The key insight: **what you see depends on where you stand**.

Your viewing angle is a point (θ_v, φ_v) on the sphere. You see
oscillators that are "facing you" — weighted by the spherical dot product:

```
  visibility = cos(θ)·cos(θ_v)·cos(Δφ) + sin(θ)·sin(θ_v)

  Where Δφ = φ_visible(t) - φ_v
```

```text
         YOU (viewing angle)
          ↘
     ┌─────╲─────┐
    ╱    ╱   ╲    ╲
   │   │  ●  │   │     ← oscillator facing you: high visibility
   │   │     │   │
    ╲    ╲   ╱    ╱
     └─────╱─────┘
           ↑
     ●  ← oscillator behind sphere: invisible (visibility < 0)
```

- **Visibility = 1.0**: oscillator directly faces you
- **Visibility = 0.0**: oscillator on the horizon (edge)
- **Visibility < 0**: oscillator on the far side (invisible)

This means the same text produces **different color spectra** depending
on your viewing angle. Rotate 90° and crimson becomes blue. The sphere
is a chameleon.

## Synchronization

In the transform model, similarity = energy delta = α·|Z₁ - Z₂|².
This is a **static** measurement.

In the oscillator model, similarity = **synchronization**. Two
oscillators are similar if they spin in harmony:

```
  sync = r × freq_factor × lat_factor

  Where:
    r            = |e^(iφ₁) + e^(iφ₂)| / 2    (phase alignment)
    freq_factor  = exp(-|ω₁ - ω₂| / Ω)         (frequency proximity)
    lat_factor   = 1 - |θ₁ - θ₂| / π            (latitude proximity)
```

- **Phase alignment (r)**: Are they pointing the same direction right now?
- **Frequency proximity**: Will they stay aligned over time?
- **Latitude proximity**: Are they on the same band of the sphere?

Two words are "similar" in oscillator mode if they:
1. Are currently in phase (same color)
2. Spin at similar speeds (stay in phase)
3. Live at similar latitudes (same brightness band)

## Sentence Coherence

A sentence's coherence in oscillator mode is the **Kuramoto order
parameter** — the degree to which all word-oscillators are in phase:

```
  r = |Σⱼ e^(iφⱼ)| / N

  r ≈ 1.0: all oscillators aligned (coherent sentence)
  r ≈ 0.0: oscillators scattered (incoherent sentence)
```

This is different from the transform model's coherence, which measures
wave norm per known word. The oscillator coherence measures **collective
synchronization** — are the words dancing together?

## Spectral Entropy

The oscillator model also measures the **diversity of the color spectrum**:

```
  H = -Σ p(c) × ln(p(c))

  Where p(c) = total amplitude of color c / total amplitude
```

- **High entropy** (~2.8): colors spread evenly across the spectrum
- **Low entropy** (~0.5): one or two colors dominate

This is unique to the oscillator model — the transform model has no
notion of "color diversity."

## Commands

```
  om eval "text"        — Evaluate text: coherence, sync, entropy, colors
  om wheel              — Show the equatorial color wheel (16 sectors)
  om sphere "text"      — Show full sphere projection (5 latitude bands)
  om compare "text"     — Compare transform vs oscillator models side by side
```

## Comparison: Transform vs Oscillator

```text
  ┌─────────────────────┬──────────────────────┬──────────────────────┐
  │                     │ Transform Model      │ Oscillator Model     │
  ├─────────────────────┼──────────────────────┼──────────────────────┤
  │ Geometry            │ 2D circle            │ 3D sphere            │
  │ Word representation │ Static phasor        │ Spinning oscillator  │
  │ Sentence            │ Wave superposition   │ Coupled oscillator   │
  │ Coherence           │ Wave norm / N        │ Kuramoto order param │
  │ Similarity          │ Energy delta         │ Synchronization      │
  │ Novelty             │ Distance from center │ (not measured)       │
  │ Resonance           │ Known word fraction  │ (not measured)       │
  │ Color diversity     │ (not measured)       │ Spectral entropy     │
  │ Time dependence     │ Static               │ Dynamic (rotates)    │
  │ Viewing angle       │ Fixed (top-down)     │ Variable (any angle) │
  │ Latitude            │ (not modeled)        │ Amplitude → latitude │
  │ Frequency           │ (not modeled)        │ Band_n → frequency   │
  └─────────────────────┴──────────────────────┴──────────────────────┘
```

### When They Agree

Both models measure coherence in [0, 1]. When they agree (>80%), you
have high confidence in the assessment — the text is clearly coherent
or clearly not, regardless of model.

### When They Disagree

When the models disagree (<50%), the text sits at a **model boundary** —
it's coherent in one framework but not the other. This is interesting:

- **Transform high, oscillator low**: Words are in the right phase
  sector but don't synchronize dynamically. The text "looks right" but
  doesn't "feel right" — like a sentence with correct grammar but
  no rhythm.

- **Oscillator high, transform low**: Words synchronize beautifully
  but their wave superposition is weak. The text "feels right" but
  doesn't "look right" — like a poetic phrase with unusual grammar.

## The Wheel

The equatorial color wheel is the most direct visualization. At any
time t, you see 16 sectors of color, each populated with the words
visible from that longitude:

```text
  ── oscillator sphere: equatorial wheel ──

     crimson │  invulnerability [1.10], malconformation [1.10]
         red │  retinite [1.15], anconal [1.10]
     scarlet │  unregeneracy [1.15], conglutinant [1.10]
      orange │  polysyllabism [1.10], recurvation [1.09]
       amber │  metalorganic [1.12], syndactilous [1.10]
        gold │  acclimatization [1.15], climatic [1.15]
      yellow │  psychology [1.36], pathogenic [1.36]    ← dominant
        lime │  idiocy [1.46], intolerable [1.46]       ← dominant
       green │  mars [1.41], danger [1.41]
     emerald │  mars [1.32], danger [1.29]
        teal │  ovaritis [1.19], eudemonistic [1.19]
        blue │  ovaritis [1.11], eudemonistic [1.11]
      indigo │  pedobaptism [0.89], netherfield [0.88]
      violet │  netherfield [1.00], circumesophagal [0.57]
     magenta │  netherfield [0.96], anthropophagous [0.86]
        rose │  circumesophagal [1.08], anthropophagous [1.06]
```

The dominant colors (yellow, lime) tell you where the facet's vocabulary
concentrates. The amplitudes tell you how familiar those words are.

## Implementation

```
src/
├── oscillator.rs          — Oscillator, OscillatorField, SphereView
└── command/
    └── om.rs              — om eval, om wheel, om sphere, om compare
```

The oscillator model is **non-destructive** — it reads from the existing
facet but doesn't modify it. You can switch between `eval` (transform)
and `om eval` (oscillator) freely. Both models coexist.
