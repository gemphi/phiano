/// Phinum lexicon — word understanding with 16/32/64 variation levels.
///
/// Each word is classified into a word class, then mapped to variation
/// slots at all three Phinum levels. The lexicon tracks which variations
/// have been seen and builds the spider net of links between them.

use super::syntax::PartOfSpeech;
use super::variants::{Phinum16, Phinum32, Phinum64, PhinumEngine, Variation, PhinumLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Word class — coarse linguistic category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WordClass {
    /// Content word: noun, verb, adjective, adverb.
    Content,
    /// Function word: preposition, conjunction, determiner, auxiliary.
    Function,
    /// Pronoun or deictic.
    Deictic,
    /// Interjection or discourse marker.
    Discourse,
}

impl WordClass {
    pub fn from_pos(pos: PartOfSpeech) -> Self {
        match pos {
            PartOfSpeech::Noun | PartOfSpeech::Verb |
            PartOfSpeech::Adjective | PartOfSpeech::Adverb => Self::Content,
            PartOfSpeech::Preposition | PartOfSpeech::Conjunction |
            PartOfSpeech::Determiner | PartOfSpeech::Auxiliary => Self::Function,
            PartOfSpeech::Pronoun => Self::Deictic,
            PartOfSpeech::Interjection => Self::Discourse,
            PartOfSpeech::Unknown => Self::Content,
        }
    }
}

/// A word's full Phinum classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordEntry {
    pub word: String,
    pub pos: PartOfSpeech,
    pub class: WordClass,
    pub v16: Variation,
    pub v32: Variation,
    pub v64: Variation,
}

impl WordEntry {
    pub fn new(word: &str) -> Self {
        let pos = PartOfSpeech::classify(word);
        let class = WordClass::from_pos(pos);
        Self {
            word: word.to_string(),
            pos,
            class,
            v16: Phinum16::classify_str(word),
            v32: Phinum32::classify_str(word),
            v64: Phinum64::classify_str(word),
        }
    }
}

/// The Phinum lexicon — tracks word classifications and variation links.
///
/// This is the "spider net" that captures language structure without
/// storing examples. It records which variation slots are occupied
/// and which are linked, giving structural instances on demand.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhinumLexicon {
    /// Word → entry map.
    pub entries: HashMap<String, WordEntry>,
    /// Links between variation slots at each level.
    /// Key: (level, index_a * count + index_b).
    pub links: HashMap<u64, u32>,
}

impl PhinumLexicon {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a word in the lexicon, creating variation links.
    pub fn register(&mut self, word: &str) -> &WordEntry {
        if !self.entries.contains_key(word) {
            let entry = WordEntry::new(word);
            self.entries.insert(word.to_string(), entry);
        }
        self.entries.get(word).unwrap()
    }

    /// Links two words' variation slots at a given level.
    pub fn link(&mut self, a: &str, b: &str, level: PhinumLevel) {
        let va = self.register(a).clone();
        let vb = self.register(b).clone();
        let (va, vb) = match level {
            PhinumLevel::N16 => (va.v16, vb.v16),
            PhinumLevel::N32 => (va.v32, vb.v32),
            PhinumLevel::N64 => (va.v64, vb.v64),
        };
        let count = level.count() as u64;
        let key = va.index as u64 * count + vb.index as u64;
        *self.links.entry(key).or_insert(0) += 1;
    }

    /// Returns the link strength between two words at a given level.
    pub fn link_strength(&self, a: &str, b: &str, level: PhinumLevel) -> u32 {
        let (va, vb) = match (self.entries.get(a), self.entries.get(b)) {
            (Some(ea), Some(eb)) => match level {
                PhinumLevel::N16 => (ea.v16, eb.v16),
                PhinumLevel::N32 => (ea.v32, eb.v32),
                PhinumLevel::N64 => (ea.v64, eb.v64),
            },
            _ => return 0,
        };
        let count = level.count() as u64;
        let key = va.index as u64 * count + vb.index as u64;
        self.links.get(&key).copied().unwrap_or(0)
    }

    /// Returns the number of registered words.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the lexicon is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Processes a sentence, registering all words and linking adjacent ones.
    pub fn process_sentence(&mut self, sentence: &str) {
        let tokens = crate::tokenizer::Tokenizer::tokenize(sentence);
        for t in &tokens {
            self.register(t);
        }
        for w in tokens.windows(2) {
            self.link(&w[0], &w[1], PhinumLevel::N16);
            self.link(&w[0], &w[1], PhinumLevel::N32);
            self.link(&w[0], &w[1], PhinumLevel::N64);
        }
    }
}
