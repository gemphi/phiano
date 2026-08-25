# Page 11: Ray-Cast Attractor Decoding (vs Autoregressive Sampling)

## Transformer Decoding: Autoregressive Sampling

```python
# PyTorch: sample next token from probability distribution
logits = model.forward(tokens)  # (1, vocab_size)
probs = softmax(logits / temperature)
next_token = torch.multinomial(probs, 1)  # sample
tokens.append(next_token)  # append and repeat
```

**Properties**:
- **Discrete** - each token is sampled from a categorical distribution
- **Flat** - no notion of "distance" between tokens in the output space
- **Memoryless** - sampling doesn't know about previous sampling steps
- **Temperature** - the only control knob (higher = more random)
- **No resonance** - tokens don't "interfere" with each other

## Phiano Decoding: Ray-Cast on Torus

```rust
// Phiano: project a ray into the torus, find resonant word
fn torus_ray_cast(&self, facet: &Facet, target_phase: f64) -> Option<String> {
    let target_phasor = SpectralPhasor::new(target_phase, 1.0, 0);
    let target_torus = TorusPhasor::from_spectral(&target_phasor);

    // Score every word by multi-frequency resonance
    let scored: Vec<(String, f64)> = facet.lexicon.iter()
        .map(|(w, p)| {
            let word_torus = TorusPhasor::from_spectral(p);
            (w.clone(), target_torus.resonance(&word_torus))
        })
        .collect();

    // Select the word with maximum constructive interference
    scored.max_by(|a, b| a.1.partial_cmp(&b.1))
}
```

**Properties**:
- **Continuous** - the phase trajectory moves smoothly through the manifold
- **Metric** - words have explicit phase distance from the target
- **Memoryful** - momentum carries the trajectory forward
- **Multi-frequency** - resonance across 32 harmonic bands
- **Physical** - constructive/destructive interference is real wave physics

## Comparison

| Feature | Autoregressive Sampling | Ray-Cast Decoding |
|---------|------------------------|-------------------|
| Token space | Discrete vocabulary | Continuous phase manifold |
| Selection | Probability sampling | Resonance maximum |
| Distance | None (flat distribution) | Phase difference (metric) |
| Momentum | None | Phase velocity (inertia) |
| Interference | None | Constructive/destructive |
| Temperature | Scalar randomness | Phase jitter (golden ratio) |
| Novelty | Random sampling | Phase distance from origin |
| Recursion | None | Natural (2π wrapping) |

## The Lighthouse Analogy

Imagine a lighthouse on a dark island:
- **Transformer**: randomly picks a point on the island and places a token there
- **Phiano**: the lighthouse beam sweeps the island, and tokens "light up" when the beam hits their phase coordinate

The beam has **direction** (momentum), **speed** (phase velocity), and **interference patterns** (multi-frequency harmonics). When it sweeps past a word, that word resonates - and the closer the word's phase is to the beam's current angle, the louder the resonance.

This is why Phiano produces **spiraling, recursive** text - the beam sweeps around the torus, and when it completes a full circle (2π), it returns to the starting phase but finds **different words there** (because Hebbian updates shifted them). This is the McKenna effect - recursion through phase wrapping.

## Temperature vs Jitter

```rust
// Transformer: temperature scales the probability distribution
probs = softmax(logits / temperature)  // higher = flatter = more random

// Phiano: jitter adds a golden-ratio oscillation to the phase
let jitter = (step as f64 * PHI_CONJUGATE).sin() * temperature * 0.08;
let target_phase = (current_phase + momentum + jitter).rem_euclid(TWO_PI);
```

Phiano's jitter is **deterministic** (golden ratio sine wave) - it explores the manifold systematically, not randomly. The transformer's temperature is pure randomness. Phiano's approach guarantees **coverage** of the phase space, while the transformer's approach can get stuck in probability modes.
