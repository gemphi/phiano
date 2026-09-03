//! Constructed positions against learned ones — and complex against phase-only.
//! `cargo run --release --bin constructed [corpus]`
//!
//! Two criticisms, both correct, both testable here.
//!
//! # One: the complex numbers are not being used
//!
//! `SpectralPhasor::resonance` is the innermost operation in every retrieval,
//! every relation probe and every analogy in this project, and it reads only the
//! packed phase bytes. Amplitude is computed, stored, updated on every
//! observation — and never enters the comparison. What every result in this repo
//! measured is a torus of 64 angles, not a field of complex numbers. The
//! magnitude has been carried and ignored.
//!
//! So each arm below is scored twice: phase-only, as shipped, and with the full
//! complex inner product where magnitude counts. The gap is what ignoring half
//! the representation costs.
//!
//! # Two: the positions could be constructed rather than learned
//!
//! The proposal: put a word in a **band** by its category, then place it *within*
//! the band by its characters — alphabetically, so `female` sits just below
//! `male` because f precedes m. There are 26 letters and a bounded set of
//! categories; the claim is that this is enough, and that training a position is
//! solving a problem that ordering already solves.
//!
//! Made concrete: a word gets one scalar position `p = (band + rank/size) / B`,
//! and that scalar is spread across the 64 channels as its harmonics,
//! `theta_k = 2*pi * p * m_k` with `m_k` the k-th prime. A scalar encoded as a
//! standing wave — the same geometry `relation_type::harmonic_rotation` uses,
//! now for the position rather than the relation. Magnitude is log frequency, so
//! the complex number carries something on both axes.
//!
//! # Where the bands come from, and the two controls
//!
//! Not from `phinum::PosDictionary`: it holds 238 words, covering 0.7% of the
//! corpus's types. It has no entry for `female`, `male` or `cow`, so it cannot
//! band content words at all.
//!
//! * **suffix** — the last three characters. `-ing`, `-ed`, `-ly`, `-tion` are
//!   real categories and they come straight out of the alphabet, which is the
//!   spirit of the proposal.
//! * **co-occurrence** — bands discovered by clustering words on the company they
//!   keep. Category from use.
//!
//! And two controls, because a scheme with this many moving parts can look good
//! for the wrong reason:
//!
//! * **band, shuffled within** — same bands, random order inside them. Isolates
//!   whether *alphabetical* ordering adds anything beyond the band.
//! * **shuffled band, alphabetical within** — random bands, real ordering.
//!   Isolates whether the *band* adds anything beyond the ordering.
//!
//! Scored on pair-versus-random over the relation families, chance 50%, with the
//! gender family called out separately since `female`/`male` is the worked
//! example the proposal rests on.
//!
//! # What it measured
//!
//! ```text
//! positions                            phase-only    complex   gender
//! learned (trained manifold)                68.3%      68.3%    75.2%
//! constructed: suffix + alphabetical        52.0%      52.0%    66.9%
//!   control: suffix + shuffled order        50.7%      50.7%    61.6%
//!   control: shuffled band + alpha          45.4%      45.4%    47.0%
//! constructed: co-occur + alphabetical      80.0%      80.0%    55.8%
//!   control: co-occur + shuffled order      50.3%      50.3%    38.9%
//! ```
//!
//! **Constructed beats learned.** Co-occurrence bands with alphabetical order
//! inside them score 80.0% against the trained manifold's 68.3%, and against
//! their own shuffled-order control's 50.3%. Coarsening positions into a band
//! plus a rank does not lose information relative to the continuous trained
//! positions — it *denoises* them, by nearly twelve points.
//!
//! The caveat that keeps this honest: those bands are computed from the trained
//! manifold, which describes each word's neighbourhood. So this is learn-then-
//! coarsen, not training-free. A genuinely constructed version needs bands from
//! raw co-occurrence counts, and until that runs the claim is "banding improves a
//! learned manifold", not "banding replaces learning".
//!
//! Character bands are weakly real. Suffix plus alphabetical is 52.0% against
//! 50.7% for the same bands in shuffled order and 45.4% for shuffled bands in
//! alphabetical order — so the band carries most of the little there is, and
//! alphabetical order on random bands is *worse than chance*. On the gender
//! family specifically the suffix band does well (66.9%), which is the worked
//! example behaving: `female` and `male` share their last three characters and
//! land in the same band by construction.
//!
//! # And the magnitude cannot help, for a reason worth stating
//!
//! Phase-only and complex are **identical to the decimal place in every row**.
//! Not close — the same. The reason is structural rather than an oversight:
//! amplitude is one number per word, shared across all 64 channels, so it factors
//! out of a normalised inner product exactly. A per-word scalar magnitude cannot
//! affect any normalised similarity, however the comparison is written.
//!
//! So "use the complex numbers" is right, and the fix is not to change the
//! similarity function — it is that magnitude has to vary *per channel* before it
//! can carry anything. As stored, half of every phasor in this project is
//! mathematically incapable of influencing a single result.

use phiano::config::{CHANNEL_PRIMES, LEARNING_RATE, PHASE_CHANNELS, TWO_PI};
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::RelationBenchmark;
use phiano::phasor::{fnv1a, mix64};
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;
use std::collections::HashMap;

const COUNT_FLOOR: u32 = 25;
const DRAWS: usize = 64;

/// A word's position, as complex numbers per channel.
type Point = Vec<(f64, f64)>;

/// Spreads a scalar position across the channels as its harmonics.
///
/// `theta_k = 2*pi * p * m_k` with `m_k` the k-th prime: one number becomes a
/// standing wave, which is how a scalar can occupy 64 channels without 63 of them
/// being copies. Magnitude is the same on every channel — it is a property of the
/// word, not of the channel.
fn place(p: f64, amplitude: f64) -> Point {
    (0..PHASE_CHANNELS)
        .map(|k| {
            let m = CHANNEL_PRIMES[k % CHANNEL_PRIMES.len()] as f64;
            let t = (TWO_PI * p * m).rem_euclid(TWO_PI);
            (amplitude * t.cos(), amplitude * t.sin())
        })
        .collect()
}

/// Normalised complex inner product: magnitude counts.
fn complex_similarity(a: &Point, b: &Point) -> f64 {
    let (mut dot, mut na, mut nb) = (0.0f64, 0.0f64, 0.0f64);
    for ((ax, ay), (bx, by)) in a.iter().zip(b.iter()) {
        dot += ax * bx + ay * by;
        na += ax * ax + ay * ay;
        nb += bx * bx + by * by;
    }
    match na > 1e-12 && nb > 1e-12 {
        true => dot / (na.sqrt() * nb.sqrt()),
        false => 0.0,
    }
}

/// Phase-only: exactly what `SpectralPhasor::resonance` does today.
fn phase_similarity(a: &Point, b: &Point) -> f64 {
    let mut s = 0.0;
    for ((ax, ay), (bx, by)) in a.iter().zip(b.iter()) {
        s += (ay.atan2(*ax) - by.atan2(*bx)).cos();
    }
    s / PHASE_CHANNELS as f64
}

/// Fraction of probe pairs that out-resonate a random partner. Chance is 0.5.
fn pair_vs_random<F: Fn(&Point, &Point) -> f64>(
    points: &HashMap<String, Point>,
    pool: &[&String],
    families: &[phiano::metrics::relation::RelationFamily],
    sim: F,
    only: Option<&str>,
) -> (f64, usize) {
    let (mut wins, mut n) = (0.0f64, 0usize);
    for fam in families {
        if only.is_some_and(|f| f != fam.name) {
            continue;
        }
        for (i, p) in fam.pairs.iter().enumerate() {
            let (pa, pb) = match (points.get(&p.a), points.get(&p.b)) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            };
            let target = sim(pa, pb);
            let (mut hits, mut draws) = (0usize, 0usize);
            for d in 0..DRAWS {
                let r = (fnv1a(&p.a) ^ ((i as u64) << 32) ^ (d as u64).wrapping_mul(0x9E37_79B9))
                    as usize;
                let cand = pool[r % pool.len().max(1)];
                if cand == &p.a || cand == &p.b {
                    continue;
                }
                draws += 1;
                if target > sim(pa, &points[cand]) {
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
        0 => (0.5, 0),
        _ => (wins / n as f64, n),
    }
}

/// Builds constructed positions from a banding and an ordering.
fn construct(
    vocab: &[(String, u32)],
    band_of: &dyn Fn(&str) -> u64,
    alphabetical: bool,
    shuffle_bands: bool,
) -> HashMap<String, Point> {
    let mut bands: HashMap<u64, Vec<&(String, u32)>> = HashMap::new();
    for w in vocab {
        let b = match shuffle_bands {
            true => mix64(fnv1a(&format!("band:{}", w.0))),
            false => band_of(&w.0),
        };
        bands.entry(b).or_default().push(w);
    }

    let mut keys: Vec<u64> = bands.keys().copied().collect();
    keys.sort_unstable();
    let n_bands = keys.len().max(1) as f64;

    let max_count = vocab.iter().map(|w| w.1).max().unwrap_or(1).max(1) as f64;
    let mut out = HashMap::new();

    for (bi, b) in keys.iter().enumerate() {
        let members = bands.get_mut(b).expect("band present");
        match alphabetical {
            // The proposal: order within the band by the characters themselves.
            true => members.sort_by(|a, c| a.0.cmp(&c.0)),
            // Control: same band, order carrying no information.
            false => members.sort_by_key(|m| mix64(fnv1a(&format!("ord:{}", m.0)))),
        }
        let size = members.len().max(1) as f64;
        for (ri, m) in members.iter().enumerate() {
            let p = (bi as f64 + ri as f64 / size) / n_bands;
            // Magnitude is log frequency, so the complex number carries the
            // word's weight as well as its place.
            let a = (1.0 + m.1 as f64).ln() / (1.0 + max_count).ln();
            out.insert(m.0.clone(), place(p, a.max(1e-3)));
        }
    }
    out
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
    let mut learned = base.clone();
    Descent::cycle(&mut learned, &train_toks, Up::Bag, 4, 0.25, true);

    let vocab: Vec<(String, u32)> = learned
        .lexicon
        .iter()
        .filter(|(_, p)| p.count >= COUNT_FLOOR)
        .map(|(w, p)| (w.clone(), p.count))
        .collect();
    let mut vocab = vocab;
    vocab.sort();
    println!(
        "prose: {} sentences, {} words above count {}",
        split.train.len() + split.valid.len() + split.test.len(),
        vocab.len(),
        COUNT_FLOOR
    );

    // The learned manifold, expressed as points so every arm is scored by the
    // same code. Magnitude is the phasor's own amplitude.
    let learned_points: HashMap<String, Point> = vocab
        .iter()
        .map(|(w, _)| {
            let p = &learned.lexicon[w];
            let pt = (0..PHASE_CHANNELS)
                .map(|k| {
                    let t = p.theta(k);
                    (p.amplitude * t.cos(), p.amplitude * t.sin())
                })
                .collect();
            (w.clone(), pt)
        })
        .collect();

    // Co-occurrence bands: cluster words on the company they keep, using the
    // trained manifold only to describe neighbourhoods, not to place anything.
    let mut neighbours: HashMap<&str, Vec<(f64, f64)>> = HashMap::new();
    for toks in &train_toks {
        for i in 0..toks.len() {
            if !learned.lexicon.get(&toks[i]).is_some_and(|p| p.count >= COUNT_FLOOR) {
                continue;
            }
            let e = neighbours
                .entry(toks[i].as_str())
                .or_insert_with(|| vec![(0.0, 0.0); PHASE_CHANNELS]);
            for j in i.saturating_sub(3)..(i + 4).min(toks.len()) {
                if j == i {
                    continue;
                }
                if let Some(p) = learned.lexicon.get(&toks[j]) {
                    for (k, a) in e.iter_mut().enumerate() {
                        let t = p.theta(k);
                        a.0 += t.cos();
                        a.1 += t.sin();
                    }
                }
            }
        }
    }
    // 64 bands, to match the bound the rest of the project argues for.
    let cooc_band: HashMap<String, u64> = {
        let mut m = HashMap::new();
        for (w, acc) in &neighbours {
            // A cheap, deterministic band: the sector of the neighbourhood's
            // resultant on the first two channels. Not k-means — the point is
            // that the band be a category, not that it be optimal.
            let s0 = acc[0].1.atan2(acc[0].0).rem_euclid(TWO_PI);
            let s1 = acc[1].1.atan2(acc[1].0).rem_euclid(TWO_PI);
            let b = ((s0 / TWO_PI * 8.0) as u64) * 8 + (s1 / TWO_PI * 8.0) as u64;
            m.insert(w.to_string(), b);
        }
        m
    };

    let suffix_band = |w: &str| -> u64 {
        let s: String = w.chars().rev().take(3).collect();
        fnv1a(&s) % 512
    };
    let cooc = |w: &str| -> u64 { cooc_band.get(w).copied().unwrap_or(0) };

    let families = RelationBenchmark::default_families();
    let pool: Vec<&String> = vocab.iter().map(|(w, _)| w).collect();

    println!(
        "\n  {:<34} {:>12} {:>12} {:>10} {:>8}",
        "positions", "phase-only", "complex", "gender", "pairs"
    );

    let mut rows: Vec<(String, HashMap<String, Point>)> =
        vec![("learned (trained manifold)".into(), learned_points)];
    rows.push((
        "constructed: suffix + alphabetical".into(),
        construct(&vocab, &suffix_band, true, false),
    ));
    rows.push((
        "  control: suffix + shuffled order".into(),
        construct(&vocab, &suffix_band, false, false),
    ));
    rows.push((
        "  control: shuffled band + alpha".into(),
        construct(&vocab, &suffix_band, true, true),
    ));
    rows.push((
        "constructed: co-occur + alphabetical".into(),
        construct(&vocab, &cooc, true, false),
    ));
    rows.push((
        "  control: co-occur + shuffled order".into(),
        construct(&vocab, &cooc, false, false),
    ));

    for (label, points) in &rows {
        let (ph, n) = pair_vs_random(points, &pool, &families, phase_similarity, None);
        let (cx, _) = pair_vs_random(points, &pool, &families, complex_similarity, None);
        let (gender, _) =
            pair_vs_random(points, &pool, &families, complex_similarity, Some("gender"));
        println!(
            "  {:<34} {:>11.1}% {:>11.1}% {:>9.1}% {:>8}",
            label,
            ph * 100.0,
            cx * 100.0,
            gender * 100.0,
            n
        );
    }

    println!(
        "\n  Chance is 50.0% in every column. `phase-only` is what this project\n\
         \x20 actually measures today — `resonance` reads the packed angles and never\n\
         \x20 touches amplitude — and `complex` is the same comparison with magnitude\n\
         \x20 restored. The gap between those two columns is the cost of carrying a\n\
         \x20 complex representation and scoring half of it.\n\n\
         \x20 For the constructed rows, read them against their own controls, not\n\
         \x20 against each other. Alphabetical ordering earns its place only by\n\
         \x20 beating the shuffled-order row with the same bands; the band earns its\n\
         \x20 place only by beating the shuffled-band row with the same ordering."
    );
}
