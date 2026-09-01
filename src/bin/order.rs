//! Where does word order belong in a phase manifold?
//!
//! `cargo run --release --bin order -- [corpus] [epochs]`
//!
//! The composition experiment found that binding a *definition* by position
//! hurts: a lexicographer's phrasing puts the same defining word at a different
//! rank in every entry, so rotating by rank scatters a concept instead of
//! locating it. That is a result about definitions, not about order — a
//! definition is a set of constraints that happens to be written in a line,
//! whereas a sentence *is* a sequence and its order is the signal.
//!
//! So this runs the same question on the sequence side, where the codebase
//! already has three constructions and only ever scored two:
//!
//! * **2-word** — recency-weighted sum of the two preceding words. Order enters
//!   as a magnitude weight only; swapping the two words barely moves it.
//! * **bound** — the same two words, each rotated by its offset from the
//!   prediction point. Order is carried in the phase.
//! * **recurrent** — diagonal complex recurrence over the whole prefix, order
//!   carried by the rotation kernel.
//!
//! Two numbers per construction: how much the context actually changes when the
//! two words are swapped (if it does not, order is not represented, whatever the
//! construction claims), and what it is worth in held-out perplexity.

use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::metrics::harness::{ContextKind, Harness, PhianoLM, Split, SweepRow};
use phiano::metrics::kn_baseline::KneserNey;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;

const BETAS: [f64; 5] = [0.5, 1.0, 2.0, 4.0, 8.0];

fn best(rows: &[SweepRow]) -> &SweepRow {
    rows.iter()
        .min_by(|a, b| a.ppl.partial_cmp(&b.ppl).unwrap_or(std::cmp::Ordering::Equal))
        .expect("non-empty sweep")
}

/// Adjacent word pairs from held-out text, for the swap diagnostic.
fn pairs(sentences: &[String], limit: usize) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for s in sentences {
        let t = Tokenizer::tokenize(s);
        for w in t.windows(2) {
            out.push((w[0].clone(), w[1].clone()));
            if out.len() >= limit {
                return out;
            }
        }
    }
    out
}

fn report(name: &str, facet: &Facet, split: &Split, kn: f64) {
    let lm = PhianoLM::with_gamma(facet, 1.0);
    let ps = pairs(&split.valid, 4000);

    println!("\n=== {} ===", name);
    println!(
        "  {:<12} {:>12} {:>10} {:>8} {:>10} {:>12}",
        "context", "swap cos", "best ppl", "best γ", "γ=0 ppl", "phase alone"
    );

    for kind in ContextKind::ALL {
        let rows = lm.sweep_kind(&split.valid, &BETAS, kind, false);
        let b = best(&rows);
        let zero = rows
            .iter()
            .filter(|r| r.gamma == 0.0)
            .map(|r| r.ppl)
            .fold(f64::INFINITY, f64::min);
        // The manifold on its own, against a uniform base: it moves when the
        // representation changes even where the mixed model's best γ does not.
        let alone = lm
            .sweep_kind(&split.valid, &BETAS, kind, true)
            .iter()
            .filter(|r| r.gamma == 1.0)
            .map(|r| r.ppl)
            .fold(f64::INFINITY, f64::min);

        // 1.0 means swapping the two context words leaves the context vector
        // unchanged — order is not in the representation at all. The recurrent
        // state has no two-word form, so it is not measurable this way.
        let swap = match kind {
            ContextKind::Recurrent => f64::NAN,
            k => lm.order_sensitivity(&ps, k),
        };

        println!(
            "  {:<12} {:>12} {:>10.2} {:>8.1} {:>10.2} {:>12.2}",
            kind.label(),
            match swap.is_finite() {
                true => format!("{:.4}", swap),
                false => "n/a".to_string(),
            },
            b.ppl,
            b.gamma,
            zero,
            alone
        );
    }
    println!("  Kneser-Ney trigram: {:.2}", kn);
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

    let (co, _) = Harness::train_and_measure(&split, &trainer, epochs, true);
    report("co-occurrence + ranking", &co, &split, kn);

    let ranking = Harness::train_ranking_only(&split, &trainer, epochs.max(1) * 4);
    report("ranking only", &ranking, &split, kn);

    println!(
        "\nswap cos is the cosine between ctx(a,b) and ctx(b,a): 1.0 means the \
         construction does not represent order at all."
    );
}
