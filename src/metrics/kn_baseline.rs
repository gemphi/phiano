//! Interpolated Kneser-Ney trigram language model.
//!
//! This is the number Phiano has to beat. A phase manifold that does not
//! outperform a well-smoothed trigram model on held-out text has not yet earned
//! its place in the pipeline, and without this baseline "the model works" has no
//! referent.
//!
//! Kneser-Ney's continuation probability is the part that matters: a word is
//! scored by *how many distinct contexts it appears in*, not by how often it
//! appears. "Francisco" is frequent but occurs almost only after "San", so KN
//! correctly refuses to predict it elsewhere. Raw maximum likelihood does not.

use crate::tokenizer::Tokenizer;
use std::collections::{HashMap, HashSet};

/// Absolute discount. 0.75 is the standard value for trigram models.
const DISCOUNT: f64 = 0.75;

/// Floor probability, so an unseen token never yields infinite perplexity.
const FLOOR: f64 = 1e-10;

#[derive(Debug, Default)]
pub struct KneserNey {
    uni: HashMap<String, u32>,
    bi: HashMap<(String, String), u32>,
    tri: HashMap<(String, String, String), u32>,
    /// context -> total count
    bi_ctx: HashMap<String, u32>,
    tri_ctx: HashMap<(String, String), u32>,
    /// context -> number of distinct continuations (for the back-off weight)
    bi_types: HashMap<String, u32>,
    tri_types: HashMap<(String, String), u32>,
    /// word -> number of distinct left contexts (continuation counts)
    cont: HashMap<String, u32>,
    total_bigram_types: u32,
    vocab: usize,
}

impl KneserNey {
    /// Builds the model from a corpus of sentences.
    pub fn train(corpus: &[String]) -> Self {
        let mut m = Self::default();
        let mut left_contexts: HashMap<String, HashSet<String>> = HashMap::new();
        let mut bi_conts: HashMap<String, HashSet<String>> = HashMap::new();
        let mut tri_conts: HashMap<(String, String), HashSet<String>> = HashMap::new();

        for sentence in corpus {
            let mut toks = Tokenizer::tokenize(sentence);
            if toks.is_empty() {
                continue;
            }
            toks.insert(0, "<s>".to_string());
            toks.insert(0, "<s>".to_string());
            toks.push("</s>".to_string());

            for w in &toks {
                *m.uni.entry(w.clone()).or_insert(0) += 1;
            }
            for w in toks.windows(2) {
                *m.bi.entry((w[0].clone(), w[1].clone())).or_insert(0) += 1;
                *m.bi_ctx.entry(w[0].clone()).or_insert(0) += 1;
                left_contexts.entry(w[1].clone()).or_default().insert(w[0].clone());
                bi_conts.entry(w[0].clone()).or_default().insert(w[1].clone());
            }
            for w in toks.windows(3) {
                *m.tri
                    .entry((w[0].clone(), w[1].clone(), w[2].clone()))
                    .or_insert(0) += 1;
                *m.tri_ctx.entry((w[0].clone(), w[1].clone())).or_insert(0) += 1;
                tri_conts
                    .entry((w[0].clone(), w[1].clone()))
                    .or_default()
                    .insert(w[2].clone());
            }
        }

        m.cont = left_contexts.iter().map(|(w, s)| (w.clone(), s.len() as u32)).collect();
        m.bi_types = bi_conts.iter().map(|(w, s)| (w.clone(), s.len() as u32)).collect();
        m.tri_types = tri_conts.iter().map(|(c, s)| (c.clone(), s.len() as u32)).collect();
        m.total_bigram_types = m.bi.len() as u32;
        m.vocab = m.uni.len();
        m
    }

    /// Continuation probability: distinct left contexts of `w` over all bigram types.
    fn p_continuation(&self, w: &str) -> f64 {
        if self.total_bigram_types == 0 {
            return 1.0 / self.vocab.max(1) as f64;
        }
        let c = *self.cont.get(w).unwrap_or(&0) as f64;
        (c / self.total_bigram_types as f64).max(1.0 / (self.vocab.max(1) as f64 * 10.0))
    }

    /// Interpolated KN bigram probability.
    fn p_bigram(&self, a: &str, b: &str) -> f64 {
        let ctx = *self.bi_ctx.get(a).unwrap_or(&0) as f64;
        if ctx == 0.0 {
            return self.p_continuation(b);
        }
        let c = *self.bi.get(&(a.to_string(), b.to_string())).unwrap_or(&0) as f64;
        let types = *self.bi_types.get(a).unwrap_or(&0) as f64;
        let lambda = DISCOUNT * types / ctx;
        ((c - DISCOUNT).max(0.0) / ctx) + lambda * self.p_continuation(b)
    }

    /// Interpolated KN trigram probability, backing off to bigram then continuation.
    pub fn probability(&self, a: &str, b: &str, c: &str) -> f64 {
        let ctx = *self.tri_ctx.get(&(a.to_string(), b.to_string())).unwrap_or(&0) as f64;
        if ctx == 0.0 {
            return self.p_bigram(b, c).max(FLOOR);
        }
        let n = *self
            .tri
            .get(&(a.to_string(), b.to_string(), c.to_string()))
            .unwrap_or(&0) as f64;
        let types = *self.tri_types.get(&(a.to_string(), b.to_string())).unwrap_or(&0) as f64;
        let lambda = DISCOUNT * types / ctx;
        (((n - DISCOUNT).max(0.0) / ctx) + lambda * self.p_bigram(b, c)).max(FLOOR)
    }

    /// Held-out perplexity: exp(-mean log probability per token).
    pub fn perplexity(&self, held_out: &[String]) -> f64 {
        let mut log_sum = 0.0f64;
        let mut n = 0usize;

        for sentence in held_out {
            let mut toks = Tokenizer::tokenize(sentence);
            if toks.is_empty() {
                continue;
            }
            toks.insert(0, "<s>".to_string());
            toks.insert(0, "<s>".to_string());
            toks.push("</s>".to_string());

            for w in toks.windows(3) {
                log_sum += self.probability(&w[0], &w[1], &w[2]).ln();
                n += 1;
            }
        }

        match n {
            0 => f64::INFINITY,
            _ => (-log_sum / n as f64).exp(),
        }
    }

    pub fn vocabulary_size(&self) -> usize {
        self.vocab
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kn_assigns_no_zero_probability() {
        let corpus: Vec<String> = vec![
            "the cat sat on the mat".into(),
            "the dog ran in the park".into(),
        ];
        let m = KneserNey::train(&corpus);
        // an entirely unseen trigram must still have positive probability
        let p = m.probability("quantum", "chromo", "dynamics");
        assert!(p > 0.0 && p.is_finite(), "p = {}", p);
    }

    #[test]
    fn test_kn_perplexity_is_finite_on_unseen_text() {
        let corpus: Vec<String> = vec!["the cat sat on the mat".into()];
        let m = KneserNey::train(&corpus);
        let ppl = m.perplexity(&["a completely different sentence".to_string()]);
        assert!(ppl.is_finite() && ppl > 0.0, "ppl = {}", ppl);
    }

    #[test]
    fn test_kn_prefers_seen_continuations() {
        let corpus: Vec<String> = vec![
            "the borrow checker prevents data races".into(),
            "the borrow checker prevents data races".into(),
            "the borrow checker prevents data races".into(),
        ];
        let m = KneserNey::train(&corpus);
        let seen = m.probability("checker", "prevents", "data");
        let unseen = m.probability("checker", "prevents", "bananas");
        assert!(seen > unseen, "{} vs {}", seen, unseen);
    }
}
