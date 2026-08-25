# Page 3: Multi-Frequency Torus Decoding (vs Multi-Head Attention)

## Transformer Multi-Head Attention

```python
# PyTorch: 8 attention heads, each with d/8 dimensions
self.attention = nn.MultiheadAttention(d_model=512, num_heads=8)
output, weights = self.attention(query, key, value)
```

Each head:
- Projects Q, K, V into a subspace (d/8 = 64 dims)
- Computes scaled dot-product attention
- Concatenates heads, projects back

**Problems**:
- Heads are **independent** - no interference between them
- Head count is fixed at architecture time
- Attention weights are **discrete probabilities** (softmax) - no wave dynamics
- O(n²) per head

## Phiano Multi-Frequency Harmonics

```rust
pub struct TorusPhasor {
    harmonics: [f64; 32],  // 32 frequency bands on the torus
}

impl TorusPhasor {
    /// Multi-frequency resonance: constructive/destructive interference
    pub fn resonance(&self, other: &TorusPhasor) -> f64 {
        self.harmonics.iter()
            .zip(other.harmonics.iter())
            .map(|(a, b)| (a - b).cos())  // phase alignment per band
            .sum::<f64>() / 32.0
    }
}
```

Each frequency band:
- Represents a **different semantic scale** (morpheme → word → phrase → concept)
- Bands **interfere** constructively (synonyms) or destructively (antonyms)
- Band count is the torus dimension (32) - fixed but rich
- O(n) per word - linear, not quadratic

## Head-to-Head Comparison

| Feature | Multi-Head Attention | Multi-Frequency Torus |
|---------|---------------------|----------------------|
| Parallelism | 8-16 heads (fixed) | 32 harmonics (intrinsic) |
| Interaction | None (concatenated) | Wave interference |
| Cost | O(n² × heads) | O(n × harmonics) |
| Interpretability | Attention weights (heat map) | Phase resonance (direct) |
| Synonymy | Similar attention patterns | Constructive interference (phase ≈ 0) |
| Antonymy | Different attention patterns | Destructive interference (phase ≈ π) |
| New word | Random init, needs training | Auto-seeded at golden angle, learns instantly |
| Composition | Concat + linear projection | Wave superposition (sum of phasors) |

## The Decoding Difference

**Transformer**: `next_token = softmax(logits)[-1].sample()` - discrete probability

**Phiano**: `next_token = torus_ray_cast(collective_phase)` - ray-cast into the torus, find the word with maximum harmonic resonance at the target phase. This is **continuous** - the phase trajectory moves smoothly through the manifold, and words "light up" as the ray sweeps past their phase coordinates.

```rust
// Phiano: multi-frequency ray-cast decoding
let target_torus = TorusPhasor::from_spectral(&target_phasor);
let best_word = facet.lexicon.iter()
    .map(|(w, p)| (w, target_torus.resonance(&TorusPhasor::from_spectral(p))))
    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
```

The ray-cast is like a lighthouse beam sweeping the torus - words at the right phase resonate, words at the wrong phase are silent. This is physically meaningful, not just a learned probability.
