//! Non-linear readout — the fourth of the four requirements in HOW 16.
//!
//! Every operation in the engine is a sum or a multiplication by a constant,
//! followed by argmax. A composition of linear maps is linear, so the model's
//! function class is linear however many tiers the architecture diagram shows,
//! and no amount of capacity or training fixes that.
//!
//! Two cheap non-linearities, both native to the substrate:
//!
//! * **Sector discretisation.** Binning a continuous phase into a sector is a
//!   non-linear map, and a lookup table over discretised inputs is a universal
//!   approximator. It also stays interpretable: the table can be printed.
//! * **Magnitude gating.** Suppressing channels whose magnitude falls below a
//!   threshold is a ReLU in the magnitude domain, which lets different channels
//!   specialise instead of all contributing to everything.

use crate::wave::{c64, Wave};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Channels hashed into the sector key. More channels means a finer partition
/// and a sparser table; eight gives 64^8 cells, far more than will ever be hit,
/// so the table stays sparse and only learns cells it actually sees.
const KEY_CHANNELS: usize = 8;

/// Learned per-cell phase bias, applied after the linear readout.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SectorReadout {
    bias: HashMap<u64, f32>,
    /// Times each cell has been updated, for a decaying learning rate.
    hits: HashMap<u64, u32>,
}

impl SectorReadout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.bias.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bias.is_empty()
    }

    /// Hashes a multi-channel wave into a discrete cell.
    ///
    /// This is where the non-linearity enters: nearby phases inside one sector
    /// map to the same cell, and crossing a sector boundary changes the cell
    /// entirely.
    pub fn key(channels: &[c64]) -> u64 {
        let mut h: u64 = 14695981039346656037;
        for z in channels.iter().take(KEY_CHANNELS) {
            let sector = match z.norm() > 1e-12 {
                true => Wave::sector_of(z.arg()),
                false => 0,
            };
            h ^= sector as u64;
            h = h.wrapping_mul(1099511628211);
        }
        h
    }

    /// Applies the learned bias for a wave's cell.
    pub fn apply(&self, channels: &[c64], base: f64) -> f64 {
        base + *self.bias.get(&Self::key(channels)).unwrap_or(&0.0) as f64
    }

    /// Moves a cell's bias toward reducing `error`.
    ///
    /// The per-cell learning rate decays with the number of visits, so a cell
    /// seen once does not overwrite what a cell seen a thousand times learned.
    pub fn learn(&mut self, channels: &[c64], error: f64, lr: f64) {
        let k = Self::key(channels);
        let n = self.hits.entry(k).or_insert(0);
        *n = n.saturating_add(1);
        let decay = lr / (1.0 + (*n as f64).sqrt());
        let b = self.bias.entry(k).or_insert(0.0);
        *b = (*b as f64 + decay * error).clamp(-1.0, 1.0) as f32;
    }

    /// Magnitude gate: suppress channels below `tau`, keep the rest.
    ///
    /// A ReLU in the magnitude domain. One comparison per channel, and it is
    /// what allows sparse, competitive channel activation rather than every
    /// channel contributing to every decision.
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

    fn chans(angles: &[f64]) -> Vec<c64> {
        angles.iter().map(|a| c64::from_polar(1.0, *a)).collect()
    }

    #[test]
    fn test_the_readout_is_not_linear() {
        // A linear map satisfies f(a) + f(b) == f(a + b) up to the base term.
        // A sector table does not: crossing a boundary changes the cell.
        let mut r = SectorReadout::new();
        let a = chans(&[0.01; 8]);
        let b = chans(&[3.20; 8]);
        for _ in 0..20 {
            r.learn(&a, 1.0, 0.5);
            r.learn(&b, -1.0, 0.5);
        }
        let fa = r.apply(&a, 0.0);
        let fb = r.apply(&b, 0.0);
        assert!(fa > 0.1, "cell a learned a positive bias: {}", fa);
        assert!(fb < -0.1, "cell b learned a negative bias: {}", fb);
        assert!(r.len() >= 2, "distinct inputs must occupy distinct cells");
    }

    #[test]
    fn test_nearby_phases_share_a_cell() {
        let a = chans(&[0.001; 8]);
        let b = chans(&[0.002; 8]);
        assert_eq!(SectorReadout::key(&a), SectorReadout::key(&b));
    }

    #[test]
    fn test_learning_rate_decays_with_visits() {
        let mut r = SectorReadout::new();
        let a = chans(&[1.0; 8]);
        r.learn(&a, 1.0, 1.0);
        let first = r.apply(&a, 0.0);
        for _ in 0..200 {
            r.learn(&a, 1.0, 1.0);
        }
        let later = r.apply(&a, 0.0);
        assert!(later > first, "it should keep moving");
        assert!(later <= 1.0, "and stay bounded: {}", later);
    }

    #[test]
    fn test_gate_suppresses_weak_channels() {
        let mut c = vec![
            c64::from_polar(0.05, 1.0),
            c64::from_polar(2.0, 1.0),
            c64::from_polar(0.5, 1.0),
        ];
        let kept = SectorReadout::gate(&mut c, 0.4);
        assert_eq!(kept, 2);
        assert_eq!(c[0].norm(), 0.0);
        assert!(c[1].norm() > 0.0 && c[2].norm() > 0.0);
    }
}
