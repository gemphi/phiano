//! The counters the learner is measured against, and the corpus reader.

use phient::prose::tokens;
use std::collections::HashMap;

pub fn sentences(path: &str) -> Vec<Vec<String>> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .split(|c| matches!(c, '.' | '!' | '?' | '\n'))
        .map(tokens)
        .map(|w| w.into_iter().filter(|t| t.chars().any(char::is_alphanumeric)).collect::<Vec<_>>())
        .filter(|w| w.len() >= 4)
        .collect()
}

/// Where `truth` stands in a model's ranked answers (1 is first), for
/// phient's `Score`; `None` when it is not among them.
pub fn rank(ranked: &[String], truth: &str) -> Option<usize> {
    ranked.iter().position(|w| w == truth).map(|i| i + 1)
}

/// The best of counting: interpolated Witten–Bell over two words back,
/// add-one at the bottom, normalised over the words it was told.
#[derive(Default)]
pub struct WittenBell {
    pub uni: HashMap<String, u64>,
    pub n: u64,
    pub bi: HashMap<String, HashMap<String, u64>>,
    pub tri: HashMap<(String, String), HashMap<String, u64>>,
}

impl WittenBell {
    pub fn tell(&mut self, line: &[String]) {
        let pad = |j: usize, d: usize| if j >= d { line[j - d].clone() } else { "<s>".to_string() };
        for j in 0..line.len() {
            let w = line[j].clone();
            *self.uni.entry(w.clone()).or_default() += 1;
            self.n += 1;
            *self.bi.entry(pad(j, 1)).or_default().entry(w.clone()).or_default() += 1;
            *self.tri.entry((pad(j, 2), pad(j, 1))).or_default().entry(w).or_default() += 1;
        }
    }

    pub fn p1(&self, w: &str) -> f64 {
        (self.uni.get(w).copied().unwrap_or(0) as f64 + 1.0) / (self.n + self.uni.len() as u64) as f64
    }

    fn mix(seen: Option<&HashMap<String, u64>>, w: &str, lower: f64) -> f64 {
        let Some(m) = seen else { return lower };
        let (c, t) = (m.values().sum::<u64>() as f64, m.len() as f64);
        (m.get(w).copied().unwrap_or(0) as f64 + t * lower) / (c + t)
    }

    pub fn p(&self, h2: &str, h1: &str, w: &str) -> f64 {
        let p2 = Self::mix(self.bi.get(h1), w, self.p1(w));
        Self::mix(self.tri.get(&(h2.to_string(), h1.to_string())), w, p2)
    }

    /// The five likeliest: among what followed the two words, what followed
    /// the one, and the most told — nothing outside those can outrank them.
    pub fn top5(&self, h2: &str, h1: &str, common: &[String]) -> Vec<String> {
        let mut c: Vec<&String> = common.iter().collect();
        if let Some(m) = self.bi.get(h1) {
            c.extend(m.keys());
        }
        if let Some(m) = self.tri.get(&(h2.to_string(), h1.to_string())) {
            c.extend(m.keys());
        }
        c.sort();
        c.dedup();
        let mut s: Vec<(f64, &String)> = c.into_iter().map(|w| (self.p(h2, h1, w), w)).collect();
        s.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.1.cmp(b.1)));
        s.into_iter().take(5).map(|(_, w)| w.clone()).collect()
    }
}
