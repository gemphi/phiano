use crate::config;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use num_complex::Complex64;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::f64::consts::PI;

/// Number of sectors dividing the 2π phase circle.
///
/// Each sector spans 2π/N radians, where N is the configured resolution.
/// Configurable via `config::SECTOR_RESOLUTION` (64, 128, 256, 512, 1024).
/// Every sector has an antipodal opposite (sector + N/2) mod N.

/// Type alias for a complex number with f64 real and imaginary parts.
///
/// This is the fundamental wave representation used throughout the system.
/// Z = A * e^(i*theta) where A is amplitude and theta is phase.
#[allow(non_camel_case_types)]
pub type c64 = Complex64;

/// Wave - operations on complex wave representations of text.
///
/// Provides methods for computing sentence waves, text waves, and
/// ray casting searches across the facet's lexicon.
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

    /// Computes the superposition wave for a list of known words.
    ///
    /// Each word's phasor is converted to its complex representation
    /// and summed. Words not in the facet are silently skipped.
    pub fn sentence(facet: &Facet, words: &[String]) -> c64 {
        words
            .iter()
            .filter_map(|w| facet.lexicon.get(w))
            .map(|p| p.to_complex())
            .sum()
    }

    /// Computes the wave for a raw text string.
    ///
    /// Tokenizes the text first, then computes the sentence wave
    /// from the resulting tokens.
    pub fn text(facet: &Facet, text: &str) -> c64 {
        let tokens = Tokenizer::tokenize(text);
        Self::sentence(facet, &tokens)
    }

    /// Ray cast: finds words that resonate with a target word.
    ///
    /// Projects parallel search rays from the target word's phasor to
    /// every other word in the facet. Words are ranked by minimal
    /// energy delta (destructive interference). Uses rayon for
    /// parallel iteration across the lexicon.
    ///
    /// Returns a sorted list of (word, energy_delta) pairs, smallest delta first.
    pub fn ray_cast_word(facet: &Facet, target_word: &str, top_k: usize) -> Vec<(String, f64)> {
        let target = match facet.lexicon.get(target_word) {
            Some(p) => p,
            None => return vec![],
        };

        let target_z = target.to_complex();

        let mut hits: Vec<(&String, f64)> = facet
            .lexicon
            .par_iter()
            .filter(|(word, _)| *word != target_word)
            .map(|(word, phasor)| {
                let delta = config::ALPHA * (target_z - phasor.to_complex()).norm_sqr();
                (word, delta)
            })
            .collect();

        if hits.len() > top_k {
            hits.select_nth_unstable_by(top_k, |a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
            hits.truncate(top_k);
        }
        hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
        hits.into_iter().map(|(w, d)| (w.clone(), d)).collect()
    }

    /// Ray cast: finds words that resonate with a given wave.
    ///
    /// Projects parallel search rays from the input wave to every
    /// word in the facet. Words are ranked by minimal energy delta.
    /// Uses rayon for parallel iteration.
    ///
    /// Returns a sorted list of (word, energy_delta) pairs, smallest delta first.
    pub fn ray_cast(facet: &Facet, wave: c64, top_k: usize) -> Vec<(String, f64)> {
        let mut hits: Vec<(&String, f64)> = facet
            .lexicon
            .par_iter()
            .map(|(word, phasor)| {
                let delta = config::ALPHA * (wave - phasor.to_complex()).norm_sqr();
                (word, delta)
            })
            .collect();

        if hits.len() > top_k {
            hits.select_nth_unstable_by(top_k, |a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
            hits.truncate(top_k);
        }
        hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
        hits.into_iter().map(|(w, d)| (w.clone(), d)).collect()
    }

    /// Returns the configured number of phase sectors.
    pub fn sector_count() -> u16 {
        config::sector_resolution()
    }

    /// Maps a phase angle to a sector index (0-63).
    ///
    /// The phase circle is divided into 64 equal sectors.
    /// Sector 0 = [0, 2π/64), sector 1 = [2π/64, 4π/64), etc.
    pub fn sector_of(phase: f64) -> u16 {
        let n = Self::sector_count();
        let normalized = phase.rem_euclid(2.0 * PI);
        let sector_size = 2.0 * PI / n as f64;
        (normalized / sector_size).floor() as u16 % n
    }

    /// Returns the antipodal (opposite) sector index.
    ///
    /// Sector + 32 mod 64 gives the diametrically opposite sector.
    /// Words in opposite sectors represent semantic antonyms.
    pub fn opposite_sector(sector: u16) -> u16 {
        let n = Self::sector_count();
        (sector + n / 2) % n
    }

    /// Returns the sector of a word's effective phase.
    ///
    /// Uses the phasor's phase plus band_n * alpha to compute
    /// the effective phase, then maps to a sector.
    pub fn word_sector(facet: &Facet, word: &str) -> Option<u16> {
        let phasor = facet.lexicon.get(word)?;
        let effective = phasor.phase + (phasor.band_n as f64 * config::ALPHA);
        Some(Self::sector_of(effective))
    }

    /// Returns the sector of a complex wave's phase angle.
    pub fn wave_sector(wave: c64) -> u16 {
        if wave.norm() < 1e-10 {
            return 0;
        }
        Self::sector_of(wave.arg())
    }

    /// Finds words in a specific sector of the phase circle.
    ///
    /// Returns up to `top_k` words sorted by amplitude (most familiar first).
    #[allow(dead_code)]
    pub fn words_in_sector(facet: &Facet, sector: u16, top_k: usize) -> Vec<(String, f64)> {
        let mut hits: Vec<(String, f64)> = facet
            .lexicon
            .par_iter()
            .filter(|(_, p)| Self::sector_of(p.phase + (p.band_n as f64 * config::ALPHA)) == sector)
            .map(|(w, p)| (w.clone(), p.amplitude))
            .collect();

        hits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        hits.into_iter().take(top_k).collect()
    }

}
