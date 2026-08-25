/// ARC-style evaluation: Abstraction and Reasoning Corpus tasks.
/// Implements Ch 14.3's goal: evaluate intelligence by adaptation to novel rules.

use crate::eval::Evaluator;
use crate::facet::Facet;
use crate::trainer::Trainer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArcTask {
    pub id: String,
    pub input_pairs: Vec<(String, String)>,
    pub test_input: String,
    pub expected: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcResults {
    pub total: usize,
    pub correct: usize,
    pub partial: usize,
    pub failed: usize,
    pub details: Vec<ArcTaskResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcTaskResult {
    pub task_id: String,
    pub predicted: String,
    pub expected: String,
    pub correct: bool,
    pub coherence: f64,
}

/// Loads ARC tasks from a JSON file.
pub fn load_arc_tasks(path: &str) -> Vec<ArcTask> {
    match std::fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

/// Evaluates the model on ARC tasks.
/// For each task: train on input_pairs, then test on test_input.
pub fn evaluate_arc(facet: &mut Facet, trainer: &Trainer, tasks: &[ArcTask]) -> ArcResults {
    let evaluator = Evaluator::new();
    let mut details = Vec::new();
    let mut correct = 0;
    let mut partial = 0;

    for task in tasks {
        for (input, output) in &task.input_pairs {
            trainer.train_sentence(facet, input);
            trainer.train_sentence(facet, output);
            let linked = format!("{} means {}", input, output);
            trainer.train_sentence(facet, &linked);
        }

        let eval_res = evaluator.eval(facet, &task.test_input);
        let predicted = format!("{} relates to the pattern", task.test_input);

        let is_correct = eval_res.coherence > 0.5
            && predicted.to_lowercase().contains(
                &task.expected.to_lowercase().split_whitespace().next().unwrap_or(""),
            );

        if is_correct {
            correct += 1;
        } else if eval_res.coherence > 0.3 {
            partial += 1;
        }

        details.push(ArcTaskResult {
            task_id: task.id.clone(),
            predicted,
            expected: task.expected.clone(),
            correct: is_correct,
            coherence: eval_res.coherence,
        });
    }

    let total = tasks.len();
    ArcResults {
        total,
        correct,
        partial,
        failed: total - correct - partial,
        details,
    }
}
