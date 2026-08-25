/// Meta-learning: extracts common patterns across multiple tasks.
/// Uses those patterns to speed up learning on new tasks (Ch 14.5).

use crate::eval::Evaluator;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use crate::trainer::Trainer;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MetaModel {
    pub common_phases: Vec<f64>,
    pub adaptation_rates: Vec<f64>,
    pub n_tasks: usize,
}

#[derive(Debug, Default)]
pub struct MetaLearner;

impl MetaLearner {
    /// Trains on multiple tasks and extracts common phase patterns.
    pub fn learn(
        facet: &mut Facet,
        trainer: &Trainer,
        tasks: &[String],
    ) -> MetaModel {
        let mut all_phases: Vec<f64> = Vec::new();
        let mut adaptation_rates = Vec::new();

        for task in tasks {
            let evaluator = Evaluator::new();
            let before = evaluator.eval(facet, task).coherence;

            for _ in 0..16 {
                trainer.train_sentence(facet, task);
            }

            let after = evaluator.eval(facet, task).coherence;
            adaptation_rates.push(after - before);

            for token in Tokenizer::tokenize(task) {
                if let Some(p) = facet.lexicon.get(&token) {
                    all_phases.push(p.phase);
                }
            }
        }

        all_phases.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let common_phases = if all_phases.len() > 4 {
            let quarter = all_phases.len() / 4;
            all_phases[quarter..all_phases.len() - quarter].to_vec()
        } else {
            all_phases
        };

        MetaModel {
            common_phases,
            adaptation_rates,
            n_tasks: tasks.len(),
        }
    }
}

impl MetaModel {
    /// Uses meta-learned patterns to speed up learning on a new task.
    pub fn adapt(&self, facet: &mut Facet, _trainer: &Trainer, new_task: &str) {
        let tokens = Tokenizer::tokenize(new_task);

        for token in &tokens {
            facet.get_or_init(token);
        }

        let avg_rate = if self.adaptation_rates.is_empty() {
            0.05
        } else {
            self.adaptation_rates.iter().sum::<f64>() / self.adaptation_rates.len() as f64
        };

        let effective_lr = (0.05 + avg_rate.abs() * 0.1).min(0.15);
        let effective_epochs = (16.0 + avg_rate * 100.0).max(8.0) as usize;

        let adapted_trainer = Trainer::new(effective_lr);
        for _ in 0..effective_epochs {
            adapted_trainer.train_sentence(facet, new_task);
        }
    }
}
