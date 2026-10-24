//! One pair key: composition keyed by the relation channel (marker or
//! offset — the trainer's key) against composition keyed by bare offsets.
//! `cargo run --release --bin pair_key [corpus] [epochs]`

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::metrics::harness::Harness;
use phiano::phinet::compose::{Composition, PairKey};
use phiano::phinet::space::{Example, FunctionSpace, Sides};
use phiano::phinet::{Adam, Fno, FnoConfig};
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;

const POOL_FLOOR: u32 = 25;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let epochs: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(5);
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 3)
        .collect();
    let split = Harness::split(corpus, 42);
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let train_sent: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    println!("corpus {path}: {} train sentences, {} words, {} marker channels", split.train.len(), facet.vocabulary_size(), facet.relations.markers());

    for (name, sides) in [("source-side functions", Sides::Source), ("directed (table = target sides)", Sides::Directed)] {
        let space = FunctionSpace::from_facet_sides(&facet, sides);
        let pool = space.pool(POOL_FLOOR);
        println!("\n=== {name} ===");
        for (kname, key) in [("offset key (1..=4)", PairKey::Offset), ("channel key (marker or offset)", PairKey::Channel(&facet.relations))] {
            let comp = Composition::from_corpus_keyed(&space, &train_sent, &pool, key);
            let n = comp.words.len();
            if n < 20 {
                println!("  {kname}: only {n} composable words");
                continue;
            }
            let (train_w, test_w) = comp.words.split_at(n * 4 / 5);
            let chance = (1..=n).map(|k| 1.0 / k as f64).sum::<f64>() / n as f64;
            let cpool: Vec<usize> = comp.words.clone();
            let fills: f64 = comp.words.iter().map(|w| comp.fillers[w].len() as f64).sum::<f64>() / n as f64;
            let ccfg = FnoConfig { space: PHASE_CHANNELS, width: 8, layers: 2, modes: 16, margin: 0.3, lr: 3e-3, tie: phical::Tie::Circle };
            let level_sets: Vec<(Vec<Example>, FunctionSpace)> = (1..=3)
                .map(|l| {
                    let (pairs, targets) = comp.cleanup_examples(&space, train_w, l);
                    (pairs.iter().map(|(q, f)| Example { input: q.clone(), target: *f }).collect(), targets)
                })
                .collect();
            let mut cleanup = Fno::new(ccfg, 11);
            let mut a = Adam::new(ccfg.lr);
            for e in 0..(epochs * 3) {
                for (ex, sp) in &level_sets {
                    cleanup.train_epoch(&sp.funcs, ex, &cpool, 32, &mut a, 700 + e as u64);
                }
            }
            println!("  {kname}: {n} composable words, {fills:.1} fillers each, chance MRR {chance:.4}");
            println!("    {:<6} {:>18} {:>18}", "level", "phase only", "cleanup K=3");
            for level in 1..=4 {
                let (m0, _) = comp.recover(&space, level, test_w, None);
                let (m3, _) = comp.recover(&space, level, test_w, Some(&mut cleanup));
                println!("    {:<6} {:>10.4} {:>6.0}x {:>10.4} {:>6.0}x", level, m0, m0 / chance, m3, m3 / chance);
            }
        }
    }
}
