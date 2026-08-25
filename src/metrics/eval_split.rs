/// Validation-aware evaluation: evaluate model on specific data splits.

use crate::data::DataSplits;
use crate::eval::Evaluator;
use crate::facet::Facet;
use crate::trainer::Trainer;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct EvalSummary {
    pub split_name: String,
    pub mean_coherence: f64,
    pub mean_novelty: f64,
    pub mean_resonance: f64,
    pub n_samples: usize,
}

impl EvalSummary {
    pub fn empty(split_name: &str) -> Self {
        Self {
            split_name: split_name.to_string(),
            mean_coherence: 0.0,
            mean_novelty: 0.0,
            mean_resonance: 0.0,
            n_samples: 0,
        }
    }
}

/// Evaluates the facet on a named split of the data.
pub fn eval_on_split(facet: &Facet, evaluator: &Evaluator, split: &DataSplits, split_name: &str) -> EvalSummary {
    let sentences: &[String] = match split_name {
        "train" => &split.train,
        "validation" => &split.validation,
        "test" => &split.test,
        _ => &split.train,
    };

    if sentences.is_empty() {
        return EvalSummary::empty(split_name);
    }

    let mut total_coh = 0.0;
    let mut total_nov = 0.0;
    let mut total_res = 0.0;

    for sentence in sentences {
        let result = evaluator.eval(facet, sentence);
        total_coh += result.coherence;
        total_nov += result.novelty;
        total_res += result.resonance;
    }

    let n = sentences.len();
    EvalSummary {
        split_name: split_name.to_string(),
        mean_coherence: total_coh / n as f64,
        mean_novelty: total_nov / n as f64,
        mean_resonance: total_res / n as f64,
        n_samples: n,
    }
}

/// Full evaluation pipeline: train on train split, tune on val, test once.
pub fn full_eval_pipeline(
    facet: &mut Facet,
    trainer: &Trainer,
    evaluator: &Evaluator,
    splits: &DataSplits,
) -> (EvalSummary, EvalSummary, EvalSummary) {
    for sentence in &splits.train {
        trainer.train_sentence(facet, sentence);
    }

    let train_eval = eval_on_split(facet, evaluator, splits, "train");
    let val_eval = eval_on_split(facet, evaluator, splits, "validation");
    let test_eval = eval_on_split(facet, evaluator, splits, "test");

    (train_eval, val_eval, test_eval)
}
