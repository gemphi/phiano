# 17 — Envision: Knowledge Gap Detection

## Gap Detection Flow

```
  Input: "the quantum cat collapsed"
         │
         ▼
  ┌──────────────────────┐
  │ Tokenizer::tokenize  │
  │ → ["the","quantum",  │
  │    "cat","collapsed"]│
  └──────────┬───────────┘
             │
             ▼
  ┌──────────────────────┐
  │ Filter unknown words │
  │                      │
  │ "the"      ✓ known   │
  │ "quantum"  ✗ UNKNOWN │
  │ "cat"      ✓ known   │
  │ "collapsed"✗ UNKNOWN │
  └──────────┬───────────┘
             │
             ▼
  ┌──────────────────────┐
  │ For each unknown:    │
  │ Find similar known   │
  │ words (top 5)        │
  └──────────┬───────────┘
             │
             ▼
  ┌──────────────────────────────────────────┐
  │ Vision {                                 │
  │   text: "I don't know 'quantum',         │
  │          'collapsed'. Define them?"      │
  │   unknown_words: ["quantum","collapsed"] │
  │   related_words: [                       │
  │     ("quantum", [                         │
  │       ("quantity", 0.72),                 │
  │       ("quantities", 0.68),               │
  │       ("quantify", 0.65)                  │
  │     ]),                                  │
  │     ("collapsed", [                       │
  │       ("collapse", 0.89),                 │
  │       ("collapses", 0.85)                 │
  │     ])                                   │
  │   ]                                      │
  │ }                                        │
  └──────────────────────────────────────────┘
```

## String Similarity

```
  score = prefix_score × 0.4 + bigram_score × 0.6

  ┌─────────────────────────────────────────────┐
  │  Prefix Score:                              │
  │  "quantum" vs "quantity"                    │
  │  Shared prefix: "quant" (5 chars)           │
  │  Shorter word: 7 chars                      │
  │  prefix_score = 5/7 = 0.714                 │
  ├─────────────────────────────────────────────┤
  │  Bigram Jaccard:                            │
  │  "quantum" bigrams:  qu,ua,an,nt,tu,um      │
  │  "quantity" bigrams: qu,ua,an,nt,ti,it,ty   │
  │  Intersection: qu,ua,an,nt (4)              │
  │  Union: 6 + 7 - 4 = 9                       │
  │  bigram_score = 4/9 = 0.444                 │
  ├─────────────────────────────────────────────┤
  │  Total: 0.714×0.4 + 0.444×0.6 = 0.569      │
  │  > 0.5 threshold → included as suggestion   │
  └─────────────────────────────────────────────┘
```
