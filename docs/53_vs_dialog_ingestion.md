# Page 8: Conversational Dialog Ingestion (vs RLHF)

## Transformer Alignment: RLHF

Reinforcement Learning from Human Feedback (RLHF) aligns transformers:

```
1. Train a reward model on human preferences (A > B rankings)
2. Fine-tune the LLM with PPO to maximize reward
3. Repeat for weeks on GPU clusters
```

**Problems**:
- Requires **thousands of ranked examples**
- **Weeks of GPU time**
- **Reward hacking** - model games the reward
- **Alignment tax** - degraded performance on benchmarks
- **Static** - can't update alignment in production
- **Opaque** - can't see what changed

## Phiano Dialog Ingestion: Multi-Turn Learning

```rust
// Phiano: learn from multi-turn conversations directly
pub fn learn_into_facet(&self, facet: &mut Facet, memo: &mut Memo, trainer: &Trainer) -> usize {
    for (turns) in &self.dialogues {
        for (i, turn) in turns.iter().enumerate() {
            // Each turn trains the manifold
            trainer.train_definition(facet, &turn.speaker, &turn.content);

            // Record syntax lags between turns (dialogue flow)
            if i > 0 {
                let prev = &turns[i - 1];
                facet.record_phase_lag(&prev.speaker, &turn.speaker);
            }
        }
    }
}
```

**Usage**: POST `/api/dialogue/learn` - instant, CPU, no labels needed.

**Advantages**:
- **No reward model** - learns directly from conversation structure
- **Seconds, not weeks** - CPU, milliseconds per dialogue
- **No reward hacking** - there's no reward to game
- **No alignment tax** - old knowledge preserved (additive)
- **Live** - can ingest new dialogues during production
- **Transparent** - every phase shift is visible

## Comparison

| Feature | RLHF | Dialog Ingestion |
|---------|------|-----------------|
| Data needed | Thousands of ranked pairs | Raw conversations |
| Time | Weeks (GPU) | Seconds (CPU) |
| Reward model | Required | Not needed |
| Forgetting | Possible (fine-tuning) | None (additive) |
| Live updates | No (requires retraining) | Yes (API call) |
| Transparency | Opaque weight changes | Visible phase shifts |
| Cost | $10K-$100K+ | ~$0 |

## The API

```bash
# Ingest multi-turn dialogues
curl -X POST http://localhost:3000/api/dialogue/learn

# Response:
{
  "dialogues_trained": 24,
  "vocabulary": 1247,
  "message": "Successfully trained on 24 multi-turn conversational dialogues"
}
```

The dialogues are ingested as **phase trajectories** - each conversation leaves a "trace" in the manifold that influences future generation. This is like how humans learn conversation patterns from experience, not from reward signals.
