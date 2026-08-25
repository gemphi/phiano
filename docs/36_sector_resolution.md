# 36 - Sector Resolution: 64 / 128 / 256 / 512 / 1024

```
┌──────────────────────────────────────────────────────────────┐
│                    PHASE CIRCLE (2π)                         │
│                                                              │
│     64 sectors          128 sectors        256 sectors       │
│  ┌───┬───┬───┐       ┌─┬─┬─┬─┬─┐      ┌┬┬┬┬┬┬┬┬┬┐          │
│  │ 0 │ 1 │ 2 │ ...   │0│1│2│3│4│ ...  │││││││││││ ...       │
│  └───┴───┴───┘       └─┴─┴─┴─┴─┘      └┴┴┴┴┴┴┴┴┴┘          │
│   2π/64 ≈ 0.098       2π/128 ≈ 0.049   2π/256 ≈ 0.025       │
│                                                              │
│  Higher N = finer granularity = more variations              │
│  Lower N  = coarser buckets  = faster search                │
│                                                              │
│  Config: config::SECTOR_RESOLUTION (must be power of 2)     │
└──────────────────────────────────────────────────────────────┘
```

## How it works

The phase circle [0, 2π) is divided into N equal sectors. Each word's
effective phase (`phasor.phase + band_n * ALPHA`) maps to exactly one
sector. The number of sectors is configurable:

| Resolution | Sector width | Use case |
|-----------|-------------|----------|
| 64 | 5.625° | Base - 64 variations, 64 colors, 64 persona archetypes |
| 128 | 2.8125° | More nuance, more shades of meaning |
| 256 | 1.40625° | High-definition semantic space |
| 512 | 0.703° | Very fine-grained, large vocabularies |
| 1024 | 0.352° | Maximum resolution, research / large-scale impersonation |

## Opposites

Every sector has an antipodal opposite: `(sector + N/2) mod N`.
Words in opposite sectors represent semantic tension/contrast.

## Color mapping

Sectors map proportionally to 16 base colors (crimson → rose) distributed
evenly across whatever N is configured. See `compose::sector_color()`.

## File references

- `src/config.rs:36-57` - `SECTOR_RESOLUTION` constant and validator
- `src/wave.rs:14-16` - `sectors()` function
- `src/wave.rs:118-130` - `sector_of()`, `opposite_sector()`
- `src/compose/mod.rs:20-30` - `sector_color()` proportional mapping
