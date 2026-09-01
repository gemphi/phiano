//! Readout - an online-learned linear reranker over phase features.
//!
//! The generation path previously scored candidates with hard-coded weights
//! (0.35 base, 0.25 phase alignment, 0.40 resonance, fixed content multiplier).
//! `ReadoutState` turns those weights into *learned* state: a 3-element weight
//! vector (one multiplier per feature) updated by a perceptron-style rule from
//! self-supervised bigram evidence during training, and persisted with the facet.
//!
//! Default weights `[1.0, 1.0, 1.0]` reproduce the previous fixed-weight
//! behavior exactly, so the upgrade is behavior-preserving until evidence
//! accumulates.

use serde::{Deserialize, Serialize};

/// Features used by the readout, in fixed order:
/// 0. phase alignment with the lag-adjusted predecessor
/// 1. rotor-dimension resonance with the predecessor
/// 2. content-word indicator (1.0 content, 0.0 function word)
pub const READOUT_FEATURES: usize = 3;

/// Online learned reranker state. Persisted inside the facet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadoutState {
    /// Feature weight multipliers (clamped to [WEIGHT_MIN, WEIGHT_MAX]).
    pub weights: [f64; READOUT_FEATURES],
    /// Perceptron learning rate.
    pub lr: f64,
    /// Total updates applied (diagnostics).
    pub updates: u64,
}

/// Lower clamp for learned weights.
pub const WEIGHT_MIN: f64 = 0.25;
/// Upper clamp for learned weights.
pub const WEIGHT_MAX: f64 = 4.0;

impl Default for ReadoutState {
    fn default() -> Self {
        Self {
            weights: [1.0; READOUT_FEATURES],
            lr: 0.02,
            updates: 0,
        }
    }
}

impl ReadoutState {
    /// Creates readout state with explicit weights (mainly for tests).
    pub fn with_weights(weights: [f64; READOUT_FEATURES]) -> Self {
        Self {
            weights,
            lr: 0.02,
            updates: 0,
        }
    }

    /// Applies the learned multipliers to the generation scoring terms.
    ///
    /// Mirrors the previous fixed formula:
    /// `score = count * (0.35 + 0.25*phase + 0.40*resonance) * content_weight`
    /// with each learned weight scaling its term.
    pub fn score(
        &self,
        ln_count: f64,
        phase_align: f64,
        resonance: f64,
        content_weight: f64,
    ) -> f64 {
        ln_count
            * (0.35 + 0.25 * self.weights[0] * phase_align + 0.40 * self.weights[1] * resonance)
            * (content_weight * self.weights[2])
    }

    /// Perceptron-style update from one self-supervised observation.
    ///
    /// `features_true` are the features of the word that actually followed
    /// (the ground truth from training text); `features_pred` are the features
    /// of the word the readout would have ranked highest instead. Weights move
    /// toward making the true continuation outscore the erroneous prediction.
    pub fn observe(&mut self, features_true: [f64; READOUT_FEATURES], features_pred: [f64; READOUT_FEATURES]) {
        for i in 0..READOUT_FEATURES {
            let delta = self.lr * (features_true[i] - features_pred[i]);
            self.weights[i] = (self.weights[i] + delta).clamp(WEIGHT_MIN, WEIGHT_MAX);
        }
        self.updates += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_reproduces_fixed_weights() {
        let r = ReadoutState::default();
        // Default weights are all 1.0 → identical to the legacy fixed formula.
        let legacy = 3.0_f64.ln_1p() * (0.35 + 0.25 * 0.8 + 0.40 * 0.5) * 1.35;
        let learned = r.score(3.0_f64.ln_1p(), 0.8, 0.5, 1.35);
        assert!((legacy - learned).abs() < 1e-12);
    }

    #[test]
    fn test_observe_moves_weights_toward_truth() {
        let mut r = ReadoutState::default();
        r.lr = 0.5;
        // True continuation had high phase alignment; prediction had none.
        r.observe([1.0, 0.0, 1.0], [0.0, 0.0, 1.0]);
        assert!(r.weights[0] > 1.0);
        assert!((r.weights[1] - 1.0).abs() < 1e-12);
        assert_eq!(r.updates, 1);
    }

    #[test]
    fn test_weights_clamped() {
        let mut r = ReadoutState::default();
        r.lr = 100.0;
        r.observe([1.0, 1.0, 1.0], [0.0, 0.0, 0.0]);
        assert!(r.weights.iter().all(|w| *w <= WEIGHT_MAX));
        r.lr = 100.0;
        r.observe([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        assert!(r.weights.iter().all(|w| *w >= WEIGHT_MIN));
    }
}
