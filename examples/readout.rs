//! HOW 16's open question, measured.
//!
//! `cargo run --release --bin readout -- [corpus] [epochs]`
//!
//! Every scoring path in this engine is linear: a sum of unit vectors dotted
//! against a candidate, then argmax. HOW 16 argued that a *non-linear* readout
//! is the fourth thing missing, and `SectorReadout` implements one — a bias
//! table conditioned jointly on the context cell and the candidate's target
//! sector, which is the only shape that can reorder a ranking.
//!
//! The question is whether it helps. The measurement is the γ sweep, run twice
//! on the same facet and the same split, with the readout fitted on `train` and
//! off. If the manifold's contribution is limited by linearity, the readout
//! should move the best γ off zero. If γ* stays at zero the readout does not
//! rescue the manifold, and HOW 16's conjecture is wrong on this corpus.
//!
//! The readout is fitted on the training split only. A lookup table fitted on
//! the text it is scored against measures memorisation.

use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::metrics::harness::{Harness, PhianoLM, Split, SweepRow};
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;

const BETAS: [f64; 3] = [0.5, 1.0, 2.0];
const READOUT_LR: f64 = 0.5;

/// Perplexity of the phase distribution alone, mixed against a uniform base.
///
/// γ = 1 against uniform strips out both the n-gram tables and word frequency,
/// so what is left is the manifold on its own. It moves when the manifold
/// changes even where the mixed model's best γ does not.
fn phase_only(lm: &PhianoLM, split: &Split, recurrent: bool) -> f64 {
    lm.sweep_against(&split.valid, &BETAS, recurrent, true)
        .iter()
        .filter(|r| r.gamma == 1.0)
        .map(|r| r.ppl)
        .fold(f64::INFINITY, f64::min)
}

struct Cell {
    best_gamma: f64,
    best_ppl: f64,
    ppl_at_zero: f64,
}

fn summarise(rows: &[SweepRow]) -> Cell {
    let best = rows
        .iter()
        .min_by(|a, b| a.ppl.partial_cmp(&b.ppl).unwrap_or(std::cmp::Ordering::Equal))
        .expect("non-empty sweep");
    let ppl_at_zero = rows
        .iter()
        .filter(|r| r.gamma == 0.0)
        .map(|r| r.ppl)
        .fold(f64::INFINITY, f64::min);
    Cell { best_gamma: best.gamma, best_ppl: best.ppl, ppl_at_zero }
}

/// One (facet, context-kind) configuration, scored with and without the readout.
fn compare(name: &str, facet: &Facet, split: &Split, recurrent: bool) {
    let mut lm = PhianoLM::with_gamma(facet, 1.0);

    let off = summarise(&lm.sweep(&split.valid, &BETAS, recurrent));
    // The phase distribution measured against a uniform base, which is the
    // sensitive test: γ* is a coarse binary and can stay at 0 while the
    // manifold itself gets materially better or worse.
    let phase_off = phase_only(&lm, split, recurrent);

    lm.fit_readout(&split.train, READOUT_LR, recurrent);
    let cells = lm.readout_cells();
    lm.reset_readout_coverage();
    let on = summarise(&lm.sweep(&split.valid, &BETAS, recurrent));
    // Coverage separates the two ways a table can fail: never hitting a fitted
    // cell on held-out text, versus hitting and not helping. Without it "no
    // effect" is unreadable.
    let coverage = lm.readout_coverage();
    let phase_on = phase_only(&lm, split, recurrent);

    let ctx = if recurrent { "recurrent" } else { "2-word" };
    println!("\n--- {} / {} context ---", name, ctx);
    println!("  readout cells fitted : {}", cells);
    println!(
        "  held-out coverage    : {:.1}%  ({})",
        coverage * 100.0,
        if coverage < 0.05 {
            "table almost never hits — it cannot help or hurt"
        } else {
            "the table is being consulted"
        }
    );
    println!(
        "  phase alone vs unif  : {:.2} → {:.2}  ({:+.2}%)",
        phase_off,
        phase_on,
        100.0 * (phase_on / phase_off - 1.0)
    );
    println!(
        "  readout off          : best γ {:.1}, ppl {:.2}  (γ=0: {:.2})",
        off.best_gamma, off.best_ppl, off.ppl_at_zero
    );
    println!(
        "  readout on           : best γ {:.1}, ppl {:.2}  (γ=0: {:.2})",
        on.best_gamma, on.best_ppl, on.ppl_at_zero
    );

    // γ=0 removes the phase term entirely, so the readout cannot touch it. If
    // these two differ, the harness is leaking and the comparison is void.
    let leak = (on.ppl_at_zero - off.ppl_at_zero).abs();
    if leak > 1e-6 {
        println!("  !! γ=0 moved by {:.3e} — the readout is leaking into the no-phase path", leak);
    }

    match (on.best_gamma > 0.0, off.best_gamma > 0.0) {
        (true, false) => println!(
            "  VERDICT              : the readout moves γ* off zero ({:.1}), \
             gaining {:.2} perplexity. Linearity was a binding constraint.",
            on.best_gamma,
            off.ppl_at_zero - on.best_ppl
        ),
        (true, true) => println!(
            "  VERDICT              : γ* was already positive; readout changes \
             best ppl by {:+.2}.",
            on.best_ppl - off.best_ppl
        ),
        (false, _) if coverage < 0.05 => println!(
            "  VERDICT              : γ* stays at 0, but the table hit only \
             {:.1}% of held-out contexts — this is a coverage failure, not a \
             verdict on non-linearity.",
            coverage * 100.0
        ),
        (false, _) => println!(
            "  VERDICT              : γ* stays at 0 with the table consulted on \
             {:.0}% of positions. A non-linear readout does not rescue the \
             manifold here — the limit is not linearity.",
            coverage * 100.0
        ),
    }
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
    let trainer = Trainer::new(LEARNING_RATE);

    // Both training regimes, because §3 showed the objective is what decides
    // whether the manifold carries anything at all. A readout that only helps
    // the weaker regime is not evidence for the readout.
    let (co_occurrence, _) = Harness::train_and_measure(&split, &trainer, epochs, true);
    let ranking_only = Harness::train_ranking_only(&split, &trainer, epochs.max(1) * 4);

    println!("\n=== non-linear readout, on versus off ===");
    for (name, facet) in [
        ("co-occurrence + ranking", &co_occurrence),
        ("ranking only", &ranking_only),
    ] {
        for recurrent in [false, true] {
            compare(name, facet, &split, recurrent);
        }
    }
}
