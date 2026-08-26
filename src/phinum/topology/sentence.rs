//! Sentence type classification for Phinum language topology.

use super::super::config::PhinumConfig;
use super::super::searle::SpeechAct;
use super::super::syntax::{PartOfSpeech, SyntaxKey, SyntaxParser};
use super::super::variants::{Phinum16, Phinum32, Phinum64, PhinumEngine, PhinumLevel, Variation};
use serde::{Deserialize, Serialize};

/// Sentence type — 16 base types expandable to 32 and 64.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SentenceType {
    pub base: u8,
    pub v16: Variation,
    pub v32: Variation,
    pub v64: Variation,
}

impl SentenceType {
    /// Classifies a sentence into a sentence type.
    pub fn classify(sentence: &str) -> Self {
        let trimmed = sentence.trim();
        let key = SyntaxParser::parse(trimmed);
        let act = SpeechAct::classify(&key);

        let base = Self::base_type(trimmed, &key, act);
        let cfg = PhinumConfig::global();
        let v16 = Phinum16::classify_hash(cfg.hash_base(base as u64, PhinumLevel::N16));
        let v32 = Phinum32::classify_hash(cfg.hash_base(base as u64, PhinumLevel::N32));
        let v64 = Phinum64::classify_hash(cfg.hash_base(base as u64, PhinumLevel::N64));

        Self { base, v16, v32, v64 }
    }

    fn base_type(text: &str, key: &SyntaxKey, act: SpeechAct) -> u8 {
        if text.ends_with('?') {
            1 // Interrogative
        } else if text.ends_with('!') {
            3 // Exclamatory
        } else if key.parts.first() == Some(&PartOfSpeech::Verb)
            || key.parts.first() == Some(&PartOfSpeech::Auxiliary)
        {
            2 // Imperative
        } else {
            match act {
                SpeechAct::Assertive => 0,
                SpeechAct::Directive => 4,
                SpeechAct::Commissive => 5,
                SpeechAct::Expressive => 6,
                SpeechAct::Declaration => 7,
            }
        }
    }

    pub fn label(self) -> &'static str {
        match self.base {
            0 => "declarative",
            1 => "interrogative",
            2 => "imperative",
            3 => "exclamatory",
            4 => "directive",
            5 => "commissive",
            6 => "expressive",
            7 => "declaration",
            _ => "complex",
        }
    }
}
