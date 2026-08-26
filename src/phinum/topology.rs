/// Language topology — sentence and paragraph types as the spider net.
///
/// The spider net captures language structure through topological
/// classification. Each sentence maps to a sentence type, each paragraph
/// to a paragraph type. The links between types form the net.
///
/// # Sentence Types (16 base)
///
/// 1. Declarative    — states a fact
/// 2. Interrogative  — asks a question
/// 3. Imperative     — gives a command
/// 4. Exclamatory    — expresses emotion
/// 5-16. Sub-types based on structure and speech act
///
/// # Paragraph Types (16 base)
///
/// 1. Narrative      — tells a story
/// 2. Descriptive    — describes something
/// 3. Expository     — explains something
/// 4. Persuasive     — argues a point
/// 5-16. Sub-types based on sentence composition

use super::iching::Hexagram;
use super::searle::SpeechAct;
use super::syntax::{SyntaxKey, SyntaxParser, PartOfSpeech};
use super::variants::{Phinum16, Phinum32, Phinum64, PhinumEngine, Variation};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
        let v16 = Phinum16::classify_hash(base as u64 * 17);
        let v32 = Phinum32::classify_hash(base as u64 * 31);
        let v64 = Phinum64::classify_hash(base as u64 * 67);

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

    /// Returns the label for this sentence type.
    pub fn label(self) -> &'static str {
        match self.base {
            0 => "declarative",
            1 => "interrogative",
            2 => "imperative",
            3 => "exclamatory",
            4 => "directive",
            5 => "commissive",
            6 => "expressive",
            7 => "declarative-act",
            _ => "complex",
        }
    }
}

/// Paragraph type — 16 base types expandable to 32 and 64.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParagraphType {
    pub base: u8,
    pub v16: Variation,
    pub v32: Variation,
    pub v64: Variation,
}

impl ParagraphType {
    /// Classifies a paragraph from its sentence types.
    pub fn classify(sentence_types: &[SentenceType]) -> Self {
        if sentence_types.is_empty() {
            return Self { base: 0, v16: Phinum16::classify_str(""), v32: Phinum32::classify_str(""), v64: Phinum64::classify_str("") };
        }

        let base = Self::base_type(sentence_types);
        let hash: u64 = sentence_types.iter()
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
        if interrogative as f64 / n > 0.5 { 1 }
        else if imperative as f64 / n > 0.5 { 2 }
        else if exclamatory as f64 / n > 0.5 { 3 }
        else if declarative as f64 / n > 0.7 { 0 }
        else { 4 }
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

/// The language spider net — captures language structure without storing examples.
///
/// Uses keyed HashMap and HashSet mappings to link abstract syntactic keys
/// (`PRON+V+PREP+V+N`) to the 64 classical I Ching hexagrams and their
/// dynamic phase transformations ("spin"). This topology gives structural instances
/// on demand — "64 ways to look at anything of any class."
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguageSpiderNet {
    /// Sentence type variation counts at each level.
    pub sentence_counts: HashMap<u64, u32>,
    /// Paragraph type variation counts at each level.
    pub paragraph_counts: HashMap<u64, u32>,
    /// Links between sentence types (transitions).
    pub type_links: HashMap<u64, u32>,
    /// Keyed map from structural syntax keys to their resonant I Ching Hexagram IDs (0..63).
    pub key_to_hexagrams: HashMap<String, HashSet<u8>>,
    /// Keyed map from Hexagram IDs (0..63) to sets of corresponding syntax keys.
    pub hexagram_to_keys: HashMap<u8, HashSet<String>>,
    /// Transitions between Hexagram harmonic states ("Song of Speech Shapes").
    pub hexagram_transitions: HashMap<u64, u32>,
    /// Speech part shape relations (e.g. "PRON" -> {"V", "AUX"}).
    pub pos_shape_relations: HashMap<String, HashSet<String>>,
    /// Total sentences processed.
    pub total_sentences: u32,
    /// Total paragraphs processed.
    pub total_paragraphs: u32,
}

impl LanguageSpiderNet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Processes a text, extracting syntactic keys and building the 64-hexagram spider net.
    pub fn process_text(&mut self, text: &str) -> ParagraphType {
        let sentences = crate::tokenizer::Tokenizer::split_sentences(text);
        let mut stypes = Vec::new();
        let mut hexagrams = Vec::new();

        for s in &sentences {
            let st = SentenceType::classify(s);
            let key = SyntaxParser::parse(s);
            let hex = Hexagram::from_syntax_key(&key);

            self.record_sentence(&st);
            self.record_syntax_key(&key, hex);

            stypes.push(st);
            hexagrams.push(hex);
        }

        for w in stypes.windows(2) {
            self.link_types(&w[0], &w[1]);
        }

        for w in hexagrams.windows(2) {
            self.link_hexagrams(w[0], w[1]);
        }

        let ptype = ParagraphType::classify(&stypes);
        self.record_paragraph(&ptype);
        ptype
    }

    fn record_sentence(&mut self, st: &SentenceType) {
        self.total_sentences += 1;
        let key = st.v64.index as u64;
        *self.sentence_counts.entry(key).or_insert(0) += 1;
    }

    fn record_paragraph(&mut self, pt: &ParagraphType) {
        self.total_paragraphs += 1;
        let key = pt.v64.index as u64;
        *self.paragraph_counts.entry(key).or_insert(0) += 1;
    }

    fn record_syntax_key(&mut self, key: &SyntaxKey, hex: Hexagram) {
        self.key_to_hexagrams
            .entry(key.key.clone())
            .or_default()
            .insert(hex.id);

        self.hexagram_to_keys
            .entry(hex.id)
            .or_default()
            .insert(key.key.clone());

        for window in key.parts.windows(2) {
            let from_code = window[0].code().to_string();
            let to_code = window[1].code().to_string();
            self.pos_shape_relations
                .entry(from_code)
                .or_default()
                .insert(to_code);
        }
    }

    fn link_types(&mut self, a: &SentenceType, b: &SentenceType) {
        let key = a.v64.index as u64 * 64 + b.v64.index as u64;
        *self.type_links.entry(key).or_insert(0) += 1;
    }

    fn link_hexagrams(&mut self, a: Hexagram, b: Hexagram) {
        let key = a.id as u64 * 64 + b.id as u64;
        *self.hexagram_transitions.entry(key).or_insert(0) += 1;
    }

    /// Spins a structural key by $\Delta\theta$ to find resonant alternative syntax forms.
    pub fn spin_structure(&self, syntax_key: &str, delta_phase: f64) -> Vec<String> {
        if let Some(hex_ids) = self.key_to_hexagrams.get(syntax_key) {
            let mut results = HashSet::new();
            for &id in hex_ids {
                let spun_hex = Hexagram::from_id(id).spin(delta_phase);
                if let Some(keys) = self.hexagram_to_keys.get(&spun_hex.id) {
                    for k in keys {
                        results.insert(k.clone());
                    }
                }
            }
            results.into_iter().collect()
        } else {
            Vec::new()
        }
    }

    /// Returns the diversity of sentence types seen (0.0 to 1.0).
    pub fn sentence_diversity(&self) -> f64 {
        if self.total_sentences == 0 { return 0.0; }
        let unique = self.sentence_counts.len() as f64;
        unique / 64.0
    }

    /// Returns the diversity of paragraph types seen (0.0 to 1.0).
    pub fn paragraph_diversity(&self) -> f64 {
        if self.total_paragraphs == 0 { return 0.0; }
        let unique = self.paragraph_counts.len() as f64;
        unique / 64.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spider_net_keyed_relations() {
        let mut net = LanguageSpiderNet::new();
        let _ptype = net.process_text("I want to hug you. Can you hear me? Yes indeed!");

        assert_eq!(net.total_sentences, 3);
        assert_eq!(net.total_paragraphs, 1);
        assert!(!net.key_to_hexagrams.is_empty());
        assert!(!net.hexagram_to_keys.is_empty());
        assert!(!net.pos_shape_relations.is_empty());

        let pron_relations = net.pos_shape_relations.get("PRON");
        assert!(pron_relations.is_some());

        assert!(net.sentence_diversity() > 0.0);
        assert!(net.paragraph_diversity() > 0.0);
    }

    #[test]
    fn test_spider_net_spin_structure() {
        let mut net = LanguageSpiderNet::new();
        net.process_text("I want to hug you. We need to love them.");

        let syntax_keys: Vec<_> = net.key_to_hexagrams.keys().cloned().collect();
        assert!(!syntax_keys.is_empty());

        let spun = net.spin_structure(&syntax_keys[0], 0.0);
        assert!(!spun.is_empty());
    }
}
