//! Experiment runner for the four questions in `docs/how/RESULTS.md` §7.
//!
//! `cargo run --release --bin experiment -- [corpus] [epochs]`
//!
//! Sweeps three axes against held-out perplexity:
//!
//! * **training**: co-occurrence + ranking, versus ranking only
//! * **context**:  a two-word centroid, versus the recurrent state over the
//!                 whole prefix
//! * **temperature** of the phase distribution, and the **mixing weight** γ
//!   against a unigram back-off
//!
//! γ = 0 removes the manifold while changing nothing else, so the best γ in each
//! configuration is a direct measurement of what the phase layer contributes.

use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::metrics::harness::{Harness, PhianoLM, SweepRow};
use phiano::metrics::kn_baseline::KneserNey;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;

const BETAS: [f64; 5] = [0.25, 0.5, 1.0, 2.0, 4.0];

fn best_row(rows: &[SweepRow]) -> &SweepRow {
    rows.iter()
        .min_by(|a, b| a.ppl.partial_cmp(&b.ppl).unwrap_or(std::cmp::Ordering::Equal))
        .expect("non-empty sweep")
}

fn report(name: &str, facet: &Facet, valid: &[String], kn: f64) -> Vec<SweepRow> {
    let lm = PhianoLM::with_gamma(facet, 1.0);
    let mut rows = lm.sweep(valid, &BETAS, false);
    rows.extend(lm.sweep(valid, &BETAS, true));

    let baseline = rows
        .iter()
        .filter(|r| r.gamma == 0.0)
        .map(|r| r.ppl)
        .fold(f64::INFINITY, f64::min);
    let best = best_row(&rows);

    println!("\n=== {} ===", name);
    println!("  no-phase baseline (γ=0)   : {:.2}", baseline);
    println!(
        "  best overall              : {:.2}  [context {}, β {:.2}, γ {:.1}]",
        best.ppl, best.context, best.beta, best.gamma
    );
    println!("  Kneser-Ney trigram        : {:.2}", kn);

    // Best γ for each (context, β) pair — the shape of the contribution.
    println!("  {:>10} {:>6} {:>8} {:>12} {:>10}", "context", "beta", "best γ", "ppl at best γ", "vs γ=0");
    for context in ["2-word", "recurrent"] {
        for beta in BETAS {
            let cell: Vec<&SweepRow> = rows
                .iter()
                .filter(|r| r.context == context && (r.beta - beta).abs() < 1e-9)
                .collect();
            if cell.is_empty() {
                continue;
            }
            let b = cell
                .iter()
                .min_by(|x, y| x.ppl.partial_cmp(&y.ppl).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap();
            let zero = cell.iter().find(|r| r.gamma == 0.0).map(|r| r.ppl).unwrap_or(f64::NAN);
            let delta = zero - b.ppl;
            println!(
                "  {:>10} {:>6.2} {:>8.1} {:>12.2} {:>+10.2}",
                context, beta, b.gamma, b.ppl, delta
            );
        }
    }
    rows
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let epochs: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);

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

    println!("corpus: {} sentences, {} epochs", corpus.len(), epochs);

    let split = Harness::split(corpus, 42);
    let kn = KneserNey::train(&split.train).perplexity(&split.valid);
    let trainer = Trainer::new(LEARNING_RATE);

    let mut all: Vec<SweepRow> = Vec::new();

    let (co_occurrence, _) = Harness::train_and_measure(&split, &trainer, epochs, true);
    all.extend(report("A. co-occurrence + ranking", &co_occurrence, &split.valid, kn));

    let ranking_only = Harness::train_ranking_only(&split, &trainer, epochs.max(1) * 4);
    all.extend(report("B. ranking objective only", &ranking_only, &split.valid, kn));

    // Control: how much information does the phase distribution carry at all?
    // Mixing against a uniform base means it only has to beat knowing nothing.
    println!("\n=== C. control — phase against a uniform base ===");
    // Reference point: the unigram back-off measured through the *same* code
    // path as the phase and uniform figures. `perplexity()` scores
    // out-of-vocabulary targets at the floor while `sweep` skips them, so
    // mixing the two would put a different denominator under the ratio.
    let unigram_ppl = PhianoLM::with_gamma(&co_occurrence, 1.0)
        .sweep_against(&split.valid, &BETAS, true, false)
        .iter()
        .filter(|r| r.gamma == 0.0)
        .map(|r| r.ppl)
        .fold(f64::INFINITY, f64::min);
    for (name, facet) in [("co-occurrence", &co_occurrence), ("ranking-only", &ranking_only)] {
        let lm = PhianoLM::with_gamma(facet, 1.0);
        let rows = lm.sweep_against(&split.valid, &BETAS, true, true);
        let uniform_only = rows
            .iter()
            .filter(|r| r.gamma == 0.0)
            .map(|r| r.ppl)
            .fold(f64::INFINITY, f64::min);
        let phase_only = rows
            .iter()
            .filter(|r| r.gamma == 1.0)
            .map(|r| r.ppl)
            .fold(f64::INFINITY, f64::min);
        let best = best_row(&rows);

        // How much of the predictive signal does the manifold recover?
        //
        // On a log scale, uniform is the floor (knowing nothing) and unigram
        // frequency is a strong, trivially-available reference. The fraction of
        // that gap the phase distribution closes is a scale-free measure of how
        // much the representation actually knows.
        let recovery = ((uniform_only.ln() - phase_only.ln())
            / (uniform_only.ln() - unigram_ppl.ln()))
            * 100.0;

        println!(
            "  {:<14} uniform {:.2} | phase {:.2} | unigram {:.2} | best γ {:.1} at {:.2}",
            name, uniform_only, phase_only, unigram_ppl, best.gamma, best.ppl
        );
        println!(
            "  {:<14} → phase {} uniform, and recovers {:.1}% of the signal unigram frequency provides",
            "",
            if phase_only < uniform_only { "beats" } else { "loses to" },
            recovery
        );
        all.extend(rows);
    }

    println!("\n--- conclusion ---");
    let best = best_row(&all);
    match best.gamma > 0.0 {
        true => println!(
            "The phase manifold contributes: best configuration is γ = {:.1} \
             ({}, β = {:.2}) at {:.2} perplexity, against {:.2} for Kneser-Ney.",
            best.gamma, best.context, best.beta, best.ppl, kn
        ),
        false => println!(
            "The phase manifold still contributes nothing: the best configuration \
             across every context, temperature and training regime is γ = 0, \
             i.e. the manifold removed. Best perplexity {:.2}, Kneser-Ney {:.2}.",
            best.ppl, kn
        ),
    }

    if let Ok(j) = serde_json::to_string_pretty(&all) {
        let _ = std::fs::write("data/experiment.json", j);
        println!("\nfull grid written to data/experiment.json");
    }
}
