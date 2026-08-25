# 40 - Training Pipeline for Persona Matching

```
┌──────────────────────────────────────────────────────────────────┐
│              PERSONA MATCHING PIPELINE                           │
│                                                                  │
│  1. FEED EXAMPLES                                                │
│     persona add alice "text1" "text2" "text3" ...               │
│     persona add bob   "text1" "text2" "text3" ...               │
│                                                                  │
│  2. EXTRACT FINGERPRINTS                                         │
│     Each persona → sector histogram → dominant sectors           │
│                                                                  │
│  3. COMPARE                                                      │
│     persona compare alice bob                                    │
│     → similarity score (cosine) + difference vector              │
│                                                                  │
│  4. IMPERSONATE                                                  │
│     persona impersonate alice "prompt"                           │
│     → biased river flow through alice's dominant sectors         │
│     → recursive refinement (propose → eval → keep → train)       │
│                                                                  │
│  5. MATCH (task/)                                                │
│     Given unknown text → find which persona wrote it             │
│     Extract fingerprint from text → compare to all personas     │
│     → highest similarity = matched persona                      │
└──────────────────────────────────────────────────────────────────┘
```

## How matching works

Given an unknown text sample, the system:

1. Tokenizes the text and maps each token to its sector
2. Builds a fingerprint (sector histogram) from the text
3. Compares against all stored persona fingerprints
4. Returns the persona with highest cosine similarity

This is **style attribution** - determining who wrote something
based on their phase-space signature.

## Training approach

```
Step 1: Load a trained facet (data/manifold.chroma)
Step 2: Feed each persona's writing samples
Step 3: The facet learns the vocabulary (Kuramoto re-tuning)
Step 4: Fingerprints are extracted from the phase distribution
Step 5: Matching compares fingerprints via cosine similarity
```

The more examples per persona, the more distinct the fingerprint.
With 3 examples, poet vs engineer similarity was 0.171.
With 10+ examples, personas become even more distinguishable.

## Sector resolution impact

Higher resolution (128, 256, 512, 1024) creates more distinct
fingerprints - personas that look similar at 64 sectors may differ
at 256. But it also requires more examples to fill the histogram.

| Resolution | Min examples/persona | Distinctiveness |
|-----------|---------------------|-----------------|
| 64 | 3-5 | Good for broad styles |
| 128 | 5-10 | Better for nuanced styles |
| 256 | 10-20 | Fine-grained attribution |
| 512 | 20-40 | Research-grade |
| 1024 | 40+ | Maximum precision |

## File references

- `src/persona/fingerprint.rs` - Fingerprint::extract(), similarity()
- `src/persona/world.rs` - PersonaWorld::compare()
- `src/persona/impersonate.rs` - Impersonator::impersonate()
- `task/` - Training examples and matching scripts
