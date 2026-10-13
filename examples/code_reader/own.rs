//! Two speeds for the code reader's conditions (docs/114), as for the
//! builders' posts (docs/113). A condition `w` stood `d` back is weighted by
//! its kind, α[d, sightings] (docs/110) — the fast speed. One seen at least
//! `seen` times in the counts may also earn its own weight, learned on the
//! development files and pulled back toward its kind's by `LAMBDA`. Keyed by
//! the token's text, so the weights carry from the development counts to the
//! test counts.

use super::calibrate::{Cond, Weights};
use std::collections::HashMap;

pub const LAMBDA: f64 = 0.01;

pub struct Own {
    pub seen: f64,
    pub delta: HashMap<(u8, String), f64>,
}

impl Own {
    /// Each condition's own weight (0 while it is at the fast speed).
    pub fn extras(&self, h: &[u32], conds: &[Cond], vocab: &[String]) -> Vec<f64> {
        conds
            .iter()
            .map(|&(d, m)| match m.values().sum::<f64>() >= self.seen {
                true => self.delta.get(&(d, vocab[h[h.len() - d as usize] as usize].clone())).copied().unwrap_or(0.0),
                false => 0.0,
            })
            .collect()
    }

    /// One step on ln P(truth) for the conditions at the slow speed.
    // One caller; a parameter struct would only repackage its arguments.
    #[allow(clippy::too_many_arguments)]
    pub fn step(&mut self, w: &Weights, base: &[f64], h: &[u32], conds: &[Cond], vocab: &[String], uni: &[f64], truth: usize, lr: f64) -> f64 {
        let extra = self.extras(h, conds, vocab);
        let p = w.score_with(base, conds, &extra, uni);
        for &(d, m) in conds.iter().filter(|(_, m)| m.values().sum::<f64>() >= self.seen) {
            let f = |t: u32, c: f64| (1.0 + c / uni[t as usize]).ln();
            let expect: f64 = m.iter().map(|(&t, &c)| p[t as usize] * f(t, c)).sum();
            let at = m.get(&(truth as u32)).map_or(0.0, |&c| f(truth as u32, c));
            let x = self.delta.entry((d, vocab[h[h.len() - d as usize] as usize].clone())).or_default();
            *x += lr * (at - expect - LAMBDA * *x);
        }
        p[truth].max(1e-300).ln()
    }

    /// The conditions that moved most from their kind.
    pub fn moved(&self, k: usize) -> String {
        let mut v: Vec<_> = self.delta.iter().collect();
        v.sort_by(|x, y| y.1.abs().total_cmp(&x.1.abs()).then(x.0.cmp(y.0)));
        v.iter().take(k).map(|((d, w), x)| format!("`{w}`@{d} {x:+.2}")).collect::<Vec<_>>().join(", ")
    }
}
