//! Phinet: what is in the existing models, and what a Fourier neural operator
//! learns on top of the function space they define.
//! `cargo run --release --bin phinet [corpus] [sentences] [epochs]`
//!
//! Three questions, in order.
//!
//! 1. **What is actually in `data/`?** `phistd::reader::inspect` opens each
//!    model file and reports it: format version, words, evidence, n-grams,
//!    dispersion. The shipped `manifold.chroma` did not load at all before the
//!    v1 reader was rewritten; this is the first look at it since.
//!
//! 2. **Does a learned operator beat the superposition?** The engine predicts
//!    the next word by superposing the context's phasors and ranking by
//!    resonance. In function space that is the identity operator applied to the
//!    context function. A Fourier neural operator is trained on the same
//!    `(context → next word)` pairs and ranked on held-out sentences against
//!    the same candidate pool. The pointwise control (`modes = 0`) has the
//!    spectral path removed and cannot move information between channels, so
//!    the gap between it and the full operator is what the spectral filter is
//!    worth.
//!
//! 3. **Does the promoted lexicon give the operator more to work with?** The
//!    same three arms run on the flat function space of the shipped v1 manifold
//!    (one angle per word, equal amplitude everywhere) and on an evidenced
//!    function space trained on the corpus with relation coupling.

use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::phinet::reader;
use phiano::phinet::space::{examples, Example, FunctionSpace, Sides, WordFunction};
use phical::rank;
use phiano::phinet::{Adam, Fno, FnoConfig};
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;

const POOL_FLOOR: u32 = 25;
const WINDOW: usize = 6;
const BATCH: usize = 64;
const HOLDOUT_EVERY: u64 = 10;
const MAX_HELD: usize = 3000;

/// The operator configuration every measurement in `docs/65` used.
fn default_config(space: usize) -> FnoConfig {
    FnoConfig { space, width: 8, layers: 2, modes: 12, margin: 0.2, lr: 2e-3, tie: phical::Tie::Circle }
}

fn run_arms(label: &str, space: &FunctionSpace, sentences: &[Vec<String>], epochs: usize) {
    // Candidate pool: words the corpus uses at least POOL_FLOOR times, so the
    // v1 manifold (which stores no counts) and the trained lexicon get the same
    // rule.
    let mut freq: std::collections::HashMap<&str, u32> = std::collections::HashMap::new();
    for s in sentences {
        for w in s {
            *freq.entry(w.as_str()).or_insert(0) += 1;
        }
    }
    let pool: Vec<usize> = (0..space.len())
        .filter(|&i| freq.get(space.words[i].as_str()).copied().unwrap_or(0) >= POOL_FLOOR)
        .collect();
    let (train, mut held) = examples(space, sentences, WINDOW, &pool, HOLDOUT_EVERY);
    held.truncate(MAX_HELD);
    println!(
        "\n=== {label}: {} words in space ({}), pool {}, {} train / {} held-out examples ===",
        space.len(),
        if space.evidenced { "evidenced" } else { "flat" },
        pool.len(),
        train.len(),
        held.len()
    );
    if train.is_empty() || held.is_empty() || pool.len() < 32 {
        println!("  not enough data");
        return;
    }

    let inputs: Vec<WordFunction> = held.iter().map(|e| e.input.clone()).collect();
    let (mrr_id, hit_id) = rank(space.table(), &inputs, &held, &pool);
    let chance = {
        // Expected reciprocal rank of a random ranking of `pool` = H_N / N.
        let n = pool.len() as f64;
        (1..=pool.len()).map(|k| 1.0 / k as f64).sum::<f64>() / n
    };
    println!("  {:<34} {:>9} {:>8} {:>10}", "arm", "MRR", "hit@10", "vs chance");
    println!("  {:<34} {:>9.5} {:>8.3} {:>9.1}x", "identity (superposition)", mrr_id, hit_id, mrr_id / chance);

    let arms = [
        ("pointwise control (modes=0)", FnoConfig { modes: 0, ..default_config(space.space()) }),
        ("fourier operator (modes=12)", default_config(space.space())),
    ];
    for (name, cfg) in arms {
        let t0 = std::time::Instant::now();
        let mut fno = Fno::new(cfg, 7);
        let mut adam = Adam::new(cfg.lr);
        let mut losses = Vec::new();
        for epoch in 0..epochs {
            let l = fno.train_epoch(space.table(), &train, &pool, BATCH, &mut adam, epoch as u64);
            losses.push(l);
        }
        let preds = fno.apply(&inputs);
        let (mrr, hit) = rank(space.table(), &preds, &held, &pool);
        println!(
            "  {:<34} {:>9.5} {:>8.3} {:>9.1}x   loss {} | {} params, {:.0}s",
            name,
            mrr,
            hit,
            mrr / chance,
            losses.iter().map(|l| format!("{l:.3}")).collect::<Vec<_>>().join(" → "),
            fno.parameter_count(),
            t0.elapsed().as_secs_f64()
        );
        if cfg.modes > 0 {
            let path = format!("data/phinet_{}.fno", label.replace(' ', "_"));
            if std::fs::write(&path, fno.to_bytes()).is_ok() {
                println!("  saved operator to {path}");
            }
        }
    }
    println!("  chance MRR for this pool: {chance:.5}");
    let _: &Example = &held[0];
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| {
        match std::path::Path::new("data/prose_corpus.txt").exists() {
            true => "data/prose_corpus.txt".to_string(),
            false => "data/rust_book_corpus.txt".to_string(),
        }
    });
    let cap: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(20_000);
    let epochs: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(4);

    // ---- 1. what is in the existing models ----
    println!("=== existing models ===");
    let mut shipped: Option<Facet> = None;
    for m in ["data/manifold.chroma", "data/memory.chroma"] {
        if !std::path::Path::new(m).exists() {
            println!("{m}: not present");
            continue;
        }
        let (report, facet) = reader::inspect(m);
        print!("{}", report.render());
        if m.ends_with("manifold.chroma") {
            shipped = facet;
        }
    }

    // ---- corpus ----
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = raw
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| Tokenizer::tokenize(l).len() >= 5)
        .take(cap)
        .collect();
    if corpus.len() < 500 {
        eprintln!("corpus too small at {path}");
        std::process::exit(1);
    }
    let sentences: Vec<Vec<String>> = corpus.iter().map(|s| Tokenizer::tokenize(s)).collect();
    println!("\ncorpus: {path}, {} sentences, {} epochs", corpus.len(), epochs);

    // ---- 2. the shipped manifold's function space ----
    if let Some(facet) = shipped {
        let space = FunctionSpace::from_facet(&facet);
        run_arms("shipped v1 manifold", &space, &sentences, epochs);
    }

    // ---- 3. an evidenced function space from the promoted lexicon ----
    let split = Harness::split(corpus.clone(), 42);
    let t0 = std::time::Instant::now();
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 2);
    println!(
        "\ntrained promoted lexicon on {} sentences in {:.0}s: {} words, {} evidenced",
        split.train.len(),
        t0.elapsed().as_secs_f64(),
        facet.vocabulary_size(),
        facet.evidenced_words()
    );
    // ---- 4. which side(s) should the reader see? (format v5+: two sides) ----
    for (label, sides) in [
        ("promoted lexicon", Sides::Source),
        ("promoted lexicon target_sides", Sides::Target),
        ("promoted lexicon both_sides", Sides::Both),
        ("promoted lexicon directed", Sides::Directed),
    ] {
        let space = FunctionSpace::from_facet_sides(&facet, sides);
        run_arms(label, &space, &sentences, epochs);
    }
}
