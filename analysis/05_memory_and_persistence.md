# 05 — Memory and Persistence Architecture

> Files examined: [`src/memory/mod.rs`](../src/memory/mod.rs), [`src/storage.rs`](../src/storage.rs),
> [`src/chunker.rs`](../src/chunker.rs), [`src/layers.rs`](../src/layers.rs),
> [`data/`](../data) (on-disk artifacts), [`docs/08_memory_layers.md`](../docs/08_memory_layers.md).

---

## 1. The Two Stores

| Store | Content | Format | Current size |
|---|---|---|---|
| `data/manifold.chroma` | The **Facet**: lexicon (word → phasor), bigrams, trigrams, phase_lags β_ij | bincode | **92,157,679 bytes (~92 MB)** |
| `data/memory.chroma` | The **Memo**: every interaction's (timestamp, wave, FNV hash, text, layer) | bincode | grows with use |

Plus on-disk corpora feeding learning: `data/chunks/` (Webster's definitions),
`data/rust_book/`, `data/definitions/`, `data/dialogues/`, `data/curriculum.json`,
`data/searle_markers.json`, `data/arc_tasks.json`, `data/stop_words.txt`.

## 2. The 16-Layer Memo: What It Actually Is

[`Memo`](../src/memory/mod.rs) records every interaction into one of 16 layers organized
as 4 bands × 4 sub-layers. Classification ([`classify_layer`](../src/memory/mod.rs), lines
102–127) is **purely structural**:

```text
band  = Surface (≤3 words) | Pattern (4–8) | Semantic (9–16) | Deep (>16)
layer = band.base + sub_layer(avg word length: ≤4, 5–6, 7–8, >8)
```

**Honest verdict:** this is an episodic interaction log binned by *text statistics*, not
a semantic memory hierarchy. The names suggest cognitive depth; the code measures word
count and average word length. Long, Latinate sentences are "deep" regardless of content;
a profound three-word aphorism is "surface."

## 3. Persistence Economics (the Real Numbers)

The 92 MB manifold breaks down approximately (per [`docs/how/13_persistence_and_cost.md`](../docs/how/13_persistence_and_cost.md)):

| Component | Share | Note |
|---|---|---|
| Trigram tables | ~40 MB | `"w_a w_b"` string keys → HashMap<String, HashMap> |
| Bigram tables | ~35 MB | nested HashMaps with String keys |
| Phase lags β_ij | ~15 MB | one f64 per observed word pair |
| **Phasors (the actual "model")** | **~2 MB** | 16 bytes × ~150k words |

**This is the punchline of the file:** the learned phase manifold — the part that makes
Phiano Phiano — is **~2% of the persisted state**. 98% is classical n-gram counting
overhead: `String` keys, boxing, HashMap overhead. The docs' own interning plan
(string interning + u32 ids) projects **92 MB → 6–10 MB**, which is achievable and is
the single highest-leverage storage fix (file 16, task 2). The advertised "~5 MB
Phinum32 / ~12 MB Phinum64" footprints become *true* only after that refactor.

## 4. The Missing Half: Recall

A memory system needs write **and** read. Audit of read paths:

- Phasors: read constantly (generation, ray-cast, eval, reasoning). ✔
- n-gram tables: read constantly (trigram/bigram candidates). ✔
- **Memo: written every turn ([`Model::iterate`](../src/model.rs)), never read back.**
  No generation, reasoning, or chat path queries `Memo::layers` for prior episodes.
- Chunk store (definitions): read by envision/definition-chain. ✔ (but only definitions,
  not the dialogue corpus `data/dialogues/`).

**Consequence:** the system has *procedural memory* (the facet — permanently shaping
behavior) and *episodic logging* (the memo — inert). What it lacks is *episodic recall*:
"I told you my daughter is allergic to peanuts" persists only insofar as the words
`peanut`, `allergic`, `Maya` kept phase-aligning. Facts stated once in passing leave
only faint phase traces, not retrievable statements. This is the sharpest single gap
between the architecture's promise and the code, and the cheapest high-value fix
(file 16, task 6: recall top-k memo entries by context-wave resonance before generation).

## 5. Persistence Mechanics

- bincode round-trip is tested; save on exit/`save` command/`/api/save`.
- Load-time **bootstrap**: if phasors exist but bigrams are empty (legacy model),
  bigrams are rebuilt from the chunk store in ~2–5 s
  ([`Model::bootstrap_bigrams`](../src/model.rs)); definition grounding then re-seeds
  phases ([`DefinitionGrounder`](../src/cognitive/grounding.rs)).
- No schema versioning: a phasor-format change invalidates old `.chroma` files
  (acceptable at v0.2, should be flagged).

## 6. Scorecard

| Aspect | Rating | Note |
|---|---|---|
| Durability of learning across sessions | **Good** | bincode save/load verified |
| Model-state size honesty | **Mixed** | 2 MB of real model inside 92 MB of n-gram freight |
| Layering semantics | **Weak** | Length-binned labels, not memory depth |
| Episodic recall | **Absent** | Memo is write-only — the critical gap |
| Scalability path | **Clear** | Interning → 6–10 MB is well-understood work |
| Reproducibility | **Excellent** | Deterministic seeds → identical rebuilds |

**Bottom line:** Phiano's *knowledge* persistence is real and cheap; its *experience*
persistence is a diary nobody rereads. Fixing recall and interning converts the memory
story from "log with pretensions" to a genuine lifelong-memory substrate.
