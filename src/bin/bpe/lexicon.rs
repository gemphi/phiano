//! Words versus Phi-4 pieces as the units on the relation circle, on Austen:
//! the same trainer, the same directed reading, the same pool floor.
//! `cargo run --release --bin bpe_lexicon`
use phiano::bpe::Bpe;
use phiano::config::LEARNING_RATE;
use phiano::metrics::harness::{Harness, Split};
use phiano::phinet::space::{examples, FunctionSpace, Sides, WordFunction};
use phical::rank;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;

use phiano::lab::sentences_of;

fn measure(name: &str, split: &Split) {
    let t0 = std::time::Instant::now();
    let facet = Harness::train_ranking_only(split, &Trainer::new(LEARNING_RATE).with_seed(0), 3);
    let space = FunctionSpace::from_facet_sides(&facet, Sides::Directed);
    let pool = space.pool(20);
    let valid: Vec<Vec<String>> = split.valid.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let (mut ex, _) = examples(&space, &valid, 6, &pool, u64::MAX);
    ex.truncate(3000);
    let inputs: Vec<WordFunction> = ex.iter().map(|e| e.input.clone()).collect();
    let chance = { let n = pool.len() as f64; (1..=pool.len()).map(|k| 1.0 / k as f64).sum::<f64>() / n };
    let (mrr, hit) = rank(space.table(), &inputs, &ex, &pool);
    let types: std::collections::HashSet<String> = split.train.iter().flat_map(|s| Tokenizer::tokenize(s)).collect();
    println!("| {name} | {} | {} | {} | {:.4} | {:.4} | {:.1}× | {:.3} | {:.0}s |", types.len(), facet.evidenced_words(), pool.len(), chance, mrr, mrr / chance, hit, t0.elapsed().as_secs_f64());
}

fn main() {
    let mut all = Vec::new();
    for f in ["emma", "persuasion", "sense_and_sensibility"] {
        all.extend(sentences_of(&format!("data/corpora/austen/{f}.txt")));
    }
    let words = Harness::split(all.clone(), 7);
    let bpe = Bpe::from_dir("data/tokenizers/phi4").expect("tokenizer");
    // A piece sentence: every piece as one lowercase alphanumeric token, `Ġ` (space) kept as the letter ġ so "the" and " the" stay distinct pieces.
    let piecify = |s: &str| -> String {
        bpe.pieces(&s.to_lowercase()).iter().map(|p| p.chars().filter(|c| c.is_alphanumeric()).collect::<String>()).filter(|p| !p.is_empty()).collect::<Vec<_>>().join(" ")
    };
    let pieces = Split { train: words.train.iter().map(|s| piecify(s)).collect(), valid: words.valid.iter().map(|s| piecify(s)).collect(), test: words.test.iter().map(|s| piecify(s)).collect() };
    println!("| units | types (train) | evidenced | pool ≥20 | chance MRR | directed MRR | × chance | hit@10 | train |\n|---|---|---|---|---|---|---|---|---|");
    measure("words", &words);
    measure("Phi-4 pieces", &pieces);
}
