# HOW 13 — Persistence & Cost

> _The economic argument. This is where the architecture is genuinely, measurably
> ahead — and where the current artifact is not yet delivering on it._

---

## 1. The mechanism

```rust
// src/storage.rs
Storage::save(&facet, "data/manifold.chroma")   // bincode
Storage::load("data/manifold.chroma")

// src/memory/mod.rs
memo.save_to_file("data/memory.chroma")
Memo::load_from_file(...)
```

Both use `bincode` over `serde` — a compact binary format with no schema
overhead. Save is called on `exit`/`quit` via `Model::scale`; load happens in
`Model::new`.

---

## 2. The cost model, computed

### Per word, phasor only

| field | type | bytes |
|:---|:---|--:|
| `phase` | f64 | 8 |
| `amplitude` | f64 | 8 |
| `band_n` | u32 | 4 |
| | | **20** (24 padded) |

With `f32` for phase and amplitude — and f32 gives ~7 decimal digits, far more
than 64-sector resolution needs — this is **12 bytes**, or 16 with the key's
length prefix. The README's "16 bytes fixed-width" is achievable; the current
struct is not there yet.

### Full lexicon

| vocabulary | phasors (f64) | phasors (f32) |
|---:|---:|---:|
| 10,000 | 240 KB | 120 KB |
| 100,000 | 2.4 MB | 1.2 MB |
| 1,000,000 | 24 MB | 12 MB |

**A million-word vocabulary in 12 MB.** That is the claim, and the arithmetic
supports it.

### Where the 92 MB comes from

`data/manifold.chroma` is **92,157,679 bytes**. Phasors account for at most a few
MB of that. The rest is the n-gram tables, and specifically their `String` keys:

```rust
pub bigrams:  HashMap<String, HashMap<String, u32>>
pub trigrams: HashMap<String, HashMap<String, u32>>     // key = "word_a word_b"
pub phase_lags: HashMap<String, HashMap<String, f64>>
```

Every follower entry serialises its key as a full UTF-8 string with a length
prefix. A trigram key duplicates both of its words. `phase_lags` duplicates the
bigram key set a third time, with an f64 payload.

**Estimated breakdown:**

| component | share |
|:---|--:|
| bigrams | ~35 MB |
| trigrams | ~40 MB |
| phase_lags | ~15 MB |
| phasors | ~2 MB |

The interning fix from HOW 04 §4 addresses all three at once:

```rust
pub struct Vocab { ids: HashMap<String, u32>, words: Vec<String> }

pub bigrams:    HashMap<u32, Vec<(u32, u32)>>,        // 8 bytes/entry
pub trigrams:   HashMap<(u32,u32), Vec<(u32,u32)>>,   // 8 bytes/entry, no alloc on lookup
pub phase_lags: HashMap<(u32,u32), f32>,              // 12 bytes/entry
```

Plus singleton pruning after bulk ingest. Projected: **92 MB → 6–10 MB**, which
lands squarely inside the README's Phinum32/64 envelope.

---

## 3. Compute cost, per operation

| operation | complexity | measured shape |
|:---|:---|:---|
| learn one sentence | O(L) with 3 `sin`/`cos` per token | ~1 µs for L = 10 |
| add one word | O(1) hash insert + one `sin` | ~100 ns |
| ray cast | O(V) parallel | ~100 µs at V = 10⁵, 8 cores |
| generate one token | O(pool) + O(V) fallback | ~100 µs–1 ms |
| save / load | O(V + n-grams) | seconds at 92 MB |

### The comparison that matters

| | Phiano (current) | Phiano (post-fix) | 7B transformer (int4) |
|:---|---:|---:|---:|
| model size | 92 MB | ~8 MB | ~3.5 GB |
| RAM to run | ~200 MB | ~30 MB | ~4 GB |
| learn one new fact | ~1 µs | ~1 µs | hours of GPU fine-tuning |
| unlearn one fact | ~1 µs | ~1 µs | not reliably possible |
| GPU required | no | no | effectively yes |
| runs on a Cortex-M7 | no (92 MB) | **plausible** | no |
| inspect why it answered | fully | fully | no |

The middle column is the product. Six orders of magnitude on the update path, two
to three on size, and full interpretability. Those are not marginal gains; they
define a different deployment class.

---

## 4. Three durability gaps

### (a) Save happens only on clean exit

```rust
// Model::iterate
if cmd == "exit" || cmd == "quit" { self.scale(); return; }
```

`Ctrl-C`, a panic, a power loss, a container stop → **everything learned in that
session is gone**. For a system whose selling point is continual learning, that
is the highest-severity issue in this document.

**Fix — periodic checkpoint plus signal handling:**

```rust
// in the REPL loop
self.turns_since_save += 1;
if self.turns_since_save >= 20 { self.scale(); self.turns_since_save = 0; }

// at startup
ctrlc::set_handler(move || { model.lock().scale(); std::process::exit(0); })?;
```

### (b) No atomic write

`Storage::save` writes directly to `data/manifold.chroma`. An interruption
mid-write leaves a truncated file, and the next `Storage::load` fails — silently
falling back to an **empty facet** in `Model::new`:

```rust
let mut facet = match Storage::load(config::CHROMA_FILE) {
    Ok(m) => m,
    Err(_) => { let _ = fs::create_dir_all("data"); Facet::new() }   // ← total loss, no warning
};
```

**Fix — write-rename, which is atomic on every mainstream filesystem:**

```rust
pub fn save(facet: &Facet, path: &str) -> std::io::Result<()> {
    let tmp = format!("{}.tmp", path);
    { let f = File::create(&tmp)?; bincode::serialize_into(BufWriter::new(f), facet)?; }
    std::fs::rename(&tmp, path)          // atomic
}
```

And make the load failure loud:

```rust
Err(e) => { eprintln!("  [WARN] could not load {}: {} — starting empty", CHROMA_FILE, e); Facet::new() }
```

### (c) The version field exists but is never checked

`ChromaHeader` already carries one — this is good design that is half-wired:

```rust
pub struct ChromaHeader { pub version: u32, pub vocabulary_size: usize, pub fine_structure_alpha: f64 }
```

`from_facet` writes `version: 1`. `Storage::load` never reads it. Instead, a
failed deserialisation falls back to `LegacySerializedFacet` — and if *that*
fails, `Model::new` starts empty (§4b). So a genuine format change surfaces as
"starting empty" rather than "this file is version 1, I expect version 2".

**Fix:** check it, and say what happened.

```rust
let sf = SerializedFacet::load_from_file(path)?;
if sf.header.version > FORMAT_VERSION {
    return Err(Error::new(ErrorKind::InvalidData,
        format!("{} is format v{}, this build reads v{}", path, sf.header.version, FORMAT_VERSION)));
}
```

`fine_structure_alpha` is stored too — so a model trained under a different α can
be detected rather than silently misinterpreted. Also worth checking.

### (d) Save clones the entire model

```rust
// SerializedFacet::from_facet
lexicon:    facet.lexicon.clone(),
bigrams:    facet.bigrams.clone(),
trigrams:   facet.trigrams.clone(),
phase_lags: facet.phase_lags.clone(),
```

Every save duplicates the whole model in RAM before writing. At the current
92 MB artifact that is a ~180 MB peak, on a system whose pitch is embedded
deployment.

**Fix — serialise by reference:**

```rust
#[derive(Serialize)]
struct FacetRef<'a> {
    header: ChromaHeader,
    lexicon: &'a HashMap<String, SpectralPhasor>,
    bigrams: &'a HashMap<String, HashMap<String, u32>>,
    trigrams: &'a HashMap<String, HashMap<String, u32>>,
    phase_lags: &'a HashMap<String, HashMap<String, f64>>,
}
```

Serde serialises references transparently; deserialisation keeps the owned
struct. Zero copies on save.

## 5. Load time

92 MB of bincode, plus:

```rust
// Model::new
if facet.bigrams.is_empty() && !facet.lexicon.is_empty() { Self::bootstrap_bigrams(&mut facet); }
DefinitionGrounder::ground_phases(&mut facet, &ChunkStore::new("data/chunks"));   // EVERY startup
```

`ground_phases` runs on **every** startup, over the entire dictionary, whether or
not anything changed. That is a full re-grounding pass — seconds of work — on each
launch, and it also means the saved phases are mutated at load time, so the
model that starts is not the model that was saved.

**Fix:** run it once, record it, skip thereafter.

```rust
#[serde(default)] pub grounded_version: u32,
...
if facet.grounded_version < GROUNDING_VERSION {
    DefinitionGrounder::ground_phases(&mut facet, &chunks);
    facet.grounded_version = GROUNDING_VERSION;
}
```

---

## 6. What this buys

- **A real deployment class.** Post-interning, this is a language model that fits
  in a microcontroller's flash and updates in microseconds. There is no
  transformer in that category, at any quantisation.
- **No inference infrastructure.** No CUDA, no ONNX runtime, no server. A single
  static Rust binary.
- **Learning and serving are the same process.** No training pipeline, no
  retraining schedule, no model registry. The artifact on disk is always current.
- **Auditable.** The entire model can be dumped to a text table and read by a
  human. In regulated settings that is worth more than accuracy points.

---

## 7. How it generalises

1. **Intern the vocabulary** (HOW 04 §4) — 92 MB → ~8 MB, and removes
   per-lookup allocation from the hot path.
2. **f32 phasors** — halves the phasor footprint at no meaningful precision cost.
3. **Atomic save + periodic checkpoint + Ctrl-C handler** (§4) — durability.
4. **Check the version field that already exists** (§4c) and serialise by reference (§4d) — no silent empty-facet starts, no 2× RAM spike on save.
5. **Skip re-grounding when unchanged** (§5) — instant startup.
6. **Then publish the numbers.** Once the artifact is 8 MB and starts in
   milliseconds, the cost table in §3 becomes a demo rather than a projection —
   and it is the most persuasive thing this project has.

---

## 8. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| Save only on exit/quit | `Model::iterate`, `Model::scale` |
| Load failure silently yields empty facet | `Model::new` match arm |
| Non-atomic write | `Storage::save` → `File::create` then serialize_into |
| Version field written but never checked | `ChromaHeader::version`, `Storage::load` |
| Save clones the whole model | `SerializedFacet::from_facet` |
| Grounding runs every startup | `Model::new` calls `ground_phases` unconditionally |
| Model file is 92 MB | `ls -la data/manifold.chroma` |
| n-gram keys are owned Strings | `Facet` field types |

---

**Next:** [HOW 14 — Lifelong Transfer](14_lifelong_transfer.md).
