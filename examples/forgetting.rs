//! Catastrophic-forgetting runner.
//!
//! `cargo run --release --bin forgetting -- [n_per_domain]`
//!
//! Domain A is the Rust Book; domain B is Webster's dictionary. They share
//! function words and almost nothing else, which is what makes the shift real.
//!
//! Reports retention against two measured bounds — a model that saw both
//! domains together, and one that never saw A at all — under both objectives.

use phiano::chunker::ChunkStore;
use phiano::config::LEARNING_RATE;
use phiano::metrics::forgetting::{ForgettingBenchmark, ForgettingReport};
use phiano::sources::clean_definition;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;

fn show(label: &str, r: &ForgettingReport) {
    println!("\n=== {} ===", label);
    println!("  A = {} ({} train, {} eval)", r.domain_a_name, r.n_train_a, r.n_eval_a);
    println!("  B = {} ({} train)", r.domain_b_name, r.n_train_b);
    println!();
    println!("  perplexity on held-out A (phase back-off, gamma=1):");
    println!("    trained on A only        : {:.2}", r.a_only_ppl);
    println!("    trained on A+B together  : {:.2}   (ceiling)", r.ceiling_ppl);
    println!("    trained on A then B      : {:.2}   (the measurement)", r.sequential_ppl);
    println!("    trained on B only        : {:.2}   (floor)", r.floor_ppl);
    println!("    counts only, A then B    : {:.2}   (n-gram tallies cannot forget)", r.counts_only_ppl);
    println!();
    println!("  retention   : {:.1}%", r.retention * 100.0);
    println!("  degradation : {:+.1}% on A after learning B", r.degradation_pct);
    println!("  {}", r.verdict);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(3000);

    let raw = match std::fs::read_to_string("data/rust_book_corpus.txt") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("could not read the Rust Book corpus: {}", e);
            std::process::exit(1);
        }
    };
    let a_all: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 5)
        .take(n + n / 4)
        .collect();

    let store = ChunkStore::new("data/chunks");
    let b_all: Vec<String> = store
        .load_all()
        .into_iter()
        .map(|(w, d)| format!("{} {}", w, clean_definition(&d)))
        .filter(|s| Tokenizer::tokenize(s).len() >= 5)
        .take(n)
        .collect();

    if a_all.len() < 100 || b_all.len() < 100 {
        eprintln!("not enough data: A={} B={}", a_all.len(), b_all.len());
        std::process::exit(1);
    }

    let split = a_all.len() * 4 / 5;
    let (a_train, a_eval) = a_all.split_at(split);

    println!(
        "domain A: {} Rust Book sentences ({} held out)\ndomain B: {} dictionary entries",
        a_train.len(),
        a_eval.len(),
        b_all.len()
    );

    let trainer = Trainer::new(LEARNING_RATE);

    let co = ForgettingBenchmark::run(
        &trainer, "Rust Book", a_train, a_eval, "Webster's", &b_all, false,
    );
    show("co-occurrence objective", &co);

    let rank = ForgettingBenchmark::run(
        &trainer, "Rust Book", a_train, a_eval, "Webster's", &b_all, true,
    );
    show("ranking objective", &rank);

    println!("\n--- verdict ---");
    let best = co.retention.max(rank.retention);
    println!(
        "best retention {:.1}% - {}",
        best * 100.0,
        match best {
            r if r >= 0.95 => "supports a strong no-forgetting claim",
            r if r >= 0.7 => "mild forgetting; 'zero catastrophic forgetting' overstates it",
            _ => "does NOT support the zero-catastrophic-forgetting claim in docs/45",
        }
    );

    if let Ok(j) = serde_json::to_string_pretty(&(&co, &rank)) {
        let _ = std::fs::write("data/forgetting.json", j);
        println!("\nwritten to data/forgetting.json");
    }
}
