mod spectral;

#[cfg(test)]
mod tests;

use crate::phasor::SpectralPhasor;
use crate::wave::{c64, Wave};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A word's interned identifier.
pub type WordId = u32;

/// String-to-id interner for the n-gram tables.
///
/// The tables previously keyed on owned `String`s: every bigram follower stored
/// a full copy of the word, every trigram key stored two, and `phase_lags`
/// duplicated the whole bigram key set a third time. On a real model that was
/// the overwhelming majority of a 92 MB artifact, against a documented 2–12 MB
/// target — and `trigram_candidates` allocated a fresh `format!("{} {}", a, b)`
/// on every *lookup*, once per candidate per generated token.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Vocab {
    ids: HashMap<String, WordId>,
    words: Vec<String>,
}

impl Vocab {
    /// Returns the id for `word`, assigning one if it is new.
    pub fn intern(&mut self, word: &str) -> WordId {
        if let Some(id) = self.ids.get(word) {
            return *id;
        }
        let id = self.words.len() as WordId;
        self.words.push(word.to_string());
        self.ids.insert(word.to_string(), id);
        id
    }

    /// Returns the id for `word` without assigning one.
    #[inline]
    pub fn id(&self, word: &str) -> Option<WordId> {
        self.ids.get(word).copied()
    }

    /// Returns the word for `id`.
    #[inline]
    pub fn word(&self, id: WordId) -> Option<&str> {
        self.words.get(id as usize).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }

    /// The full id→word table, for serialization.
    pub fn words(&self) -> &[String] {
        &self.words
    }

    /// Rebuilds an interner from a stored id→word table.
    pub fn from_words(words: Vec<String>) -> Self {
        let ids = words
            .iter()
            .enumerate()
            .map(|(i, w)| (w.clone(), i as WordId))
            .collect();
        Self { ids, words }
    }
}

/// Counts needed to score one n-gram continuation.
#[derive(Debug, Clone, Copy)]
pub struct NgramStats {
    /// Times this exact continuation was seen.
    pub count: u32,
    /// Times the context was seen at all.
    pub total: u32,
    /// Distinct continuations observed for the context.
    pub types: u32,
}

/// Sorted association list of `(id, count)`.
///
/// A sorted `Vec` with binary search costs 8 bytes per entry against roughly a
/// hundred for a nested `HashMap<String, u32>` entry, and it has no per-context
/// hash-table allocation.
type Followers = Vec<(WordId, u32)>;

#[inline]
fn bump(list: &mut Followers, id: WordId) {
    match list.binary_search_by_key(&id, |(k, _)| *k) {
        Ok(i) => list[i].1 = list[i].1.saturating_add(1),
        Err(i) => list.insert(i, (id, 1)),
    }
}

#[inline]
fn lookup(list: &[(WordId, u32)], id: WordId) -> u32 {
    match list.binary_search_by_key(&id, |(k, _)| *k) {
        Ok(i) => list[i].1,
        Err(_) => 0,
    }
}

#[inline]
fn total_of(list: &[(WordId, u32)]) -> u32 {
    list.iter().map(|(_, c)| *c).sum()
}

/// Facet - the core lexicon mapping words to complex phasors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Facet {
    /// The word-to-phasor lexicon.
    pub lexicon: HashMap<String, SpectralPhasor>,
    /// Interner shared by every n-gram table.
    #[serde(default)]
    pub vocab: Vocab,
    /// Bigram transitions: id(a) → sorted [(id(b), count)].
    #[serde(default)]
    pub bigrams: HashMap<WordId, Followers>,
    /// Trigram transitions: (id(a), id(b)) → sorted [(id(c), count)].
    #[serde(default)]
    pub trigrams: HashMap<(WordId, WordId), Followers>,
    /// Learned Kuramoto-Sakaguchi phase lags β_ij for a → b.
    #[serde(default)]
    pub phase_lags: HashMap<(WordId, WordId), f32>,
    /// Version of the definition-grounding pass already applied to this facet.
    #[serde(default)]
    pub grounded_version: u32,
    /// Flat vocabulary list used for frequency-biased negative sampling.
    #[serde(skip)]
    pub sample_pool: Vec<String>,
}

impl Facet {
    /// Creates an empty facet with no words.
    pub fn new() -> Self {
        Self {
            lexicon: HashMap::new(),
            vocab: Vocab::default(),
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

    /// True when no transition statistics have been recorded yet.
    pub fn has_ngrams(&self) -> bool {
        !self.bigrams.is_empty()
    }

    /// Gets or initializes a phasor, seeded from the word's *identity*.
    ///
    /// Seeding previously used `word.len() * PHI`, which depends only on
    /// character length: `cat`, `the`, `dog` and `war` all began at exactly
    /// 4.854102 rad, and a 100k vocabulary started from about twenty distinct
    /// positions. [`SpectralPhasor::seeded`] hashes the word instead.
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
    pub fn rebuild_sample_pool(&mut self) {
        let target = self.lexicon.len();
        if !self.sample_pool.is_empty() && self.sample_pool.len() >= target {
            return;
        }
        // Sorted, not HashMap-iteration order.
        //
        // `lexicon` is a HashMap, and Rust seeds HashMap hashing randomly per
        // process. Building the pool by iterating it made the negative-sample
        // sequence differ on every run of the same binary on the same data, so
        // training was not reproducible and neither was anything measured on
        // top of it. Two runs of the identical composition experiment returned
        // analogy MRR 0.1066 and 0.0521 — the same ranking, magnitudes a factor
        // of two apart. An experiment whose effect size moves that much between
        // runs cannot support a claim about an effect of that size.
        //
        // Sorting costs one O(V log V) pass per rebuild and makes every
        // downstream measurement checkable.
        let mut words: Vec<(&String, &SpectralPhasor)> = self.lexicon.iter().collect();
        words.sort_unstable_by(|a, b| a.0.cmp(b.0));

        let mut pool = Vec::with_capacity(target * 2);
        for (word, phasor) in words {
            let reps = ((phasor.count as f64).sqrt().round() as usize).clamp(1, 8);
            for _ in 0..reps {
                pool.push(word.clone());
            }
        }
        self.sample_pool = pool;
    }

    /// Draws a frequency-biased negative sample.
    #[inline]
    pub fn sample_negative(&self, r: u64) -> Option<&String> {
        if self.sample_pool.is_empty() {
            return None;
        }
        self.sample_pool.get((r % self.sample_pool.len() as u64) as usize)
    }

    /// A negative sample that is not definitionally related to `target`.
    ///
    /// Uniform negative sampling will sometimes draw a word from the target's
    /// own definition, and the contrastive update then pushes apart exactly the
    /// pair the dictionary says belongs together — the training signal working
    /// against itself. Dict2vec (Tissier et al., EMNLP 2017) calls filtering
    /// these out *controlled negative sampling* and measures it discarding
    /// around 2% of drawn negatives; the count is small, and every one of them
    /// was an update in the wrong direction.
    ///
    /// Falls back to the unfiltered draw after `tries` attempts, so a word whose
    /// definition covers much of the pool cannot stall training.
    pub fn sample_negative_controlled(
        &self,
        r: u64,
        target: &str,
        graph: &crate::conception::DefinitionGraph,
        tries: u32,
    ) -> Option<&String> {
        let mut x = r | 1;
        for _ in 0..tries {
            let cand = self.sample_negative(x)?;
            if !graph.is_related(target, cand) {
                return Some(cand);
            }
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
        }
        self.sample_negative(r)
    }

    /// Mean phase coherence between two words across all channels.
    pub fn resonance(&self, word_a: &str, word_b: &str) -> f64 {
        match (self.lexicon.get(word_a), self.lexicon.get(word_b)) {
            (Some(a), Some(b)) => a.resonance(b),
            _ => 0.0,
        }
    }

    /// Circular dispersion of the lexicon's channel-0 phases.
    ///
    /// 1.0 means phases are spread uniformly around the circle; 0.0 means every
    /// word sits at the same angle. Dispersion falling while `coherence` rises
    /// is the signature of collapse rather than learning.
    pub fn phase_dispersion(&self) -> f64 {
        self.dispersion_above(0)
    }

    /// [`Facet::phase_dispersion`] restricted to words seen at least `floor` times.
    ///
    /// The global figure is dominated by the tail. A vocabulary of 30,000 words
    /// whose 500 most frequent members have collapsed onto one angle still reads
    /// ~0.98, because the 29,500 rare words retain their initialisation and
    /// average away. Every task the model is scored on — next-word ranking,
    /// relational probes, sentence completion — draws its candidates from the
    /// frequent band, so that is the band whose dispersion has to be watched.
    pub fn dispersion_above(&self, floor: u32) -> f64 {
        let (n, sx, sy) = self
            .lexicon
            .values()
            .filter(|p| p.count >= floor)
            .fold((0usize, 0.0f64, 0.0f64), |(n, x, y), p| {
                (n + 1, x + p.phase.cos(), y + p.phase.sin())
            });
        if n == 0 {
            return 1.0;
        }
        1.0 - (sx.hypot(sy) / n as f64)
    }

    /// Dispersion of the `k` most frequent words.
    ///
    /// A rank cut rather than a count cut, for when the absolute frequencies are
    /// corpus-dependent but the shape of the band is not.
    pub fn dispersion_top(&self, k: usize) -> f64 {
        if k == 0 {
            return 1.0;
        }
        let mut counts: Vec<u32> = self.lexicon.values().map(|p| p.count).collect();
        if counts.len() <= k {
            return self.phase_dispersion();
        }
        // Select the k-th largest count; ties admit a few extra words, which is
        // the conservative direction (a wider band can only look more dispersed).
        counts.sort_unstable_by(|a, b| b.cmp(a));
        self.dispersion_above(counts[k - 1])
    }

    /// Gini coefficient of sector occupancy.
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
    pub fn average_amplitude(&self) -> f64 {
        if self.lexicon.is_empty() {
            return 0.0;
        }
        let total: f64 = self.lexicon.values().map(|p| p.amplitude).sum();
        total / self.lexicon.len() as f64
    }

    /// Returns the most common band level (n) across all phasors.
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
    pub fn centroid(&self) -> c64 {
        if self.lexicon.is_empty() {
            return Wave::zero();
        }
        Wave::from_sum(self.lexicon.values().map(|p| p.to_complex()))
    }

    // ── n-gram statistics ──────────────────────────────────────────────────

    /// Records a bigram (word_a, word_b) co-occurrence.
    pub fn record_bigram(&mut self, word_a: &str, word_b: &str) {
        if word_a == word_b {
            return;
        }
        let (a, b) = (self.vocab.intern(word_a), self.vocab.intern(word_b));
        bump(self.bigrams.entry(a).or_default(), b);
    }

    /// Records a trigram (word_a, word_b, word_c) co-occurrence.
    pub fn record_trigram(&mut self, word_a: &str, word_b: &str, word_c: &str) {
        if word_a == word_c || word_b == word_c {
            return;
        }
        let a = self.vocab.intern(word_a);
        let b = self.vocab.intern(word_b);
        let c = self.vocab.intern(word_c);
        bump(self.trigrams.entry((a, b)).or_default(), c);
    }

    /// Candidate next words given a current word, with their counts.
    /// Only returns words that exist in the lexicon.
    pub fn next_word_candidates(&self, current: &str) -> Vec<(String, u32)> {
        let id = match self.vocab.id(current) {
            Some(i) => i,
            None => return Vec::new(),
        };
        match self.bigrams.get(&id) {
            None => Vec::new(),
            Some(list) => list
                .iter()
                .filter_map(|(wid, c)| self.vocab.word(*wid).map(|w| (w, *c)))
                .filter(|(w, _)| self.lexicon.contains_key(*w))
                .map(|(w, c)| (w.to_string(), c))
                .collect(),
        }
    }

    /// Candidate next words given two preceding words.
    ///
    /// The context key is a `(u32, u32)` tuple, so this no longer allocates a
    /// joined `String` on every call — which, in the decode loop, was one heap
    /// allocation per candidate per generated token.
    pub fn trigram_candidates(&self, word_a: &str, word_b: &str) -> Vec<(String, u32)> {
        let key = match (self.vocab.id(word_a), self.vocab.id(word_b)) {
            (Some(a), Some(b)) => (a, b),
            _ => return Vec::new(),
        };
        match self.trigrams.get(&key) {
            None => Vec::new(),
            Some(list) => list
                .iter()
                .filter_map(|(wid, c)| self.vocab.word(*wid).map(|w| (w, *c)))
                .filter(|(w, _)| self.lexicon.contains_key(*w))
                .map(|(w, c)| (w.to_string(), c))
                .collect(),
        }
    }

    /// Raw maximum-likelihood trigram probability. Returns 0.0 for unseen
    /// continuations; prefer [`Facet::trigram_discounted`] for scoring.
    pub fn trigram_probability(&self, word_a: &str, word_b: &str, word_c: &str) -> f64 {
        match self.trigram_stats(word_a, word_b, word_c) {
            Some(s) if s.total > 0 => s.count as f64 / s.total as f64,
            _ => 0.0,
        }
    }

    /// Raw maximum-likelihood bigram probability.
    pub fn bigram_probability(&self, word_a: &str, word_b: &str) -> f64 {
        match self.bigram_stats(word_a, word_b) {
            Some(s) if s.total > 0 => s.count as f64 / s.total as f64,
            _ => 0.0,
        }
    }

    /// Counts for one bigram continuation, or `None` if the context is unseen.
    pub fn bigram_stats(&self, word_a: &str, word_b: &str) -> Option<NgramStats> {
        let a = self.vocab.id(word_a)?;
        let list = self.bigrams.get(&a)?;
        let b = self.vocab.id(word_b);
        Some(NgramStats {
            count: b.map(|b| lookup(list, b)).unwrap_or(0),
            total: total_of(list),
            types: list.len() as u32,
        })
    }

    /// Counts for one trigram continuation, or `None` if the context is unseen.
    pub fn trigram_stats(&self, word_a: &str, word_b: &str, word_c: &str) -> Option<NgramStats> {
        let key = (self.vocab.id(word_a)?, self.vocab.id(word_b)?);
        let list = self.trigrams.get(&key)?;
        let c = self.vocab.id(word_c);
        Some(NgramStats {
            count: c.map(|c| lookup(list, c)).unwrap_or(0),
            total: total_of(list),
            types: list.len() as u32,
        })
    }

    /// Absolute discount used by the smoothed n-gram estimators.
    const DISCOUNT: f64 = 0.75;

    /// Smoothed bigram probability as `(discounted, backoff_mass)`.
    ///
    /// The raw estimator returns exactly 0.0 for an unseen continuation, which
    /// makes held-out likelihood infinite and the model brittle off
    /// distribution. Callers distribute the back-off mass over whatever base
    /// distribution they prefer.
    pub fn bigram_discounted(&self, word_a: &str, word_b: &str) -> (f64, f64) {
        match self.bigram_stats(word_a, word_b) {
            Some(s) if s.total > 0 => {
                let lambda = Self::DISCOUNT * s.types as f64 / s.total as f64;
                (((s.count as f64 - Self::DISCOUNT).max(0.0)) / s.total as f64, lambda)
            }
            _ => (0.0, 1.0),
        }
    }

    /// Smoothed trigram probability as `(discounted, backoff_mass)`.
    pub fn trigram_discounted(&self, word_a: &str, word_b: &str, word_c: &str) -> (f64, f64) {
        match self.trigram_stats(word_a, word_b, word_c) {
            Some(s) if s.total > 0 => {
                let lambda = Self::DISCOUNT * s.types as f64 / s.total as f64;
                (((s.count as f64 - Self::DISCOUNT).max(0.0)) / s.total as f64, lambda)
            }
            _ => (0.0, 1.0),
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
    /// discards the model. Prefer vocabulary interning for footprint, which is
    /// lossless.
    ///
    /// Returns `(bigrams_dropped, trigrams_dropped)`.
    pub fn prune_singletons(&mut self) -> (usize, usize) {
        let mut bi_dropped = 0usize;
        for list in self.bigrams.values_mut() {
            let before = list.len();
            list.retain(|(_, c)| *c > 1);
            bi_dropped += before - list.len();
        }
        self.bigrams.retain(|_, l| !l.is_empty());

        let mut tri_dropped = 0usize;
        for list in self.trigrams.values_mut() {
            let before = list.len();
            list.retain(|(_, c)| *c > 1);
            tri_dropped += before - list.len();
        }
        self.trigrams.retain(|_, l| !l.is_empty());

        self.phase_lags.retain(|(a, _), _| self.bigrams.contains_key(a));

        (bi_dropped, tri_dropped)
    }

    /// Pointwise mutual information of an adjacent pair, in nats.
    ///
    /// High PMI marks a pair whose meaning is not the sum of its parts —
    /// `hot dog`, `borrow checker` — which is exactly where an additive
    /// sentence representation fails and a multiplicative binding is wanted.
    pub fn pmi(&self, word_a: &str, word_b: &str) -> f64 {
        let joint = self.bigram_stats(word_a, word_b).map(|s| s.count).unwrap_or(0) as f64;
        if joint <= 0.0 {
            return f64::NEG_INFINITY;
        }
        let total: f64 = self
            .bigrams
            .values()
            .map(|l| l.iter().map(|(_, c)| *c as f64).sum::<f64>())
            .sum();
        if total <= 0.0 {
            return f64::NEG_INFINITY;
        }
        let ca = self.bigram_stats(word_a, word_a).map(|s| s.total).unwrap_or(0) as f64;
        let cb: f64 = self
            .vocab
            .id(word_b)
            .map(|bid| {
                self.bigrams
                    .values()
                    .map(|l| lookup(l, bid) as f64)
                    .sum::<f64>()
            })
            .unwrap_or(0.0);
        if ca <= 0.0 || cb <= 0.0 {
            return f64::NEG_INFINITY;
        }
        ((joint / total) / ((ca / total) * (cb / total))).ln()
    }

    /// Total number of stored n-gram entries, across both tables.
    pub fn ngram_entries(&self) -> usize {
        self.bigrams.values().map(|l| l.len()).sum::<usize>()
            + self.trigrams.values().map(|l| l.len()).sum::<usize>()
    }

    /// Directional asymmetry of a word pair, from counts alone, in [-1, 1].
    ///
    /// `+1` means `b` only ever follows `a`; `-1` the reverse; `0` that the two
    /// orders are equally common. This is the syntactic fact β is meant to
    /// encode, and it is a property of the corpus rather than of the manifold.
    pub fn order_asymmetry(&self, word_a: &str, word_b: &str) -> f64 {
        let fwd = self.bigram_stats(word_a, word_b).map(|s| s.count).unwrap_or(0) as f64;
        let rev = self.bigram_stats(word_b, word_a).map(|s| s.count).unwrap_or(0) as f64;
        match fwd + rev > 0.0 {
            true => (fwd - rev) / (fwd + rev),
            false => 0.0,
        }
    }

    /// The lag β_ij *should* take for this pair, anchored outside the geometry.
    ///
    /// The observed lag was previously measured as `θ_b − θ_a` — the difference
    /// between the very phases the β term is pushing around. β chased the
    /// phases, the phases were moved by β, and nothing outside the loop held it
    /// in place, so it converged on zero: the collapse fixed point expressed in
    /// the syntax layer. Anchoring to count asymmetry gives it something to
    /// converge *to*.
    pub fn target_phase_lag(&self, word_a: &str, word_b: &str) -> f64 {
        crate::config::SYNTACTIC_LAG_BETA * self.order_asymmetry(word_a, word_b)
    }

    /// Records an observed word-order lag and blends it into β_ij.
    pub fn record_phase_lag(&mut self, word_a: &str, word_b: &str, observed: f64) {
        if word_a == word_b {
            return;
        }
        let (a, b) = (self.vocab.intern(word_a), self.vocab.intern(word_b));
        let entry = self
            .phase_lags
            .entry((a, b))
            .or_insert(crate::config::SYNTACTIC_LAG_BETA as f32);
        let rate = crate::config::SYNTAX_LAG_LEARN_RATE;
        *entry = ((*entry as f64 * (1.0 - rate) + observed * rate)
            .rem_euclid(crate::config::TWO_PI)) as f32;
    }

    /// Returns the learned syntactic lag β_ij, or the default subject→verb lag.
    pub fn phase_lag(&self, word_a: &str, word_b: &str) -> f64 {
        match (self.vocab.id(word_a), self.vocab.id(word_b)) {
            (Some(a), Some(b)) => self
                .phase_lags
                .get(&(a, b))
                .map(|v| *v as f64)
                .unwrap_or(crate::config::SYNTACTIC_LAG_BETA),
            _ => crate::config::SYNTACTIC_LAG_BETA,
        }
    }
}

impl Default for Facet {
    fn default() -> Self {
        Self::new()
    }
}
