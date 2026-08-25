# Page 2: The Phase Manifold Alternative - C^32 Torus Topology

## Transformer Embedding Space

Transformers embed tokens into R^d (typically d=512, 768, 4096):

```python
# PyTorch: token → dense vector
embedding = nn.Embedding(vocab_size, d_model)
x = embedding(token_ids)  # shape: (batch, seq, d_model)
```

- Vectors live in **flat Euclidean space**
- Distance = cosine similarity (dot product)
- No native topology - all words exist in an unstructured blob
- Position injected via RoPE (rotation matrix bolted on)

## Phiano's Phase Manifold

Each word is a `SpectralPhasor` on a 32-dimensional complex torus T^32:

```rust
pub struct SpectralPhasor {
    pub phase: f64,      // θ ∈ [0, 2π) - angular position
    pub amplitude: f64,  // r ∈ [0, 2.0] - salience/familiarity
    pub band_n: u32,     // harmonic frequency band
}
```

The torus has **native topology**:
- **Phase distance** = angular difference (mod 2π) - synonyms cluster
- **Amplitude** = familiarity - frequently used words are brighter
- **Frequency band** = semantic depth - concrete vs abstract
- **Position is intrinsic** - no external encoding needed

## Topological Comparison

| Property | Transformer (R^d) | Phiano (C^32 torus) |
|----------|-------------------|---------------------|
| Space | Flat, unstructured | Toroidal, structured |
| Distance | Cosine similarity | Phase difference (mod 2π) |
| Position | External (RoPE) | Intrinsic (phase angle) |
| Salience | Learned norm | Amplitude (updated per use) |
| Semantic depth | Hidden in dimensions | Frequency band (explicit) |
| Interference | Attention weights | Wave superposition |
| Periodicity | None | Natural (2π wrapping) |
| Visualization | Requires t-SNE/UMAP | Direct (phase circle) |

## Why a Torus?

The torus T^D = S^1 × S^1 × ... × S^1 (D times) has:
- **Compact topology** - no infinity, all points bounded
- **Natural periodicity** - phase wraps at 2π (recursion!)
- **Multi-frequency structure** - each S^1 is a harmonic band
- **Group structure** - phase addition = translation (easy composition)

The transformer's R^d has none of these. It's a flat space where position must be artificially injected and periodicity doesn't exist.

## The Golden Ratio Seeding

Phiano initializes word phases using the golden angle (2π/φ²):

```rust
let phase = (word.len() as f64 * GOLDEN_ANGLE).rem_euclid(TWO_PI);
```

This produces the **most uniform distribution** of points on the circle (sunflower spiral), ensuring maximum initial separation between words. The transformer's random initialization has no such guarantee.
