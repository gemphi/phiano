mod spectral;

#[cfg(test)]
mod tests;

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
    /// Version of the definition-grounding pass already applied to this facet.
    /// Grounding is skipped at startup when this matches `GROUNDING_VERSION`.
    #[serde(default)]
    pub grounded_version: u32,
    /// Flat vocabulary list used for frequency-biased negative sampling.
    /// Rebuilt on demand; never persisted.
    #[serde(skip)]
    pub sample_pool: Vec<String>,
}

impl Facet {
    /// Creates an empty facet with no words.
    pub fn new() -> Self {
        Self {
            lexicon: HashMap::new(),
            bigrams: HashMap::new(),
            trigrams: HashMap::new(),
            phase_lags: HashMap::new(),
            grounded_version: 0,
            sample_pool: Vec::new(),
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

    /// Gets or initializes a phasor, seeded from the word's *identity*.
    ///
    /// Seeding previously used `word.len() * PHI`, which depends only on
    /// character length: `cat`, `the`, `dog` and `war` all began at exactly
    /// 4.854102 rad, and a 100k vocabulary started from about twenty distinct
    /// positions. [`SpectralPhasor::seeded`] hashes the word instead, giving
    /// every word its own position in every channel.
    pub fn get_or_init(&mut self, word: &str) -> &mut SpectralPhasor {
        self.lexicon.entry(word.to_string()).or_insert_with(|| {
            SpectralPhasor::seeded(
                word,
                crate::config::AMPLITUDE_INITIAL,
                crate::config::BAND_N_INITIAL,
            )
        })
    }

    /// Fills phase channels for any phasor loaded from a pre-multi-channel
    /// model file, preserving each word's learned base phase on channel 0.
    /// Returns the number of phasors migrated.
    pub fn migrate_channels(&mut self) -> usize {
        let words: Vec<String> = self
            .lexicon
            .iter()
            .filter(|(_, p)| p.channels_unset())
            .map(|(w, _)| w.clone())
            .collect();
        for w in &words {
            if let Some(p) = self.lexicon.get_mut(w) {
                p.ensure_channels(w);
            }
        }
        words.len()
    }

    /// Rebuilds the flat sampling pool if it has drifted from the lexicon.
    ///
    /// Words are repeated in proportion to `sqrt(count)`, which is the usual
    /// compromise between uniform and unigram sampling: frequent words are
    /// drawn more often, but not so much that rare words are never negatives.
    pub fn rebuild_sample_pool(&mut self) {
        let target = self.lexicon.len();
        if !self.sample_pool.is_empty() && self.sample_pool.len() >= target {
            return;
        }
        let mut pool = Vec::with_capacity(target * 2);
        for (word, phasor) in &self.lexicon {
            let reps = ((phasor.count as f64).sqrt().round() as usize).clamp(1, 8);
            for _ in 0..reps {
                pool.push(word.clone());
            }
        }
        self.sample_pool = pool;
    }

    /// Draws a frequency-biased negative sample. Returns `None` until
    /// [`Facet::rebuild_sample_pool`] has been called at least once.
    #[inline]
    pub fn sample_negative(&self, r: u64) -> Option<&String> {
        if self.sample_pool.is_empty() {
            return None;
        }
        self.sample_pool.get((r % self.sample_pool.len() as u64) as usize)
    }

    /// Mean phase coherence between two words across all channels.
    /// Returns 0.0 if either word is unknown.
    pub fn resonance(&self, word_a: &str, word_b: &str) -> f64 {
        match (self.lexicon.get(word_a), self.lexicon.get(word_b)) {
            (Some(a), Some(b)) => a.resonance(b),
            _ => 0.0,
        }
    }

    /// Circular dispersion of the lexicon's channel-0 phases.
    ///
    /// 1.0 means phases are spread uniformly around the circle; 0.0 means every
    /// word sits at the same angle. Kuramoto coupling is attraction-only, so
    /// dispersion falling toward zero while `coherence` rises is the signature
    /// of the manifold collapsing rather than learning. Log both.
    pub fn phase_dispersion(&self) -> f64 {
        let n = self.lexicon.len();
        if n == 0 {
            return 1.0;
        }
        let (sx, sy) = self
            .lexicon
            .values()
            .fold((0.0f64, 0.0f64), |(x, y), p| (x + p.phase.cos(), y + p.phase.sin()));
        1.0 - (sx.hypot(sy) / n as f64)
    }

    /// Gini coefficient of sector occupancy — 0.0 is perfectly even, 1.0 means
    /// one sector holds the entire vocabulary.
    pub fn sector_gini(&self) -> f64 {
        let n_sectors = crate::config::SECTOR_RESOLUTION as usize;
        let width = crate::config::TWO_PI / n_sectors as f64;
        let mut hist = vec![0u64; n_sectors];
        for p in self.lexicon.values() {
            let s = (p.phase / width).floor() as usize % n_sectors;
            hist[s] += 1;
        }
        hist.sort_unstable();
        let total: u64 = hist.iter().sum();
        if total == 0 {
            return 0.0;
        }
        let n = hist.len() as f64;
        let mut cum = 0.0;
        for (i, &c) in hist.iter().enumerate() {
            cum += (2.0 * (i as f64 + 1.0) - n - 1.0) * c as f64;
        }
        (cum / (n * total as f64)).clamp(0.0, 1.0)
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

    /// Absolute discount used by the smoothed n-gram estimators.
    const DISCOUNT: f64 = 0.75;

    /// Smoothed transition probability from `word_a` to `word_b`.
    ///
    /// Absolute discounting with a back-off mass, so an unseen bigram is
    /// improbable rather than impossible. [`Facet::bigram_probability`] is raw
    /// maximum likelihood and returns exactly 0.0 for anything unseen, which
    /// makes held-out likelihood infinite and the model brittle off
    /// distribution — the problem thirty years of language-model research is
    /// about, and the answer is known.
    ///
    /// Returns `(discounted_probability, backoff_mass)`. Callers distribute the
    /// back-off mass over whatever base distribution they prefer.
    pub fn bigram_discounted(&self, word_a: &str, word_b: &str) -> (f64, f64) {
        match self.bigrams.get(word_a) {
            None => (0.0, 1.0),
            Some(followers) => {
                let total: u32 = followers.values().sum();
                if total == 0 {
                    return (0.0, 1.0);
                }
                let n = *followers.get(word_b).unwrap_or(&0) as f64;
                let types = followers.len() as f64;
                let lambda = Self::DISCOUNT * types / total as f64;
                (((n - Self::DISCOUNT).max(0.0)) / total as f64, lambda)
            }
        }
    }

    /// Smoothed trigram probability, as `(discounted, backoff_mass)`.
    pub fn trigram_discounted(&self, word_a: &str, word_b: &str, word_c: &str) -> (f64, f64) {
        let key = format!("{} {}", word_a, word_b);
        match self.trigrams.get(&key) {
            None => (0.0, 1.0),
            Some(followers) => {
                let total: u32 = followers.values().sum();
                if total == 0 {
                    return (0.0, 1.0);
                }
                let n = *followers.get(word_c).unwrap_or(&0) as f64;
                let types = followers.len() as f64;
                let lambda = Self::DISCOUNT * types / total as f64;
                (((n - Self::DISCOUNT).max(0.0)) / total as f64, lambda)
            }
        }
    }

    /// Drops n-grams seen only once, and any context left empty.
    ///
    /// **This is a size/quality trade, not a free win.** Measured on the Rust
    /// Book corpus (7,757 sentences, 6,016 vocabulary): the table shrinks 80.7%
    /// — 136,807 entries to 26,338 — and held-out perplexity worsens 81%,
    /// from 148.92 to 269.89.
    ///
    /// The reason is corpus size. On a small corpus most n-grams *are*
    /// singletons and they carry most of the coverage, so discarding them
    /// discards the model. Pruning pays off only where repetition is heavy
    /// enough that singletons are genuinely noise. Prefer vocabulary interning
    /// for footprint, which is lossless.
    ///
    /// Returns `(bigrams_dropped, trigrams_dropped)`.
    pub fn prune_singletons(&mut self) -> (usize, usize) {
        let mut bi_dropped = 0usize;
        for followers in self.bigrams.values_mut() {
            let before = followers.len();
            followers.retain(|_, c| *c > 1);
            bi_dropped += before - followers.len();
        }
        self.bigrams.retain(|_, f| !f.is_empty());

        let mut tri_dropped = 0usize;
        for followers in self.trigrams.values_mut() {
            let before = followers.len();
            followers.retain(|_, c| *c > 1);
            tri_dropped += before - followers.len();
        }
        self.trigrams.retain(|_, f| !f.is_empty());

        // Phase lags are keyed by the same pairs; drop any whose bigram is gone.
        let live: std::collections::HashSet<&String> = self.bigrams.keys().collect();
        let stale: Vec<String> = self
            .phase_lags
            .keys()
            .filter(|k| !live.contains(k))
            .cloned()
            .collect();
        for k in stale {
            self.phase_lags.remove(&k);
        }

        (bi_dropped, tri_dropped)
    }

    /// Total number of stored n-gram entries, across both tables.
    pub fn ngram_entries(&self) -> usize {
        self.bigrams.values().map(|f| f.len()).sum::<usize>()
            + self.trigrams.values().map(|f| f.len()).sum::<usize>()
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
