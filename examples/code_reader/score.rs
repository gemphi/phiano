//! Scoring and small helpers for the code reader.

use super::models::{Counts, POST};
use std::collections::HashMap;

pub use phiano::lab::flag;

pub struct Tally {
    pub name: String,
    pub n: f64,
    top1: f64,
    top5: f64,
    rr: f64,
    nll: f64,
}

impl Tally {
    pub fn new(name: &str) -> Self {
        Tally { name: name.to_string(), n: 0.0, top1: 0.0, top5: 0.0, rr: 0.0, nll: 0.0 }
    }
    pub fn add(&mut self, p: &[f64], truth: usize) {
        let t = p[truth];
        let above = p.iter().filter(|&&x| x > t).count();
        self.n += 1.0;
        self.top1 += (above == 0) as usize as f64;
        self.top5 += (above < 5) as usize as f64;
        self.rr += 1.0 / (above + 1) as f64;
        self.nll -= t.max(1e-12).ln();
    }
    pub fn ppl(&self) -> f64 {
        (self.nll / self.n.max(1.0)).exp()
    }
    pub fn row(&self) -> String {
        let n = self.n.max(1.0);
        format!("| {} | {:.1}% | {:.1}% | {:.3} | {:.1} |", self.name, 100.0 * self.top1 / n, 100.0 * self.top5 / n, self.rr / n, self.ppl())
    }
}

pub fn encode(vocab: &[String], files: &[&(String, Vec<String>)]) -> Vec<Vec<u32>> {
    let id: HashMap<&str, u32> = vocab.iter().enumerate().map(|(i, w)| (w.as_str(), i as u32)).collect();
    files.iter().map(|(_, t)| t.iter().map(|x| id.get(x.as_str()).copied().unwrap_or(0)).collect()).collect()
}

pub fn posts(c: &Counts, uni: &[f64]) -> HashMap<(u8, u32), f64> {
    c.skip.keys().map(|&(d, w)| ((d, w), c.post(d, w, uni))).filter(|(_, x)| *x >= POST).collect()
}

/// Every `step`-th position of the test sequences, after the first `skip`
/// tokens, about `n` in all: (history, truth).
pub fn positions(test: &[Vec<u32>], skip: usize, n: usize) -> Vec<(&[u32], usize)> {
    let all: Vec<(usize, usize)> = test.iter().enumerate().flat_map(|(f, s)| (skip..s.len()).map(move |i| (f, i))).collect();
    let step = (all.len() / n.max(1)).max(1);
    all.iter().step_by(step).map(|&(f, i)| (&test[f][..i], test[f][i] as usize)).collect()
}

const KEYWORDS: [&str; 38] = [
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while",
];

/// A name: starts with a letter or underscore and is not a keyword.
pub fn is_identifier(t: &str) -> bool {
    t.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_') && !KEYWORDS.contains(&t) && t != "⟨unk⟩"
}
