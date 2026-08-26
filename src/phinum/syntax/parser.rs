use super::super::variants::{Phinum16, Phinum32, Phinum64, PhinumEngine, Variation};
use super::PartOfSpeech;
use serde::{Deserialize, Serialize};

/// A structural key — the sequence of POS tags for a sentence.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SyntaxKey {
    /// The POS sequence, e.g. [Pronoun, Verb, Preposition, Verb, Noun].
    pub parts: Vec<PartOfSpeech>,
    /// The compact string form, e.g. "PRON+V+PREP+V+N".
    pub key: String,
}

impl SyntaxKey {
    /// Returns the number of tokens in the key.
    pub fn len(&self) -> usize {
        self.parts.len()
    }

    /// Returns true if the key is empty.
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /// Returns the variation slot at Phinum16 level.
    pub fn variation_16(&self) -> Variation {
        Phinum16::classify_str(&self.key)
    }

    /// Returns the variation slot at Phinum32 level.
    pub fn variation_32(&self) -> Variation {
        Phinum32::classify_str(&self.key)
    }

    /// Returns the variation slot at Phinum64 level.
    pub fn variation_64(&self) -> Variation {
        Phinum64::classify_str(&self.key)
    }
}

/// Parses sentences into structural keys.
pub struct SyntaxParser;

impl SyntaxParser {
    /// Parses a sentence into a [`SyntaxKey`].
    pub fn parse(sentence: &str) -> SyntaxKey {
        let tokens: Vec<String> = crate::tokenizer::Tokenizer::tokenize(sentence);
        let parts: Vec<PartOfSpeech> = tokens.iter()
            .map(|t| PartOfSpeech::classify(t))
            .collect();
        let key = parts.iter()
            .map(|p| p.code())
            .collect::<Vec<_>>()
            .join("+");
        SyntaxKey { parts, key }
    }

    /// Parses multiple sentences and returns their structural keys.
    pub fn parse_many(text: &str) -> Vec<SyntaxKey> {
        crate::tokenizer::Tokenizer::split_sentences(text)
            .iter()
            .map(|s| Self::parse(s))
            .collect()
    }
}
