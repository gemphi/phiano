use crate::phasor::SpectralPhasor;
use crate::wave::{c64, Wave};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Facet - the core lexicon mapping words to complex phasors.
///
/// Each word in the facet has a `SpectralPhasor` representing its position
/// in a continuous 2*pi phase space. Semantic similarity between words is
/// measured by destructive interference (energy delta) between their
/// complex wave representations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Facet {
    /// The word-to-phasor lexicon.
    pub lexicon: HashMap<String, SpectralPhasor>,
    /// Bigram transition counts: word_a -> {word_b -> count}.
    #[serde(default)]
    pub bigrams: HashMap<String, HashMap<String, u32>>,
    /// Trigram transition counts: "word_a word_b" -> {word_c -> count}.
    /// Key is two words joined by a space for O(1) lookup.
    #[serde(default)]
    pub trigrams: HashMap<String, HashMap<String, u32>>,
    /// Learned Kuramoto-Sakaguchi phase lags β_ij for word_a → word_b.
    #[serde(default)]
    pub phase_lags: HashMap<String, HashMap<String, f64>>,
}

impl Facet {
    /// Creates an empty facet with no words.
    pub fn new() -> Self {
        Self {
            lexicon: HashMap::new(),
            bigrams: HashMap::new(),
            trigrams: HashMap::new(),
            phase_lags: HashMap::new(),
        }
    }

    /// Returns the number of words in the lexicon.
    pub fn vocabulary_size(&self) -> usize {
        self.lexicon.len()
    }

    /// Returns true if the facet contains the given word.
    pub fn contains_word(&self, word: &str) -> bool {
        self.lexicon.contains_key(word)
    }

    /// Returns a reference to the phasor for the given word, if it exists.
    pub fn get_phasor(&self, word: &str) -> Option<&SpectralPhasor> {
        self.lexicon.get(word)
    }

    /// Gets or initializes a phasor for a word at a deterministic seed phase.
    pub fn get_or_init(&mut self, word: &str) -> &mut SpectralPhasor {
        self.lexicon.entry(word.to_string()).or_insert_with(|| {
            let seed_phase = (word.len() as f64 * crate::config::PHI) % (2.0 * std::f64::consts::PI);
            SpectralPhasor::new(seed_phase, crate::config::AMPLITUDE_INITIAL, crate::config::BAND_N_INITIAL)
        })
    }

    /// Computes the average amplitude across all phasors in the lexicon.
    ///
    /// Returns 0.0 if the lexicon is empty.
    pub fn average_amplitude(&self) -> f64 {
        match self.lexicon.is_empty() {
            true => return 0.0,
            false => {}
        }
        let total: f64 = self.lexicon.values().map(|p| p.amplitude).sum();
        total / self.lexicon.len() as f64
    }

    /// Returns the most common band level (n) across all phasors.
    ///
    /// This indicates the dominant energy sub-band in the facet.
    /// Returns 1 if the lexicon is empty.
    pub fn dominant_band(&self) -> u32 {
        let mut counts: HashMap<u32, u32> = HashMap::new();
        for phasor in self.lexicon.values() {
            *counts.entry(phasor.band_n).or_insert(0) += 1;
        }
        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(band, _)| band)
            .unwrap_or(1)
    }

    /// Computes the centroid wave - the sum of all phasor complex representations.
    ///
    /// The centroid represents the "center of mass" of the facet's semantic space.
    /// Returns zero if the lexicon is empty.
    pub fn centroid(&self) -> c64 {
        match self.lexicon.is_empty() {
            true => return Wave::zero(),
            false => {}
        }
        Wave::from_sum(self.lexicon.values().map(|p| p.to_complex()))
    }

    /// Records a bigram (word_a, word_b) co-occurrence, incrementing its count.
    pub fn record_bigram(&mut self, word_a: &str, word_b: &str) {
        match word_a == word_b {
            true => return,
            false => {}
        }
        *self.bigrams
            .entry(word_a.to_string())
            .or_default()
            .entry(word_b.to_string())
            .or_insert(0) += 1;
    }

    /// Returns candidate next words given a current word, sorted by transition count (descending).
    /// Only returns words that exist in the lexicon.
    pub fn next_word_candidates(&self, current: &str) -> Vec<(String, u32)> {
        match self.bigrams.get(current) {
            Some(followers) => followers
                .iter()
                .filter(|(w, _)| self.lexicon.contains_key(*w))
                .map(|(w, c)| (w.clone(), *c))
                .collect(),
            None => Vec::new(),
        }
    }

    /// Records a trigram (word_a, word_b, word_c) co-occurrence.
    pub fn record_trigram(&mut self, word_a: &str, word_b: &str, word_c: &str) {
        match word_a == word_c || word_b == word_c {
            true => return,
            false => {}
        }
        let key = format!("{} {}", word_a, word_b);
        *self.trigrams
            .entry(key)
            .or_default()
            .entry(word_c.to_string())
            .or_insert(0) += 1;
    }

    /// Returns candidate next words given two preceding words, sorted by count.
    pub fn trigram_candidates(&self, word_a: &str, word_b: &str) -> Vec<(String, u32)> {
        let key = format!("{} {}", word_a, word_b);
        match self.trigrams.get(&key) {
            Some(followers) => followers
                .iter()
                .filter(|(w, _)| self.lexicon.contains_key(*w))
                .map(|(w, c)| (w.clone(), *c))
                .collect(),
            None => Vec::new(),
        }
    }

    /// Returns the transition probability for a trigram.
    #[allow(dead_code)]
    pub fn trigram_probability(&self, word_a: &str, word_b: &str, word_c: &str) -> f64 {
        let key = format!("{} {}", word_a, word_b);
        match self.trigrams.get(&key) {
            Some(followers) => {
                let total: u32 = followers.values().sum();
                match total == 0 {
                    true => 0.0,
                    false => followers.get(word_c).map(|c| *c as f64 / total as f64).unwrap_or(0.0),
                }
            }
            None => 0.0,
        }
    }

    /// Returns the transition probability from word_a to word_b.
    /// Returns 0.0 if no bigram exists.
    pub fn bigram_probability(&self, word_a: &str, word_b: &str) -> f64 {
        match self.bigrams.get(word_a) {
            Some(followers) => {
                let total: u32 = followers.values().sum();
                match total == 0 {
                    true => 0.0,
                    false => followers
                        .get(word_b)
                        .map(|c| *c as f64 / total as f64)
                        .unwrap_or(0.0),
                }
            }
            None => 0.0,
        }
    }

    /// Records an observed word-order lag and blends it into β_ij.
    pub fn record_phase_lag(&mut self, word_a: &str, word_b: &str, observed: f64) {
        match word_a == word_b {
            true => return,
            false => {}
        }
        let entry = self
            .phase_lags
            .entry(word_a.to_string())
            .or_default()
            .entry(word_b.to_string())
            .or_insert(crate::config::SYNTACTIC_LAG_BETA);
        let rate = crate::config::SYNTAX_LAG_LEARN_RATE;
        *entry = (*entry * (1.0 - rate) + observed * rate).rem_euclid(crate::config::TWO_PI);
    }

    /// Returns the learned syntactic lag β_ij, or the default subject→verb lag.
    pub fn phase_lag(&self, word_a: &str, word_b: &str) -> f64 {
        self.phase_lags
            .get(word_a)
            .and_then(|m| m.get(word_b))
            .copied()
            .unwrap_or(crate::config::SYNTACTIC_LAG_BETA)
    }
}

impl Default for Facet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_facet() {
        let f = Facet::new();
        assert_eq!(f.vocabulary_size(), 0);
        assert!(!f.contains_word("hello"));
        assert_eq!(f.average_amplitude(), 0.0);
        assert_eq!(f.dominant_band(), 1);
    }

    #[test]
    fn test_add_and_query() {
        let mut f = Facet::new();
        f.lexicon.insert("hello".into(), SpectralPhasor::new(0.5, 1.0, 2));
        assert_eq!(f.vocabulary_size(), 1);
        assert!(f.contains_word("hello"));
        assert!(!f.contains_word("world"));
        assert_eq!(f.get_phasor("hello").unwrap().band_n, 2);
    }

    #[test]
    fn test_average_amplitude() {
        let mut f = Facet::new();
        f.lexicon.insert("a".into(), SpectralPhasor::new(0.0, 1.0, 1));
        f.lexicon.insert("b".into(), SpectralPhasor::new(0.0, 3.0, 1));
        assert!((f.average_amplitude() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_dominant_band() {
        let mut f = Facet::new();
        f.lexicon.insert("a".into(), SpectralPhasor::new(0.0, 1.0, 1));
        f.lexicon.insert("b".into(), SpectralPhasor::new(0.0, 1.0, 3));
        f.lexicon.insert("c".into(), SpectralPhasor::new(0.0, 1.0, 3));
        assert_eq!(f.dominant_band(), 3);
    }
}
