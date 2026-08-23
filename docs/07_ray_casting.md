# 07 — Ray Casting Search

## Parallel Ray Projection

```
  Target word: "cat" at Z_cat

  ┌───────────────────────────────────────────────┐
  │  Facet Lexicon (parallel via rayon)           │
  │                                               │
  │  ● "dog"     ──ray──►  Δ₁ = α·|Z_cat - Z_dog│²   │
  │  ● "mouse"   ──ray──►  Δ₂ = α·|Z_cat - Z_mouse|² │
  │  ● "tree"    ──ray──►  Δ₃ = α·|Z_cat - Z_tree|²  │
  │  ● "run"     ──ray──►  Δ₄ = α·|Z_cat - Z_run|²   │
  │  ● "food"    ──ray──►  Δ₅ = α·|Z_cat - Z_food|²  │
  │  ● "pet"     ──ray──►  Δ₆ = α·|Z_cat - Z_pet|²   │
  │  ...                                          │
  └───────────────────────────────────────────────┘
                    │
                    ▼
  ┌───────────────────────────────────────────────┐
  │  Sort by Δ (ascending)                        │
  │                                               │
  │  Rank 1: "pet"     Δ = 0.00012               │
  │  Rank 2: "dog"     Δ = 0.00031               │
  │  Rank 3: "mouse"   Δ = 0.00044               │
  │  Rank 4: "animal"  Δ = 0.00082               │
  │  Rank 5: "food"    Δ = 0.00120               │
  └───────────────────────────────────────────────┘
```

## Two Ray Cast Modes

```
  Mode 1: ray_cast_word (synonym search)
    Source: a single word's phasor
    Target: all other words in lexicon
    Usage: synonym <word> [count]

    Z_target ─────► ● ● ● ● ● ●  (all words)
                    sort by Δ, take top_k

  Mode 2: ray_cast (resonance search)
    Source: a sentence wave (superposition)
    Target: all words in lexicon
    Usage: resonance "text" [count]

    Z_sentence ───► ● ● ● ● ● ●  (all words)
                    sort by Δ, take top_k
```

## Performance

```
  N = vocabulary size (e.g., 30,000 words)

  Per ray cast:
    N complex subtractions  (O(N))
    N norm_sqr computations (O(N))
    N multiplications by α  (O(N))
    Sort N elements         (O(N log N))
    Take top_k              (O(k))

  Parallelized with rayon:
    N/cores operations per thread
    For N=30k, 8 cores → ~3,750 per thread
```
