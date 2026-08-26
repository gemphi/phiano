//! I Ching (易經) 64 Hexagrams & Trigram Spin Module for Phinum.

pub mod table;
#[cfg(test)]
mod tests;
pub mod trigram;

pub use table::{archetype_name, KING_WEN_MAP};
pub use trigram::Trigram;

use super::searle::SpeechAct;
use super::syntax::SyntaxKey;
use crate::config::TWO_PI;
use serde::{Deserialize, Serialize};

/// A 64-Hexagram definition representing a harmonic state on the Phinum manifold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hexagram {
    /// Binary index $0 \dots 63$ (6-bit state).
    pub id: u8,
    /// Traditional King Wen sequence number $1 \dots 64$.
    pub number: u8,
    /// Upper Trigram (outer aspect).
    pub upper: Trigram,
    /// Lower Trigram (inner aspect).
    pub lower: Trigram,
}

impl Hexagram {
    /// Creates a hexagram from its 6-bit binary index ($0 \dots 63$).
    pub fn from_id(id: u8) -> Self {
        let clamped = id % 64;
        let lower = Trigram::from_bits(clamped & 0b111);
        let upper = Trigram::from_bits((clamped >> 3) & 0b111);
        let number = KING_WEN_MAP[clamped as usize % 64];
        Self { id: clamped, number, upper, lower }
    }

    /// Creates a hexagram from phase angle $\theta \in [0, 2\pi)$.
    pub fn from_phase(phase: f64) -> Self {
        let normalized = phase.rem_euclid(TWO_PI);
        let id = ((normalized / TWO_PI) * 64.0).floor() as u8 % 64;
        Self::from_id(id)
    }

    /// Creates a hexagram from a structural syntax key hash.
    pub fn from_syntax_key(key: &SyntaxKey) -> Self {
        let hash = key.key.bytes().fold(0xcbf29ce484222325u64, |acc, b| {
            (acc ^ (b as u64)).wrapping_mul(0x100000001b3)
        });
        let id = (hash % 64) as u8;
        Self::from_id(id)
    }

    /// Returns the central phase angle $\theta_k = \frac{2\pi k}{64}$.
    pub fn phase_angle(self) -> f64 {
        (self.id as f64 / 64.0) * TWO_PI
    }

    /// Spins the hexagram by a continuous phase delta $\Delta\theta$.
    pub fn spin(self, delta_phase: f64) -> Self {
        let new_phase = self.phase_angle() + delta_phase;
        Self::from_phase(new_phase)
    }

    /// Mutates the hexagram by flipping changing lines (Yin $\leftrightarrow$ Yang) via a 6-bit bitmask.
    pub fn changing_lines(self, mask: u8) -> Self {
        let new_id = (self.id ^ (mask & 0b111111)) % 64;
        Self::from_id(new_id)
    }

    /// Maps this hexagram to a default Searle speech act category.
    pub fn speech_act(self) -> SpeechAct {
        match self.upper {
            Trigram::Unity | Trigram::Thunder => SpeechAct::Declaration,
            Trigram::Lake | Trigram::Fire => SpeechAct::Expressive,
            Trigram::Wind => SpeechAct::Directive,
            Trigram::Water => SpeechAct::Commissive,
            Trigram::Mountain | Trigram::Earth => SpeechAct::Assertive,
        }
    }

    /// Returns the Unicode character for the hexagram (`\u{4DC0}` to `\u{4DFF}`).
    pub fn unicode_char(self) -> char {
        let code = 0x4DC0 + (self.number.saturating_sub(1) as u32 % 64);
        char::from_u32(code).unwrap_or('䷀')
    }

    /// Returns the English name / archetype of the hexagram.
    pub fn archetype_name(self) -> &'static str {
        table::archetype_name(self.number)
    }
}
