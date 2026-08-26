//! Sentence structure parser and keyed Part-of-Speech dictionary.
//!
//! Maps sentences to structural keys (e.g. `PRON+V+PREP+V+N`) using keyed
//! HashMaps and HashSets rather than hardcoded pattern blocks.
//!
//! These structural keys are the foundation of the spider net —
//! capturing the *shape* and *relationships* of language without storing raw text.

use super::variants::{Phinum16, Phinum32, Phinum64, PhinumEngine, Variation};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

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

    /// Classifies a word into a POS tag using the global keyed dictionary.
    pub fn classify(word: &str) -> Self {
        PosDictionary::global().classify(word)
    }
}

// Keyed word list definitions organized by grammatical class
const PRONOUN_KEYS: &[&str] = &[
    "i", "you", "he", "she", "it", "we", "they",
    "me", "him", "her", "us", "them",
    "my", "your", "his", "its", "our", "their",
    "mine", "yours", "hers", "ours", "theirs",
    "this", "that", "these", "those",
    "who", "whom", "which", "what",
];

const DETERMINER_KEYS: &[&str] = &[
    "a", "an", "the", "some", "any", "each", "every",
    "all", "both", "either", "neither", "no",
];

const AUXILIARY_KEYS: &[&str] = &[
    "is", "are", "was", "were", "be", "been", "being",
    "am", "do", "does", "did", "have", "has", "had",
    "can", "could", "will", "would", "shall", "should",
    "may", "might", "must", "ought",
];

const PREPOSITION_KEYS: &[&str] = &[
    "to", "of", "in", "on", "at", "for", "with", "from",
    "as", "by", "about", "into", "through", "during",
    "before", "after", "above", "below", "between",
    "under", "over", "against", "among", "behind",
    "beyond", "within", "without", "upon", "toward",
    "towards", "until", "off", "out", "up", "down",
];

const CONJUNCTION_KEYS: &[&str] = &[
    "and", "or", "but", "if", "so", "than",
    "because", "while", "although", "though", "unless",
    "since", "where", "when", "whether",
];

const INTERJECTION_KEYS: &[&str] = &[
    "oh", "ah", "wow", "hey", "hi", "hello", "bye",
    "yes", "okay", "ok", "please", "thanks",
];

const ADVERB_KEYS: &[&str] = &[
    "not", "very", "really", "just", "also", "too",
    "always", "never", "often", "sometimes", "usually",
    "now", "then", "here", "there", "today", "tomorrow",
    "yesterday", "soon", "late", "early", "quickly",
    "slowly", "well", "badly", "only", "even", "still",
];

const ADJECTIVE_KEYS: &[&str] = &[
    "good", "bad", "great", "small", "large", "big",
    "new", "old", "young", "first", "last", "next",
    "same", "different", "own", "other", "such",
    "more", "most", "less", "least", "many", "much",
    "few", "little", "enough", "whole", "half",
];

const VERB_KEYS: &[&str] = &[
    "want", "hug", "like", "love", "need", "wish", "hope",
    "see", "look", "hear", "listen", "say", "tell", "speak", "ask",
    "know", "think", "believe", "understand", "remember", "forget",
    "go", "come", "make", "take", "give", "get", "find", "use",
    "help", "try", "feel", "run", "walk", "hold", "touch", "embrace",
    "write", "read", "learn", "teach", "play", "live", "stay", "leave",
    "open", "close", "start", "stop", "call", "send", "show", "bring",
];

/// A keyed dictionary mapping words to POS tags and vice versa.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PosDictionary {
    /// Keyed map: word -> PartOfSpeech
    pub word_to_pos: HashMap<String, PartOfSpeech>,
    /// Reverse keyed map: PartOfSpeech -> Set of words
    pub pos_to_words: HashMap<PartOfSpeech, HashSet<String>>,
}

impl PosDictionary {
    /// Creates an empty dictionary.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds the standard default keyed dictionary.
    pub fn default_lexicon() -> Self {
        let mut dict = Self::new();
        dict.register_batch(PRONOUN_KEYS, PartOfSpeech::Pronoun);
        dict.register_batch(DETERMINER_KEYS, PartOfSpeech::Determiner);
        dict.register_batch(AUXILIARY_KEYS, PartOfSpeech::Auxiliary);
        dict.register_batch(PREPOSITION_KEYS, PartOfSpeech::Preposition);
        dict.register_batch(CONJUNCTION_KEYS, PartOfSpeech::Conjunction);
        dict.register_batch(INTERJECTION_KEYS, PartOfSpeech::Interjection);
        dict.register_batch(ADVERB_KEYS, PartOfSpeech::Adverb);
        dict.register_batch(ADJECTIVE_KEYS, PartOfSpeech::Adjective);
        dict.register_batch(VERB_KEYS, PartOfSpeech::Verb);
        dict
    }

    /// Accesses the global static keyed dictionary singleton.
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<PosDictionary> = OnceLock::new();
        INSTANCE.get_or_init(Self::default_lexicon)
    }

    /// Registers a single word under a POS tag.
    pub fn register(&mut self, word: &str, pos: PartOfSpeech) {
        let w = word.trim().to_lowercase();
        if !w.is_empty() {
            self.word_to_pos.insert(w.clone(), pos);
            self.pos_to_words.entry(pos).or_default().insert(w);
        }
    }

    /// Registers a batch of words under a POS tag.
    pub fn register_batch(&mut self, words: &[&str], pos: PartOfSpeech) {
        for &w in words {
            self.register(w, pos);
        }
    }

    /// Queries the POS tag for a given word.
    pub fn lookup(&self, word: &str) -> Option<PartOfSpeech> {
        let w = word.trim().to_lowercase();
        self.word_to_pos.get(&w).copied()
    }

    /// Returns all registered words for a given POS category.
    pub fn words_for(&self, pos: PartOfSpeech) -> Vec<String> {
        self.pos_to_words
            .get(&pos)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Classifies a word using keyed lookup and fallback morphological heuristics.
    pub fn classify(&self, word: &str) -> PartOfSpeech {
        let w = word.trim().to_lowercase();
        if let Some(pos) = self.lookup(&w) {
            return pos;
        }

        if Self::looks_like_verb(&w) {
            PartOfSpeech::Verb
        } else {
            PartOfSpeech::Noun
        }
    }

    fn looks_like_verb(w: &str) -> bool {
        w.ends_with("ing") || w.ends_with("ed") || (w.ends_with('s') && w.len() > 3)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyed_dictionary_lookups() {
        let dict = PosDictionary::default_lexicon();
        assert_eq!(dict.lookup("i"), Some(PartOfSpeech::Pronoun));
        assert_eq!(dict.lookup("the"), Some(PartOfSpeech::Determiner));
        assert_eq!(dict.lookup("is"), Some(PartOfSpeech::Auxiliary));
        assert_eq!(dict.lookup("to"), Some(PartOfSpeech::Preposition));
        assert_eq!(dict.lookup("and"), Some(PartOfSpeech::Conjunction));

        let pronouns = dict.words_for(PartOfSpeech::Pronoun);
        assert!(pronouns.contains(&"i".to_string()));
        assert!(pronouns.contains(&"we".to_string()));
    }

    #[test]
    fn test_dynamic_word_registration() {
        let mut dict = PosDictionary::new();
        dict.register("shall", PartOfSpeech::Auxiliary);
        assert_eq!(dict.lookup("shall"), Some(PartOfSpeech::Auxiliary));
    }

    #[test]
    fn test_syntax_parser_keyed_pipeline() {
        let key = SyntaxParser::parse("i want to hug you");
        assert_eq!(key.key, "PRON+V+PREP+V+PRON");
        assert_eq!(key.parts[0], PartOfSpeech::Pronoun);
        assert_eq!(key.parts[2], PartOfSpeech::Preposition);
    }
}
