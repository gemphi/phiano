/// Sentence structure parser — maps sentences to structural keys.
///
/// Parses sentences into part-of-speech sequences like:
///   "i want to hug you" => Subject + Verb + Preposition + Verb + Noun
///
/// These structural keys are the foundation of the spider net —
/// they capture the *shape* of language without storing the content.

use super::variants::{Phinum16, Phinum32, Phinum64, PhinumEngine, Variation};
use serde::{Deserialize, Serialize};

/// Part-of-speech tags used in structural keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartOfSpeech {
    Noun,
    Pronoun,
    Verb,
    Auxiliary,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Determiner,
    Interjection,
    Unknown,
}

impl PartOfSpeech {
    /// Returns the short code for this POS tag.
    pub fn code(self) -> &'static str {
        match self {
            Self::Noun => "N",
            Self::Pronoun => "PRON",
            Self::Verb => "V",
            Self::Auxiliary => "AUX",
            Self::Adjective => "ADJ",
            Self::Adverb => "ADV",
            Self::Preposition => "PREP",
            Self::Conjunction => "CONJ",
            Self::Determiner => "DET",
            Self::Interjection => "INTJ",
            Self::Unknown => "?",
        }
    }

    /// Classifies a word into a POS tag using closed-class heuristics.
    pub fn classify(word: &str) -> Self {
        let w = word.to_lowercase();
        match w.as_str() {
            // Pronouns
            "i" | "you" | "he" | "she" | "it" | "we" | "they" |
            "me" | "him" | "her" | "us" | "them" |
            "my" | "your" | "his" | "its" | "our" | "their" |
            "mine" | "yours" | "hers" | "ours" | "theirs" |
            "this" | "that" | "these" | "those" |
            "who" | "whom" | "which" | "what" => Self::Pronoun,

            // Determiners
            "a" | "an" | "the" | "some" | "any" | "each" | "every" |
            "all" | "both" | "either" | "neither" | "no" => Self::Determiner,

            // Auxiliary verbs
            "is" | "are" | "was" | "were" | "be" | "been" | "being" |
            "am" | "do" | "does" | "did" | "have" | "has" | "had" |
            "can" | "could" | "will" | "would" | "shall" | "should" |
            "may" | "might" | "must" | "ought" => Self::Auxiliary,

            // Prepositions
            "to" | "of" | "in" | "on" | "at" | "for" | "with" | "from" |
            "as" | "by" | "about" | "into" | "through" | "during" |
            "before" | "after" | "above" | "below" | "between" |
            "under" | "over" | "against" | "among" | "behind" |
            "beyond" | "within" | "without" | "upon" | "toward" |
            "towards" | "until" | "off" | "out" | "up" | "down" => Self::Preposition,

            // Conjunctions
            "and" | "or" | "but" | "if" | "so" | "than" |
            "because" | "while" | "although" | "though" | "unless" |
            "since" | "where" | "when" | "whether" => Self::Conjunction,

            // Interjections
            "oh" | "ah" | "wow" | "hey" | "hi" | "hello" | "bye" |
            "yes" | "okay" | "ok" | "please" | "thanks" => Self::Interjection,

            // Adverbs (common)
            "not" | "very" | "really" | "just" | "also" | "too" |
            "always" | "never" | "often" | "sometimes" | "usually" |
            "now" | "then" | "here" | "there" | "today" | "tomorrow" |
            "yesterday" | "soon" | "late" | "early" | "quickly" |
            "slowly" | "well" | "badly" | "only" | "even" | "still" => Self::Adverb,

            // Adjectives (common)
            "good" | "bad" | "great" | "small" | "large" | "big" |
            "new" | "old" | "young" | "first" | "last" | "next" |
            "same" | "different" | "own" | "other" | "such" |
            "more" | "most" | "less" | "least" | "many" | "much" |
            "few" | "little" | "enough" | "whole" | "half" => Self::Adjective,

            _ => {
                if Self::looks_like_verb(&w) {
                    Self::Verb
                } else {
                    Self::Noun
                }
            }
        }
    }

    fn looks_like_verb(w: &str) -> bool {
        w.ends_with("ing") || w.ends_with("ed") || w.ends_with("s")
    }
}

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
