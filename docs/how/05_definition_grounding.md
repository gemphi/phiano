# HOW 05 — Definition Grounding

> _The strongest idea in the system: a word's position is not arbitrary, it is the
> centre of mass of what the word means._

---

## 1. The mechanism

Instead of leaving a word at its synthetic `len × φ` seed, place it at the
amplitude-weighted centroid of the words in its **dictionary definition**, then
relax halfway there:

$$\theta_w \leftarrow \theta_w + \tfrac{1}{2}\,\Delta\!\left(\theta_w,\ \operatorname{atan2}\Big(\sum_{d \in D(w)} A_d\sin\theta_d,\ \sum_{d \in D(w)} A_d\cos\theta_d\Big)\right)$$

where Δ is the shortest signed angular difference (wrapped into (−π, π]).

**In the source:** `src/cognitive/grounding.rs :: DefinitionGrounder::ground_phases`.

Called automatically at startup (`Model::new`) whenever the lexicon is non-empty,
and at the end of every curriculum run.

---

## 2. Why this is the right idea

Dictionary definitions are a **compositional semantic bootstrap**. A dictionary is
a closed system in which every word is defined by other words, and a small core
(~2,000 words in Longman's defining vocabulary) is sufficient to define all the
rest. That means:

- Meaning is recoverable from structure alone, with no labelled data.
- The grounding is auditable — you can read *why* a word sits where it sits.
- It attacks HOW 01's collision problem exactly where it hurts: after grounding,
  `cat` and `war` no longer share a position, because their definitions do not
  share words.
- It costs one pass over the dictionary. `data/chunks/` holds 26 shards; the whole
  operation is seconds.

This is a serious, defensible design decision and it deserves more prominence in
the project's documentation than it currently gets.

---

## 3. Worked example

Suppose the lexicon already holds:

| word | θ | A |
|:--|--:|--:|
| `small` | 0.30 | 1.2 |
| `furry` | 0.50 | 1.1 |
| `animal` | 0.70 | 1.4 |
| `cat` | 4.854102 | 1.0 |

and `data/chunks/c/c.json` defines `cat` → `"a small furry animal"`.

**Tokenise:** `["a", "small", "furry", "animal"]`. `a` is unknown → skipped
(`facet.lexicon.get(token)` returns `None`).

**Centroid of the definition:**

$$\sum A\cos\theta = 1.2(0.95534) + 1.1(0.87758) + 1.4(0.76484) = 1.14641 + 0.96534 + 1.07078 = 3.18253$$
$$\sum A\sin\theta = 1.2(0.29552) + 1.1(0.47943) + 1.4(0.64422) = 0.35462 + 0.52737 + 0.90191 = 1.78390$$

$$\theta_{\text{centroid}} = \operatorname{atan2}(1.78390,\ 3.18253) = 0.51117 \text{ rad}$$

**Signed shortest difference:**

$$\Delta = 0.51117 - 4.854102 = -4.34293$$

which is < −π, so wrap: $\Delta \mathrel{+}= 2\pi \Rightarrow \Delta = 1.94026$

**Relax halfway:**

$$\theta_{\texttt{cat}} = (4.854102 + 0.5 \times 1.94026) \bmod 2\pi = 5.824232$$

`cat` has moved 1.94 rad — nearly a third of the circle — from a
length-determined seed toward the neighbourhood of its own meaning. In sector
terms: **sector 49 → sector 59**, and it is now travelling toward the cluster
{`small`, `furry`, `animal`} at sectors 3–7.

The 0.5 damping is deliberate and correct: it stops a single definition from
teleporting a word, and it makes repeated grounding passes converge geometrically
(each pass halves the remaining distance).

---

## 4. Three problems, all fixable

### (a) Order dependence

```rust
for (word, def) in &entries { ... }
```

`entries` comes from `ChunkStore::load_all()` — filesystem order. Grounding
`cat` uses the *current* phases of `small`, `furry`, `animal`; if those words are
grounded later in the same pass, `cat`'s result reflects their pre-grounding
positions. The output depends on directory iteration order, which is not stable
across machines.

**Fix — two-phase update (Jacobi rather than Gauss–Seidel):**

```rust
let mut updates: Vec<(String, f64)> = Vec::with_capacity(entries.len());
for (word, def) in &entries {
    if let Some(target) = centroid_of_definition(facet, def) {   // read-only pass
        updates.push((word.clone(), target));
    }
}
for (word, target) in updates {                                   // write pass
    if let Some(p) = facet.lexicon.get_mut(&word) {
        let d = wrap_signed(target - p.phase);
        p.phase = (p.phase + 0.5 * d).rem_euclid(TWO_PI);
    }
}
```

Deterministic, order-independent, same cost.

### (b) Single pass

One pass propagates meaning one hop through the definition graph. Definitions
are recursive — `cat` → `animal` → `organism` → `living` — so the fixed point
needs iteration:

```rust
for round in 0..5 {
    let moved = ground_phases_jacobi(facet, chunk_store);
    if moved < facet.lexicon.len() / 100 { break; }   // converged
}
```

Five rounds with 0.5 damping leaves 3% of the initial error — effectively
converged, and still seconds of compute. This is the highest-value line change in
the grounding module.

### (c) Function words dominate the centroid

`"a small furry animal"` — if `a`, `the`, `of`, `is` are in the lexicon (they
will be after any training), they contribute to every definition's centroid with
saturated amplitude 2.0. Since they appear in *all* definitions, they pull every
word toward one shared point, which is HOW 02's collapse re-entering through the
grounding door.

**Fix — the filter already exists:**

```rust
for token in &def_tokens {
    if Tokenizer::is_function_word(token) { continue; }   // one line
    ...
}
```

Or better, IDF-weight: a definition word's contribution ∝ 1/log(number of
definitions it appears in). Both are cheap; the `continue` is free.

---

## 5. The definition chain — recursive curiosity

`Trainer::learn_definition_chain` is the operational partner to grounding:

```rust
// src/trainer/mod.rs
fn learn_chain_recursive(&self, facet, chunk_store, word, depth_left, learned, visited) {
    if depth_left == 0 || visited.contains(word) { return; }
    visited.insert(word);
    if let Some(p) = facet.lexicon.get(word) { if p.amplitude > 5.0 { return; } }

    let definition = chunk_store.load_definition(word)?;
    self.train_definition(facet, word, &definition);
    learned.push(word);

    for token in Tokenizer::tokenize(&definition) {
        if !facet.lexicon.contains_key(token) && !visited.contains(token) {
            self.learn_chain_recursive(facet, chunk_store, token, depth_left - 1, ...);
        }
    }
}
```

This is a **depth-limited DFS over the definition graph with cycle detection**,
and it is exactly right in structure. Encounter an unknown word → fetch its
definition → train on it → recurse into whatever in that definition is still
unknown. Default depth is 3 (`DEFINITION_CHAIN_DEPTH`).

### Worked example

`learn "photosynthesis"`, depth 3:

```
photosynthesis  →  "process by which plants convert light into energy"
  ├─ plants     →  "living organisms that grow in soil"
  │    ├─ organisms → "individual living things"       [depth 3 — recursion stops]
  │    └─ soil      → "the top layer of earth"         [depth 3 — recursion stops]
  ├─ convert    →  "change from one form to another"
  └─ energy     →  "the capacity to do work"
```

Six new words, one user request, no labels, no gradient. That is a genuinely good
active-learning loop.

### One live bug

```rust
match facet.lexicon.get(word) {
    Some(phasor) if phasor.amplitude > 5.0 => return,
    _ => {}
}
```

`AMPLITUDE_MAX` is **2.0** (`src/config/constants.rs`). Amplitude can never
exceed 2.0, so `> 5.0` is never true and this early-exit is dead code. The
intended behaviour — "skip words I already know well" — never fires, so the
chain re-trains well-known words on every call.

**Fix:** `Some(p) if p.amplitude > AMPLITUDE_MAX * 0.9 => return,` — and reference
the constant rather than a literal, so it cannot drift again.

---

## 6. How it generalises — grounding beyond the dictionary

The mechanism is not dictionary-specific. `ground_phases` takes *any* mapping from
a symbol to a bag of symbols and places the symbol at the bag's centroid. That
generalises directly:

| Domain | The "definition" | What grounding gives you |
|:---|:---|:---|
| Code | a function's body tokens | functions positioned near what they call |
| Wikipedia | the lead paragraph | entities positioned near their descriptions |
| Chat history | the turns a term appears in | user-specific jargon grounded in use |
| APIs | docstring + signature | endpoints positioned near their semantics |
| Any corpus at all | a window around each mention | distributional grounding, no dictionary needed |

`src/sources/` already has `wiktionary.rs`, `wiki_bulk.rs`, `api.rs`, `local.rs`,
`json.rs` — the ingestion side of this is largely built. What is missing is the
generalised call:

```rust
pub trait Groundable { fn definition_of(&self, symbol: &str) -> Option<String>; }
pub fn ground_from<G: Groundable>(facet: &mut Facet, src: &G, rounds: usize) -> usize;
```

One trait, and every source in `src/sources/` becomes a grounding source.

**With multi-phase representations (HOW 01), this becomes materially more
powerful**: instead of collapsing a definition to one angle, each of the D
channels grounds independently, so `cat` can be near `animal` in one channel and
near `pet` in another without the two constraints fighting over a single number.
On a circle, those two facts are in direct competition; in D channels they
coexist. That is the concrete sense in which widening the representation is what
makes grounding scale.

---

## 7. What this buys

- **Non-arbitrary positions**, derived from a real semantic resource, auditable
  word by word.
- **A repair for the seeding collision** for all dictionary-covered vocabulary.
- **Zero labels, zero gradient, seconds of compute.**
- **A curiosity loop** (`learn_definition_chain`) that acquires vocabulary on
  demand with cycle safety and depth limits.

---

## 8. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| Grounding relaxes 50% toward definition centroid | `0.5 * diff` in `ground_phases` |
| Uses current (mutating) phases — order dependent | the single `for (word, def)` loop |
| Runs one pass only | no outer loop in `ground_phases` |
| Function words are not filtered | no `is_function_word` check in the token loop |
| `amplitude > 5.0` guard is dead | `AMPLITUDE_MAX = 2.0` in `src/config/constants.rs` |
| Chain depth default is 3 | `DEFINITION_CHAIN_DEPTH` |

---

**Next:** [HOW 06 — Sentence Superposition](06_sentence_superposition.md).
