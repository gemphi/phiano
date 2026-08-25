# Page 13: Phase Topology Visualization (vs TensorBoard)

## Transformer Visualization: TensorBoard

```python
# PyTorch: log metrics to TensorBoard
writer = SummaryWriter()
writer.add_scalar('loss', loss, epoch)
writer.add_histogram('attention_weights', attn, epoch)
writer.add_graph(model, input)
```

TensorBoard shows:
- Loss curves (scalar over time)
- Attention heat maps (token × token matrices)
- Weight histograms (distribution of values)
- Computation graph (static DAG of layers)

**Limitations**:
- **After the fact** - visualizes training logs, not live inference
- **Proxy metrics** - loss/accuracy don't show what the model "thinks"
- **Attention ≠ understanding** - heat maps show where the model looks, not why
- **No topology** - can't see the "shape" of the model's knowledge
- **External tool** - separate from the model itself

## Phiano Visualization: Native Phase Topology

Phiano's manifold is **inherently visualizable** - every word has a phase angle, amplitude, and frequency band that can be directly rendered:

```
┌─────────────────────────────────────────────────────────┐
│  PHASE TOPOLOGY (live)                                   │
│                                                          │
│           ●mushroom                                      │
│          / ↕ (anti-phase)                                │
│    ●spore    ●growing ──→ ●insisting                     │
│         \    ↕ (β=0.12)                                  │
│    ●dream    ●mycelium   ●existence                      │
│         \                /                               │
│          ●wave ──→ ●cresting                             │
│                                                          │
│  φ = 2.31 → 0.03 → 1.12 (novelty burst)                 │
│  momentum=0.83  coherence=0.73  novelty=0.81             │
│  R (order param) = 0.67                                  │
└─────────────────────────────────────────────────────────┘
```

**What you see**:
- **Words positioned by phase angle** on a circle (0 to 2π)
- **Edges as coupling lines** (thickness = bigram weight, color = coupling type)
- **Collective phase arrow** rotating in real-time during generation
- **Novelty bursts** when the phase jumps to a new basin
- **Training happening live** - phases shifting, edges strengthening
- **Anti-phase correction** - watch the "wrong" word get pushed π away

## Comparison

| Feature | TensorBoard | Phiano PUI |
|---------|------------|------------|
| Timing | After training | During inference (live) |
| What you see | Loss curves, attention matrices | Phase topology, word positions |
| Topology | None (flat metrics) | Full manifold (torus projection) |
| Interpretability | Proxy (loss/accuracy) | Direct (phase, amplitude, resonance) |
| Integration | External tool | Native PUI panel |
| Correction | Can't see what went wrong | See the anti-phase pulse propagate |
| Agent contributions | N/A | See which of 16 agents contributed what |
| Physical meaning | None (learned weights) | Phase synchronization (R parameter) |

## The PUI Advantage

The PUI (Phiano UI) is not just a dashboard - it's a **cognitive instrument**:

| Panel | What It Shows | Transformer Equivalent |
|-------|--------------|----------------------|
| Chat | Live conversation with phase metadata | ChatGPT (no internals visible) |
| Phase Topology | Word positions on torus, coupling edges | Nothing (TensorBoard can't do this) |
| Oscillator | 3D Kuramoto sphere with sync visualization | Nothing |
| Infinity Resonance | Multi-frequency harmonic spectrum | Nothing |
| Stats | Vocabulary, bigram counts, order parameter R | Model size, parameter count |
| Learn | Live training with progress | Training logs (after the fact) |
| Eval | Coherence/novelty/resonance scores | Loss/accuracy |
| Dictionary | Word definitions + phase assignments | Token embeddings (opaque) |
| Docs | Interactive comparison with PyTorch | N/A |

The transformer's internals are **black boxes** - you can only see inputs and outputs. Phiano's internals are **glass boxes** - every phase, every coupling, every agent contribution is visible and meaningful.
