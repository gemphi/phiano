# 20 - Chunking & Parallel Training

## Chunk Split

```
  websters_dictionary.json (22 MB, 30k+ words)
         │
         ▼
  ┌──────────────────────────────────┐
  │  ChunkStore::split()             │
  │                                  │
  │  Group by first letter:          │
  │  a → {apple, ant, ...}           │
  │  b → {banana, bat, ...}          │
  │  ...                             │
  │  z → {zebra, zoo, ...}           │
  └──────────┬───────────────────────┘
             │
             ▼
  data/chunks/
    ├── a/a.json    (2,000 words)
    ├── b/b.json    (1,800 words)
    ├── c/c.json    (3,500 words)
    ├── ...
    └── z/z.json    (500 words)

  Written in parallel with rayon.
```

## Parallel Ingestion

```
  train 50
       │
       ▼
  ┌────────────────────────────────────────────┐
  │  ChunkStore::ingest_parallel()             │
  │                                            │
  │  Step 1: Load all chunks (parallel)        │
  │  ┌─────┬─────┬─────┬─────┬─────┐         │
  │  │ a/  │ b/  │ c/  │ ... │ z/  │         │
  │  │a.json│b.json│c.json│     │z.json│       │
  │  └──┬──┴──┬──┴──┬──┴─────┴──┬──┘         │
  │     │     │     │            │             │
  │     └─────┴─────┴────────────┘             │
  │               │                            │
  │               ▼                            │
  │  All entries: Vec<(word, def)>             │
  │  (30,000+ pairs)                           │
  └──────────────┬─────────────────────────────┘
                 │
                 ▼
  ┌────────────────────────────────────────────┐
  │  Step 2: Train for N epochs                │
  │                                            │
  │  for epoch in 0..50:                       │
  │    for (word, def) in entries:             │
  │      trainer.train_definition(facet,       │
  │                                word, def)  │
  │    print progress                          │
  │                                            │
  │  [epoch 1/50]  30214 words, 45.2s         │
  │  [epoch 2/50]  30214 words, 44.8s         │
  │  ...                                       │
  │  [epoch 50/50] 30214 words, 44.1s         │
  └────────────────────────────────────────────┘
```

## Training Metrics

```
  TrainingMetrics {
      epochs_completed: 50,
      words_learned: 30214,
      total_time: Duration::from_secs(2245),
  }

  words_per_sec() = 30214 / 2245 ≈ 13.5

  [metrics] 50 epochs, 30214 words, 2245s total, 13 words/sec
```
