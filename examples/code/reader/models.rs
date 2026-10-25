//! The readers compared on Rust code: the next token given the ones before.
//! Every reader gives a probability for every token of the vocabulary.
//!
//! - **unigram**: how often each token occurs;
//! - **bigram / trigram**: interpolated absolute discounting (D = 0.75),
//!   the standard n-gram model, down to the unigram;
//! - **Phient movement**: each of the last K tokens is a word under the
//!   condition `next@d` (d places back). It holds a post there if its counts
//!   move the answer by at least 0.5 nats from the unigram (D2), and the
//!   posts are composed by movement (D4): ln P(v) + Σ ln P(v | w, next@d) / P(v).

use std::collections::HashMap;

pub const D: f64 = 0.75;
pub const POST: f64 = 0.5;

#[derive(Default)]
pub struct CountModel {
    pub uni: Vec<f64>,
    pub total: f64,
    /// context (as ids, most recent last) ⇒ (next ⇒ count)
    pub ctx: HashMap<Vec<u32>, HashMap<u32, f64>>,
    /// (d, word) ⇒ (next ⇒ count): what follows `word` d places later.
    pub skip: HashMap<(u8, u32), HashMap<u32, f64>>,
}

impl CountModel {
    pub fn new(seqs: &[Vec<u32>], v: usize, k: usize) -> Self {
        let mut c = CountModel { uni: vec![0.0; v], ..Default::default() };
        for s in seqs {
            for (i, &t) in s.iter().enumerate() {
                c.uni[t as usize] += 1.0;
                c.total += 1.0;
                for n in 1..=3usize.min(i) {
                    *c.ctx.entry(s[i - n..i].to_vec()).or_default().entry(t).or_default() += 1.0;
                }
                for d in 1..=k.min(i) {
                    *c.skip.entry((d as u8, s[i - d])).or_default().entry(t).or_default() += 1.0;
                }
            }
        }
        c
    }

    pub fn unigram(&self) -> Vec<f64> {
        let v = self.uni.len() as f64;
        self.uni.iter().map(|x| (x + 1.0) / (self.total + v)).collect()
    }

    /// Interpolated absolute discounting over contexts of length `order - 1`.
    pub fn ngram(&self, history: &[u32], order: usize, uni: &[f64]) -> Vec<f64> {
        let mut p = uni.to_vec();
        for n in 1..order {
            if history.len() < n {
                break;
            }
            let Some(m) = self.ctx.get(&history[history.len() - n..]) else { break };
            let total: f64 = m.values().sum();
            let lambda = D * m.len() as f64 / total;
            for x in p.iter_mut() {
                *x *= lambda;
            }
            for (&t, &c) in m {
                p[t as usize] += (c - D).max(0.0) / total;
            }
        }
        p
    }

    /// KL of `skip[(d, w)]` (smoothed toward the unigram) from the unigram.
    pub fn post(&self, d: u8, w: u32, uni: &[f64]) -> f64 {
        let Some(m) = self.skip.get(&(d, w)) else { return 0.0 };
        let n: f64 = m.values().sum();
        let mut kl = 0.0;
        let mut seen = 0.0;
        for (&t, &c) in m {
            let q = uni[t as usize];
            let p = (c + q) / (n + 1.0);
            kl += p * (p / q).ln();
            seen += q;
        }
        kl + (1.0 - seen) / (n + 1.0) * (1.0 / (n + 1.0)).ln()
    }

    /// The size knob: each (d, w) keeps only its `r` strongest relations
    /// (by count, ties by id). Posts are read before, from the full counts.
    pub fn prune(&mut self, r: usize) {
        for m in self.skip.values_mut().filter(|m| m.len() > r) {
            let mut v: Vec<(u32, f64)> = m.iter().map(|(&t, &c)| (t, c)).collect();
            v.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
            *m = v.into_iter().take(r).collect();
        }
    }

    /// The conditions that hold a post for the next token: (d, the counts
    /// of what followed the word d places back).
    pub fn conditions<'a>(&'a self, history: &[u32], k: usize, posts: &HashMap<(u8, u32), f64>) -> Vec<(u8, &'a HashMap<u32, f64>)> {
        (1..=k.min(history.len()))
            .filter_map(|d| {
                let key = (d as u8, history[history.len() - d]);
                (posts.get(&key).copied().unwrap_or(0.0) >= POST).then(|| (d as u8, &self.skip[&key]))
            })
            .collect()
    }

    /// Movement over the last `k` tokens from a `base` distribution (the
    /// unigram, or the trigram), each word's move weighted by `alpha`,
    /// normalised over the vocabulary. With α = 1 and the unigram as base it
    /// is D4 exactly; α < 1 undoes the overcounting of words that say the
    /// same thing (chosen on development files, never on the test).
    pub fn movement(&self, history: &[u32], k: usize, uni: &[f64], base: &[f64], alpha: f64, posts: &HashMap<(u8, u32), f64>) -> Vec<f64> {
        let mut s: Vec<f64> = base.iter().map(|x| x.max(1e-300).ln()).collect();
        for d in 1..=k.min(history.len()) {
            let w = history[history.len() - d];
            if posts.get(&(d as u8, w)).copied().unwrap_or(0.0) < POST {
                continue;
            }
            let m = &self.skip[&(d as u8, w)];
            for (&t, &c) in m {
                s[t as usize] += alpha * (1.0 + c / uni[t as usize]).ln();
            }
        }
        let top = s.iter().cloned().fold(f64::MIN, f64::max);
        let e: Vec<f64> = s.iter().map(|x| (x - top).exp()).collect();
        let z: f64 = e.iter().sum();
        e.into_iter().map(|x| x / z).collect()
    }
}
