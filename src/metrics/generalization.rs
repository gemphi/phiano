/// Generalization metrics: local vs extreme generalization scoring.
/// Implements Ch 14.2 concepts: local generalization (interpolation)
/// vs extreme generalization (abstraction/reasoning).

use crate::config::TWO_PI;
use crate::eval::Evaluator;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use std::f64::consts::PI;

/// Measures how well the model handles new examples close to the training distribution.
/// Uses phase-space proximity: test words near training words in phase space.
pub fn local_generalization_score(
    facet: &Facet,
    train_words: &[String],
    test_words: &[String],
) -> f64 {
    if train_words.is_empty() || test_words.is_empty() {
        return 0.0;
    }

    let evaluator = Evaluator::new();
    let mut total = 0.0;
    let mut count = 0;

    for test_word in test_words {
        if !facet.lexicon.contains_key(test_word) {
            continue;
        }
        let test_phase = facet.lexicon[test_word].phase;

        let mut min_dist = f64::MAX;
        for train_word in train_words {
            if let Some(tp) = facet.lexicon.get(train_word) {
                let mut diff = (tp.phase - test_phase).abs();
                if diff > PI {
                    diff = TWO_PI - diff;
                }
                min_dist = min_dist.min(diff);
            }
        }

        if min_dist < 0.5 {
            let sentence = format!("{} is related to the topic", test_word);
            total += evaluator.eval(facet, &sentence).coherence;
            count += 1;
        }
    }

    if count > 0 { total / count as f64 } else { 0.0 }
}

/// Measures performance on words far from the training distribution.
/// These are "unknown unknowns" — genuinely unfamiliar situations.
pub fn extreme_generalization_score(
    facet: &Facet,
    train_words: &[String],
    novel_words: &[String],
) -> f64 {
    if train_words.is_empty() || novel_words.is_empty() {
        return 0.0;
    }

    let evaluator = Evaluator::new();
    let mut total = 0.0;
    let mut count = 0;

    for novel_word in novel_words {
        if !facet.lexicon.contains_key(novel_word) {
            continue;
        }
        let novel_phase = facet.lexicon[novel_word].phase;

        let mut min_dist = f64::MAX;
        for train_word in train_words {
            if let Some(tp) = facet.lexicon.get(train_word) {
                let mut diff = (tp.phase - novel_phase).abs();
                if diff > PI {
                    diff = TWO_PI - diff;
                }
                min_dist = min_dist.min(diff);
            }
        }

        if min_dist > 2.0 {
            let sentence = format!("{} is a new concept", novel_word);
            total += evaluator.eval(facet, &sentence).coherence;
            count += 1;
        }
    }

    if count > 0 { total / count as f64 } else { 0.0 }
}

/// The generalization gap: difference between local and extreme performance.
/// A large gap indicates the model relies on interpolation, not abstraction.
pub fn generalization_gap(local: f64, extreme: f64) -> f64 {
    local - extreme
}

/// Comprehensive generalization assessment.
pub fn assess_generalization(
    facet: &Facet,
    train_words: &[String],
    test_words: &[String],
    novel_words: &[String],
) -> GeneralizationReport {
    let local = local_generalization_score(facet, train_words, test_words);
    let extreme = extreme_generalization_score(facet, train_words, novel_words);
    GeneralizationReport {
        local_score: local,
        extreme_score: extreme,
        gap: generalization_gap(local, extreme),
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GeneralizationReport {
    pub local_score: f64,
    pub extreme_score: f64,
    pub gap: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generalization_gap() {
        let gap = generalization_gap(0.8, 0.3);
        assert!((gap - 0.5).abs() < 0.001);
    }
}
