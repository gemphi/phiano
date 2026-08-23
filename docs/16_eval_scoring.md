# 16 — Evaluation Scoring

## Three Dimensions

```
  ┌──────────────────────────────────────────────────┐
  │  RESONANCE                                        │
  │  = known_tokens / total_tokens                    │
  │  Range: [0.0, 1.0]                               │
  │  "How much of this text do I know?"              │
  │                                                  │
  │  "the cat sat" with all known → 1.0             │
  │  "xyzzy qux foobar" with none known → 0.0       │
  └──────────────────────────────────────────────────┘

  ┌──────────────────────────────────────────────────┐
  │  COHERENCE                                        │
  │  = |Z_sentence| / N_known                         │
  │  Range: [0.0, 1.0] (clamped)                     │
  │  "How well do the words align?"                  │
  │                                                  │
  │  Words in same direction → high |Z| → high score │
  │  Words cancelling each other → low |Z| → low    │
  └──────────────────────────────────────────────────┘

  ┌──────────────────────────────────────────────────┐
  │  NOVELTY                                          │
  │  = 1 - exp(-5 · |Z_sentence - Z_centroid|)       │
  │  Range: [0.0, 1.0]                               │
  │  "How different is this from what I know?"       │
  │                                                  │
  │  Close to centroid → low novelty                 │
  │  Far from centroid → high novelty                │
  └──────────────────────────────────────────────────┘
```

## Overall Score

```
  overall = coherence × 0.45 + resonance × 0.40 + novelty × 0.15

  ┌────────────────────────────────────────────┐
  │  Weight allocation:                        │
  │                                            │
  │  ████████████████████░░░░  coherence 45%  │
  │  ███████████████████░░░░░░  resonance 40%  │
  │  ██████░░░░░░░░░░░░░░░░░░░  novelty   15%  │
  └────────────────────────────────────────────┘

  Coherence matters most — aligned words = understanding.
  Resonance is nearly equal — knowing words is fundamental.
  Novelty is a tiebreaker — new but coherent is best.
```

## Verdict Decision Tree

```
  resonance < 0.3?
    ├─ YES → Noise
    └─ NO → coherence < 0.2?
              ├─ YES → novelty > 0.7?
              │         ├─ YES → DissonantNovel
              │         └─ NO  → Incoherent
              └─ NO → coherence > 0.7?
                        ├─ YES → novelty < 0.3?
                        │         ├─ YES → CoherentGrounded
                        │         └─ NO → novelty > 0.6?
                        │                   ├─ YES → CoherentNovel
                        │                   └─ NO → CoherentFamiliar
                        └─ NO → coherence > 0.5?
                                  ├─ YES → novelty > 0.5?
                                  │         ├─ YES → ModerateNovel
                                  │         └─ NO  → CoherentFamiliar
                                  └─ NO  → WeaklyCoherent
```
