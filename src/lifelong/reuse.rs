/// Feature and architecture reuse: extracts and applies reusable phase patterns.
/// Implements Ch 14.5's modular reuse across tasks.

use crate::config::{SECTOR_RESOLUTION, TWO_PI};
use crate::facet::Facet;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FeatureSet {
    pub phase_pattern: Vec<f64>,
    pub sector_distribution: Vec<u16>,
    pub label: String,
}

#[derive(Debug, Default)]
pub struct FeatureReuse;

impl FeatureReuse {
    /// Extracts reusable phase patterns from the facet.
    pub fn extract(facet: &Facet) -> Vec<FeatureSet> {
        let mut features = Vec::new();

        let sector_width = TWO_PI / SECTOR_RESOLUTION as f64;
        let mut sectors: Vec<Vec<String>> = vec![Vec::new(); SECTOR_RESOLUTION as usize];

        for (word, phasor) in &facet.lexicon {
            let sector = (phasor.phase / sector_width).floor() as usize % SECTOR_RESOLUTION as usize;
            sectors[sector].push(word.clone());
        }

        for (idx, words) in sectors.iter().enumerate() {
            if words.len() >= 3 {
                let phases: Vec<f64> = words
                    .iter()
                    .filter_map(|w| facet.lexicon.get(w).map(|p| p.phase))
                    .collect();

                let mut dist = vec![0u16; SECTOR_RESOLUTION as usize];
                for &p in &phases {
                    let s = (p / sector_width).floor() as usize % SECTOR_RESOLUTION as usize;
                    dist[s] += 1;
                }

                features.push(FeatureSet {
                    phase_pattern: phases,
                    sector_distribution: dist,
                    label: format!("sector_{}", idx),
                });
            }
        }

        features
    }

    /// Applies pre-learned features to a new facet (transfer learning).
    pub fn apply(facet: &mut Facet, features: &[FeatureSet]) {
        for fs in features {
            for &phase in &fs.phase_pattern {
                let word = format!("meta_{}", fs.label);
                facet.get_or_init(&word);
                if let Some(p) = facet.lexicon.get_mut(&word) {
                    p.phase = phase;
                }
            }
        }
    }

    /// Computes similarity between two feature sets.
    pub fn similarity(a: &FeatureSet, b: &FeatureSet) -> f64 {
        if a.phase_pattern.is_empty() || b.phase_pattern.is_empty() {
            return 0.0;
        }

        let min_len = a.phase_pattern.len().min(b.phase_pattern.len());
        let mut total = 0.0;
        for i in 0..min_len {
            let mut diff = (a.phase_pattern[i] - b.phase_pattern[i]).abs();
            if diff > std::f64::consts::PI {
                diff = TWO_PI - diff;
            }
            total += 1.0 - diff / std::f64::consts::PI;
        }
        total / min_len as f64
    }
}
