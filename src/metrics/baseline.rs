/// Baseline scoring: random, frequency, and phase baselines.
/// These give lower bounds to compare against the full model.

use crate::config::TWO_PI;
use crate::eval::Evaluator;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use std::f64::consts::PI;

#[derive(Debug, Default)]
pub struct Baselines;

impl Baselines {
    /// Random baseline: score from random word selection.
    pub fn random(facet: &Facet, _prompt: &str) -> f64 {
        let evaluator = Evaluator::new();
        let words: Vec<String> = facet.lexicon.keys().take(16).cloned().collect();
        if words.is_empty() {
            return 0.0;
        }
        let response: String = words.iter().take(8).cloned().collect::<Vec<_>>().join(" ");
        evaluator.eval(facet, &response).coherence
    }

    /// Frequency baseline: score from most frequent words (highest amplitude).
    pub fn frequency(facet: &Facet, _prompt: &str) -> f64 {
        let evaluator = Evaluator::new();
        let mut words: Vec<(String, f64)> = facet
            .lexicon
            .iter()
            .map(|(w, p)| (w.clone(), p.amplitude))
            .collect();
        words.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let response: String = words.iter().take(8).map(|(w, _)| w.clone()).collect::<Vec<_>>().join(" ");
        evaluator.eval(facet, &response).coherence
    }

    /// Phase baseline: score from nearest-phase words (no n-gram, no attention).
    pub fn phase(facet: &Facet, prompt: &str) -> f64 {
        let evaluator = Evaluator::new();
        let tokens = Tokenizer::tokenize(prompt);
        if tokens.is_empty() {
            return 0.0;
        }

        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        for token in &tokens {
            if let Some(p) = facet.lexicon.get(token) {
                sum_x += p.amplitude * p.phase.cos();
                sum_y += p.amplitude * p.phase.sin();
            }
        }
        let target_phase = sum_y.atan2(sum_x).rem_euclid(TWO_PI);

        let mut candidates: Vec<(String, f64)> = facet
            .lexicon
            .iter()
            .map(|(w, p)| {
                let mut diff = (p.phase - target_phase).abs();
                if diff > PI {
                    diff = TWO_PI - diff;
                }
                (w.clone(), diff)
            })
            .collect();
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let response: String = candidates.iter().take(8).map(|(w, _)| w.clone()).collect::<Vec<_>>().join(" ");
        evaluator.eval(facet, &response).coherence
    }

    /// Returns all three baseline scores as a tuple.
    pub fn all(facet: &Facet, prompt: &str) -> (f64, f64, f64) {
        (
            Self::random(facet, prompt),
            Self::frequency(facet, prompt),
            Self::phase(facet, prompt),
        )
    }
}
