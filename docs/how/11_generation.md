# HOW 11 — Generation

> _How one token is chosen. A phase target is advanced, an n-gram pool is
> retrieved, and the pool is re-ranked by resonance. Then it is done again._

---

## 1. The loop

```rust
// src/generate.rs :: Generator::decode  (simplified, faithful to the order)
for step in 0..max_tokens {
    let jitter     = (step as f64 * PHI_CONJUGATE).sin() * temperature * 0.08;
    let flow_bias  = 0.45 * (flow.collective_phase - current_phase).sin();
    let target     = (current_phase + phase_momentum + jitter + flow_bias).rem_euclid(TWO_PI);

    let next = attractor_select(facet, &flow, &prev_word, &last_word, &recent, target)?;

    // stop conditions
    if is_function_word(&next) { streak += 1; if streak >= 4 { break; } } else { streak = 0; }
    recent.insert(next.clone());
    evict_old(&mut recent, &generated, 12);
    apply_phase_kick(facet, &last_word, &next, &mut current_phase, &mut phase_momentum);
    generated.push(next);
    if generated.len() >= 20 { break; }
}
```

The decode loop is a **phase walk**: a point moves around the circle with
momentum, and at each step the model emits whichever admissible word sits nearest
where the point has arrived.

---

## 2. Where the target comes from

Three terms, in order of magnitude:

| term | value | role |
|:---|:---|:---|
| `phase_momentum` | starts at `SYNTACTIC_MOMENTUM_DEFAULT` = 0.15 | forward drift — the "syntactic velocity" |
| `flow_bias` | `0.45·sin(flow.collective_phase − current)` | pull toward the prompt's collective phase |
| `jitter` | `sin(step·(1/φ))·temperature·0.08` | deterministic pseudo-randomness |

### The jitter is not random

```rust
let jitter = (step as f64 * PHI_CONJUGATE).sin() * self.temperature * 0.08;
```

It is a **deterministic function of the step index alone**. The same prompt with
the same facet and the same temperature produces the same output every time.
Temperature scales the amplitude of a fixed sinusoid; it does not introduce
entropy.

That is a defensible choice — reproducible generation is genuinely useful — but
it should be documented as *determinism*, not sampling. If real sampling is
wanted:

```rust
// softmax over the candidate scores, temperature as a real temperature
let probs: Vec<f64> = scores.iter().map(|s| (s / self.temperature).exp()).collect();
let total: f64 = probs.iter().sum();
let mut r = rng.gen::<f64>() * total;
```

### Worked example — the first three targets

Prompt `"rust memory"`, `current_phase` = context phase = 1.05,
`flow.collective_phase` = 1.20, momentum = 0.15, temperature = 0.8.

| step | jitter | flow_bias | target |
|---:|--:|--:|--:|
| 0 | sin(0)·0.064 = 0.0000 | 0.45·sin(0.15) = 0.06718 | 1.05 + 0.15 + 0 + 0.06718 = **1.26718** |
| 1 | sin(0.618)·0.064 = 0.03707 | recomputed after kick | ~1.45 |
| 2 | sin(1.236)·0.064 = 0.06046 | ... | ~1.62 |

The point advances roughly 0.17–0.20 rad per token — about two sectors — which is
a sensible traversal rate: fast enough to move through the manifold in a
20-token utterance, slow enough that consecutive words stay related.

---

## 3. Candidate selection — the real hierarchy

```rust
// attractor_select
trigram_candidates(prev, last)   → top 12  → pick_ngram
  ↓ (empty)
next_word_candidates(last)       → top 16  → pick_ngram
  ↓ (empty)
torus_ray_cast(target_phase)     → pool of TORUS_DECODE_POOL × 4 = 192
```

So the grammar comes from n-grams, and the manifold picks within the pool. This
is the point made in HOW 04 §3, now visible in the control flow.

### The scoring function

```rust
let capped     = (count as f64).min(24.0).ln_1p();                    // frequency, log-damped, capped
let phase_align= (p.phase - target_phase).cos().max(0.0);             // manifold agreement
let resonance  = flow.resonance_with(facet, word);                    // PhaseFlow agreement
let content    = if is_function_word(word) { 0.55 } else { 1.35 };    // content-word boost
let score      = capped * (0.35 + 0.25*phase_align + 0.40*resonance) * content;
```

This is well-constructed and worth reading closely:

- **`min(24.0).ln_1p()`** — caps how much a very frequent n-gram can dominate, then
  log-compresses. Prevents `the` winning every slot. Good.
- **The `0.35` floor** inside the bracket means an n-gram candidate can never
  score zero from phase disagreement alone — frequency always retains a voice.
  Sensible hedging.
- **`content` multiplier 1.35 vs 0.55** — a 2.45× preference for content words. A
  blunt instrument, but it works, and combined with the 4-function-word-streak
  break it is what stops the classic n-gram failure mode of `of the of the`.
- **`.max(0.0)` on `cos`** — words more than 90° from target contribute nothing
  rather than negatively. Correct for a multiplicative score.

### Worked example — one selection

`target_phase` = 1.26718, candidates from `trigram_candidates("rust", "memory")`:

| word | count | θ | capped | phase_align | resonance | content | score |
|:--|--:|--:|--:|--:|--:|--:|--:|
| `safety` | 18 | 1.20 | ln(19)=2.944 | cos(−0.067)=0.998 | 0.85 | 1.35 | 2.944·(0.35+0.2495+0.34)·1.35 = **3.732** |
| `is` | 40 | 2.90 | ln(25)=3.219 | cos(1.633)=0.0 → max(0)=0.0 | 0.30 | 0.55 | 3.219·(0.35+0+0.12)·0.55 = **0.832** |
| `management` | 5 | 1.90 | ln(6)=1.792 | cos(0.633)=0.806 | 0.55 | 1.35 | 1.792·(0.35+0.2015+0.22)·1.35 = **1.867** |

`safety` wins — the highest-resonance, best phase-aligned content word — even
though `is` has more than twice the raw count. The scoring is doing exactly what
it was designed to do.

---

## 4. The filters

### `speakable`

```rust
n >= 2 && n <= 16 && word.chars().all(|c| c.is_ascii_alphabetic()) && !boilerplate(word)
```

Excludes single characters, very long tokens, anything non-alphabetic — and
therefore **all numbers**. `speakable("42")` is `false`. The model cannot emit a
numeral. For a general-purpose learner that is a hard functional gap, not a
stylistic filter.

### `boilerplate`

A hardcoded list of ~45 dictionary artefacts: `pertaining`, `genus`, `obsolete`,
`webster`, `shak`, `milton`, `edifieth`, `bloodguiltiness`…

This is honest about what it is — a cleanup for Webster's-derived training data —
and it is effective. But it is a **symptom**: the model learned dictionary
metadata as if it were language, because ingestion did not separate the two.

**The upstream fix** belongs in `src/sources/`:

```rust
// strip part-of-speech markers, etymology brackets, and citation attributions
// BEFORE they reach the trainer, rather than blocking them at generation time
fn clean_definition(raw: &str) -> String { ... }
```

Once ingestion is clean, `boilerplate` shrinks to nothing, and — importantly — the
model stops *positioning* those words in the manifold as though they were content.
Blocking at output does not undo the damage done at training time.

Also note `"tokenizer"` is on the boilerplate list, which will surprise anyone
using this system to talk about its own implementation.

### Repetition control

```rust
recent_words: HashSet<String>       // last ~12 tokens
evict_old(&mut recent, &generated, 12);
```

A hard block, not a penalty. A word in the last 12 tokens **cannot** be re-emitted.
This eliminates loops, and it also makes it impossible to write
`the cat sat on the mat` (two `the`s within 12 tokens). A soft penalty would be
better:

```rust
let repeat_penalty = if recent.contains(word) { 0.15 } else { 1.0 };
let score = capped * (...) * content * repeat_penalty;
```

### Length caps

`max_tokens` from the constructor, then `if generated.len() >= 20 { break; }` —
a hardcoded 20-token ceiling that silently overrides any larger `max_tokens`.
Worth either removing or promoting to a named constant.

---

## 5. What this buys

- **Genuinely hybrid decoding.** A count model supplies grammaticality; a
  continuous manifold supplies topical steering. That is a real architecture, and
  the scoring function combining them is thoughtfully weighted.
- **Constant-memory context.** `ContextWaveBuffer` is two f64s and a ring buffer,
  for an unbounded window. No KV cache, no O(N²).
- **Reproducible output** — same input, same output, always. For a debuggable
  on-device system that is a feature.
- **Live trajectory instrumentation.** `PhaseFlow` records every step's
  resonance, novelty and momentum, so a generation can be replayed and plotted.
  Very few systems expose their decoding dynamics this legibly.

---

## 6. The ceiling

The generator inherits every limit upstream of it:

| inherited from | limit |
|:---|:---|
| HOW 04 | candidate pool cannot span more than 2 tokens of history |
| HOW 06 | `ContextWaveBuffer` is order-blind, so long-range steering is weak |
| HOW 01 | phase alignment discriminates at 64-sector granularity |
| HOW 02 | under collapse, `phase_align` → 1.0 for everything, and scoring reduces to `capped × content` — i.e. pure frequency |

That last row is the important one: **as the manifold collapses, the generator
degenerates into a plain n-gram sampler**, smoothly and without error. The output
still looks fine. Nothing warns you.

---

## 7. How it generalises

1. **Real sampling** (§2) — softmax over scores with genuine RNG; keep a
   `--deterministic` flag for reproducible runs.
2. **Recurrent context state** (HOW 06 §7b) — `h_t = λe^{iω}h_{t-1} + z_t` gives
   the target phase actual long-range memory, which is the only way the phase
   term can contribute information the trigram table lacks.
3. **Allow numerals** — drop `is_ascii_alphabetic` in favour of a token-class
   check, or the model can never state a quantity.
4. **Soft repetition penalty** instead of a hard block (§4).
5. **Beam search.** `src/synthesis/search.rs` already implements beam search for
   programs; the decoder is greedy. A beam of 4 over the joint score would
   measurably improve coherence at ~4× cost, and the code to copy is in-tree.
6. **Clean at ingestion, not at output** (§4) — and then delete `boilerplate`.
7. **Log degeneration.** Track the fraction of steps where `phase_align > 0.99`
   for *all* candidates; when that fraction rises, the manifold has stopped
   contributing and the ablation in HOW 04 §3 will show it.

---

## 8. Checklist for this document

| Claim | Where to verify |
|:---|:---|
| Jitter is deterministic in `step` | `(step as f64 * PHI_CONJUGATE).sin()` |
| Trigram → bigram → ray-cast order | `attractor_select` |
| Score formula and weights | `pick_ngram` |
| Function-word streak of 4 breaks generation | `decode`, `function_streak` |
| Hard 20-token cap | `if generated_tokens.len() >= 20 { break; }` |
| Numbers are unspeakable | `speakable` → `is_ascii_alphabetic` |
| `attention_pick` exists but is unused | `#[allow(dead_code)]` on it |

---

**Next:** [HOW 12 — Memory Layers](12_memory_layers.md).
