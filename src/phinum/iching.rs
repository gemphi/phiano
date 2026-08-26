//! I Ching (易經) 64 Hexagrams & Trigram Spin Module for Phinum.
//!
//! Maps the 64 classical hexagrams ($2^6 = 64 = 8 \times 8$) directly to the
//! 64 harmonic sectors and perspective manifolds of the Phinum engine.
//! Language topology is modeled as dynamic state transformations across
//! changing lines (Yin $\leftrightarrow$ Yang) and circular phase spins.
//!
//! # Architecture
//!
//! ```text
//!              8 Trigrams (Bagua: 2^3 = 8)
//!       [☰ ☱ ☲ ☳ ☴ ☵ ☶ ☷]
//!                 │
//!                 ▼ (Tensor Product: 8 × 8)
//!      64 Hexagrams (䷀ ... ䷿ : 2^6 = 64)
//!                 │
//!   ┌─────────────┼─────────────┐
//!   ▼             ▼             ▼
//! Phinum16     Phinum32      Phinum64
//! (Coarse)    (Balanced)     (Complete)
//!   │             │             │
//!   └─────────────┬─────────────┘
//!                 ▼
//!   Language Spider-Net Topology
//! ```

use super::searle::SpeechAct;
use super::syntax::SyntaxKey;
use crate::config::TWO_PI;
use serde::{Deserialize, Serialize};

/// The 8 fundamental Trigrams (Bagua / 八卦).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Trigram {
    /// 乾 (Qián) — Heaven / Creative (111)
    Heaven,
    /// 兌 (Duì) — Lake / Joyous (110)
    Lake,
    /// 離 (Lí) — Fire / Clinging / Clarity (101)
    Fire,
    /// 震 (Zhèn) — Thunder / Arousing (100)
    Thunder,
    /// 巽 (Xùn) — Wind / Gentle (011)
    Wind,
    /// 坎 (Kǎn) — Water / Abyssal (010)
    Water,
    /// 艮 (Gèn) — Mountain / Keeping Still (001)
    Mountain,
    /// 坤 (Kūn) — Earth / Receptive (000)
    Earth,
}

impl Trigram {
    /// Returns the 3-bit binary representation (0..7).
    pub fn bits(self) -> u8 {
        match self {
            Self::Earth => 0b000,
            Self::Mountain => 0b001,
            Self::Water => 0b010,
            Self::Wind => 0b011,
            Self::Thunder => 0b100,
            Self::Fire => 0b101,
            Self::Lake => 0b110,
            Self::Heaven => 0b111,
        }
    }

    /// Creates a trigram from 3-bit binary value (0..7).
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0b111 {
            0b000 => Self::Earth,
            0b001 => Self::Mountain,
            0b010 => Self::Water,
            0b011 => Self::Wind,
            0b100 => Self::Thunder,
            0b101 => Self::Fire,
            0b110 => Self::Lake,
            _ => Self::Heaven,
        }
    }

    /// Returns the Unicode symbol for the trigram.
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Heaven => "☰",
            Self::Lake => "☱",
            Self::Fire => "☲",
            Self::Thunder => "☳",
            Self::Wind => "☴",
            Self::Water => "☵",
            Self::Mountain => "☶",
            Self::Earth => "☷",
        }
    }

    /// Returns the pinyin name of the trigram.
    pub fn name_pinyin(self) -> &'static str {
        match self {
            Self::Heaven => "Qián",
            Self::Lake => "Duì",
            Self::Fire => "Lí",
            Self::Thunder => "Zhèn",
            Self::Wind => "Xùn",
            Self::Water => "Kǎn",
            Self::Mountain => "Gèn",
            Self::Earth => "Kūn",
        }
    }
}

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
        let number = Self::id_to_king_wen(clamped);
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
            Trigram::Heaven | Trigram::Thunder => SpeechAct::Declaration,
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
        match self.number {
            1 => "The Creative (Heaven)",
            2 => "The Receptive (Earth)",
            3 => "Difficulty at the Beginning",
            4 => "Youthful Folly",
            5 => "Waiting (Nourishment)",
            6 => "Conflict",
            7 => "The Army",
            8 => "Holding Together (Union)",
            9 => "Small Taming",
            10 => "Treading (Conduct)",
            11 => "Peace",
            12 => "Standstill (Stagnation)",
            13 => "Fellowship with Men",
            14 => "Possession in Great Measure",
            15 => "Modesty",
            16 => "Enthusiasm",
            17 => "Following",
            18 => "Work on the Decayed",
            19 => "Approach",
            20 => "Contemplation (View)",
            21 => "Biting Through",
            22 => "Grace",
            23 => "Splitting Apart",
            24 => "Return (The Turning Point)",
            25 => "Innocence (The Unexpected)",
            26 => "Great Taming",
            27 => "Providing Nourishment",
            28 => "Preponderance of the Great",
            29 => "The Abyssal (Water)",
            30 => "The Clinging (Fire)",
            31 => "Influence (Wooing)",
            32 => "Duration",
            33 => "Retreat",
            34 => "Great Power",
            35 => "Progress",
            36 => "Darkening of the Light",
            37 => "The Family",
            38 => "Opposition",
            39 => "Obstruction",
            40 => "Deliverance",
            41 => "Decrease",
            42 => "Increase",
            43 => "Breakthrough (Resoluteness)",
            44 => "Coming to Meet",
            45 => "Gathering Together",
            46 => "Pushing Upward",
            47 => "Oppression (Exhaustion)",
            48 => "The Well",
            49 => "Revolution (Molting)",
            50 => "The Cauldron",
            51 => "The Arousing (Shock)",
            52 => "Keeping Still (Mountain)",
            53 => "Development (Gradual Progress)",
            54 => "The Marrying Maiden",
            55 => "Abundance",
            56 => "The Wanderer",
            57 => "The Gentle (Wind)",
            58 => "The Joyous (Lake)",
            59 => "Dispersion (Dissolution)",
            60 => "Limitation",
            61 => "Inner Truth",
            62 => "Small Preponderance",
            63 => "After Completion",
            64 => "Before Completion",
            _ => "The Harmonic Hexagram",
        }
    }

    fn id_to_king_wen(id: u8) -> u8 {
        // Deterministic mapping from 6-bit binary index to King Wen order (1..64)
        const KING_WEN_MAP: [u8; 64] = [
            2, 23, 8, 20, 16, 35, 45, 12, 15, 52, 39, 53, 62, 56, 31, 33,
            7, 4, 29, 59, 40, 64, 47, 6, 46, 18, 48, 57, 32, 50, 28, 44,
            24, 27, 3, 42, 51, 21, 17, 25, 36, 22, 63, 37, 55, 30, 49, 13,
            19, 41, 60, 61, 54, 38, 58, 10, 11, 26, 5, 9, 34, 14, 43, 1,
        ];
        KING_WEN_MAP[id as usize % 64]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigram_bits_and_symbols() {
        let heaven = Trigram::Heaven;
        assert_eq!(heaven.bits(), 0b111);
        assert_eq!(heaven.symbol(), "☰");
        assert_eq!(Trigram::from_bits(0b111), Trigram::Heaven);

        let earth = Trigram::Earth;
        assert_eq!(earth.bits(), 0b000);
        assert_eq!(earth.symbol(), "☷");
        assert_eq!(Trigram::from_bits(0b000), Trigram::Earth);
    }

    #[test]
    fn test_hexagram_creation_and_spin() {
        let hex_0 = Hexagram::from_id(0);
        assert_eq!(hex_0.lower, Trigram::Earth);
        assert_eq!(hex_0.upper, Trigram::Earth);

        let hex_63 = Hexagram::from_id(63);
        assert_eq!(hex_63.lower, Trigram::Heaven);
        assert_eq!(hex_63.upper, Trigram::Heaven);

        let spun = hex_0.spin(std::f64::consts::PI);
        assert_eq!(spun.id, 32);

        let mutated = hex_0.changing_lines(0b000001);
        assert_eq!(mutated.id, 1);
    }

    #[test]
    fn test_syntax_key_to_hexagram() {
        let key = crate::phinum::SyntaxParser::parse("i want to hug you");
        let hex = Hexagram::from_syntax_key(&key);
        assert!(hex.id < 64);
        assert!(!hex.archetype_name().is_empty());
    }
}
