/// Capacity tuning: sweep hyperparameters and select best by validation coherence.

use crate::eval::Evaluator;
use crate::facet::Facet;
use crate::trainer::Trainer;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CapacityConfig {
    pub best_learning_rate: f64,
    pub best_epochs: usize,
    pub best_coherence: f64,
    pub tried_configs: usize,
}

impl Default for CapacityConfig {
    fn default() -> Self {
        Self {
            best_learning_rate: 0.05,
            best_epochs: 64,
            best_coherence: 0.0,
            tried_configs: 0,
        }
    }
}

/// Sweeps learning rate and epoch count, returns best config by validation coherence.
pub fn tune_capacity(
    facet: &mut Facet,
    val_sentences: &[String],
) -> CapacityConfig {
    let evaluator = Evaluator::new();
    let learning_rates = [0.01, 0.03, 0.05, 0.08, 0.12];
    let epoch_counts = [16, 32, 64];

    let mut best = CapacityConfig::default();
    let mut tried = 0usize;

    for &lr in &learning_rates {
        for &epochs in &epoch_counts {
            let trainer = Trainer::new(lr);
            for _ in 0..epochs {
                for sentence in val_sentences {
                    trainer.train_sentence(facet, sentence);
                }
            }

            let mut total_coh = 0.0;
            for sentence in val_sentences {
                total_coh += evaluator.eval(facet, sentence).coherence;
            }
            let mean_coh = total_coh / val_sentences.len().max(1) as f64;
            tried += 1;

            if mean_coh > best.best_coherence {
                best = CapacityConfig {
                    best_learning_rate: lr,
                    best_epochs: epochs,
                    best_coherence: mean_coh,
                    tried_configs: tried,
                };
            }
        }
    }

    best
}
