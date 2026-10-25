//! The evaluation harness runner.
//!
//! `cargo run --release --bin evaluate -- [corpus_path] [epochs]`
//!
//! Trains on 80% of the corpus, measures held-out perplexity on the next 10%,
//! and compares against a Kneser-Ney trigram baseline built from the same
//! training split. The remaining 10% is never touched here — it is reserved for
//! a single final measurement once the design is frozen.

use phiano::metrics::harness::Harness;
use phiano::tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let epochs: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(12);

    let raw = match std::fs::read_to_string(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("could not read {}: {}", path, e);
            std::process::exit(1);
        }
    };

    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();

    println!("corpus: {} sentences from {}", corpus.len(), path);
    println!("epochs: {}\n", epochs);

    let report = Harness::run(corpus, epochs);

    println!("{:<6} {:>10} {:>10} {:>12} {:>9} {:>10} {:>10} {:>7}",
        "epoch", "train", "valid", "valid(-phase)", "coher", "disp", "gini", "vocab");
    for m in &report.epochs {
        println!(
            "{:<6} {:>10.2} {:>10.2} {:>12.2} {:>9.4} {:>10.4} {:>10.4} {:>7}",
            m.epoch, m.train_ppl, m.valid_ppl, m.valid_ppl_no_phase, m.coherence,
            m.phase_dispersion, m.sector_gini, m.vocab_size
        );
    }

    println!("\n--- results ---");
    println!("Kneser-Ney trigram baseline : {:.2}", report.kn_trigram_ppl);
    println!("Phiano best valid perplexity: {:.2}  (epoch {})",
        report.phiano_best_valid_ppl, report.best_epoch);
    println!("Phiano final valid          : {:.2}", report.phiano_final_valid_ppl);
    println!("Attraction-only ablation    : {:.2}", report.attraction_only_best_ppl);
    let best = &report.epochs[report.best_epoch];
    println!("  at best epoch, no phase   : {:.2}", best.valid_ppl_no_phase);
    let delta = best.valid_ppl_no_phase - best.valid_ppl;
    println!(
        "Phase layer contribution    : {:+.2} perplexity ({})",
        -delta,
        if delta > 0.0 { "phase helps" } else { "phase does not help" }
    );
    println!("Final-epoch, no phase       : {:.2}", report.no_phase_backoff_ppl);

    println!("\n--- phase mixing sweep (best epoch, held-out) ---");
    println!("{:>7} {:>12}", "gamma", "perplexity");
    for (g, ppl) in &report.gamma_sweep {
        let mark = if (*g - report.best_gamma).abs() < 1e-9 { "  <-- best" } else { "" };
        println!("{:>7.1} {:>12.2}{}", g, ppl, mark);
    }
    println!("\n--- same sweep, predictive-heavy training (4 extra ranking passes) ---");
    println!("{:>7} {:>12}", "gamma", "perplexity");
    for (g, ppl) in &report.gamma_sweep_predictive {
        let mark = if (*g - report.best_gamma_predictive).abs() < 1e-9 { "  <-- best" } else { "" };
        println!("{:>7.1} {:>12.2}{}", g, ppl, mark);
    }

    println!(
        "\ngamma = 0.0 is the model with the phase manifold removed; \
         gamma = 1.0 is the manifold alone as the back-off distribution."
    );
    println!(
        "best gamma: {:.1} (co-occurrence training), {:.1} (predictive-heavy)",
        report.best_gamma, report.best_gamma_predictive
    );
    println!("\n{}", report.verdict);

    let _ = std::fs::create_dir_all("data");
    match serde_json::to_string_pretty(&report) {
        Ok(j) => {
            let _ = std::fs::write("data/evaluation.json", j);
            println!("\nfull log written to data/evaluation.json");
        }
        Err(e) => eprintln!("could not serialise report: {}", e),
    }
}
