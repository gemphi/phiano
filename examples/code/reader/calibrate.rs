//! Calibrating movement (docs/110). docs/109 found movement ranks the next
//! token better than the trigram but is overconfident: one global α either
//! keeps the ranking or the calibration, not both. Here each condition gets
//! the weight its kind has earned: a weight per (distance d, how often the
//! word was seen at that distance), learned by likelihood on the development
//! files. Whether a condition seen a thousand times earns more trust than
//! one seen twice (the two speeds of paper 19) is left to the data.
//!
//! score(v) = β · ln base(v) + Σ_conditions α[d, n] · ln(1 + c(v) / P(v)),
//! normalised over the vocabulary; the gradient of ln P(truth) in each weight
//! is its feature at the truth minus its expectation.

use std::collections::HashMap;

pub const BUCKETS: usize = 6;

/// n < 4, < 16, < 64, < 256, < 1024, more.
pub fn bucket(n: f64) -> usize {
    ((n.max(1.0).ln() / 4f64.ln()) as usize).min(BUCKETS - 1)
}

pub type Cond<'a> = (u8, &'a HashMap<u32, f64>);

pub struct Weighting {
    pub beta: f64,
    pub a: Vec<f64>,
}

fn softmax(mut s: Vec<f64>) -> Vec<f64> {
    let top = s.iter().cloned().fold(f64::MIN, f64::max);
    let mut z = 0.0;
    for x in s.iter_mut() {
        *x = (*x - top).exp();
        z += *x;
    }
    s.iter_mut().for_each(|x| *x /= z);
    s
}

impl Weighting {
    pub fn new(k: usize, alpha: f64) -> Self {
        Weighting { beta: 1.0, a: vec![alpha; k * BUCKETS] }
    }

    fn slot(d: u8, m: &HashMap<u32, f64>) -> usize {
        (d as usize - 1) * BUCKETS + bucket(m.values().sum())
    }

    pub fn score(&self, base: &[f64], conds: &[Cond], uni: &[f64]) -> Vec<f64> {
        self.score_with(base, conds, &vec![0.0; conds.len()], uni)
    }

    /// As `score`, each condition's weight moved by its own `extra` (the
    /// slow speed, `own.rs`).
    pub fn score_with(&self, base: &[f64], conds: &[Cond], extra: &[f64], uni: &[f64]) -> Vec<f64> {
        let mut s: Vec<f64> = base.iter().map(|x| self.beta * x.max(1e-300).ln()).collect();
        for (&(d, m), x) in conds.iter().zip(extra) {
            let a = (self.a[Self::slot(d, m)] + x).max(0.0);
            for (&t, &c) in m {
                s[t as usize] += a * (1.0 + c / uni[t as usize]).ln();
            }
        }
        softmax(s)
    }

    /// One step of Adagrad ascent on ln P(truth); returns ln P(truth).
    pub fn step(&mut self, base: &[f64], conds: &[Cond], uni: &[f64], truth: usize, g2: &mut [f64], lr: f64) -> f64 {
        let p = self.score(base, conds, uni);
        let mut g = vec![0.0; self.a.len() + 1];
        let lb: f64 = p.iter().zip(base).map(|(p, b)| p * b.max(1e-300).ln()).sum();
        g[0] = base[truth].max(1e-300).ln() - lb;
        for &(d, m) in conds {
            let f = |t: u32, c: f64| (1.0 + c / uni[t as usize]).ln();
            let expect: f64 = m.iter().map(|(&t, &c)| p[t as usize] * f(t, c)).sum();
            let at = m.get(&(truth as u32)).map_or(0.0, |&c| f(truth as u32, c));
            g[1 + Self::slot(d, m)] += at - expect;
        }
        for (i, gi) in g.iter().enumerate() {
            g2[i] += gi * gi;
            let w = if i == 0 { &mut self.beta } else { &mut self.a[i - 1] };
            *w = (*w + lr * gi / (g2[i].sqrt() + 1e-8)).max(0.0);
        }
        p[truth].max(1e-300).ln()
    }

    /// The learned weights as a table: rows d, columns sightings.
    pub fn table(&self, k: usize) -> String {
        let mut s = format!("β (weight of the n-gram) = {:.2}\n\n| d | n<4 | <16 | <64 | <256 | <1024 | ≥1024 |\n|---|---|---|---|---|---|---|\n", self.beta);
        for d in 0..k {
            let row: Vec<String> = (0..BUCKETS).map(|b| format!("{:.2}", self.a[d * BUCKETS + b])).collect();
            s += &format!("| {} | {} |\n", d + 1, row.join(" | "));
        }
        s
    }
}

/// The cache reader (Tu, Su & Devanbu 2014, "On the localness of
/// software"): how often each token occurred in the last `n` tokens of this
/// file.
pub fn cache(history: &[u32], n: usize, v: usize) -> Vec<f64> {
    let h = &history[history.len().saturating_sub(n)..];
    let mut p = vec![0.0; v];
    h.iter().for_each(|&t| p[t as usize] += 1.0 / h.len().max(1) as f64);
    p
}

pub fn mix(p: &[f64], q: &[f64], lambda: f64) -> Vec<f64> {
    p.iter().zip(q).map(|(a, b)| lambda * a + (1.0 - lambda) * b).collect()
}

/// λ in the grid with the least perplexity, from each position's truth
/// probability under the two readers.
pub fn best_lambda(pairs: &[(f64, f64)]) -> f64 {
    let nll = |l: f64| -> f64 { pairs.iter().map(|(a, b)| -(l * a + (1.0 - l) * b).max(1e-300).ln()).sum() };
    [0.5, 0.6, 0.7, 0.8, 0.9, 0.95].into_iter().min_by(|x, y| nll(*x).total_cmp(&nll(*y))).unwrap()
}
