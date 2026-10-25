//! The tuner: full-batch Adam on the softmax negative log-likelihood, with early stopping.

use super::features::N_FEATURES;
use super::positions::Position;
use super::GenWeight;

/// Index of the `repeat` feature: fit in the tuner but *not* applied at
/// decode time, where repetition is a sampling constraint instead. Real text
/// repeats function words, so the fitted weight is positive (+0.55 on the
/// Rust Book); a low-temperature sampler exploits that into loops ("the
/// type of the type of…", distinct-token ratio 0.97 → 0.66). The weight is
/// reported for what it is; `GenWeight::for_decoding` removes it.
pub const REPEAT_FEATURE: usize = 4;

/// Decode-time logit penalty for a word emitted in the last twelve tokens —
/// a sampling constraint, not a model parameter (`ln 0.15`, the hand scorer's
/// soft penalty).
pub const REPEAT_DECODE_PENALTY: f64 = -1.897_119_984_885_881;

/// Fits a log-linear scorer by full-batch Adam on the mean NLL.
pub struct Tuner {
    pub lr: f64,
    pub epochs: usize,
    /// L2 toward zero on every weight but the count term.
    pub l2: f64,
}

impl Default for Tuner {
    fn default() -> Self {
        Self { lr: 0.05, epochs: 300, l2: 1e-4 }
    }
}

impl Tuner {
    /// Mean negative log-likelihood of the true candidate under `w`.
    pub fn nll(w: &[f64; N_FEATURES], positions: &[Position]) -> f64 {
        if positions.is_empty() {
            return f64::NAN;
        }
        let mut total = 0.0;
        for p in positions {
            let scores: Vec<f64> = p.feats.iter().map(|x| w.iter().zip(&x.0).map(|(a, b)| a * b).sum()).collect();
            let max = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let z: f64 = scores.iter().map(|s| (s - max).exp()).sum();
            total -= scores[p.truth] - max - z.ln();
        }
        total / positions.len() as f64
    }

    /// Gradient of the mean NLL.
    fn grad(w: &[f64; N_FEATURES], positions: &[Position]) -> [f64; N_FEATURES] {
        let mut g = [0.0; N_FEATURES];
        for p in positions {
            let scores: Vec<f64> = p.feats.iter().map(|x| w.iter().zip(&x.0).map(|(a, b)| a * b).sum()).collect();
            let max = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let ex: Vec<f64> = scores.iter().map(|s| (s - max).exp()).collect();
            let z: f64 = ex.iter().sum();
            for (i, x) in p.feats.iter().enumerate() {
                let prob = ex[i] / z;
                let coef = prob - if i == p.truth { 1.0 } else { 0.0 };
                for (gf, xf) in g.iter_mut().zip(&x.0) {
                    *gf += coef * xf;
                }
            }
        }
        let n = positions.len().max(1) as f64;
        for v in g.iter_mut() {
            *v /= n;
        }
        g
    }

    /// Fits from `init`, early-stopping on `heldout`. Returns the weights and
    /// the held-out NLL per epoch.
    pub fn fit(&self, init: [f64; N_FEATURES], train: &[Position], heldout: &[Position]) -> (GenWeight, Vec<f64>) {
        let mut w = init;
        let (mut m, mut v) = ([0.0; N_FEATURES], [0.0; N_FEATURES]);
        let (b1, b2, eps) = (0.9, 0.999, 1e-8);
        let mut best = (Self::nll(&w, heldout), w);
        let mut trace = vec![best.0];
        for t in 1..=self.epochs {
            let mut g = Self::grad(&w, train);
            for f in 1..N_FEATURES {
                g[f] += self.l2 * w[f];
            }
            for f in 0..N_FEATURES {
                m[f] = b1 * m[f] + (1.0 - b1) * g[f];
                v[f] = b2 * v[f] + (1.0 - b2) * g[f] * g[f];
                let mh = m[f] / (1.0 - b1.powi(t as i32));
                let vh = v[f] / (1.0 - b2.powi(t as i32));
                w[f] -= self.lr * mh / (vh.sqrt() + eps);
            }
            let h = Self::nll(&w, heldout);
            trace.push(h);
            if h < best.0 {
                best = (h, w);
            }
        }
        (
            GenWeight {
                w: best.1,
                heldout_nll: best.0,
                baseline_nll: Self::nll(&GenWeight::counts_only().w, heldout),
                positions: train.len(),
            },
            trace,
        )
    }
}
