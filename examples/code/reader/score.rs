//! Scoring and small helpers for the code reader.

use super::models::{CountModel, POST};
use phient::study::Score;
use std::collections::HashMap;

pub use phiano::lab::flag;

/// One position of a model's distribution `p`, scored into phient's `Score`:
/// the truth's rank among the probabilities, and its probability.
pub fn add(s: &mut Score, p: &[f64], truth: usize) {
    let t = p[truth];
    s.rank(Some(1 + p.iter().filter(|&&x| x > t).count()));
    s.prob(-t.max(1e-12).ln());
}

/// `| name | top-1 | top-5 | MRR | perplexity |`
pub fn row(name: &str, s: &Score) -> String {
    format!("| {name} | {:.1}% | {:.1}% | {:.3} | {:.1} |", s.top1(), s.top5(), s.mrr(), s.perplexity())
}

pub fn encode(vocab: &[String], files: &[&(String, Vec<String>)]) -> Vec<Vec<u32>> {
    let id: HashMap<&str, u32> = vocab.iter().enumerate().map(|(i, w)| (w.as_str(), i as u32)).collect();
    files.iter().map(|(_, t)| t.iter().map(|x| id.get(x.as_str()).copied().unwrap_or(0)).collect()).collect()
}

pub fn posts(c: &CountModel, uni: &[f64]) -> HashMap<(u8, u32), f64> {
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
