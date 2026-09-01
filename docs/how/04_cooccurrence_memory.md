# HOW 04 — Co-occurrence Memory

> _The phase manifold gets the documentation. The HashMap does the work. This
> document is about telling the two apart._

---

## 1. The mechanism

Alongside the phasor lexicon, `Facet` keeps two count tables:

```rust
pub bigrams:  HashMap<String, HashMap<String, u32>>,             // a → {b: count}
pub trigrams: HashMap<String, HashMap<String, u32>>,             // "a b" → {c: count}
```

populated on every training pass:

```rust
// src/trainer/mod.rs :: train_sentence
for window in tokens.windows(2) { facet.record_bigram(&window[0], &window[1]); }
for window in tokens.windows(3) { facet.record_trigram(&w[0], &w[1], &w[2]); }
```

and read back as maximum-likelihood transition probabilities:

$$P(b \mid a) = \frac{C(a,b)}{\sum_{b'} C(a,b')} \qquad P(c \mid a,b) = \frac{C(a,b,c)}{\sum_{c'} C(a,b,c')}$$

**In the source:** `src/facet/mod.rs` — `record_bigram`, `record_trigram`,
`bigram_probability`, `trigram_probability`, `next_word_candidates`,
`trigram_candidates`.

This is a **classical n-gram language model**. Not an analogy — the same
data structure and the same estimator as a 1990s trigram model.

---

## 2. Worked example — counts and probabilities

Train on:

```
the cat sat on the mat
the cat ran on the road
the dog sat on the mat
```

### Bigram table for `the`

| follower | count |
|:--|--:|
| `cat` | 2 |
| `dog` | 1 |
| `mat` | 2 |
| `road` | 1 |

total = 6

$$P(\texttt{cat} \mid \texttt{the}) = 2/6 = 0.333, \quad P(\texttt{mat}\mid\texttt{the}) = 0.333, \quad P(\texttt{dog}\mid\texttt{the}) = 0.167$$

### Trigram table for `the cat`

| follower | count |
|:--|--:|
| `sat` | 1 |
| `ran` | 1 |

$$P(\texttt{sat} \mid \texttt{the cat}) = 0.5$$

### The dedup guards

```rust
// record_bigram
match word_a == word_b { true => return, false => {} }
// record_trigram
match word_a == word_c || word_b == word_c { true => return, false => {} }
```

Self-transitions are dropped. This prevents `the the the` degeneracy in
generation, which is a sensible engineering choice — but note it makes the
estimator **not** a maximum-likelihood estimator of the corpus, so any perplexity
computed from these tables is measuring a slightly different distribution than
the text. Worth knowing before quoting numbers.

---

## 3. Where fluency actually comes from

Trace a generation step (`src/generate.rs :: decode`) and the ordering is:

1. `trigram_candidates(prev, last)` — if non-empty, the candidate pool is the
   trigram followers.
2. else `next_word_candidates(last)` — bigram followers.
3. else fall back to `Wave::ray_cast` over the whole lexicon.
4. the candidate pool is then **re-ranked** by phase proximity, momentum, and
   repetition penalties.

So the phase layer is a **re-ranker over an n-gram candidate set**. When the
n-gram tables have coverage, they determine what is grammatical; the manifold
chooses among grammatical options. When coverage runs out, the manifold picks
alone — and step 3 is where output quality visibly degrades.

This is not a criticism of the design. Hybrid count-plus-continuous models are a
legitimate and often strong architecture. It is a criticism of the *attribution*:
sentences that look fluent are evidence about the HashMap, not about the phase
manifold, and the current benchmarks do not separate the two.

### The experiment that separates them

```rust
// tests/ablation.rs
#[test]
fn phase_layer_contributes() {
    let facet = load_trained_facet();

    let with_phase    = decode_with_phase_reranking(&facet, prompts);
    let without_phase = decode_ngram_only(&facet, prompts);       // step 4 disabled
    let phase_only    = decode_phase_only(&facet, prompts);       // steps 1-2 disabled

    // report held-out perplexity for all three
}
```

If `with_phase` does not beat `without_phase` on held-out text, the manifold is
decoration. If it does, you have a publishable result and a number to quote. Right
now neither is known, and that is the most important open question about the
system.

---

## 4. The size problem

`data/manifold.chroma` is **92 MB**.

The README advertises 2 MB / 5 MB / 12 MB for Phinum16/32/64. The gap is the
n-gram tables. A `HashMap<String, HashMap<String, u32>>` stores every key as an
owned `String`, so a bigram entry costs roughly:

- outer key `String`: 24 bytes header + heap bytes
- inner `HashMap` allocation: ~48 bytes minimum
- per follower: 24-byte `String` header + heap bytes + 4-byte count + hashing slack

Trigram keys are worse, because `format!("{} {}", a, b)` **allocates a fresh
joined string per lookup as well as per insert**:

```rust
// src/facet/mod.rs — this allocates on every single query
let key = format!("{} {}", word_a, word_b);
```

In a decode loop that is one heap allocation per generated token per candidate.

### How it generalises — interned IDs

```rust
pub struct Vocab { ids: HashMap<String, u32>, words: Vec<String> }

pub bigrams:  HashMap<u32, Vec<(u32, u32)>>,          // sorted, binary-searched
pub trigrams: HashMap<(u32, u32), Vec<(u32, u32)>>,   // tuple key, zero alloc
```

- Bigram entry: 8 bytes instead of ~100.
- Trigram lookup: zero allocations instead of one `String` per call.
- Expected footprint: **92 MB → under 10 MB** for the same information, which
  brings the artifact back in line with the README's claim.

Add count pruning (`drop entries with count == 1` after ingestion) and it drops
again by roughly half, because singleton n-grams are the majority of the table and
contribute almost nothing to a smoothed estimate.

---

## 5. The smoothing gap

The current estimator is raw maximum likelihood:

```rust
followers.get(word_b).map(|c| *c as f64 / total as f64).unwrap_or(0.0)
```

An unseen bigram gets probability **exactly 0.0**, which makes perplexity
infinite and makes the model brittle on anything out of distribution. This is the
problem that thirty years of language-modelling research is about, and the answer
is known.

**Kneser–Ney** is the standard, and it is ~30 lines:

$$P_{KN}(w_i \mid w_{i-1}) = \frac{\max(C(w_{i-1},w_i) - D,\ 0)}{C(w_{i-1})} + \lambda(w_{i-1})\, P_{\text{cont}}(w_i)$$

$$P_{\text{cont}}(w_i) = \frac{|\{w' : C(w', w_i) > 0\}|}{|\{(w',w'') : C(w',w'') > 0\}|}$$

with discount D ≈ 0.75 and $\lambda(w_{i-1}) = \frac{D}{C(w_{i-1})}\cdot|\{w : C(w_{i-1},w) > 0\}|$.

The continuation probability is the clever part: it scores a word by *how many
distinct contexts it appears in*, not how often it appears. `Francisco` is
frequent but appears only after `San`, so KN correctly refuses to predict it
elsewhere. Raw MLE gets this wrong.

Adding KN does two things at once: it makes generation robust off-distribution,
and it gives you the **baseline you must beat** (HOW 15). A phase manifold that
does not outperform Kneser–Ney trigrams on held-out text has not yet earned its
place in the pipeline.

---

## 6. What this buys

- **Genuine sequential coverage.** Bigrams + trigrams give real local grammar
  from the first sentence, with no training run.
- **Instant, exact, inspectable updates.** `record_bigram` is O(1) and its effect
  on generation is immediate and traceable to a specific count.
- **Perfect memorisation of seen sequences** — which is a strength for a system
  whose pitch is on-device personal learning: it will reproduce your phrasings
  exactly.
- The **bootstrap path** (`Model::bootstrap_bigrams`) rebuilds transitions from
  the dictionary without retraining phases, which is a nice separation of the two
  memories.

---

## 7. The ceiling

An n-gram model's ceiling is fixed and known: it cannot represent any dependency
longer than n−1 tokens. `the keys to the cabinet ___` cannot agree with `keys`
through a trigram window. No amount of phase re-ranking fixes this, because the
candidate set is already wrong by the time re-ranking runs.

The manifold *could* supply long-range information — `ContextWaveBuffer` is
exactly the right place for it — but as HOW 03 showed, that buffer is an
order-blind sum, so its contribution to disambiguating a long dependency is
close to nil.

---

## 8. How it generalises

1. **Intern the vocabulary** (§4) — 10× size reduction, removes per-token
   allocations from the decode loop.
2. **Kneser–Ney smoothing** (§5) — no zero probabilities, and gives you the
   baseline number.
3. **Prune singletons** after bulk ingestion — halves the table, negligible
   quality cost.
4. **Make the manifold contribute long-range structure** by replacing the summed
   context buffer with the recurrent complex state of HOW 11:
   $h_t = \lambda R\, h_{t-1} + z_t$. That gives the re-ranker information the
   trigram table structurally cannot have, which is the only way the hybrid beats
   its own baseline.
5. **Report the ablation** (§3). Until it exists, the contribution of the phase
   layer to fluency is unmeasured.

---

## 9. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| Bigram/trigram tables are MLE n-grams | `bigram_probability`, `trigram_probability` |
| Trigram key allocates per lookup | `format!("{} {}", ...)` in `trigram_candidates` |
| Generation prefers trigram → bigram → ray-cast | `src/generate.rs :: decode` |
| Unseen n-grams get probability 0.0 | `.unwrap_or(0.0)` in `bigram_probability` |
| Model file is 92 MB | `ls -la data/manifold.chroma` |
| Self-transitions are dropped | guards in `record_bigram` / `record_trigram` |

---

**Next:** [HOW 05 — Definition Grounding](05_definition_grounding.md) — the best
idea in the codebase.
