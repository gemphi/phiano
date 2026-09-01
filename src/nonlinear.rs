//! Non-linear readout — the fourth requirement in HOW 16.
//!
//! Every operation in the engine is a sum or a multiplication by a constant,
//! followed by argmax. A composition of linear maps is linear, so the model's
//! function class is linear however many tiers the architecture diagram shows,
//! and neither capacity nor training fixes that.
//!
//! The readout is a **conditional** table: given the sector pattern of the
//! context, a learned bias per *target* sector. That conditioning is what makes
//! it non-linear and what makes it useful — a bias that depended only on the
//! context would add the same constant to every candidate and could not change
//! a ranking at all.
//!
//! Discretisation is the non-linearity, and a lookup table over discretised
//! inputs is a universal approximator. It also stays interpretable: the table
//! can be printed.

use crate::wave::{c64, Wave};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Default channels hashed into the context key.
pub const KEY_CHANNELS: usize = 4;

/// Default key resolution: buckets per channel.
///
/// The full sector grid is 64 wide, and 64⁴ is 16.7 million cells — a table
/// that fine memorises the training split and is a cache miss on every held-out
/// context, so it cannot change a held-out score at all. The key resolution is
/// therefore a *separate*, coarser quantisation from the target sector grid,
/// and it is the parameter that trades expressiveness against coverage.
pub const KEY_SECTORS: usize = 8;

/// Target sectors the bias vector covers.
const TARGET_SECTORS: usize = 64;

#[derive(Debug, Serialize, Deserialize)]
pub struct SectorReadout {
    /// context cell → bias per target sector
    bias: HashMap<u64, Vec<f32>>,
    hits: HashMap<u64, u32>,
    /// Channels hashed into the key, and buckets per channel. Stored so a table
    /// can only ever be queried at the resolution it was fitted at.
    key_channels: usize,
    key_sectors: usize,
    /// Held-out diagnostics: lookups made, and lookups that found a cell.
    ///
    /// Atomic because scoring runs under rayon; relaxed ordering because these
    /// are a diagnostic ratio, not a synchronisation point.
    #[serde(skip)]
    lookups: std::sync::atomic::AtomicU64,
    #[serde(skip)]
    found: std::sync::atomic::AtomicU64,
}

impl Default for SectorReadout {
    fn default() -> Self {
        Self::with_shape(KEY_CHANNELS, KEY_SECTORS)
    }
}

impl SectorReadout {
    pub fn new() -> Self {
        Self::default()
    }

    /// A table at an explicit key resolution.
    pub fn with_shape(key_channels: usize, key_sectors: usize) -> Self {
        Self {
            bias: HashMap::new(),
            hits: HashMap::new(),
            key_channels: key_channels.max(1),
            key_sectors: key_sectors.clamp(2, TARGET_SECTORS),
            lookups: std::sync::atomic::AtomicU64::new(0),
            found: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Number of context cells the table has learned.
    pub fn cells(&self) -> usize {
        self.bias.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bias.is_empty()
    }

    /// Fraction of `bias_for` calls that landed in a fitted cell.
    ///
    /// This is the number that decides whether a discretised readout can
    /// generalise at all. At coverage 0 the table is a no-op on held-out text
    /// however well it fits the training split.
    pub fn coverage(&self) -> f64 {
        use std::sync::atomic::Ordering::Relaxed;
        match self.lookups.load(Relaxed) {
            0 => 0.0,
            n => self.found.load(Relaxed) as f64 / n as f64,
        }
    }

    pub fn reset_coverage(&self) {
        use std::sync::atomic::Ordering::Relaxed;
        self.lookups.store(0, Relaxed);
        self.found.store(0, Relaxed);
    }

    /// Hashes a multi-channel context into a discrete cell.
    ///
    /// Nearby phases inside one bucket map to the same cell; crossing a bucket
    /// boundary changes the cell entirely. That discontinuity is the point.
    pub fn key(&self, channels: &[f64]) -> u64 {
        let mut h: u64 = 14695981039346656037;
        let width = std::f64::consts::TAU / self.key_sectors as f64;
        for a in channels.iter().take(self.key_channels) {
            let b = (a.rem_euclid(std::f64::consts::TAU) / width).floor() as u64;
            h ^= b % self.key_sectors as u64;
            h = h.wrapping_mul(1099511628211);
        }
        h
    }

    /// Sector index of a target phase.
    #[inline]
    pub fn target_sector(phase: f64) -> usize {
        (Wave::sector_of(phase) as usize) % TARGET_SECTORS
    }

    /// Learned bias for a target sector in a context cell.
    #[inline]
    pub fn bias_for(&self, key: u64, target_sector: usize) -> f64 {
        use std::sync::atomic::Ordering::Relaxed;
        self.lookups.fetch_add(1, Relaxed);
        match self.bias.get(&key) {
            Some(v) => {
                self.found.fetch_add(1, Relaxed);
                v.get(target_sector % TARGET_SECTORS).copied().unwrap_or(0.0) as f64
            }
            None => 0.0,
        }
    }

    /// Raises the bias of the sector a true continuation fell in, and lowers a
    /// sampled wrong one — the same contrastive shape as the phase objective.
    ///
    /// The per-cell rate decays with visits so a cell seen once cannot overwrite
    /// what a cell seen a thousand times learned.
    pub fn learn(&mut self, key: u64, positive: usize, negative: usize, lr: f64) {
        let n = self.hits.entry(key).or_insert(0);
        *n = n.saturating_add(1);
        let rate = (lr / (1.0 + (*n as f64).sqrt())) as f32;

        let v = self.bias.entry(key).or_insert_with(|| vec![0.0; TARGET_SECTORS]);
        let p = positive % TARGET_SECTORS;
        let q = negative % TARGET_SECTORS;
        v[p] = (v[p] + rate).clamp(-2.0, 2.0);
        if q != p {
            v[q] = (v[q] - rate * 0.5).clamp(-2.0, 2.0);
        }
    }

    /// Magnitude gate: suppress channels below `tau`. A ReLU in the magnitude
    /// domain, allowing sparse, competitive channel activation.
    pub fn gate(channels: &mut [c64], tau: f64) -> usize {
        let mut kept = 0;
        for z in channels.iter_mut() {
            match z.norm() >= tau {
                true => kept += 1,
                false => *z = c64::new(0.0, 0.0),
            }
        }
        kept
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bias_is_conditional_not_constant() {
        // A bias depending only on the context would add the same value to every
        // candidate and could never change a ranking.
        let mut r = SectorReadout::new();
        let k = r.key(&[0.1, 0.1, 0.1, 0.1]);
        for _ in 0..30 {
            r.learn(k, 5, 40, 0.5);
        }
        assert!(r.bias_for(k, 5) > 0.0, "the observed target sector rises");
        assert!(r.bias_for(k, 40) < 0.0, "the sampled wrong sector falls");
        assert_ne!(r.bias_for(k, 5), r.bias_for(k, 40));
    }

    #[test]
    fn test_distinct_contexts_get_distinct_cells() {
        let r = SectorReadout::new();
        let a = r.key(&[0.01, 0.01, 0.01, 0.01]);
        let b = r.key(&[3.20, 3.20, 3.20, 3.20]);
        assert_ne!(a, b);
        let near = r.key(&[0.011, 0.011, 0.011, 0.011]);
        assert_eq!(a, near, "phases inside one bucket share a cell");
    }

    /// A finer key distinguishes more contexts and covers fewer of them. This
    /// is the trade-off `KEY_SECTORS` exists to control, and the reason the key
    /// grid is coarser than the target grid: at 64 buckets over 4 channels the
    /// cell space is 16.7M and an unseen context essentially never hits.
    #[test]
    fn test_finer_keys_cover_less() {
        let contexts: Vec<Vec<f64>> = (0..200)
            .map(|i| {
                let t = i as f64 * 0.031;
                vec![t.sin() + 1.0, t.cos() + 1.0, (2.0 * t).sin() + 1.0, t * 0.7]
            })
            .collect();

        let coverage_at = |sectors: usize| {
            let mut r = SectorReadout::with_shape(4, sectors);
            for c in contexts.iter().take(100) {
                let k = r.key(c);
                r.learn(k, 1, 2, 0.5);
            }
            r.reset_coverage();
            for c in contexts.iter().skip(100) {
                let k = r.key(c);
                r.bias_for(k, 1);
            }
            r.coverage()
        };

        let coarse = coverage_at(4);
        let fine = coverage_at(64);
        assert!(
            coarse > fine,
            "coarse key must cover more held-out contexts: {} vs {}",
            coarse,
            fine
        );
    }

    #[test]
    fn test_rate_decays_and_stays_bounded() {
        let mut r = SectorReadout::new();
        let k = r.key(&[1.0; 4]);
        r.learn(k, 3, 9, 1.0);
        let first = r.bias_for(k, 3);
        for _ in 0..500 {
            r.learn(k, 3, 9, 1.0);
        }
        let later = r.bias_for(k, 3);
        assert!(later > first);
        assert!(later <= 2.0, "bounded: {}", later);
    }

    #[test]
    fn test_gate_suppresses_weak_channels() {
        let mut c = vec![
            c64::from_polar(0.05, 1.0),
            c64::from_polar(2.0, 1.0),
            c64::from_polar(0.5, 1.0),
        ];
        assert_eq!(SectorReadout::gate(&mut c, 0.4), 2);
        assert_eq!(c[0].norm(), 0.0);
    }
}
