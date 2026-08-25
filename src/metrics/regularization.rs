/// Regularization controls: amplitude decay, phase jitter, band capping.
/// Prevents overfitting in the phase-oscillator model.

use crate::config::{AMPLITUDE_MAX, BAND_N_INITIAL};
use crate::facet::Facet;

/// Applies amplitude decay to prevent amplitude explosion.
/// Scales all amplitudes by (1 - decay_rate).
pub fn apply_amplitude_decay(facet: &mut Facet, decay_rate: f64) {
    for phasor in facet.lexicon.values_mut() {
        phasor.amplitude *= (1.0 - decay_rate).max(0.0);
        if phasor.amplitude > AMPLITUDE_MAX {
            phasor.amplitude = AMPLITUDE_MAX;
        }
    }
}

/// Applies random phase jitter to prevent phase collapse.
/// Adds uniform noise in [-jitter, +jitter] to each word's phase.
pub fn apply_phase_jitter(facet: &mut Facet, jitter: f64) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    for (word, phasor) in facet.lexicon.iter_mut() {
        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        let noise = ((hasher.finish() as f64 / f64::MAX) - 0.5) * 2.0 * jitter;
        phasor.phase = (phasor.phase + noise).rem_euclid(crate::config::TWO_PI);
    }
}

/// Caps band_n growth to prevent unbounded familiarity.
pub fn apply_band_regularization(facet: &mut Facet, max_band: u32) {
    for phasor in facet.lexicon.values_mut() {
        if phasor.band_n > max_band {
            phasor.band_n = max_band;
        }
    }
}

/// Resets underused words (amplitude below threshold) to initial state.
pub fn prune_low_amplitude(facet: &mut Facet, threshold: f64) -> usize {
    let mut pruned = 0;
    for phasor in facet.lexicon.values_mut() {
        if phasor.amplitude < threshold {
            phasor.amplitude = 1.0;
            phasor.band_n = BAND_N_INITIAL;
            pruned += 1;
        }
    }
    pruned
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplitude_decay() {
        let mut facet = Facet::new();
        facet.get_or_init("rust");
        facet.get_or_init("code");

        apply_amplitude_decay(&mut facet, 0.1);
        for p in facet.lexicon.values() {
            assert!(p.amplitude <= 1.0);
        }
    }

    #[test]
    fn test_band_regularization() {
        let mut facet = Facet::new();
        facet.get_or_init("rust");
        for p in facet.lexicon.values_mut() {
            p.band_n = 100;
        }
        apply_band_regularization(&mut facet, 10);
        for p in facet.lexicon.values() {
            assert!(p.band_n <= 10);
        }
    }
}
