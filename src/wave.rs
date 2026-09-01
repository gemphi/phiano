use crate::config;
use crate::config::{GOLDEN_ANGLE, PHASE_CHANNELS, TWO_PI};
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use num_complex::Complex64;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::f64::consts::PI;

/// Type alias for a complex number with f64 real and imaginary parts.
#[allow(non_camel_case_types)]
pub type c64 = Complex64;

/// Wave - operations on complex wave representations of text.
pub struct Wave;

impl Wave {
    /// Returns the zero wave (no amplitude, no phase).
    pub fn zero() -> c64 {
        c64::new(0.0, 0.0)
    }

    /// Sums an iterator of complex waves into a single superposition wave.
    pub fn from_sum(iter: impl Iterator<Item = c64>) -> c64 {
        iter.sum()
    }

    /// Unordered superposition of a list of known words.
    ///
    /// This is a bag: `Z(a b) == Z(b a)`. That is the correct representation for
    /// measuring *agreement* among words — which is what coherence asks — but
    /// not for representing *what was said*. Use [`Wave::sentence_bound`] for
    /// anything where order carries meaning.
    pub fn sentence(facet: &Facet, words: &[String]) -> c64 {
        words
            .iter()
            .filter_map(|w| facet.lexicon.get(w))
            .map(|p| p.to_complex())
            .sum()
    }

    /// Order-sensitive superposition: each word is bound to its position.
    ///
    /// Position `i` rotates the word by `i * GOLDEN_ANGLE` before summing. This
    /// is circular-convolution binding in the phase domain, and the golden angle
    /// is used because its irrationality means no two positions ever collide.
    ///
    /// `Z("dog bites man") != Z("man bites dog")`, which a plain sum cannot
    /// express — and without which negation scope, argument roles and causal
    /// direction are all unrepresentable.
    pub fn sentence_bound(facet: &Facet, words: &[String]) -> c64 {
        words
            .iter()
            .enumerate()
            .filter_map(|(i, w)| facet.lexicon.get(w).map(|p| (i, p)))
            .map(|(i, p)| c64::from_polar(p.amplitude, p.effective_phase() + i as f64 * GOLDEN_ANGLE))
            .sum()
    }

    /// Per-channel superposition across the full phase torus.
    ///
    /// Returns `PHASE_CHANNELS` complex numbers — the multi-channel counterpart
    /// of [`Wave::sentence`]. Set `bound` to make it order-sensitive.
    pub fn sentence_channels(facet: &Facet, words: &[String], bound: bool) -> Vec<c64> {
        let mut out = vec![Self::zero(); PHASE_CHANNELS];
        for (i, w) in words.iter().enumerate() {
            if let Some(p) = facet.lexicon.get(w) {
                let roll = if bound { i as f64 * GOLDEN_ANGLE } else { 0.0 };
                for k in 0..PHASE_CHANNELS {
                    out[k] += c64::from_polar(p.amplitude, p.theta(k) + roll);
                }
            }
        }
        out
    }

    /// Computes the wave for a raw text string (unordered).
    pub fn text(facet: &Facet, text: &str) -> c64 {
        let tokens = Tokenizer::tokenize(text);
        Self::sentence(facet, &tokens)
    }

    /// Computes the order-sensitive wave for a raw text string.
    pub fn text_bound(facet: &Facet, text: &str) -> c64 {
        let tokens = Tokenizer::tokenize(text);
        Self::sentence_bound(facet, &tokens)
    }

    /// Ray cast: finds words that resonate with a target word.
    ///
    /// Ranks by mean phase coherence across all channels, blended with
    /// familiarity. The previous metric, `α·|Z_q − Z_w|²`, expands to
    /// `A_q² + A_w² − 2A_qA_w cos Δφ`, so a word with an amplitude close to the
    /// query's could outrank an exact phase match; and the leading α, being a
    /// positive constant on every candidate, could not affect the ranking at all.
    ///
    /// Returns `(word, delta)` sorted ascending, where delta = 1 − score, so
    /// smaller is still better for every existing caller.
    pub fn ray_cast_word(facet: &Facet, target_word: &str, top_k: usize) -> Vec<(String, f64)> {
        let target = match facet.lexicon.get(target_word) {
            Some(p) => *p,
            None => return vec![],
        };

        let mut hits: Vec<(&String, f64)> = facet
            .lexicon
            .par_iter()
            .filter(|(word, _)| *word != target_word)
            .map(|(word, phasor)| {
                let sim = 0.5 * (target.resonance(phasor) + 1.0);
                let fam = phasor.amplitude / config::AMPLITUDE_MAX;
                (word, 1.0 - sim * (0.8 + 0.2 * fam))
            })
            .collect();

        Self::take_smallest(&mut hits, top_k)
    }

    /// Ray cast from an arbitrary complex wave.
    ///
    /// A query wave carries only one angle, so this compares channel 0 —
    /// angular distance, with familiarity as an explicit, tunable secondary
    /// term rather than an artefact of the algebra.
    pub fn ray_cast(facet: &Facet, wave: c64, top_k: usize) -> Vec<(String, f64)> {
        if wave.norm() < 1e-12 {
            return vec![];
        }
        let q = wave.arg();

        let mut hits: Vec<(&String, f64)> = facet
            .lexicon
            .par_iter()
            .map(|(word, phasor)| {
                let d = (q - phasor.effective_phase()).cos().max(0.0);
                let fam = phasor.amplitude / config::AMPLITUDE_MAX;
                (word, 1.0 - d * (0.8 + 0.2 * fam))
            })
            .collect();

        Self::take_smallest(&mut hits, top_k)
    }

    /// Ray cast from a multi-channel query wave (see [`Wave::sentence_channels`]).
    ///
    /// This is the retrieval the torus representation exists for: the query and
    /// every candidate are compared across all `PHASE_CHANNELS`, so the number
    /// of distinguishable outcomes is not bounded by one circle's resolution.
    pub fn ray_cast_channels(facet: &Facet, query: &[c64], top_k: usize) -> Vec<(String, f64)> {
        let qs: Vec<f64> = query.iter().map(|z| z.arg()).collect();
        if qs.is_empty() {
            return vec![];
        }

        let mut hits: Vec<(&String, f64)> = facet
            .lexicon
            .par_iter()
            .map(|(word, phasor)| {
                let mut sum = 0.0;
                for (k, &qk) in qs.iter().enumerate() {
                    sum += (qk - phasor.theta(k)).cos();
                }
                let sim = 0.5 * (sum / qs.len() as f64 + 1.0);
                let fam = phasor.amplitude / config::AMPLITUDE_MAX;
                (word, 1.0 - sim * (0.8 + 0.2 * fam))
            })
            .collect();

        Self::take_smallest(&mut hits, top_k)
    }

    /// Partial-selects the `top_k` smallest deltas, then sorts just those.
    fn take_smallest(hits: &mut Vec<(&String, f64)>, top_k: usize) -> Vec<(String, f64)> {
        if hits.len() > top_k && top_k > 0 {
            hits.select_nth_unstable_by(top_k, |a, b| {
                a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal)
            });
            hits.truncate(top_k);
        }
        hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
        hits.iter().map(|(w, d)| ((*w).clone(), *d)).collect()
    }

    /// Binds a wave to a role angle (phase addition).
    #[inline]
    pub fn bind(z: c64, role_phase: f64) -> c64 {
        z * c64::from_polar(1.0, role_phase)
    }

    /// Unbinds a wave from a role angle (phase subtraction).
    #[inline]
    pub fn unbind(z: c64, role_phase: f64) -> c64 {
        z * c64::from_polar(1.0, -role_phase)
    }

    /// Builds a bound proposition: subject, verb and object each bound to a
    /// distinct role angle, then superposed.
    ///
    /// `query_role` recovers a filler by unbinding. This is what makes
    /// `dog bites man` and `man bites dog` different objects to the model.
    pub fn proposition(facet: &Facet, subject: &str, verb: &str, object: &str) -> c64 {
        let roles = [
            (subject, 0.0),
            (verb, GOLDEN_ANGLE),
            (object, 2.0 * GOLDEN_ANGLE),
        ];
        roles
            .iter()
            .filter_map(|(w, r)| facet.lexicon.get(*w).map(|p| (p, *r)))
            .map(|(p, r)| c64::from_polar(p.amplitude, p.effective_phase() + r))
            .sum()
    }

    /// Recovers the most likely filler of a role from a bound proposition.
    ///
    /// `slot` is 0 for subject, 1 for verb, 2 for object.
    pub fn query_role(facet: &Facet, z: c64, slot: usize, top_k: usize) -> Vec<(String, f64)> {
        Self::ray_cast(facet, Self::unbind(z, slot as f64 * GOLDEN_ANGLE), top_k)
    }

    /// Returns the configured number of phase sectors.
    pub fn sector_count() -> u16 {
        config::PhiConfig::sector_resolution()
    }

    /// Maps a phase angle to a sector index.
    pub fn sector_of(phase: f64) -> u16 {
        let n = Self::sector_count();
        let normalized = phase.rem_euclid(TWO_PI);
        let sector_size = TWO_PI / n as f64;
        (normalized / sector_size).floor() as u16 % n
    }

    /// Returns the antipodal (opposite) sector index.
    pub fn opposite_sector(sector: u16) -> u16 {
        let n = Self::sector_count();
        (sector + n / 2) % n
    }

    /// Returns the sector of a word's effective phase.
    pub fn word_sector(facet: &Facet, word: &str) -> Option<u16> {
        facet.lexicon.get(word).map(|p| Self::sector_of(p.effective_phase()))
    }

    /// Returns the sector of a complex wave's phase angle.
    pub fn wave_sector(wave: c64) -> u16 {
        if wave.norm() < 1e-10 {
            return 0;
        }
        Self::sector_of(wave.arg())
    }

    /// Finds words in a specific sector of the phase circle.
    #[allow(dead_code)]
    pub fn words_in_sector(facet: &Facet, sector: u16, top_k: usize) -> Vec<(String, f64)> {
        let mut hits: Vec<(String, f64)> = facet
            .lexicon
            .par_iter()
            .filter(|(_, p)| Self::sector_of(p.effective_phase()) == sector)
            .map(|(w, p)| (w.clone(), p.amplitude))
            .collect();

        hits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        hits.into_iter().take(top_k).collect()
    }

    /// Occupancy histogram across sectors — the cheapest manifold-health check.
    pub fn sector_histogram(facet: &Facet) -> Vec<usize> {
        let n = Self::sector_count() as usize;
        let mut hist = vec![0usize; n];
        for p in facet.lexicon.values() {
            hist[Self::sector_of(p.effective_phase()) as usize] += 1;
        }
        hist
    }
}

#[cfg(test)]
mod wave_tests {
    use super::*;
    use crate::trainer::Trainer;

    fn facet_with(words: &[&str]) -> Facet {
        let mut f = Facet::new();
        for w in words {
            f.get_or_init(w);
        }
        f
    }

    #[test]
    fn test_plain_sentence_is_order_invariant() {
        let f = facet_with(&["dog", "bites", "man"]);
        let a = Wave::text(&f, "dog bites man");
        let b = Wave::text(&f, "man bites dog");
        assert!((a - b).norm() < 1e-12, "the bag representation is order-free by design");
    }

    #[test]
    fn test_bound_sentence_is_order_sensitive() {
        let f = facet_with(&["dog", "bites", "man"]);
        let a = Wave::text_bound(&f, "dog bites man");
        let b = Wave::text_bound(&f, "man bites dog");
        assert!((a - b).norm() > 0.1, "positional binding must distinguish word order");
    }

    #[test]
    fn test_negation_order_distinguished() {
        let f = facet_with(&["not", "safe"]);
        let a = Wave::text_bound(&f, "not safe");
        let b = Wave::text_bound(&f, "safe not");
        assert!((a - b).norm() > 0.1);
    }

    #[test]
    fn test_role_binding_roundtrip() {
        let mut f = facet_with(&["dog", "bites", "man", "cat", "sleeps"]);
        let t = Trainer::new(0.05);
        t.train_sentence(&mut f, "the dog bites the man");

        let z = Wave::proposition(&f, "dog", "bites", "man");
        let subj = Wave::query_role(&f, z, 0, 5);
        assert!(!subj.is_empty());

        let z2 = Wave::proposition(&f, "man", "bites", "dog");
        assert!((z - z2).norm() > 0.05, "propositions must differ by argument order");
    }

    #[test]
    fn test_ray_cast_prefers_phase_over_amplitude() {
        let mut f = Facet::new();
        // exact phase twin at low familiarity vs a distant word at high familiarity
        let mut twin = crate::phasor::SpectralPhasor::seeded("query", 1.0, 1);
        twin.amplitude = 1.0;
        f.lexicon.insert("twin".into(), twin);

        let mut far = crate::phasor::SpectralPhasor::seeded("query", 1.0, 1);
        for k in 0..PHASE_CHANNELS {
            far.set_theta(k, far.theta(k) + std::f64::consts::PI);
        }
        far.amplitude = crate::config::AMPLITUDE_MAX;
        f.lexicon.insert("far".into(), far);

        f.lexicon.insert("query".into(), crate::phasor::SpectralPhasor::seeded("query", 1.0, 1));

        let hits = Wave::ray_cast_word(&f, "query", 2);
        assert_eq!(hits[0].0, "twin", "an exact phase match must beat a familiar distant word");
    }

    #[test]
    fn test_ray_cast_delta_is_ascending() {
        let f = facet_with(&["alpha", "beta", "gamma", "delta", "epsilon"]);
        let hits = Wave::ray_cast_word(&f, "alpha", 4);
        for w in hits.windows(2) {
            assert!(w[0].1 <= w[1].1, "callers rely on ascending delta");
        }
    }
}
