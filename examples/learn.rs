//! Fit the generator's candidate scorer to held-out text.
//! `cargo run --release --bin learn [corpus] [max_train_positions]`
//!
//! Replaces the hand-tuned `0.35 / 0.25 / 0.40 / 0.55 / 1.35 / 0.15` with
//! weights that maximise the likelihood of the *actual* next word among the
//! n-gram's candidates on held-out sentences. Reports counts-only, the hand
//! scorer, and the learned scorer on the same positions, and writes
//! `data/learned.json` for the generator to pick up.

use phiano::config::{LEARNED_FILE, LEARNING_RATE, OPERATOR_FILE};
use phiano::learn::{corpus_positions, GenWeight, Tuner, FEATURE_NAMES};
use phient::metrics::harness::Harness;
use phient::net::space::Sides;
use phiano::tokenizer::Tokenizer;
use phient::train::trainer::Trainer;

use phiano::learn::{CANDIDATES, WINDOW};
use phiano::generate::OperatorGuide;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let cap: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(40_000);
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 3)
        .collect();
    let split = Harness::split(corpus, 42);
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let mut guide = match std::path::Path::new(OPERATOR_FILE).exists() {
        true => OperatorGuide::load(OPERATOR_FILE, &facet, Sides::Directed, WINDOW).ok(),
        false => None,
    };
    println!(
        "lexicon {} words; operator feature {}",
        facet.vocabulary_size(),
        if guide.is_some() { "on" } else { "off (no data/phinet_generate.fno)" }
    );
    let t0 = std::time::Instant::now();
    // Fit on the validation split and evaluate on the test split: the
    // n-gram tables were built on the training split and cover 98% of its
    // positions, which is not the distribution the generator meets.
    let (train, seen_t, cov_t) = corpus_positions(&facet, &split.valid, cap, &mut guide);
    let (held, seen_h, cov_h) = corpus_positions(&facet, &split.test, cap, &mut guide);
    println!(
        "positions: fit on valid {} (of {} seen, {:.0}% covered by the n-gram's top {CANDIDATES}), evaluate on test {} (of {}, {:.0}%), {:.0}s",
        train.len(), seen_t, 100.0 * cov_t as f64 / seen_t.max(1) as f64, held.len(), seen_h, 100.0 * cov_h as f64 / seen_h.max(1) as f64, t0.elapsed().as_secs_f64()
    );

    let counts = GenWeight::counts_only();
    let hand = GenWeight::hand();
    println!("\n  {:<24} held-out NLL {:>7.4}   ppl among candidates {:>7.2}", "counts only", Tuner::nll(&counts.w, &held), Tuner::nll(&counts.w, &held).exp());
    println!("  {:<24} held-out NLL {:>7.4}   ppl among candidates {:>7.2}", "hand-tuned (as shipped)", Tuner::nll(&hand.w, &held), Tuner::nll(&hand.w, &held).exp());
    let (learned, trace) = Tuner::default().fit(counts.w, &train, &held);
    println!("  {:<24} held-out NLL {:>7.4}   ppl among candidates {:>7.2}   ({} epochs, best at {})", "learned", learned.heldout_nll, learned.heldout_nll.exp(), trace.len() - 1,
        trace.iter().enumerate().min_by(|a, b| a.1.partial_cmp(b.1).unwrap()).map(|x| x.0).unwrap_or(0));
    println!("\n  learned weights:");
    for (n, w) in FEATURE_NAMES.iter().zip(&learned.w) {
        println!("    {n:<14} {w:+.4}");
    }
    learned.save(LEARNED_FILE).expect("write learned.json");
    println!("\n  saved {LEARNED_FILE}");
}
