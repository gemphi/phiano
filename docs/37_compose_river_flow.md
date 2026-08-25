# 37 - Compose: River Flow Through Color Sectors

```
┌──────────────────────────────────────────────────────────────────┐
│                    RIVER FLOW MODEL                              │
│                                                                  │
│   Source ──→ Clockwise drift ──→ Tension ──→ Resolution          │
│  (sector)    (adjacent sectors)   (opposite)  (back via CCW)     │
│                                                                  │
│   crimson ──→ red ──→ scarlet ──→ ... ──→ violet ──→ ... ──→ red │
│                                                                  │
│   Each sector gathers words from:                                │
│   • Its own sector (6 words)                                     │
│   • Adjacent sectors ±1 (3 words, shades of meaning)             │
│   • At tension: also from source (contrast)                      │
│   • Ray-cast resonant words from the prompt wave                 │
│                                                                  │
│   64 (or 128/256/...) variations generated - one per sector      │
└──────────────────────────────────────────────────────────────────┘
```

## Flower-Hayes Cognitive Process Model (1981)

The compose module implements the Flower-Hayes writing model:

```
  PLANNING ──→ TRANSLATING ──→ REVIEWING
                                ├── Evaluating (better.rs)
                                └── Revising  (worse.rs)
       ↑                              │
       └──── MONITOR (tune.rs) ───────┘
              "Is this good enough?"
              "Should I re-plan?"
```

### Key insight: writing is recursive, not linear

A child learning to write doesn't go plan→write→edit in a straight line.
They cycle: try, evaluate, keep the better, discard the worse, try again.
Each attempt reshapes their internal model. Phiano does the same -
Kuramoto phase relaxation on the winning texts literally re-tunes the facet.

## Module structure

```
src/compose/
├── mod.rs       - Composition struct, sector_color(), Display
├── flow.rs      - RiverFlow: generates N sector variations
├── better.rs    - Evaluator: scores each variation (coherence/novelty/resonance)
├── worse.rs     - Discarder: keeps top N, discards bottom N, trains on better
└── tune.rs      - CompositionTuner (Monitor): recursive refinement loop
```

## The recursive loop

```
1. PROPOSE:  flow.rs generates N sector variations
2. EVALUATE: better.rs scores all N (coherence, novelty, resonance)
3. DISCARD:  worse.rs keeps top 16, discards bottom 16
4. TRAIN:    worse.rs trains facet on the 16 kept texts (Kuramoto)
5. RECURSE:  generate again with the re-tuned facet
6. CONVERGE: stop when top score stops improving
```

## Usage

```
compose "prompt"                              - 3 rounds, no examples
compose "prompt" 5                            - 5 rounds
compose "prompt" 3 "example 1" "example 2"   - learn examples first, 3 rounds
```

## File references

- `src/compose/mod.rs` - Composition struct, sector_color()
- `src/compose/flow.rs` - RiverFlow::trace(), generate_variations()
- `src/compose/better.rs` - Evaluator, SectorScore
- `src/compose/worse.rs` - Discarder, DiscardResult
- `src/compose/tune.rs` - CompositionTuner::refine()
- `src/command/compose.rs` - REPL command handler
