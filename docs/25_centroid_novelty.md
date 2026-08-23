# 25 — Centroid & Novelty

## Facet Centroid

```
  Z_centroid = Σᵢ Aᵢ · e^(i·(φᵢ + nᵢ·α))

  The "center of mass" of the entire semantic space.

  ┌──────────────────────────────────────────────┐
  │  Im                                           │
  │   │                                           │
  │   │    ● ●                                    │
  │   │   ● ●●     ← words clustered              │
  │   │  ● ●●●                                    │
  │   │   ● ★ ← centroid                          │
  │   │    ●  ●                                   │
  │   │     ● ●                                   │
  │   ┼────────────────────── Re                  │
  │   │                                           │
  │   │  ● (outlier word)                         │
  └──────────────────────────────────────────────┘

  Computed by summing ALL phasors' complex representations.
  O(N) where N = vocabulary size.
```

## Novelty Score

```
  novelty = 1 - exp(-5 · |Z_sentence - Z_centroid|)

  ┌──────────────────────────────────────────────┐
  │  Distance from centroid → Novelty             │
  │                                              │
  │  0.0 ──────────────────── (same as centroid) │
  │       │                                      │
  │  0.2 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ (close, familiar)   │
  │       │                                      │
  │  0.5 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ (moderate)          │
  │       │                                      │
  │  0.8 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ (far, novel)        │
  │       │                                      │
  │  1.0 ──────────────────── (very far, unique) │
  └──────────────────────────────────────────────┘

  The exp(-5·d) function creates a smooth sigmoid:
    d=0.0 → novelty=0.0   (identical to centroid)
    d=0.2 → novelty=0.63  (somewhat novel)
    d=0.5 → novelty=0.92  (very novel)
    d=1.0 → novelty=0.99  (almost completely novel)
```

## Example

```
  Sentence: "the cat sat on the mat"
  (very common words, well-trained)

  Z_sentence ≈ Z_centroid
  |Z_sentence - Z_centroid| ≈ 0.1
  novelty = 1 - exp(-0.5) = 0.39  → familiar

  Sentence: "quantum entanglement decoheres"
  (rare words, far from centroid)

  Z_sentence far from Z_centroid
  |Z_sentence - Z_centroid| ≈ 2.5
  novelty = 1 - exp(-12.5) ≈ 1.0  → very novel
```
