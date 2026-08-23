/// Fingerprint — a style signature extracted from text examples.
///
/// Captures how a persona's text distributes across the phase circle's
/// sectors. Each text sample is converted to a wave, mapped to a sector,
/// and the distribution is accumulated.

mod traits;

use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use crate::wave::{Wave, sectors};
use std::fmt;

pub struct Fingerprint {
    /// How many text samples were used to build this fingerprint.
    pub sample_count: usize,
    /// Histogram: frequency of each sector across all samples.
    pub sector_histogram: Vec<f64>,
    /// Dominant sectors sorted by frequency (sector index, weight).
    pub dominant: Vec<(u16, f64)>,
    /// Average text length (in tokens) across samples.
    pub avg_length: f64,
    /// Sector diversity (entropy of the histogram).
    pub diversity: f64,
}

impl Fingerprint {
    fn is_stop_word(word: &str) -> bool {
        crate::config::is_stop_word(word)
    }

    /// Extracts a fingerprint from text examples using the facet.
    pub fn extract(facet: &Facet, examples: &[String]) -> Self {
        let n = sectors() as usize;
        let mut histogram = vec![0.0f64; n];
        let mut total_length = 0usize;

        for example in examples {
            let tokens = Tokenizer::tokenize(example);
            total_length += tokens.len();

            let content_tokens: Vec<String> = tokens
                .iter()
                .filter(|t| !Self::is_stop_word(t))
                .cloned()
                .collect();

            let wave = if content_tokens.is_empty() {
                Wave::sentence(facet, &tokens)
            } else {
                Wave::sentence(facet, &content_tokens)
            };
            let sector = Wave::wave_sector(wave) as usize;
            if sector < n {
                histogram[sector] += 1.0;
            }

            for token in &content_tokens {
                if let Some(word_sector) = Wave::word_sector(facet, token) {
                    let s = word_sector as usize;
                    if s < n {
                        let amp = facet.get_phasor(token).map(|p| p.amplitude).unwrap_or(1.0);
                        let weight = crate::config::FINGERPRINT_WORD_WEIGHT / (1.0 + amp);
                        histogram[s] += weight;
                    }
                }
            }
        }

        let total: f64 = histogram.iter().sum();
        if total > 0.0 {
            for h in &mut histogram { *h /= total; }
        }

        let mut dominant: Vec<(u16, f64)> = histogram
            .iter()
            .enumerate()
            .map(|(i, &w)| (i as u16, w))
            .collect();
        dominant.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        dominant.retain(|(_, w)| *w > 0.0);

        let entropy: f64 = histogram
            .iter()
            .filter(|&&h| h > 0.0)
            .map(|&h| -h * h.ln())
            .sum();

        let avg_length = if examples.is_empty() {
            0.0
        } else {
            total_length as f64 / examples.len() as f64
        };

        Fingerprint {
            sample_count: examples.len(),
            sector_histogram: histogram,
            dominant, avg_length,
            diversity: entropy,
        }
    }

    /// Computes cosine similarity between two fingerprints.
    pub fn similarity(&self, other: &Fingerprint) -> f64 {
        let n = self.sector_histogram.len().min(other.sector_histogram.len());
        if n == 0 { return 0.0; }

        let mut dot = 0.0f64;
        let mut norm_a = 0.0f64;
        let mut norm_b = 0.0f64;

        for i in 0..n {
            let a = self.sector_histogram[i];
            let b = other.sector_histogram[i];
            dot += a * b;
            norm_a += a * a;
            norm_b += b * b;
        }

        if norm_a == 0.0 || norm_b == 0.0 { return 0.0; }
        dot / (norm_a.sqrt() * norm_b.sqrt())
    }

    /// Computes likelihood of a text fingerprint given this persona's distribution.
    pub fn likelihood(&self, text: &Fingerprint) -> f64 {
        let n = self.sector_histogram.len().min(text.sector_histogram.len());
        if n == 0 { return 0.0; }

        let mut log_likelihood = 0.0f64;
        let mut weight_sum = 0.0f64;
        let epsilon = 1e-6;

        for i in 0..n {
            let text_weight = text.sector_histogram[i];
            if text_weight > 0.0 {
                let persona_prob = self.sector_histogram[i].max(epsilon);
                log_likelihood += text_weight * persona_prob.ln();
                weight_sum += text_weight;
            }
        }

        if weight_sum == 0.0 { return 0.0; }

        let avg_ll = log_likelihood / weight_sum;
        let max_ll = 0.0;
        let min_ll = (epsilon).ln();
        1.0 - ((avg_ll - max_ll) / (min_ll - max_ll)).clamp(0.0, 1.0)
    }

    /// Returns the top N dominant sectors with their weights.
    pub fn dominant_sectors(&self, top_n: usize) -> Vec<(u16, f64)> {
        self.dominant.iter().take(top_n).cloned().collect()
    }

    /// Returns the sectors where this persona differs most from another.
    pub fn difference_vector(&self, other: &Fingerprint) -> Vec<(u16, f64)> {
        let n = self.sector_histogram.len().min(other.sector_histogram.len());
        let mut diffs = Vec::with_capacity(n);

        for i in 0..n {
            let diff = self.sector_histogram[i] - other.sector_histogram[i];
            diffs.push((i as u16, diff));
        }

        diffs.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap_or(std::cmp::Ordering::Equal));
        diffs
    }

    /// Derives personality traits from the dominant sector colors.
    pub fn personality_traits(&self) -> Vec<String> {
        traits::personality_traits(self)
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  samples: {}", self.sample_count)?;
        writeln!(f, "  avg length: {:.1} tokens", self.avg_length)?;
        writeln!(f, "  diversity (entropy): {:.3}", self.diversity)?;
        writeln!(f, "  dominant sectors:")?;
        for (sector, weight) in self.dominant_sectors(8) {
            let color = crate::compose::sector_color(sector);
            writeln!(f, "    sector {} ({}) weight {:.4}", sector, color, weight)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;

