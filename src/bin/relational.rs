//! Does breaking channel symmetry build the dimensions?
//! `cargo run --release --bin relational [corpus]`
//!
//! `bin/dimensions` found no relation-specific dimensions in the shipped
//! manifold and named the reason: training is symmetric in the channels, so
//! nothing can push one toward meaning something. `bin/constructed` found the
//! magnitude inert, and named that reason too: one amplitude per word factors
//! out of a normalised comparison exactly.
//!
//! [`phiano::relational`] fixes both. A word carries a phase *and a magnitude*
//! per channel; channel `r` belongs to relation `r`; an observation moves the
//! channel of the relation it observed and no other. The relations come from the
//! text — relative offset, and the most frequent word standing between a pair.
//!
//! This runs the same three tests `bin/dimensions` ran, on the new
//! representation, so the two pages are directly comparable:
//!
//! 1. Does a relation family have a channel, beyond what a permuted-filler null
//!    reaches on its own best channel?
//! 2. Do different families claim different channels?
//! 3. Does the best single dimension beat the average of all of them — the
//!    practical form of the claim, and the one that failed at 43.9% before?
//!
//! And the comparison that decides whether any of it was worth doing:
//! pair-versus-random against the shipped trained manifold, on identical probes.

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::{RelationBenchmark, RelationFamily};
use phiano::phasor::fnv1a;
use phiano::relational::{RelationVocab, RelationalFacet};
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;

const COUNT_FLOOR: u32 = 25;
const DRAWS: usize = 64;

/// Pairs a channel must cover before it can be a family's best channel.
const MIN_COVERAGE: usize = 6;

fn permute(pairs: &[(String, String)], seed: u64) -> Vec<(String, String)> {
    let mut fillers: Vec<String> = pairs.iter().map(|p| p.1.clone()).collect();
    let mut r = seed | 1;
    for i in (1..fillers.len()).rev() {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        fillers.swap(i, (r % (i as u64 + 1)) as usize);
    }
    pairs.iter().zip(fillers).map(|(p, f)| (p.0.clone(), f)).collect()
}

/// Per-channel agreement for a family, over the pairs that have evidence there.
///
/// The averaging matters. Dividing by the whole family size scores a channel as
/// zero wherever a pair has never been seen — so a sparse marker channel where
/// three pairs agree perfectly reports 3/22, and only channels every pair
/// populates can ever come first. The first version did that, and duly reported
/// all ten families peaking on offset channels, which is what the measurement
/// could say rather than what the data said.
///
/// Here each channel is averaged over its own covered pairs, and channels with
/// too little coverage are excluded rather than diluted — a channel two pairs
/// agree on is not evidence that the family lives there.
fn profile(f: &RelationalFacet, pairs: &[(String, String)]) -> (Vec<f64>, Vec<usize>, usize) {
    let mut acc = vec![0.0f64; PHASE_CHANNELS];
    let mut cov = vec![0usize; PHASE_CHANNELS];
    let mut n = 0usize;
    for (a, b) in pairs {
        let (pa, pb) = match (f.words.get(a), f.words.get(b)) {
            (Some(x), Some(y)) => (x, y),
            _ => continue,
        };
        for k in 0..PHASE_CHANNELS {
            if pa.mag[k] > 0.0 && pb.mag[k] > 0.0 {
                acc[k] += (pa.theta[k] - pb.theta[k]).cos();
                cov[k] += 1;
            }
        }
        n += 1;
    }
    for k in 0..PHASE_CHANNELS {
        acc[k] = match cov[k] >= MIN_COVERAGE {
            true => acc[k] / cov[k] as f64,
            // Not "zero agreement" — no measurement. Excluded from the maximum.
            false => f64::NEG_INFINITY,
        };
    }
    (acc, cov, n)
}

fn argmax(v: &[f64]) -> (usize, f64) {
    v.iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, x)| (i, *x))
        .unwrap_or((0, 0.0))
}

fn pair_vs_random<F: Fn(&str, &str) -> f64>(
    pool: &[String],
    families: &[RelationFamily],
    known: &dyn Fn(&str) -> bool,
    sim: F,
) -> f64 {
    let (mut wins, mut n) = (0.0f64, 0usize);
    for fam in families {
        for (i, p) in fam.pairs.iter().enumerate() {
            if !known(&p.a) || !known(&p.b) {
                continue;
            }
            let target = sim(&p.a, &p.b);
            let (mut hits, mut draws) = (0usize, 0usize);
            for d in 0..DRAWS {
                let r = (fnv1a(&p.a) ^ ((i as u64) << 32) ^ (d as u64).wrapping_mul(0x9E37_79B9))
                    as usize;
                let cand = &pool[r % pool.len().max(1)];
                if cand == &p.a || cand == &p.b {
                    continue;
                }
                draws += 1;
                if target > sim(&p.a, cand) {
                    hits += 1;
                }
            }
            if draws > 0 {
                wins += hits as f64 / draws as f64;
                n += 1;
            }
        }
    }
    match n {
        0 => 0.5,
        _ => wins / n as f64,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/prose_corpus.txt".to_string());
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = raw
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| Tokenizer::tokenize(l).len() >= 5)
        .collect();
    if corpus.len() < 1000 {
        eprintln!("corpus too small at {path}. Run `bin/prepare_prose` first.");
        std::process::exit(1);
    }

    let split = Harness::split(corpus, 42);
    let toks: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();

    // The shipped path, for comparison.
    let base = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let mut shipped = base.clone();
    Descent::cycle(&mut shipped, &toks, Up::Bag, 4, 0.25, true);

    // The new one.
    let vocab = RelationVocab::learn(&toks);
    let rel = RelationalFacet::train(&toks, &vocab, 2);

    println!(
        "shipped {} words; relational {} words, {} marker channels + {} offset",
        shipped.vocabulary_size(),
        rel.len(),
        vocab.markers(),
        phiano::relational::OFFSET_CHANNELS
    );
    println!("  {} of {} channels survived the per-channel collapse guard", rel.live, PHASE_CHANNELS);
    println!(
        "  channels 0..12 are: {}",
        (0..12).map(|k| vocab.name(k)).collect::<Vec<_>>().join(", ")
    );

    let families = RelationBenchmark::default_families();

    // ---- 1 and 2 ----
    println!(
        "\n  {:<14} {:>6} {:>16} {:>6} {:>10} {:>10} {:>9}",
        "family", "pairs", "best channel", "cov", "agreement", "null best", "excess"
    );
    let mut peaks: Vec<usize> = Vec::new();
    let mut excesses: Vec<f64> = Vec::new();
    for fam in &families {
        let pairs: Vec<(String, String)> =
            fam.pairs.iter().map(|p| (p.a.clone(), p.b.clone())).collect();
        let (prof, cov, n) = profile(&rel, &pairs);
        if n < 5 {
            continue;
        }
        let (k, v) = argmax(&prof);
        if !v.is_finite() {
            println!("  {:<14} {:>6} {:>16}", fam.name, n, "no covered channel");
            continue;
        }
        let (n1, _, _) = profile(&rel, &permute(&pairs, 0x9E37_79B9_7F4A_7C15));
        let (n2, _, _) = profile(&rel, &permute(&pairs, 0xC2B2_AE3D_27D4_EB4F));
        let nb1 = argmax(&n1).1;
        let nb2 = argmax(&n2).1;
        let null_best = match nb1.is_finite() && nb2.is_finite() {
            true => (nb1 + nb2) / 2.0,
            false => f64::NAN,
        };
        println!(
            "  {:<14} {:>6} {:>16} {:>6} {:>10.4} {:>10.4} {:>+9.4}",
            fam.name,
            n,
            format!("{} ({})", k, vocab.name(k)),
            cov[k],
            v,
            null_best,
            v - null_best
        );
        peaks.push(k);
        excesses.push(v - null_best);
    }
    let distinct: std::collections::HashSet<usize> = peaks.iter().copied().collect();
    let worst = excesses.iter().cloned().fold(f64::INFINITY, f64::min);
    println!(
        "\n  {} families on {} distinct channels. Worst excess {:+.4} — that is the\n\
         \x20 noise scale, and no positive row counts unless it clears it.",
        peaks.len(),
        distinct.len(),
        worst
    );

    // ---- 3, and the comparison ----
    let mut pool: Vec<String> = rel
        .counts
        .iter()
        .filter(|(_, c)| **c >= COUNT_FLOOR)
        .map(|(w, _)| w.clone())
        .collect();
    pool.sort();
    let known = |w: &str| rel.words.contains_key(w) && shipped.lexicon.contains_key(w);

    println!("\n  {:<40} {:>14}", "similarity", "pair>random");
    let shipped_score = pair_vs_random(&pool, &families, &known, |a, b| shipped.resonance(a, b));
    println!("  {:<40} {:>13.1}%", "shipped: mean of 64 channels", shipped_score * 100.0);

    let rel_all = pair_vs_random(&pool, &families, &known, |a, b| rel.similarity(a, b));
    println!("  {:<40} {:>13.1}%", "relational: evidence-weighted, all", rel_all * 100.0);

    let rel_max = pair_vs_random(&pool, &families, &known, |a, b| {
        (0..PHASE_CHANNELS).map(|k| rel.on(a, b, k)).fold(f64::NEG_INFINITY, f64::max)
    });
    println!("  {:<40} {:>13.1}%", "relational: best single dimension", rel_max * 100.0);

    let rel_top8 = pair_vs_random(&pool, &families, &known, |a, b| {
        let mut v: Vec<f64> = (0..PHASE_CHANNELS).map(|k| rel.on(a, b, k)).collect();
        v.sort_by(|p, q| q.partial_cmp(p).unwrap_or(std::cmp::Ordering::Equal));
        v[..8].iter().sum::<f64>() / 8.0
    });
    println!("  {:<40} {:>13.1}%", "relational: mean of best 8", rel_top8 * 100.0);

    println!(
        "\n  Chance is 50.0%. The shipped row scored 61.3% with the mean and 43.9%\n\
         \x20 with the best dimension — averaging beat specialising, because there was\n\
         \x20 nothing specialised to find. If the symmetry break worked, the best\n\
         \x20 dimension is no longer the worst option here."
    );
}
