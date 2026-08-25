# Page 4: Asymmetric Syntax Coupling (vs Positional Encoding)

## Transformer Positional Encoding

Transformers have **no native word order** - attention is permutation-invariant. Position must be injected:

```python
# PyTorch: RoPE (Rotary Position Embedding)
def apply_rope(x, pos):
    # Rotate each head's dimension by position angle
    angle = pos / (10000 ** (2i / d))
    x_rotated = x * cos(angle) + rotate90(x) * sin(angle)
    return x_rotated
```

**Problems**:
- Position is **external** - bolted onto the representation
- RoPE rotation is **symmetric** - position 5→6 is the same as 6→5
- No **directional syntax** - "dog bites man" vs "man bites dog" differ only in embedding
- Fixed positional function - can't learn word-specific ordering patterns

## Phiano's Asymmetric Syntax Lag (β_ij)

Phiano learns a **directional phase lag** between word pairs:

```rust
// β_ij: the learned phase offset from word i to word j
// This is ASYMMETRIC: β(dog→bites) ≠ β(bites→dog)

impl Facet {
    pub fn record_phase_lag(&mut self, prev: &str, next: &str) {
        let lag = (self.lexicon[next].phase - self.lexicon[prev].phase)
            .rem_euclid(2.0 * PI);
        // EMA update: learn the directional lag
        self.phase_lags.entry((prev.into(), next.into()))
            .and_modify(|v| *v = (1.0 - SYNTAX_LAG_LEARN_RATE) * *v
                + SYNTAX_LAG_LEARN_RATE * lag)
            .or_insert(lag);
    }
}
```

**Advantages**:
- Position is **intrinsic** - phase angle IS the word's position
- Lag is **asymmetric** - "dog→bites" has different lag than "bites→dog"
- Lag is **learned per word pair** - not a fixed function
- Lag evolves with usage - **adaptive syntax**

## Comparison

| Feature | RoPE (Transformer) | Phase Lag (Phiano) |
|---------|-------------------|-------------------|
| Position source | External rotation | Intrinsic phase angle |
| Symmetry | Symmetric (i→j = j→i) | Asymmetric (β_ij ≠ β_ji) |
| Learning | Fixed function | Per-pair EMA (adaptive) |
| Word specificity | Same for all words | Different for each pair |
| Direction | Implicit (via absolute position) | Explicit (directional lag) |
| Update during inference | No | Yes (Hebbian per use) |
| Memory cost | O(1) per position | O(pairs) - but sparse |

## How It Drives Generation

During generation, the phase lag steers the context wave:

```rust
// The β_ij lag is applied as a phase kick
let beta = facet.phase_lag(prev_word, next_word);
let phase_diff = (next_phasor.phase - current_phase + beta).sin();
*current_phase += 0.35 * phase_diff;  // steer toward next word
```

This means the sentence "the cat sat on the mat" has a **specific phase trajectory** through the manifold, and that trajectory is **learned and refined** every time similar sentences are processed. The transformer has no equivalent - it just re-computes attention from scratch each time.
