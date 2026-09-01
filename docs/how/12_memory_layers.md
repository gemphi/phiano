# HOW 12 — Memory Layers

> _Two hierarchies, sixteen layers and four bands. One stratifies experience by
> surface form; the other stratifies the manifold by resolution. Only one of them
> is currently connected to anything._

---

## 1. The two hierarchies

| | `Memo` (`src/memory/mod.rs`) | `HierarchicalPhaseField` (`src/layers.rs`) |
|:---|:---|:---|
| stores | every interaction, with its wave | cluster centroids of the lexicon |
| depth | 16 layers in 4 bands | 4 layers |
| resolution | by text shape | 64 → 32 → 16 → 8 sectors |
| persisted | yes, `data/memory.chroma` | no, rebuilt on demand |
| read by | `stats`, `layers` command | `resonate_depth` |

---

## 2. `Memo` — how an interaction is filed

```rust
// src/memory/mod.rs :: classify_layer
let band = match word_count {
    0..=3   => Surface,     // base 0
    4..=8   => Pattern,     // base 4
    9..=16  => Semantic,    // base 8
    _       => Deep,        // base 12
};
let sub_layer = match avg_word_len { 0..=4 => 0, 5..=6 => 1, 7..=8 => 2, _ => 3 };
layer = (band.base_layer() + sub_layer).min(15)
```

### Worked example

| input | words | avg len | band | sub | layer |
|:---|--:|--:|:---|--:|--:|
| `hello` | 1 | 5 | Surface (0) | 1 | **1** |
| `what is rust` | 3 | 4 | Surface (0) | 0 | **0** |
| `explain memory safety in rust` | 5 | 5 | Pattern (4) | 1 | **5** |
| `the borrow checker prevents data races at compile time` | 9 | 5 | Semantic (8) | 1 | **9** |
| `ownership is a discipline for enforcing memory safety without runtime garbage collection overhead` | 13 | 7 | Semantic (8) | 2 | **10** |
| a 30-word paragraph of long technical words | 30 | 9 | Deep (12) | 3 | **15** |

Each entry stores:

```rust
pub struct ContextWaveEntry {
    pub timestamp_ms: u64,
    pub superposition_wave: (f64, f64),   // the c64 as (re, im)
    pub text_hash: u64,                   // FNV-1a
    pub text: String,                     // full text
    pub layer: usize,
}
```

### The honest reading

The classifier is **word count and average word length**. Those are surface
statistics. So "Deep" means *long sentence with long words*, not *deep
understanding*, and a profound three-word statement (`consciousness is
substrate-independent`) files as Surface layer 3 while a rambling list of
polysyllables files as Deep layer 15.

The band names promise semantics the classifier does not deliver. That is a
documentation problem more than a code problem — rename them
`Short/Medium/Long/VeryLong` and the module becomes honest and still useful, or
keep the names and make the classifier earn them (§6).

### The duplication

`entries` and `layers[..]` both hold **full clones** of every entry, including
the complete `text` String:

```rust
self.entries.push(entry.clone());
self.layers[layer].push(entry);
```

So memory is stored twice. `data/memory.chroma` is currently 3,362 bytes, so this
does not matter yet — but it doubles linearly forever, and the fix is one line:

```rust
pub layers: [Vec<usize>; MEMORY_LAYERS],   // indices into `entries`
```

---

## 3. `HierarchicalPhaseField` — coarse-graining the manifold

```
Layer 0: words                    (the lexicon itself)
Layer 1: 32 concept clusters      built from words
Layer 2: 16 domain sectors        built from layer 1
Layer 3: 8  meta-patterns         built from layer 2
```

Each layer bins the level below by phase sector and stores an amplitude-weighted
circular centroid per bin.

### Worked example

Six words in the lexicon:

| word | θ | A |
|:--|--:|--:|
| `rust` | 0.10 | 1.5 |
| `cargo` | 0.15 | 1.2 |
| `crate` | 0.20 | 1.1 |
| `python` | 3.10 | 1.4 |
| `pip` | 3.15 | 1.0 |
| `wheel` | 3.20 | 1.0 |

Layer 1 has 32 sectors, width 2π/32 = 0.19635.

- `rust` (0.10) → sector 0; `cargo` (0.15) → sector 0; `crate` (0.20) → sector 1
- `python`/`pip`/`wheel` (3.10–3.20) → sectors 15, 16, 16

**Sector 0 centroid:**
$\sum A\cos = 1.5(0.99500) + 1.2(0.98877) = 1.49250 + 1.18652 = 2.67902$
$\sum A\sin = 1.5(0.09983) + 1.2(0.14944) = 0.14975 + 0.17933 = 0.32908$
phase = atan2(0.32908, 2.67902) = **0.12212**, amplitude = 2.69916/2 = **1.34958**,
`member_count` = 2.

Layer 2 then bins those centroids into 16 sectors, layer 3 into 8. The
`build_layer_from_prev_layer` function correctly propagates `member_count`
upward, so the meta-layers know how much of the lexicon each abstraction covers.

The maths is right, and the idea — a coarse-to-fine index over the manifold — is
exactly the structure you want for fast retrieval and for representing
abstraction.

### What is missing: it is not used

Grep for `HierarchicalPhaseField`:

- built in `src/command/layers.rs` — for a display command
- `resonate_depth` — called only from that display path (and the unit test)
- also owned by `InstructionEngine` (`src/instruction.rs:155`), which calls
  `self.phase_field.build_hierarchy(facet)` at line 357 — and then never queries
  it: the next line delegates to `generate_response`, which does not receive the
  field. The hierarchy is rebuilt on every instruction and discarded.
- **not referenced at all by** `src/wave.rs`, `src/generate.rs`, `src/eval.rs`,
  `src/reasoning/mod.rs`

So it is computed in two places, shown to the user in one, and never consulted
during retrieval or generation. Two things it should be doing:

**(a) Coarse-to-fine retrieval** — the 64× speedup from HOW 07:

```rust
pub fn ray_cast_hier(field: &HierarchicalPhaseField, facet: &Facet, w: c64, k: usize) -> Vec<(String,f64)> {
    // 1. find the best layer-3 meta-pattern (8 comparisons)
    // 2. descend to its layer-2 children (≤16)
    // 3. descend to layer-1 (≤32)
    // 4. exhaustive only within the surviving sector
}
```

**(b) Abstraction for reasoning** — a layer-2 node *is* a learned domain concept.
`reasoning/analogy.rs` and `reasoning/abstraction.rs` should be operating on
these nodes rather than on raw words. This is the piece that would let the system
answer "what kind of thing is this?" without a lookup table.

---

## 4. What this buys

- **Full episodic memory, persisted.** Every interaction, with its wave, its
  timestamp and its text, in `data/memory.chroma`. Most systems keep nothing.
  This is the substrate for genuine personalisation.
- **A ready-made novelty signal.** As noted in HOW 08 §4, the nearest stored wave
  is a far better novelty metric than centroid distance, and the data is already
  on disk.
- **A correct multi-resolution index**, already implemented and tested
  (`test_hierarchical_phase_field`), waiting to be wired in.
- **FNV-1a hashing** — fast, non-cryptographic, appropriate choice for
  deduplication.

---

## 5. The ceiling

Nothing reads `Memo` back into inference. Trace it:

```
Model::iterate → memo.record(wave, line)      ← write
Model::scale   → memo.save_to_file()          ← write
stats / layers commands                        ← display only
```

There is no `memo.recall()`. The model has a complete, timestamped, wave-indexed
record of everything it has ever processed, and it never consults it when
answering. For a system whose distinguishing claim is continual personal
learning, that is the largest unrealised asset in the codebase.

---

## 6. How it generalises

### (a) Recall — the missing function

```rust
impl Memo {
    /// The k most wave-similar past interactions.
    pub fn recall(&self, query: c64, k: usize) -> Vec<&ContextWaveEntry> {
        let mut scored: Vec<(&ContextWaveEntry, f64)> = self.entries.iter()
            .map(|e| {
                let z = c64::new(e.superposition_wave.0, e.superposition_wave.1);
                (e, (query - z).norm())
            }).collect();
        scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        scored.into_iter().take(k).map(|(e, _)| e).collect()
    }

    /// Recency-weighted recall — recent memories win ties.
    pub fn recall_weighted(&self, query: c64, k: usize, half_life_ms: f64) -> Vec<&ContextWaveEntry> {
        let now = now_ms() as f64;
        let mut scored: Vec<(&ContextWaveEntry, f64)> = self.entries.iter()
            .map(|e| {
                let z = c64::new(e.superposition_wave.0, e.superposition_wave.1);
                let age = (now - e.timestamp_ms as f64).max(0.0);
                let recency = 0.5f64.powf(age / half_life_ms);
                (e, (query - z).norm() / recency.max(1e-6))
            }).collect();
        scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        scored.into_iter().take(k).map(|(e, _)| e).collect()
    }
}
```

Then in `Model::iterate`, before dispatch:

```rust
let q = Wave::text(&self.facet, line);
let context = self.memo.recall_weighted(q, 3, 7.0 * 24.0 * 3600_000.0);  // 1-week half-life
// prepend the recalled texts to the generation context
```

That single addition turns the system from stateless-per-turn into genuinely
conversational, using data it has been faithfully collecting all along.

### (b) Semantic layer classification

Replace surface statistics with manifold statistics:

```rust
fn classify_layer_v2(facet: &Facet, text: &str) -> usize {
    let toks = Tokenizer::tokenize(text);
    let known = toks.iter().filter(|t| facet.contains_word(t)).count();
    let z = Wave::sentence(facet, &toks);
    let coherence = if known > 0 { (z.norm() / known as f64).min(1.0) } else { 0.0 };
    let spread    = sector_spread(facet, &toks);          // how many distinct sectors touched

    let band = match spread {                              // conceptual breadth, not length
        0..=1 => 0,   // single-topic
        2..=3 => 4,   // linked topics
        4..=6 => 8,   // multi-domain
        _     => 12,  // broadly integrative
    };
    let sub = ((coherence * 3.99) as usize).min(3);        // how tightly it holds together
    (band + sub).min(15)
}
```

Now "Deep" means *spans many domains coherently*, which is what the band names
have been claiming all along.

### (c) Consolidation

Real memory systems compress. Add a periodic pass that merges near-duplicate
entries (same `text_hash`, or wave distance below a threshold) into a single
entry with a `count`, and promotes anything seen more than N times into the
facet as a trained sentence. That is a sleep/replay cycle, it is cheap, and it
keeps `memory.chroma` bounded as usage grows.

---

## 7. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| Layer = f(word count, avg word length) | `Memo::classify_layer` |
| Entries stored twice | `entries.push(entry.clone())` then `layers[layer].push(entry)` |
| Nothing reads `Memo` back | grep for uses of `memo.` outside `record`/`save`/`len`/`layer_count` |
| Hierarchy never informs retrieval | `HierarchicalPhaseField` appears only in `command/layers.rs` and `instruction.rs`; the latter builds it and discards it |
| Layer sector counts 64/32/16/8 | `LAYER_SECTORS` in `src/layers.rs` |
| Hierarchy maths is tested | `test_hierarchical_phase_field` |

---

**Next:** [HOW 13 — Persistence & Cost](13_persistence_and_cost.md).
