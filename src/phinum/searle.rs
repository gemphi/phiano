/// Searle's speech act classification — 16/32/64 variants.
///
/// John Searle's taxonomy of illocutionary acts provides the framework
/// for classifying *what a sentence does* (not just what it says).
///
/// # Base 5 Categories (Searle)
///
/// 1. **Assertive** — stating facts, claiming truth
/// 2. **Directive** — requesting, commanding, asking
/// 3. **Commissive** — promising, committing, obligating
/// 4. **Expressive** — expressing feelings, attitudes
/// 5. **Declaration** — bringing about a state by uttering
///
/// # Phinum Expansion
///
/// - **Phinum16**: 16 sub-categories (5 base × ~3 sub-types each)
/// - **Phinum32**: 32 sub-categories (16 × 2 further refinements)
/// - **Phinum64**: 64 sub-categories (32 × 2, full granularity)

use super::config::PhinumConfig;
use super::syntax::SyntaxKey;
use super::variants::{Phinum16, Phinum32, Phinum64, PhinumEngine, PhinumLevel, Variation};
use serde::{Deserialize, Serialize};

/// The 5 base Searle speech act categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpeechAct {
    /// Stating facts, claiming truth.
    Assertive,
    /// Requesting, commanding, asking.
    Directive,
    /// Promising, committing, obligating.
    Commissive,
    /// Expressing feelings, attitudes.
    Expressive,
    /// Bringing about a state by uttering.
    Declaration,
}

impl SpeechAct {
    /// Returns the label for this speech act.
    pub fn label(self) -> &'static str {
        match self {
            Self::Assertive => "assertive",
            Self::Directive => "directive",
            Self::Commissive => "commissive",
            Self::Expressive => "expressive",
            Self::Declaration => "declaration",
        }
    }

    /// Classifies a sentence into a speech act using structural heuristics.
    pub fn classify(key: &SyntaxKey) -> Self {
        let k = key.key.as_str();
        if k.contains("INTJ") || k.contains("ADV+ADJ") {
            Self::Expressive
        } else if k.starts_with("PRON+V") || k.starts_with("N+V") {
            if k.contains("PREP") || k.ends_with("N") || k.ends_with("PRON") {
                Self::Assertive
            } else {
                Self::Directive
            }
        } else if k.contains("V+PREP") || k.contains("AUX+V") {
            Self::Commissive
        } else if k.contains("V+N") || k.ends_with("V") {
            Self::Directive
        } else {
            Self::Assertive
        }
    }

    /// Returns the 16-level sub-category index for this speech act.
    pub fn sub16(self, key: &SyntaxKey) -> Variation {
        let base = self as u8;
        let sub = (key.len() as u8) % 3;
        let idx = base * 3 + sub;
        let cfg = PhinumConfig::global();
        Phinum16::classify_hash(cfg.hash_base(idx as u64, PhinumLevel::N16))
    }

    /// Returns the 32-level sub-category index for this speech act.
    pub fn sub32(self, key: &SyntaxKey) -> Variation {
        let base = self as u8;
        let sub = (key.len() as u8) % 6;
        let idx = base * 6 + sub;
        let cfg = PhinumConfig::global();
        Phinum32::classify_hash(cfg.hash_base(idx as u64, PhinumLevel::N32))
    }

    /// Returns the 64-level sub-category index for this speech act.
    pub fn sub64(self, key: &SyntaxKey) -> Variation {
        let base = self as u8;
        let sub = (key.len() as u8) % 12;
        let idx = base * 12 + sub;
        let cfg = PhinumConfig::global();
        Phinum64::classify_hash(cfg.hash_base(idx as u64, PhinumLevel::N64))
    }
}

/// Classifies sentences into Searle speech acts with Phinum granularity.
pub struct SearleClassifier;

impl SearleClassifier {
    /// Classifies a single sentence.
    pub fn classify(sentence: &str) -> (SpeechAct, SyntaxKey) {
        let key = crate::phinum::SyntaxParser::parse(sentence);
        let act = SpeechAct::classify(&key);
        (act, key)
    }

    /// Classifies a sentence at all three Phinum levels.
    pub fn classify_full(sentence: &str) -> SearleClassification {
        let key = crate::phinum::SyntaxParser::parse(sentence);
        let act = SpeechAct::classify(&key);
        SearleClassification {
            act,
            key,
            v16: act.sub16(&crate::phinum::SyntaxParser::parse(sentence)),
            v32: act.sub32(&crate::phinum::SyntaxParser::parse(sentence)),
            v64: act.sub64(&crate::phinum::SyntaxParser::parse(sentence)),
        }
    }
}

/// Full Searle classification at all Phinum levels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearleClassification {
    pub act: SpeechAct,
    pub key: SyntaxKey,
    pub v16: Variation,
    pub v32: Variation,
    pub v64: Variation,
}
