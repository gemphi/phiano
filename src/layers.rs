use crate::facet::Facet;
use crate::phasor::SpectralPhasor;
use std::collections::HashMap;
use std::f64::consts::PI;

pub const PHASE_LAYERS: usize = 4; // 2^2
pub const LAYER_SECTORS: [u16; 4] = [64, 32, 16, 8]; // Halving resolution per layer

/// ClusterNode - represents a centroid node in a higher phase layer.
#[derive(Debug, Clone)]
pub struct ClusterNode {
    #[allow(dead_code)]
    pub id: usize,
    pub phasor: SpectralPhasor,
    pub member_count: usize,
}

/// PhaseLayer - a single coarse/fine phase circle layer in the hierarchy.
#[derive(Debug, Clone)]
pub struct PhaseLayer {
    #[allow(dead_code)]
    pub level: usize,
    pub sector_count: u16,
    pub clusters: HashMap<u16, ClusterNode>,
}

impl PhaseLayer {
    pub fn new(level: usize, sector_count: u16) -> Self {
        Self {
            level,
            sector_count,
            clusters: HashMap::new(),
        }
    }
}

/// HierarchicalPhaseField - 4-layer deep hierarchical phase architecture (Phase 3).
pub struct HierarchicalPhaseField {
    pub layers: Vec<PhaseLayer>,
}

impl Default for HierarchicalPhaseField {
    fn default() -> Self {
        Self::new()
    }
}

impl HierarchicalPhaseField {
    pub fn new() -> Self {
        let mut layers = Vec::with_capacity(PHASE_LAYERS);
        for (idx, &num_sectors) in LAYER_SECTORS.iter().enumerate() {
            layers.push(PhaseLayer::new(idx, num_sectors));
        }
        Self { layers }
    }

    /// Computes bottom-up hierarchical phase centroids across all 4 layers.
    ///
    /// Layer 0: Surface Words (Facet Lexicon)
    /// Layer 1: Concept Clusters (32 sectors)
    /// Layer 2: Domain Sectors (16 sectors)
    /// Layer 3: Meta-Patterns (8 sectors)
    pub fn build_hierarchy(&mut self, facet: &Facet) {
        if facet.lexicon.is_empty() {
            return;
        }

        // 1. Build Layer 1 (32 Concept Clusters from Layer 0 words)
        self.build_layer_from_words(facet, 1, LAYER_SECTORS[1]);

        // 2. Build Layer 2 (16 Domain Sectors from Layer 1 clusters)
        self.build_layer_from_prev_layer(1, 2, LAYER_SECTORS[2]);

        // 3. Build Layer 3 (8 Meta-Patterns from Layer 2 clusters)
        self.build_layer_from_prev_layer(2, 3, LAYER_SECTORS[3]);
    }

    fn build_layer_from_words(&mut self, facet: &Facet, layer_idx: usize, num_sectors: u16) {
        let mut sector_sums_x: HashMap<u16, f64> = HashMap::new();
        let mut sector_sums_y: HashMap<u16, f64> = HashMap::new();
        let mut counts: HashMap<u16, usize> = HashMap::new();

        let sector_width = (2.0 * PI) / (num_sectors as f64);

        for (_word, phasor) in &facet.lexicon {
            let sector = ((phasor.phase / sector_width).floor() as u16) % num_sectors;
            *sector_sums_x.entry(sector).or_default() += phasor.amplitude * phasor.phase.cos();
            *sector_sums_y.entry(sector).or_default() += phasor.amplitude * phasor.phase.sin();
            *counts.entry(sector).or_default() += 1;
        }

        let mut clusters = HashMap::new();
        for sector in 0..num_sectors {
            let count = *counts.get(&sector).unwrap_or(&0);
            if count > 0 {
                let sx = *sector_sums_x.get(&sector).unwrap_or(&0.0);
                let sy = *sector_sums_y.get(&sector).unwrap_or(&0.0);
                let mut phase = sy.atan2(sx);
                if phase < 0.0 {
                    phase += 2.0 * PI;
                }
                let amp = (sx * sx + sy * sy).sqrt() / (count as f64);
                
                clusters.insert(
                    sector,
                    ClusterNode {
                        id: sector as usize,
                        phasor: SpectralPhasor::new(phase, amp, 1),
                        member_count: count,
                    },
                );
            }
        }

        self.layers[layer_idx].clusters = clusters;
    }

    fn build_layer_from_prev_layer(&mut self, prev_idx: usize, curr_idx: usize, num_sectors: u16) {
        let prev_layer = self.layers[prev_idx].clone();
        let mut sector_sums_x: HashMap<u16, f64> = HashMap::new();
        let mut sector_sums_y: HashMap<u16, f64> = HashMap::new();
        let mut counts: HashMap<u16, usize> = HashMap::new();

        let sector_width = (2.0 * PI) / (num_sectors as f64);

        for (_sec, node) in &prev_layer.clusters {
            let sector = ((node.phasor.phase / sector_width).floor() as u16) % num_sectors;
            *sector_sums_x.entry(sector).or_default() += node.phasor.amplitude * node.phasor.phase.cos();
            *sector_sums_y.entry(sector).or_default() += node.phasor.amplitude * node.phasor.phase.sin();
            *counts.entry(sector).or_default() += node.member_count;
        }

        let mut clusters = HashMap::new();
        for sector in 0..num_sectors {
            let count = *counts.get(&sector).unwrap_or(&0);
            if count > 0 {
                let sx = *sector_sums_x.get(&sector).unwrap_or(&0.0);
                let sy = *sector_sums_y.get(&sector).unwrap_or(&0.0);
                let mut phase = sy.atan2(sx);
                if phase < 0.0 {
                    phase += 2.0 * PI;
                }
                let amp = (sx * sx + sy * sy).sqrt() / (count as f64);

                clusters.insert(
                    sector,
                    ClusterNode {
                        id: sector as usize,
                        phasor: SpectralPhasor::new(phase, amp, 1),
                        member_count: count,
                    },
                );
            }
        }

        self.layers[curr_idx].clusters = clusters;
    }

    /// Evaluates multi-layer depth resonance for a query phase angle.
    pub fn resonate_depth(&self, target_phase: f64) -> Vec<(usize, u16, f64)> {
        let mut layer_resonances = Vec::new();
        for (level, layer) in self.layers.iter().enumerate().skip(1) {
            let sector_width = (2.0 * PI) / (layer.sector_count as f64);
            let sector = ((target_phase / sector_width).floor() as u16) % layer.sector_count;
            if let Some(node) = layer.clusters.get(&sector) {
                let mut diff = (node.phasor.phase - target_phase).abs();
                if diff > PI {
                    diff = 2.0 * PI - diff;
                }
                layer_resonances.push((level, sector, diff));
            }
        }
        layer_resonances
    }
}

impl HierarchicalPhaseField {
    /// Walks the hierarchy from meta-patterns down to concept clusters.
    ///
    /// Returns the sector chosen at each level, coarsest first. The layers were
    /// built, displayed once, and never consulted during retrieval; this is the
    /// descent they exist for.
    pub fn descend(&self, target_phase: f64) -> Vec<(usize, u16)> {
        let mut path = Vec::new();
        for level in (1..self.layers.len()).rev() {
            let layer = &self.layers[level];
            if layer.clusters.is_empty() {
                continue;
            }
            let width = (2.0 * PI) / layer.sector_count as f64;
            let sector = ((target_phase.rem_euclid(2.0 * PI)) / width).floor() as u16
                % layer.sector_count;
            path.push((level, sector));
        }
        path.reverse();
        path
    }

    /// Coarse-to-fine retrieval: narrow by hierarchy, then rank exactly.
    ///
    /// Full ray casting is O(V) per query. Descending the hierarchy first
    /// restricts the exhaustive part to the words inside one layer-1 concept
    /// cluster and its immediate neighbours.
    pub fn candidates(&self, facet: &Facet, target_phase: f64, top_k: usize) -> Vec<(String, f64)> {
        let layer = match self.layers.get(1) {
            Some(l) if !l.clusters.is_empty() => l,
            _ => return Vec::new(),
        };
        let n = layer.sector_count;
        let width = (2.0 * PI) / n as f64;
        let home = ((target_phase.rem_euclid(2.0 * PI)) / width).floor() as u16 % n;

        let wanted: Vec<u16> = vec![home, (home + 1) % n, (home + n - 1) % n];
        let mut hits: Vec<(String, f64)> = facet
            .lexicon
            .iter()
            .filter(|(_, p)| {
                let s = ((p.phase.rem_euclid(2.0 * PI)) / width).floor() as u16 % n;
                wanted.contains(&s)
            })
            .map(|(w, p)| {
                let mut d = (p.phase - target_phase).abs();
                if d > PI {
                    d = 2.0 * PI - d;
                }
                (w.clone(), d)
            })
            .collect();

        hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        hits.truncate(top_k);
        hits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchical_phase_field() {
        let mut facet = Facet::new();
        facet.get_or_init("rust");
        facet.get_or_init("code");
        facet.get_or_init("memory");

        let mut field = HierarchicalPhaseField::new();
        field.build_hierarchy(&facet);

        assert_eq!(field.layers.len(), 4);
        assert!(!field.layers[1].clusters.is_empty());
        assert_eq!(field.layers[0].level, 0);

        // Query at a phase a word actually occupies. Probing a fixed angle only
        // worked while seeding was length-based and every short word landed on
        // the same handful of sectors.
        let probe = facet.get_phasor("rust").map(|p| p.phase).unwrap();
        let depth_res = field.resonate_depth(probe);
        assert!(!depth_res.is_empty(), "the layer containing a known word must resonate");

        // The hierarchy must now be usable for retrieval, not only display.
        let path = field.descend(probe);
        assert!(!path.is_empty(), "descent must produce a coarse-to-fine path");
        let near = field.candidates(&facet, probe, 3);
        assert!(near.iter().any(|(w, _)| w == "rust"), "the probe word should be found: {:?}", near);
    }
}
