# Page 10: Context Wave Buffer (vs KV Cache)

## Transformer KV Cache

```python
# PyTorch: cache past key-value pairs to avoid recomputation
past_kv = []
for token in generate_tokens:
    k, v = self.attention.kv_proj(token)
    past_kv.append((k, v))
    # Attention uses ALL past k,v: O(n²) per step
    attn = attention(q, past_k, past_v)
```

**Properties**:
- Stores **all past K,V tensors** - O(n × d) memory
- **No decay** - token at position 0 has same weight as token at position n
- **No forgetting** - everything is retained (context window limit)
- **Expensive** - memory grows linearly with sequence length
- **Opaque** - can't inspect what the model "remembers"

## Phiano Context Wave Buffer

```rust
pub struct ContextWaveBuffer {
    sum_x: f64,  // running cosine sum (ONE number)
    sum_y: f64,  // running sine sum (ONE number)
    tokens: VecDeque<String>,  // ring buffer
}

impl ContextWaveBuffer {
    fn push_turn(&mut self, facet: &Facet, text: &str) {
        // EXPONENTIAL DECAY: old context fades
        self.sum_x *= 0.5;
        self.sum_y *= 0.5;
        // ADD new context as wave superposition
        for token in tokens {
            self.sum_x += phasor.amplitude * phasor.phase.cos();
            self.sum_y += phasor.phase.sin();
        }
    }

    fn context_phase(&self) -> f64 {
        self.sum_y.atan2(self.sum_x).rem_euclid(2.0 * PI)
    }
}
```

**Properties**:
- Stores **one complex number** - O(1) memory
- **Exponential decay** - recent context dominates, old fades naturally
- **Natural forgetting** - decay handles it (like human memory)
- **Cheap** - constant memory, regardless of conversation length
- **Transparent** - `context_phase()` and `context_amplitude()` are directly inspectable

## Comparison

| Feature | KV Cache | Context Wave Buffer |
|---------|---------|-------------------|
| Memory | O(n × d) - grows with sequence | O(1) - constant |
| Decay | None (all equal weight) | Exponential (recent dominates) |
| Forgetting | None (until context limit) | Natural (decay-based) |
| Context limit | Fixed (4K-128K tokens) | Unlimited (ring buffer) |
| Inspectability | Opaque tensors | phase + amplitude (2 numbers) |
| Momentum | None | Phase velocity (accumulates) |
| Physical analog | Tape recorder | Wave on a pond |

## The Momentum Advantage

The context wave has **momentum** - the phase doesn't just sit at the current position, it has a velocity:

```rust
let mut phase_momentum = SYNTACTIC_MOMENTUM_DEFAULT;  // 0.15

// Each token updates momentum based on phase difference
phase_momentum = 0.85 * phase_momentum + 0.15 * phase_diff.abs();
```

This means the conversation has **inertia** - if the topic has been moving in a certain direction (phase increasing), it tends to continue. The KV cache has no equivalent - it's a static memory, not a dynamic wave with momentum.

## The Puddle vs the Tape Recorder

Think of it this way:
- **KV cache** = tape recorder: stores everything, plays back exactly, no fading
- **Context wave** = ripple in a pond: recent ripples are strong, old ones fade, and the water has momentum

Human memory works like the pond, not the tape recorder. Phiano's context buffer is cognitively plausible.
