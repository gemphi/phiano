# 13 — Energy Delta Calculation

## Formula

```
  Δ = α · |Z₁ - Z₂|²

  Where:
    α = 1/137  (fine-structure constant)
    Z = complex wave representation
    |·|² = squared magnitude = re² + im²

  Expanded:
    Δ = α · ((re₁-re₂)² + (im₁-im₂)²)
```

## Geometric Interpretation

```
  Im
   │
   │  ● Z₁
   │  │
   │  │  |Z₁-Z₂| = distance between points
   │  │
   │  │      ● Z₂
   │  │     ╱
   │  │    ╱
   │  │   ╱  Δ = α × distance²
   │  │  ╱
   │  │ ╱
   │  │╱
   ┼────────────────── Re

  Small distance² → small Δ → semantically similar
  Large distance² → large Δ → semantically different
```

## Pairwise Examples

```
  Word pair          |Z₁-Z₂|²    Δ = α·|Z₁-Z₂|²   Interpretation
  ─────────────────  ─────────    ───────────────   ─────────────
  cat / dog          0.02         0.000146          very similar
  cat / pet          0.01         0.000073          extremely similar
  cat / tree         1.50         0.010949          dissimilar
  cat / quantum      3.89         0.028381          very dissimilar
  cat / cat          0.00         0.000000          identical
```

## In Ray Casting

```
  For each word in lexicon:
    Δ_word = α · |Z_target - Z_word|²

  Sort all Δ_word ascending → nearest neighbors first

  The α scaling keeps Δ values small and comparable
  across different vocabulary sizes.
```
