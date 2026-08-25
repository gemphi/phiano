/// Training distribution tracking: detects distribution shift over time.

use crate::config::TWO_PI;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use std::f64::consts::PI;

/// Tracks the running mean phase and amplitude of training inputs.
#[derive(Debug, Clone)]
pub struct DistributionTracker {
    mean_phase: f64,
    mean_amplitude: f64,
    n_updates: usize,
}

impl Default for DistributionTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl DistributionTracker {
    pub fn new() -> Self {
        Self {
            mean_phase: 0.0,
            mean_amplitude: 0.0,
            n_updates: 0,
        }
    }

    /// Updates the tracker with a new training input.
    pub fn update(&mut self, facet: &Facet, text: &str) {
        let tokens = Tokenizer::tokenize(text);
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut count = 0;

        for token in &tokens {
            if let Some(p) = facet.lexicon.get(token) {
                sum_x += p.amplitude * p.phase.cos();
                sum_y += p.amplitude * p.phase.sin();
                count += 1;
            }
        }

        if count == 0 {
            return;
        }

        let input_phase = sum_y.atan2(sum_x).rem_euclid(TWO_PI);
        let input_amp = (sum_x * sum_x + sum_y * sum_y).sqrt() / count as f64;

        let n = self.n_updates as f64 + 1.0;
        self.mean_phase = ((self.mean_phase * self.n_updates as f64) + input_phase) / n;
        self.mean_amplitude = ((self.mean_amplitude * self.n_updates as f64) + input_amp) / n;
        self.n_updates += 1;
    }

    /// Returns how far an input is from the running distribution.
    pub fn shift_score(&self, facet: &Facet, text: &str) -> f64 {
        if self.n_updates == 0 {
            return 0.0;
        }

        let tokens = Tokenizer::tokenize(text);
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut count = 0;

        for token in &tokens {
            if let Some(p) = facet.lexicon.get(token) {
                sum_x += p.amplitude * p.phase.cos();
                sum_y += p.amplitude * p.phase.sin();
                count += 1;
            }
        }

        if count == 0 {
            return 1.0;
        }

        let input_phase = sum_y.atan2(sum_x).rem_euclid(TWO_PI);
        let mut diff = (input_phase - self.mean_phase).abs();
        if diff > PI {
            diff = TWO_PI - diff;
        }

        diff / PI
    }

    pub fn n_updates(&self) -> usize {
        self.n_updates
    }
}
