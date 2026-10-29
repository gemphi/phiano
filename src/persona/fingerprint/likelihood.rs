#[allow(unused_imports)]
use super::*;

impl Fingerprint {

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

    pub fn personality_traits(&self) -> Vec<String> {
        traits::PersonalityMapper::personality_traits(self)
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  samples: {}", self.sample_count)?;
        writeln!(f, "  avg length: {:.1} tokens", self.avg_length)?;
        writeln!(f, "  diversity (entropy): {:.3}", self.diversity)?;
        writeln!(f, "  dominant sectors:")?;
        for (sector, weight) in self.dominant_sectors(8) {
            let color = phient::generate::compose::SectorPalette::color(sector);
            writeln!(f, "    sector {} ({}) weight {:.4}", sector, color, weight)?;
        }
        Ok(())
    }
}
