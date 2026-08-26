/// Spectral methods for Facet — physics-derived color, distance, and identity.
///
/// These methods use the gemgum binding layer to map words through the
/// phiton/chiton 2-in-1 light architecture, providing physics-based
/// alternatives to the raw phase-delta similarity used in the base model.

use super::Facet;
use crate::gemgum::{Gemgum, SpectralIdentity};
use crate::phiton::PhitonColor;

impl Facet {
    /// Returns the physics-derived color for a word.
    ///
    /// Uses the gemgum binding layer to map the word's phasor through
    /// the phiton spectral domain to a [`PhitonColor`].
    #[allow(dead_code)]
    pub fn word_color(&self, word: &str) -> Option<PhitonColor> {
        self.lexicon.get(word).map(|p| Gemgum::phasor_to_color(p))
    }

    /// Computes the golden-ratio-weighted semantic distance between two words.
    ///
    /// Uses [`Gemgum::semantic_distance`] which combines color distance,
    /// time distance, and quantum distance — all weighted by φ.
    /// Returns `f64::MAX` if either word is not in the lexicon.
    #[allow(dead_code)]
    pub fn semantic_distance(&self, word_a: &str, word_b: &str) -> f64 {
        match (self.lexicon.get(word_a), self.lexicon.get(word_b)) {
            (Some(a), Some(b)) => Gemgum::semantic_distance(a, b),
            _ => f64::MAX,
        }
    }

    /// Computes the standing-wave coupling strength between two words.
    ///
    /// Uses [`Gemgum::word_coupling`] which combines interference,
    /// wave resonance, and spectral coupling.
    /// Returns 0.0 if either word is not in the lexicon.
    pub fn word_coupling(&self, word_a: &str, word_b: &str) -> f64 {
        match (self.lexicon.get(word_a), self.lexicon.get(word_b)) {
            (Some(a), Some(b)) => Gemgum::word_coupling(a, b),
            _ => 0.0,
        }
    }

    /// Returns the full spectral identity of a word — its 2-in-1 light representation.
    ///
    /// Uses [`Gemgum::spectral_identity`] to produce the complete
    /// phiton (particle) + chiton (wave) + color representation.
    #[allow(dead_code)]
    pub fn spectral_identity(&self, word: &str) -> Option<SpectralIdentity> {
        self.lexicon.get(word).map(|p| Gemgum::spectral_identity(p))
    }
}
