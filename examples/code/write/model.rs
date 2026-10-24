//! One reader, fitted on a set of files as in docs/110: counts from four
//! fifths, the weights α[d, sightings] learned on the fifth, then counts
//! from all of them.

use super::calibrate::Weighting;
use super::models::CountModel;
use super::score;
use std::collections::HashMap;

pub const K: usize = 8;

pub struct Model {
    pub vocab: Vec<String>,
    pub id: HashMap<String, u32>,
    pub c: CountModel,
    pub uni: Vec<f64>,
    pub posts: HashMap<(u8, u32), f64>,
    pub w: Weighting,
}

impl Model {
    pub fn fit(all: &[&(String, Vec<String>)]) -> Self {
        let mut freq: HashMap<&str, usize> = HashMap::new();
        all.iter().for_each(|(_, t)| t.iter().for_each(|x| *freq.entry(x.as_str()).or_default() += 1));
        let mut vocab: Vec<String> = freq.iter().filter(|(_, &c)| c >= 2).map(|(w, _)| w.to_string()).collect();
        vocab.sort();
        vocab.insert(0, "⟨unk⟩".into());
        let part = |p: &str| phistd::mix64(phiano::phasor::fnv1a(p)) % 5;
        let (core, dev): (Vec<&(String, Vec<String>)>, Vec<_>) = all.iter().partition(|(p, _)| part(p) != 0);
        let mut w = Weighting::new(K, 0.35);
        {
            let c = CountModel::new(&score::encode(&vocab, &core), vocab.len(), K);
            let uni = c.unigram();
            let posts = score::posts(&c, &uni);
            let test = score::encode(&vocab, &dev);
            let pos = score::positions(&test, 8, 8000);
            let mut g2 = vec![0.0; w.a.len() + 1];
            for _ in 0..3 {
                pos.iter().for_each(|&(h, t)| {
                    w.step(&c.ngram(h, 4, &uni), &c.conditions(h, K, &posts), &uni, t, &mut g2, 0.05);
                });
            }
        }
        let c = CountModel::new(&score::encode(&vocab, all), vocab.len(), K);
        let uni = c.unigram();
        let posts = score::posts(&c, &uni);
        let id = vocab.iter().enumerate().map(|(i, w)| (w.clone(), i as u32)).collect();
        Model { vocab, id, c, uni, posts, w }
    }

    pub fn four(&self, h: &[u32]) -> Vec<f64> {
        self.c.ngram(h, 4, &self.uni)
    }

    pub fn moved(&self, h: &[u32]) -> Vec<f64> {
        self.w.score(&self.c.ngram(h, 4, &self.uni), &self.c.conditions(h, K, &self.posts), &self.uni)
    }

    pub fn encode(&self, toks: &[String]) -> Vec<u32> {
        toks.iter().map(|x| self.id.get(x).copied().unwrap_or(0)).collect()
    }
}
