# Page 5: Riverflow - Phiano's Forward Pass (vs PyTorch's forward())

## PyTorch's Forward Pass

```python
# PyTorch: the computation graph is defined by the forward() method
class TransformerBlock(nn.Module):
    def forward(self, x):
        # 1. Self-attention (O(n²))
        attn = self.attention(x, x, x)
        # 2. Residual + LayerNorm
        x = self.norm1(x + attn)
        # 3. Feed-forward
        x = self.norm2(x + self.ffn(x))
        return x

# The graph is BUILT during forward(), then DISCARDED after backward()
# Parameters persist in self.weight, self.bias, etc.
```

**Key properties**:
- Dynamic graph: rebuilt every forward pass
- Parameters persist in `nn.Module` attributes
- Backward pass: autograd traverses the graph in reverse
- The graph is **invisible** during inference - you can't see the data flowing

## Phiano's Riverflow

The riverflow is Phiano's equivalent: a **dynamic phase propagation** through the manifold.

```rust
// Phiano: the "forward pass" is phase wave propagation
pub fn generate(&self, facet: &Facet, ctx: &mut ContextWaveBuffer, prompt: &str) -> String {
    ctx.push_turn(facet, prompt);  // absorb prompt into context wave

    let mut current_phase = ctx.context_phase();    // starting phase
    let mut momentum = SYNTACTIC_MOMENTUM_DEFAULT;   // initial velocity

    for step in 0..max_tokens {
        // 1. Compute target phase (current + momentum + jitter)
        let target = (current_phase + momentum + jitter).rem_euclid(TWO_PI);

        // 2. Ray-cast into torus to find resonant word
        let word = torus_ray_cast(facet, target);

        // 3. Apply phase kick (Hebbian + syntax lag)
        let beta = facet.phase_lag(prev, word);
        current_phase += 0.35 * (word.phase - current_phase + beta).sin();

        // 4. Update momentum (acceleration from phase difference)
        momentum = 0.85 * momentum + 0.15 * phase_diff.abs();

        // 5. Hebbian update - LEARN during generation
        facet.adjust_phase(word, current_phase);
    }
}
```

**Key properties**:
- Dynamic topology: phase trajectory emerges from coupling
- Parameters persist in `Facet` (the "model")
- Backward pass: Hebbian plasticity (phase shifts = gradients)
- The flow is **visible** - every word's phase, amplitude, and coupling is inspectable

## Side-by-Side

| Aspect | PyTorch forward() | Phiano Riverflow |
|--------|-------------------|------------------|
| Graph type | Dynamic DAG of tensor ops | Dynamic trajectory on torus |
| Parameters | nn.Module weights | Facet phasors + phase lags |
| Data flow | Tensors through layers | Phase waves through manifold |
| Cost | O(n² × layers) | O(n × vocab) per token |
| Learning at inference | No | Yes (Hebbian per token) |
| Momentum | Optimizer momentum (training only) | Phase momentum (always on) |
| Visualization | Requires TensorBoard | Native (phase trajectory) |
| Recursion | None (linear sequence) | Natural (2π wrapping) |
| Interpretability | Attention weights | Phase resonance scores |

## The Context Wave Buffer = KV Cache (But Better)

Transformers use a KV cache to avoid recomputing attention on past tokens. Phiano uses a **ContextWaveBuffer** - a running superposition wave:

```rust
pub struct ContextWaveBuffer {
    sum_x: f64,  // running cosine sum
    sum_y: f64,  // running sine sum
    tokens: VecDeque<String>,
}

// Each new turn DECAYS old context and ADDS new
fn push_turn(&mut self, facet: &Facet, text: &str) {
    self.sum_x *= 0.5;  // exponential decay
    self.sum_y *= 0.5;
    for token in tokens {
        self.sum_x += phasor.amplitude * phasor.phase.cos();
        self.sum_y += phasor.amplitude * phasor.phase.sin();
    }
}
```

- **KV cache**: stores all past K,V tensors (O(n × d) memory)
- **Context wave**: stores a single complex number (O(1) memory)
- KV cache has **no decay** - all context is equally weighted
- Context wave has **exponential decay** - recent context dominates, old fades
- KV cache is **additive only** - can't forget
- Context wave **naturally forgets** - decay handles it automatically
