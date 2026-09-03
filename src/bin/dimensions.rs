//! Is channel `k` the *k-th relation*?
//! `cargo run --release --bin dimensions [corpus]`
//!
//! The architecture under test: a word occupies 64 dimensions on the complex
//! plane, and each dimension is one of the 64 possible relations it can stand in.
//! Channel 17 is not "channel 17" — it is whatever relation 17 is, and a word's
//! value there says how that word participates in it. Composition then has
//! somewhere to happen: to compose along a relation, you work in its dimension.
//!
//! Phiano does not do this. Its 64 channels are interchangeable — hash-derived
//! angles differing only in decay and frequency — and **every score in this
//! project averages over all of them**. `SpectralPhasor::resonance` sums 64
//! cosines and divides by 64.
//!
//! That averaging is fatal to the proposed architecture. If channel 17 says
//! "these two words stand in a gender relation" and the other 63 say nothing,
//! the mean reports 1/64th of the signal and calls it noise. `bin/bands` already
//! found the channel profile flat, but it averaged within bands of eight, which
//! would bury a single specialised channel just as effectively.
//!
//! # What would show the architecture is there
//!
//! Three things, in order of strength:
//!
//! 1. **A family has a channel.** Gender pairs agree far more on some channel
//!    than shuffled pairs do on their best channel. Compared against a
//!    permuted-filler null, because the maximum over 64 channels is upward-biased
//!    and a null that has not paid the same bias is not a floor.
//! 2. **Different families have different channels.** If gender, plural and
//!    antonym all peak on the same channel, that channel is not a relation — it
//!    is whichever dimension happens to carry the most variance.
//! 3. **Max beats mean.** If relations live in single dimensions, scoring a pair
//!    by its best channel should beat averaging over all of them. This is the
//!    practical form of the claim, and the cheapest thing to act on if true.
//!
//! A flat result here does not refute the architecture. It says the architecture
//! is not present — which would be unsurprising, since nothing in the training
//! objective ever asks a channel to specialise. That distinction matters: "the
//! design is wrong" and "the design was never built" call for different work.
//!
//! # What it measured
//!
//! ```text
//! family        pairs  best chan  agreement  null best    excess
//! gender           22          1     0.4571     0.3322   +0.1249
//! nationality      25         25     0.5431     0.4262   +0.1169
//! quality          14         13     0.5371     0.4181   +0.1191
//! antonym          33         31     0.4697     0.3647   +0.1050
//! comparative      27          1     0.4020     0.3112   +0.0908
//! hypernym         34         40     0.3618     0.2724   +0.0894
//! number           32         53     0.3838     0.3837   +0.0001
//! agent            26         58     0.3182     0.3172   +0.0010
//! past_tense       35         59     0.4481     0.4542   -0.0061
//! negation         16         33     0.3752     0.5344   -0.1592
//!
//! similarity                   pair>random
//! mean of 64 (as shipped)            61.3%
//! best single dimension              43.9%
//! mean of best 8                     57.1%
//! ```
//!
//! **The architecture is not present in this manifold, and the practical test
//! fails outright.** Scoring a pair by its best dimension gives 43.9% — *below*
//! the 50% chance line, and far below the 61.3% of averaging all 64. Taking the
//! best eight is also worse than taking all of them. If relations lived in single
//! dimensions this ordering would be reversed, and it is not close.
//!
//! The per-family column looks more encouraging and should not be read that way.
//! Six of ten families have a best channel exceeding their null by 0.09 to 0.12,
//! and the ten families peak on nine distinct channels rather than crowding onto
//! one — which is the shape the architecture predicts. But **negation's null beat
//! its signal by 0.1592**, which is larger than any family's excess. That is the
//! noise scale, measured on the same page: with 64 channels and 14 to 35 pairs,
//! a maximum over 64 wanders by more than the effects being reported. The peaks
//! are not established, and a version of this table that omitted the two negative
//! rows would have looked like a discovery.
//!
//! Why the maximum fails at all is worth stating: the best of 64 channels is near
//! 1.0 for *every* pair, related or not, so it saturates and discriminates
//! nothing. Averaging is what makes agreement informative here.
//!
//! # What the architecture would require
//!
//! Not a different similarity function — a different objective. Phiano's training
//! is **symmetric in the channels**: seeds are hashes, the Kuramoto step treats
//! every channel alike, and `channel_kernel` varies only decay and frequency.
//! Under a symmetric objective no channel can become "the gender channel",
//! because nothing distinguishes it from the other 63 and nothing pushes toward
//! the distinction. The dimensions cannot specialise by accident.
//!
//! Making them relation-specific means breaking that symmetry on purpose: bind
//! relation `r` to channel `r` during training, so that observing a gender pair
//! moves channel `r_gender` and leaves the rest alone. That is a real design and
//! this measurement is not evidence against it — it is evidence that it has never
//! been built. The fixed harmonic basis in `relation_type` is the nearest thing
//! present, and it works precisely because it *does* impose structure across
//! channels rather than leaving them interchangeable.

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::{RelationBenchmark, RelationFamily};
use phiano::phasor::{fnv1a, mix64};
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;

const COUNT_FLOOR: u32 = 25;
const DRAWS: usize = 64;

/// Per-channel agreement of a family's pairs: `cos(theta_k(b) - theta_k(a))`,
/// averaged over the pairs that survive the floor.
fn channel_profile(facet: &Facet, pairs: &[(String, String)]) -> (Vec<f64>, usize) {
    let mut acc = vec![0.0f64; PHASE_CHANNELS];
    let mut n = 0usize;
    for (a, b) in pairs {
        let (pa, pb) = match (facet.lexicon.get(a), facet.lexicon.get(b)) {
            (Some(x), Some(y)) => (x, y),
            _ => continue,
        };
        for (k, s) in acc.iter_mut().enumerate() {
            *s += (pb.theta(k) - pa.theta(k)).cos();
        }
        n += 1;
    }
    if n > 0 {
        for s in acc.iter_mut() {
            *s /= n as f64;
        }
    }
    (acc, n)
}

/// Same pairs, fillers permuted. Destroys the relation, keeps the vocabulary and
/// the pair count — so the null pays the same maximum-over-64 bias the signal
/// does.
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

fn argmax(v: &[f64]) -> (usize, f64) {
    v.iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, x)| (i, *x))
        .unwrap_or((0, 0.0))
}

/// Pair-versus-random under a similarity, chance 50%.
fn pair_vs_random<F: Fn(&Facet, &str, &str) -> f64>(
    facet: &Facet,
    pool: &[&String],
    families: &[RelationFamily],
    sim: F,
) -> f64 {
    let (mut wins, mut n) = (0.0f64, 0usize);
    for fam in families {
        for (i, p) in fam.pairs.iter().enumerate() {
            if !facet.lexicon.contains_key(&p.a) || !facet.lexicon.contains_key(&p.b) {
                continue;
            }
            let target = sim(facet, &p.a, &p.b);
            let (mut hits, mut draws) = (0usize, 0usize);
            for d in 0..DRAWS {
                let r = (fnv1a(&p.a) ^ ((i as u64) << 32) ^ (d as u64).wrapping_mul(0x9E37_79B9))
                    as usize;
                let cand = pool[r % pool.len().max(1)];
                if cand == &p.a || cand == &p.b {
                    continue;
                }
                draws += 1;
                if target > sim(facet, &p.a, cand) {
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

fn mean_sim(facet: &Facet, a: &str, b: &str) -> f64 {
    match (facet.lexicon.get(a), facet.lexicon.get(b)) {
        (Some(x), Some(y)) => x.resonance(y),
        _ => 0.0,
    }
}

/// The best single dimension, rather than the average of all of them.
fn max_sim(facet: &Facet, a: &str, b: &str) -> f64 {
    match (facet.lexicon.get(a), facet.lexicon.get(b)) {
        (Some(x), Some(y)) => (0..PHASE_CHANNELS)
            .map(|k| (y.theta(k) - x.theta(k)).cos())
            .fold(f64::NEG_INFINITY, f64::max),
        _ => 0.0,
    }
}

/// The mean of the best eight dimensions — between the two, in case a relation
/// occupies a few channels rather than exactly one.
fn top8_sim(facet: &Facet, a: &str, b: &str) -> f64 {
    match (facet.lexicon.get(a), facet.lexicon.get(b)) {
        (Some(x), Some(y)) => {
            let mut v: Vec<f64> = (0..PHASE_CHANNELS)
                .map(|k| (y.theta(k) - x.theta(k)).cos())
                .collect();
            v.sort_by(|p, q| q.partial_cmp(p).unwrap_or(std::cmp::Ordering::Equal));
            v[..8].iter().sum::<f64>() / 8.0
        }
        _ => 0.0,
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
    let base = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let train_toks: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let mut facet = base.clone();
    Descent::cycle(&mut facet, &train_toks, Up::Bag, 4, 0.25, true);

    let families = RelationBenchmark::default_families();
    println!("facet {} words\n", facet.vocabulary_size());

    // ---- 1 and 2: does a family have a channel, and is it its own? ----
    println!(
        "  {:<14} {:>6} {:>10} {:>10} {:>10} {:>9}",
        "family", "pairs", "best chan", "agreement", "null best", "excess"
    );

    let mut peaks: Vec<(String, usize, f64)> = Vec::new();
    for fam in &families {
        let pairs: Vec<(String, String)> =
            fam.pairs.iter().map(|p| (p.a.clone(), p.b.clone())).collect();
        let (prof, n) = channel_profile(&facet, &pairs);
        if n < 5 {
            continue;
        }
        let (k, v) = argmax(&prof);

        // Two independent permutations, so the null's own spread is visible in
        // the excess rather than hidden inside it.
        let (n1, _) = channel_profile(&facet, &permute(&pairs, 0x9E37_79B9_7F4A_7C15));
        let (n2, _) = channel_profile(&facet, &permute(&pairs, 0xC2B2_AE3D_27D4_EB4F));
        let null_best = (argmax(&n1).1 + argmax(&n2).1) / 2.0;

        println!(
            "  {:<14} {:>6} {:>10} {:>10.4} {:>10.4} {:>+9.4}",
            fam.name,
            n,
            k,
            v,
            null_best,
            v - null_best
        );
        peaks.push((fam.name.clone(), k, v - null_best));
    }

    let distinct: std::collections::HashSet<usize> = peaks.iter().map(|p| p.1).collect();
    println!(
        "\n  {} families peak on {} distinct channels. Sharing one channel would\n\
         \x20 mean it is not a relation but simply the dimension with the most\n\
         \x20 variance; needing many would be the architecture working.",
        peaks.len(),
        distinct.len()
    );

    // ---- 3: does the best dimension beat the average of all of them? ----
    let mut pool: Vec<&String> = facet
        .lexicon
        .iter()
        .filter(|(_, p)| p.count >= COUNT_FLOOR)
        .map(|(w, _)| w)
        .collect();
    pool.sort();

    println!("\n  {:<28} {:>14}", "similarity", "pair>random");
    for (label, f) in [
        ("mean of 64 (as shipped)", &mean_sim as &dyn Fn(&Facet, &str, &str) -> f64),
        ("best single dimension", &max_sim),
        ("mean of best 8", &top8_sim),
    ] {
        let s = pair_vs_random(&facet, &pool, &families, f);
        println!("  {:<28} {:>13.1}%", label, s * 100.0);
    }

    println!(
        "\n  Chance is 50.0%. If relations lived in single dimensions, the best\n\
         \x20 dimension would beat the average over all 64 — that is the whole\n\
         \x20 practical content of the claim, and the cheapest thing to act on if it\n\
         \x20 holds. A flat result does not refute the architecture; it says nothing\n\
         \x20 in the training objective ever asked a channel to specialise, so there\n\
         \x20 is no reason one would have."
    );
}
