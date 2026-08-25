# Page 12: Continuous Online Learning (vs Gradient Descent)

## Transformer Learning: Gradient Descent

```
1. Collect dataset (millions of examples)
2. Initialize model (random weights)
3. For epoch in 1..N:
   For batch in dataset:
     logits = model(batch)
     loss = cross_entropy(logits, targets)
     loss.backward()  # chain rule through all layers
     optimizer.step()  # w -= lr * gradient
4. Deploy (frozen model, no more learning)
```

**Properties**:
- **Batch** - learns from groups of examples
- **Offline** - training and inference are separate phases
- **Expensive** - GPU clusters, days to months
- **Frozen** - model doesn't learn after deployment
- **Forgetting** - new training can overwrite old knowledge

## Phiano Learning: Continuous Online Plasticity

```rust
// Every API call trains the model - no separate training phase
pub fn chat(state: SharedModel, prompt: String) -> String {
    let mut model = state.lock();
    // 1. Learn from the prompt (new words, new bigrams)
    model.trainer.train_definition(&mut model.facet, &prompt);
    // 2. Generate response (also learns during generation)
    let response = model.generator.generate(&model.facet, &mut model.ctx, &prompt);
    // 3. Learn from the response (self-training)
    model.trainer.train_definition(&mut model.facet, &response);
    // 4. Record in memory
    model.memo.record((wave.re, wave.im), &prompt);
    response
}
```

**Properties**:
- **Single example** - learns from one input at a time
- **Online** - training and inference are the same thing
- **Cheap** - CPU, milliseconds per example
- **Live** - model learns continuously, even during chat
- **Additive** - new knowledge adds new phasors, old ones preserved

## Comparison

| Feature | Gradient Descent | Online Plasticity |
|---------|-----------------|-------------------|
| Data | Batch (millions) | Single example |
| Phase | Offline (train then deploy) | Online (train = inference) |
| Cost | GPU, days-months | CPU, milliseconds |
| After deployment | Frozen | Continuously learning |
| Forgetting | Catastrophic | None (additive) |
| Signal | Global loss gradient | Local phase difference |
| Optimization | Adam/SGD | Kuramoto relaxation |
| Convergence | Loss → plateau | R → 1 (synchronization) |
| Human analogy | Studying for exams | Learning by conversation |

## The Human Analogy

- **Transformer training** = studying for an exam: intense, batch, then done
- **Phiano learning** = having a conversation: each exchange teaches something, naturally, continuously

Humans don't need to "retrain" to learn a new fact - someone tells you, and you know it. Phiano works the same way. The transformer requires a full training run to incorporate new knowledge.

## The Four Pillars of Continuous Learning

| Pillar | Mechanism | API |
|--------|-----------|-----|
| Multi-Frequency Torus | 32-harmonic resonance decoding | `/api/generate` |
| Asymmetric Syntax Coupling | Directional β_ij phase lags | `/api/chat` |
| Dialog Ingestion | Multi-turn conversation training | `/api/dialogue/learn` |
| In-Chat Self-Correction | Anti-phase pulse (π repulsion) | `!correct wrong\|correct` |

All four operate **during inference** - no separate training phase. The model is always learning, always adapting, always correcting.
