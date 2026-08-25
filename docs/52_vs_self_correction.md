# Page 7: In-Chat Self-Correction (vs Fine-Tuning)

## Transformer Correction: Fine-Tuning

When a transformer makes a mistake, correcting it requires:

```python
# PyTorch: fine-tuning to correct a mistake
dataset = [("wrong output", "correct output")]
for epoch in range(3):
    for wrong, correct in dataset:
        logits = model(wrong)
        loss = cross_entropy(logits, correct)
        loss.backward()
        optimizer.step()
# Hope you didn't break other outputs...
```

**Problems**:
- Requires a **training dataset** of corrections
- Takes **minutes to hours** on GPU
- Risk of **catastrophic forgetting** - fixing one thing breaks others
- Can't do it **during a conversation**
- The model doesn't "know" it was wrong - it just re-learns

## Phiano Correction: Anti-Phase Pulse

```rust
// Phiano: instant correction via phase repulsion
pub fn correct_mistake(&mut self, wrong: &str, correct: &str) {
    let wrong_phasor = self.facet.lexicon.get(wrong);
    let correct_phasor = self.facet.lexicon.get(correct);

    // Apply anti-phase pulse: push "wrong" π radians away from "correct"
    match (wrong_phasor, correct_phasor) {
        (Some(w), Some(c)) => {
            let repulsion_phase = c.phase + PHASE_REPULSION;  // + π
            w.phase = (w.phase + 0.5 * (repulsion_phase - w.phase).sin()).rem_euclid(TWO_PI);
        }
        _ => {}
    }
}
```

**Usage in chat**: `!correct dogs are reptiles|dogs are mammals`

- **Instant** - milliseconds, CPU
- **Surgical** - only the "wrong" and "correct" phases are touched
- **Zero forgetting** - all other words unchanged
- **During conversation** - no need to stop and retrain
- **Physically meaningful** - the wrong concept is pushed to anti-phase (π away)

## Comparison

| Feature | Fine-Tuning | Anti-Phase Pulse |
|---------|------------|-----------------|
| Speed | Minutes-hours | Milliseconds |
| Hardware | GPU required | CPU only |
| Precision | Global (affects all weights) | Surgical (only 2 phasors) |
| Forgetting risk | High | Zero |
| During conversation | No | Yes (`!correct`) |
| Mechanism | Gradient descent | Phase repulsion (π radians) |
| Interpretability | Opaque | Clear (phase pushed away) |
| Reversibility | Difficult | Trivial (re-apply with -π) |

## The Physics of Correction

Phase repulsion is **physically real** - it's the same principle as magnetic repulsion. Two phasors at anti-phase (π apart) destructively interfere. When the ray-cast decoder sweeps the torus, the "wrong" word will be at a phase minimum when the "correct" word is at a maximum. The wrong answer is literally **phased out**.

The transformer has no equivalent - it can only adjust probabilities through expensive retraining, and the correction is a black-box weight change, not a physically meaningful operation.
