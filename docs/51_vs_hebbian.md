# Page 6: Hebbian Wave Plasticity (vs Backpropagation)

## Transformer Training: Backpropagation

```python
# PyTorch: gradient descent through the computation graph
loss = cross_entropy(logits, targets)
loss.backward()  # autograd: chain rule through all layers
optimizer.step()  # update weights: w -= lr * gradient
optimizer.zero_grad()
```

**Properties**:
- Requires **labeled data** (input → target pairs)
- **Batch training** - gradients averaged over mini-batches
- **Global signal** - loss propagates through entire network
- **Catastrophic forgetting** - new training overwrites old weights
- **Expensive** - requires GPU clusters, hours/days
- **Offline** - can't learn during inference

## Phiano Training: Hebbian Wave Plasticity

```rust
// Phiano: phase relaxation + Hebbian coupling
pub fn train_definition(&self, facet: &mut Facet, word: &str, def: &str) {
    let tokens = Tokenizer::tokenize(def);
    for token in &tokens {
        let target_phase = facet.get_or_init(token).phase;
        // Kuramoto relaxation: move word's phase toward definition tokens
        let diff = (target_phase - word_phase).sin();
        word_phasor.phase += LEARNING_RATE * diff;
        // Amplitude growth: familiarity increases
        word_phasor.amplitude = (word_phasor.amplitude + AMPLITUDE_INCREMENT).min(AMPLITUDE_MAX);
        // Record directional syntax lag
        facet.record_phase_lag(prev_token, token);
    }
}
```

**Properties**:
- **No labeled data needed** - learns from definitions, text, dialogue
- **Online training** - one example at a time, instant
- **Local signal** - each word's phase shifts toward its context
- **Zero forgetting** - new words add new phasors, old ones unchanged
- **Cheap** - CPU, milliseconds
- **Live** - learns during inference (every chat message trains the model)

## Comparison

| Feature | Backpropagation | Hebbian Plasticity |
|---------|----------------|-------------------|
| Data requirement | Labeled (input → target) | Unlabeled (text, definitions) |
| Signal type | Global loss gradient | Local phase difference |
| Batch size | Mini-batch (32-4096) | Single example |
| Forgetting | Catastrophic | None (additive) |
| Cost | GPU, hours/days | CPU, milliseconds |
| Inference learning | No | Yes (always on) |
| Optimization | Gradient descent | Phase relaxation (Kuramoto) |
| Memory | Stores all gradients | Updates phase in-place |
| Convergence | Loss plateau | Phase synchronization (R → 1) |

## The Order Parameter R

Instead of loss curves, Phiano monitors the **Kuramoto order parameter**:

$$R = \left|\frac{1}{N}\sum_{j} e^{i\theta_j}\right|$$

- R → 1: all phases synchronized (coherent understanding)
- R → 0: phases dispersed (incoherent/novel input)

This is a **physically meaningful** metric - it measures actual synchronization, not a proxy loss function. The transformer has no equivalent.
