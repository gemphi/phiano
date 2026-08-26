//! Paragraph and discourse type classification for Phinum language topology.

use super::sentence::SentenceType;
use super::super::variants::{Phinum16, Phinum32, Phinum64, PhinumEngine, Variation};
use serde::{Deserialize, Serialize};

/// Paragraph type — composed from sentence types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParagraphType {
    pub base: u8,
    pub v16: Variation,
    pub v32: Variation,
    pub v64: Variation,
}

impl ParagraphType {
    /// Classifies a sequence of sentence types into a paragraph type.
    pub fn classify(sentence_types: &[SentenceType]) -> Self {
        if sentence_types.is_empty() {
            return Self {
                base: 0,
                v16: Phinum16::classify_str(""),
                v32: Phinum32::classify_str(""),
                v64: Phinum64::classify_str(""),
            };
        }

        let base = Self::base_type(sentence_types);
        let hash: u64 = sentence_types
            .iter()
            .map(|st| st.base as u64)
            .fold(0u64, |acc, b| acc * 16 + b);
        let v16 = Phinum16::classify_hash(hash);
        let v32 = Phinum32::classify_hash(hash);
        let v64 = Phinum64::classify_hash(hash);

        Self { base, v16, v32, v64 }
    }

    fn base_type(types: &[SentenceType]) -> u8 {
        let declarative = types.iter().filter(|t| t.base == 0).count();
        let interrogative = types.iter().filter(|t| t.base == 1).count();
        let imperative = types.iter().filter(|t| t.base == 2).count();
        let exclamatory = types.iter().filter(|t| t.base == 3).count();

        let n = types.len() as f64;
        if interrogative as f64 / n > 0.5 {
            1
        } else if imperative as f64 / n > 0.5 {
            2
        } else if exclamatory as f64 / n > 0.5 {
            3
        } else if declarative as f64 / n > 0.7 {
            0
        } else {
            4
        }
    }

    pub fn label(self) -> &'static str {
        match self.base {
            0 => "narrative",
            1 => "interrogative",
            2 => "directive",
            3 => "expressive",
            4 => "mixed",
            _ => "complex",
        }
    }
}
