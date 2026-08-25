# 27 - Verdict Enum & Display

## Verdict Variants

```
  ┌─────────────────────────────────────────────────────┐
  │  Verdict (9 variants)                               │
  │                                                     │
  │  ┌─────────────────┐  ┌─────────────────┐         │
  │  │ Empty           │  │ Noise           │         │
  │  │ (no tokens)     │  │ (mostly unknown)│         │
  │  └─────────────────┘  └─────────────────┘         │
  │  ┌─────────────────┐  ┌─────────────────┐         │
  │  │ DissonantNovel  │  │ Incoherent      │         │
  │  │ (novel but      │  │ (words don't    │         │
  │  │  incoherent)    │  │  resonate)      │         │
  │  └─────────────────┘  └─────────────────┘         │
  │  ┌─────────────────┐  ┌─────────────────┐         │
  │  │ CoherentGrounded│  │ CoherentNovel   │         │
  │  │ (coherent,      │  │ (coherent +     │         │
  │  │  not novel)     │  │  novel = best)  │         │
  │  └─────────────────┘  └─────────────────┘         │
  │  ┌─────────────────┐  ┌─────────────────┐         │
  │  │ ModerateNovel   │  │ CoherentFamiliar│         │
  │  │ (moderate +     │  │ (coherent,      │         │
  │  │  novel)         │  │  not novel)     │         │
  │  └─────────────────┘  └─────────────────┘         │
  │  ┌─────────────────┐                              │
  │  │ WeaklyCoherent  │                              │
  │  │ (marginal)      │                              │
  │  └─────────────────┘                              │
  └─────────────────────────────────────────────────────┘
```

## Display Output

```
  Evaluator output (implements Display):

  ┌──────────────────────────────────────────────────────┐
  │  Coherence: 0.85  Novelty: 0.72  Resonance: 1.00    │
  │  Overall: 0.89                                       │
  │  Verdict: Coherent and novel - insightful            │
  └──────────────────────────────────────────────────────┘

  vs. old format() method - now uses {} format specifier:
    println!("{}", eval);  // uses Display trait
```

## Score → Verdict Matrix

```
              │ novelty < 0.3 │ 0.3-0.6 │ novelty > 0.6
  ────────────┼──────────────┼─────────┼──────────────
  coh > 0.7   │ Grounded     │ Familiar│ Novel ★
  coh 0.5-0.7 │ Familiar     │ Familiar│ ModerateNovel
  coh 0.2-0.5 │ WeaklyCoherent (regardless of novelty)
  coh < 0.2   │ Incoherent   │ Incoherent│ DissonantNovel

  res < 0.3   │ Noise (regardless of coherence/novelty)
```
