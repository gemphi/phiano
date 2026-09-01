# HOW 09 — Envision

> _The model's one genuinely agentic behaviour: noticing a hole in itself and
> asking for it to be filled._

---

## 1. The mechanism

After every input, tokenise, find tokens absent from the lexicon, and for each
one propose the most string-similar known words as candidate relatives.

```rust
// src/envision.rs :: Envision::detect_gaps
let unknown: Vec<String> = tokens.iter().filter(|t| !facet.contains_word(t)).cloned().collect();
if unknown.is_empty() { return None; }

for word in &unknown {
    let mut candidates: Vec<(String, f64)> = facet.lexicon.keys()
        .map(|kw| (kw.clone(), Self::string_similarity(word, kw)))
        .filter(|(_, s)| *s > 0.5)
        .collect();
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    candidates.truncate(5);
}
```

Similarity is **40% prefix overlap + 60% character-bigram Jaccard**:

$$\text{sim}(a,b) = 0.4\frac{|\text{common prefix}|}{\min(|a|,|b|)} + 0.6\frac{|B_a \cap B_b|}{|B_a \cup B_b|}$$

Wired into the loop at `src/model.rs :: Model::iterate` → `Model::envision`, which
runs after every dispatched command.

---

## 2. Worked example — `"tokenizer"` against a small lexicon

Known: `token`, `tokens`, `organizer`, `rust`.

### `token` vs `tokenizer`

- prefix: `t-o-k-e-n` = 5 shared; min length = 5 → prefix score **1.0**
- bigrams(`token`) = {to, ok, ke, en} (4)
- bigrams(`tokenizer`) = {to, ok, ke, en, ni, iz, ze, er} (8)
- intersection 4, union 8 → Jaccard **0.5**
- sim = 0.4(1.0) + 0.6(0.5) = **0.700** ✓ above threshold

### `tokens` vs `tokenizer`

- prefix 5, min length 6 → 0.8333
- bigrams(`tokens`) = {to, ok, ke, en, ns} (5); intersection 4, union 9 → 0.4444
- sim = 0.4(0.8333) + 0.6(0.4444) = **0.6000** ✓

### `organizer` vs `tokenizer`

- prefix 0 → 0.0
- bigrams(`organizer`) = {or, rg, ga, an, ni, iz, ze, er} (8); intersection with
  `tokenizer` = {ni, iz, ze, er} = 4; union 12 → 0.3333
- sim = 0.4(0) + 0.6(0.3333) = **0.2000** ✗ below threshold

### `rust` vs `tokenizer`

- prefix 0, bigrams {ru, us, st}, intersection 0 → sim **0.000** ✗

**Output:**

```
[envision] I don't know 'tokenizer'. Can you define them?
  Is 'tokenizer' related to token (0.70), tokens (0.60)?
```

The suffix-sharing distractor `organizer` was correctly rejected because the
prefix term (weight 0.4) went to zero. The 40/60 split is well chosen for
morphology: prefix catches inflection and derivation, bigrams catch the rest.

---

## 3. What this buys

This loop is the best-designed *behaviour* in the system, distinct from the best
*idea* (grounding, HOW 05).

- **It closes the loop between not knowing and learning.** The model detects a
  gap, names it, proposes a hypothesis, and asks. That is an active-learning
  agent, not a passive predictor.
- **The suggestions are useful even when wrong.** Offering `token` for
  `tokenizer` gives the user a one-word confirmation instead of asking for a full
  definition.
- **It is honest.** "I don't know X" is a claim no LLM makes reliably, and it
  comes here from a lookup that cannot hallucinate: `contains_word` is exact.
- **Composed with `learn_definition_chain` (HOW 05), it becomes autonomous.**
  Detect gap → fetch definition → train → recurse into that definition's gaps.
  That is a self-directed curriculum driven by encountered ignorance.

---

## 4. The ceiling

### (a) It is orthographic, not semantic

`string_similarity` compares **spellings**. So:

| unknown word | suggested | actually related? |
|:---|:---|:---|
| `photosynthesis` | `photograph`, `photon` | no — shared Greek root, unrelated meaning |
| `automobile` | `automatic`, `autonomy` | no |
| `car` | `card`, `care`, `cart` | no |
| `vehicle` | *(nothing above 0.5)* | the actual relative was `car` |

The model has a phase manifold precisely so that semantic relatedness is
computable, and the envision path does not consult it. This is the clearest case
in the codebase of a capability being built and then not used.

### (b) O(V) scan per unknown word

```rust
facet.lexicon.keys().map(|kw| (kw.clone(), string_similarity(word, kw)))
```

Every key is **cloned** during scoring, before filtering. For V = 100,000 and 3
unknown words in a sentence that is 300,000 string allocations per input, most of
them discarded immediately.

**Fix — filter first, clone last:**

```rust
let mut candidates: Vec<(&str, f64)> = facet.lexicon.keys()
    .filter(|kw| (kw.len() as i32 - word.len() as i32).abs() <= 3)   // cheap prefilter
    .map(|kw| (kw.as_str(), Self::string_similarity(word, kw)))
    .filter(|(_, s)| *s > 0.5)
    .collect();
```

The length prefilter alone removes ~80% of candidates before any bigram work,
and borrowing instead of cloning removes the allocations entirely.

### (c) No memory of what was asked

`detect_gaps` is stateless. The same unknown word asked about in ten consecutive
turns produces the same question ten times. There is no record of "I asked about
`tokenizer` and got no answer", so there is no way to prioritise, escalate, or
stop asking.

---

## 5. How it generalises

### (a) Use the manifold — hybrid orthographic + semantic suggestion

An unknown word has no phase, but its **context** does. The surrounding tokens in
the same sentence give you a wave, and `ray_cast` turns that wave into semantic
neighbours:

```rust
pub fn detect_gaps_v2(&self, facet: &Facet, text: &str) -> Option<Vision> {
    let tokens = Tokenizer::tokenize(text);
    let unknown: Vec<&String> = tokens.iter().filter(|t| !facet.contains_word(t)).collect();
    if unknown.is_empty() { return None; }

    // context wave from the KNOWN words of the same sentence
    let ctx = Wave::sentence(facet, &tokens);

    for word in &unknown {
        let orth: Vec<(String, f64)> = orthographic_candidates(facet, word, 5);
        let sem:  Vec<(String, f64)> = Wave::ray_cast(facet, ctx, 5).into_iter()
            .map(|(w, d)| (w, 1.0 / (1.0 + d))).collect();
        let merged = merge_by_score(orth, sem, 0.4, 0.6);   // spelling 40%, context 60%
        // ...
    }
}
```

For `"I need to fix the tokenizer before parsing"`, the context wave sits near
`parse`, `fix`, `code` — so the merged suggestion becomes *"Is 'tokenizer'
related to `token` (spelling), `parse` (context)?"*, which is a materially better
question and uses the machinery the project is built on.

### (b) Track the questions

```rust
pub struct GapLedger {
    asked:   HashMap<String, u32>,      // word → times asked
    answered: HashSet<String>,
    first_seen: HashMap<String, u64>,   // timestamp
}
```

Then: ask about the most frequently-encountered unanswered gap first; stop asking
after three unanswered attempts; and surface "here are the 10 words you use that
I still don't know" as a command. That turns a per-turn reflex into a
prioritised learning agenda — which is the difference between curiosity and a
tic.

### (c) Escalate to a source before asking the user

The pieces already exist in `src/sources/`:

```
unknown word
  ├─ 1. ChunkStore (local Webster's)       — instant, offline
  ├─ 2. wiktionary.rs / api.rs             — network
  ├─ 3. wiki_bulk.rs                       — encyclopaedic fallback
  └─ 4. ask the user                       — last resort
```

Only ask the human when all four fail. That makes the loop autonomous in the
common case and respectful of the user's attention in the rare one.

---

## 6. The bigger point

Envision is the mechanism that makes the "learns anything" claim *plausible in
principle*. A system that can (i) detect that it lacks a concept, (ii) obtain a
description of it, (iii) integrate it, and (iv) recurse into the sub-concepts it
still lacks, has the right control loop for open-ended acquisition.

What limits it today is not the loop — the loop is right. It is that step (iii),
integration, deposits the new concept into a one-dimensional space using a
collapsing update rule (HOW 01, HOW 02). The agent's *reach* is good; its
*storage* is the bottleneck.

Fix the representation, and this loop is the thing that fills it.

---

## 7. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| Similarity is 0.4 prefix + 0.6 bigram Jaccard | `Envision::string_similarity` |
| Threshold 0.5, top 5 | `filter(|(_, s)| *s > 0.5)`, `truncate(5)` |
| Suggestions never consult phases | no `lexicon.get(...)` value use in `detect_gaps` |
| Every key is cloned during scoring | `.map(|kw| (kw.clone(), ...))` |
| Runs after every input | `Model::iterate` → `Model::envision` |
| Chain learning exists and composes with it | `Trainer::learn_definition_chain` |

---

**Next:** [HOW 10 — Anti-Phase Correction](10_anti_phase_correction.md).
