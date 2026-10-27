#[allow(unused_imports)]
use super::*;
use phiecs::phasor::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::phinet::space::{examples, FunctionSpace, Sides};
use phiano::storage::Storage;
use phiano::tokenizer::Tokenizer;

/// Everything an arm trains and scores against.
pub(crate) struct Data {
    pub space: FunctionSpace,
    pub pool: Vec<usize>,
    pub train: Vec<Example>,
    pub valid: Vec<Example>,
    pub test: Vec<Example>,
    pub chance: f64,
    pub id_valid: f64,
    pub id_test: f64,
    pub id_hit: f64,
}

use phiano::lab::sentences_of;

/// Three novels, the shipped voice lexicon, directed reading, pool of words
/// seen 20+ times; 2,000 validation and test examples.
pub(crate) fn load(train_limit: Option<usize>) -> Data {
    let mut all = Vec::new();
    for f in ["emma", "persuasion", "sense_and_sensibility"] {
        all.extend(sentences_of(&format!("data/corpora/austen/{f}.txt")));
    }
    let split = Harness::split(all, 7);
    let voice: Facet = Storage::load("data/austen_voice.chroma").expect("run bin/austen first");
    let space = FunctionSpace::from_facet_sides(&voice, Sides::Directed);
    let pool = space.pool(20);
    let tok = |v: &Vec<String>| v.iter().map(|s| Tokenizer::tokenize(s)).collect::<Vec<_>>();
    let (mut train, _) = examples(&space, &tok(&split.train), 6, &pool, u64::MAX);
    if let Some(n) = train_limit {
        train.truncate(n);
    }
    let (mut valid, _) = examples(&space, &tok(&split.valid), 6, &pool, u64::MAX);
    valid.truncate(2000);
    let (mut test, _) = examples(&space, &tok(&split.test), 6, &pool, u64::MAX);
    test.truncate(2000);
    let chance = (1..=pool.len()).map(|k| 1.0 / k as f64).sum::<f64>() / pool.len() as f64;
    let vin: Vec<WordFunction> = valid.iter().map(|e| e.input.clone()).collect();
    let tin: Vec<WordFunction> = test.iter().map(|e| e.input.clone()).collect();
    let (id_valid, _) = rank(space.table(), &vin, &valid, &pool);
    let (id_test, id_hit) = rank(space.table(), &tin, &test, &pool);
    Data { space, pool, train, valid, test, chance, id_valid, id_test, id_hit }
}

/// The order an epoch visits the training examples in: fixed by the epoch
/// number, so a resumed run sees exactly the sequence an uninterrupted one
/// would.
pub(crate) fn epoch_order(n: usize, epoch: u64) -> Vec<usize> {
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&i| phistd::mix64(i as u64 ^ epoch.wrapping_mul(0x9E37_79B9_7F4A_7C15)));
    order
}
