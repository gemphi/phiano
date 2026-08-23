# 30 — Training Pipeline (Full Run)

## Complete Training Sequence

```
  Step 1: Chunk the dictionary
  ┌──────────────────────────────────────────────────┐
  │  phiano> chunk data/websters_dictionary.json     │
  │                                                  │
  │  [chunk] 30214 words into 26 letter groups       │
  │  [chunk] 30214 words split into data/chunks/     │
  └──────────────────────────────────────────────────┘

  Step 2: Train from chunks
  ┌──────────────────────────────────────────────────┐
  │  phiano> train 50                                │
  │                                                  │
  │  [chunk] Found 26 chunk files                    │
  │  [ingest] 30214 entries, 50 epochs               │
  │                                                  │
  │  [epoch 1/50]  30214 words, 45.2s               │
  │  [epoch 2/50]  30214 words, 44.8s               │
  │  [epoch 3/50]  30214 words, 44.5s               │
  │  ...                                             │
  │  [epoch 50/50] 30214 words, 44.1s               │
  │                                                  │
  │  [metrics] 50 epochs, 30214 words, 2245s, 13 w/s │
  └──────────────────────────────────────────────────┘

  Step 3: Save
  ┌──────────────────────────────────────────────────┐
  │  phiano> save                                    │
  │  [saved] data/manifold.chroma (30214 words)      │
  └──────────────────────────────────────────────────┘

  Step 4: Verify
  ┌──────────────────────────────────────────────────┐
  │  phiano> stats                                   │
  │  Vocabulary:     30214 words                     │
  │  Avg amplitude:  1.2847                          │
  │  Dominant band:  n=1                             │
  │  Centroid wave:  (-12.4, 8.2)                    │
  │  Memory entries: 0                               │
  │                                                  │
  │  phiano> synonym cat 5                           │
  │  Rank 1: pet              ΔC = 0.00012          │
  │  Rank 2: dog              ΔC = 0.00031          │
  │  Rank 3: animal           ΔC = 0.00044          │
  │  Rank 4: mouse            ΔC = 0.00082          │
  │  Rank 5: kitten           ΔC = 0.00120          │
  └──────────────────────────────────────────────────┘
```

## Time Estimates

```
  ┌────────────────────────────────────────────────┐
  │  Phase          │ Time      │ Notes             │
  ├─────────────────┼───────────┼──────────────────┤
  │  Chunk split    │ ~3 sec    │ 22 MB JSON parse  │
  │  Epoch 1        │ ~45 sec   │ 30k definitions   │
  │  Epoch 2-50     │ ~44 sec   │ (stable, no new   │
  │                 │ each      │  words to init)   │
  │  Total 50 epochs│ ~37 min   │ 50 × 44.5s        │
  │  Save           │ <1 sec    │ bincode serialize │
  │  Load           │ <1 sec    │ bincode deserialize│
  │  Synonym search │ ~10 ms    │ parallel ray cast │
  └─────────────────┴───────────┴──────────────────┘
```
