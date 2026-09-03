//! Do the levels of language separate by rate?
//! `cargo run --release --bin bands [corpus] [seed]`
//!
//! Bentov, *Stalking the Wild Pendulum*, ch. 3, "Hierarchies of motion": each
//! level up the ladder of structures moves much more slowly than the one below.
//! Electrons orbit at enormous rates; the molecules built from them vibrate more
//! slowly; the cells built from those respond more slowly still — "a process of
//! gradually stepping down the high vibratory rates of the atoms, to the
//! 'reasonable' vibratory rates of the molecules, to the 'acceptable' frequency
//! response of the cells." Nothing in that hierarchy is a sum of the level below.
//! Each level has its own rate, and that rate is what makes it a level.
//!
//! Phiano already has the ingredient and has never checked it. `channel_kernel`
//! gives channel `k` a decay of `CONTEXT_LAMBDA^(1 + 3k/64)` and a frequency of
//! `2*pi*p_k/PRIME_MODULUS` with `p_k` the k-th prime. Low channels are slow with
//! long memory; high channels are fast with short memory. That is a spread of
//! rates designed in and then used as though all 64 channels were
//! interchangeable — every score in this project averages over all of them.
//!
//! # The question, and what would answer it
//!
//! Score two tasks band by band:
//!
//! * **word level** — do related words resonate more than unrelated ones, using
//!   only this band's channels? (`pair_vs_random`, chance 50%.)
//! * **sentence level** — does the accumulated state of three sentences pick the
//!   true continuation out of 50 candidates, using only this band? (chance
//!   1/50.)
//!
//! If the levels are frequency-separated, the two curves peak in *different*
//! bands — sentences low and slow, words high and fast. If they peak together,
//! the channel index carries no hierarchy and the per-channel kernel is
//! decoration.
//!
//! Bentov's ch. 5 sharpens the prediction into something falsifiable: a system
//! "interacts strongly within a band of frequencies," with the energy-exchange
//! curve peaked in its own band and tailing into the neighbours. So the
//! prediction is *overlapping but distinctly peaked* profiles, not a clean
//! partition — and a flat profile refutes it.
//!
//! # What it measured
//!
//! ```text
//! after descent   band  channels  kernel |z|  word pair>rnd  sentence MRR
//!                    0       0-7      0.9075          59.8%        0.0903
//!                    1      8-15      0.8796          63.8%        0.0881
//!                    2     16-23      0.8525          62.1%        0.0892
//!                    3     24-31      0.8263          64.4%        0.0874
//!                    4     32-39      0.8008          63.3%        0.0850
//!                    5     40-47      0.7762          63.0%        0.0861
//!                    6     48-55      0.7523          62.4%        0.0887
//!                    7     56-63      0.7291          63.3%        0.0808
//!                  all      0-63           -          68.6%        0.0946
//!             chance                                   50.0%        0.0900
//! ```
//!
//! **The hierarchy-of-rates prediction is refuted.** No band peaks beyond the
//! spread of the others — the best word band is +1.16 sd, the best sentence band
//! +1.12 sd, and on the bottom-up manifold +1.60 and +2.05 on profiles that are
//! entirely at chance. The channel index carries no linguistic hierarchy. The
//! kernel's spread of decay rates from 0.91 down to 0.73 is, for these two tasks,
//! decoration. That the prime spacing cut perplexity 11% is a separate fact and
//! is not level separation.
//!
//! **The hologram property holds instead, and it is strong.** One band of eight
//! channels keeps 69% of the word-level effect that all sixty-four carry — 12.8
//! of 18.6 points over chance. The relational signal is distributed across
//! channels rather than localised in any of them, so losing seven-eighths of the
//! representation costs under a third of the effect. That is Bentov ch. 1 on the
//! hologram — information stored as an interference pattern is recoverable from a
//! piece of the film at lower fidelity — and it is the robustness claim this
//! architecture can actually support, as opposed to the one it was designed for.
//!
//! The sentence row is the same null it has been all along: every band at or
//! below chance, 0.0946 against 0.0900 using everything.

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::RelationBenchmark;
use phiano::phasor::fnv1a;
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;

/// Channels per band. Eight bands of eight over the 64-channel torus.
const BAND: usize = 8;

/// Sentences of context before the prediction point.
const CONTEXT: usize = 3;
/// Candidates ranked per item.
const CANDIDATES: usize = 50;
/// Items scored. Enough for a stable mean, small enough to finish.
const ITEMS: usize = 600;
/// Random partners drawn per relation pair.
const DRAWS: usize = 32;

/// Words below this were barely trained and sit at their hash seed.
const COUNT_FLOOR: u32 = 25;

/// Does `a` resonate with `b` more than with a random word, on this band alone?
///
/// Chance is 0.5 by construction, on every band, so the bands are comparable to
/// each other — which a raw resonance would not be, since bands differ in how
/// much they were moved by training.
fn pair_vs_random(facet: &Facet, pool: &[&String], lo: usize, hi: usize) -> f64 {
    let families = RelationBenchmark::default_families();
    let (mut wins, mut n) = (0.0f64, 0usize);

    for fam in &families {
        for (i, p) in fam.pairs.iter().enumerate() {
            let (pa, pb) = match (facet.lexicon.get(&p.a), facet.lexicon.get(&p.b)) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            };
            let target = pa.resonance_band(pb, lo, hi);
            let (mut hits, mut draws) = (0usize, 0usize);
            for d in 0..DRAWS {
                let r = (fnv1a(&p.a) ^ ((i as u64) << 32) ^ (d as u64).wrapping_mul(0x9E37_79B9))
                    as usize;
                let cand = pool[r % pool.len().max(1)];
                if cand == &p.a || cand == &p.b {
                    continue;
                }
                draws += 1;
                if target > pa.resonance_band(&facet.lexicon[cand], lo, hi) {
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

/// A sentence's state on one band: the circular mean of its words' channels.
fn state_band(facet: &Facet, words: &[String], lo: usize, hi: usize) -> Vec<(f64, f64)> {
    let mut acc = vec![(0.0f64, 0.0f64); hi.saturating_sub(lo)];
    for w in words {
        if let Some(p) = facet.lexicon.get(w) {
            for (i, k) in (lo..hi).enumerate() {
                let t = p.theta(k);
                acc[i].0 += t.cos();
                acc[i].1 += t.sin();
            }
        }
    }
    acc
}

fn similarity(a: &[(f64, f64)], b: &[(f64, f64)]) -> f64 {
    let (mut s, mut n) = (0.0f64, 0usize);
    for (x, y) in a.iter().zip(b.iter()) {
        let (nx, ny) = (x.0.hypot(x.1), y.0.hypot(y.1));
        if nx > 1e-12 && ny > 1e-12 {
            s += (x.0 * y.0 + x.1 * y.1) / (nx * ny);
            n += 1;
        }
    }
    match n {
        0 => 0.0,
        _ => s / n as f64,
    }
}

struct Item {
    context: Vec<Vec<String>>,
    candidates: Vec<Vec<String>>,
    truth: usize,
}

/// The identical item set is used for every band. A comparison across bands is
/// only a comparison if the task does not move.
fn build_items(held: &[String], seed: u64) -> Vec<Item> {
    let toks: Vec<Vec<String>> = held.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let mut items = Vec::new();
    let mut i = CONTEXT;
    while i < toks.len() && items.len() < ITEMS {
        let truth_toks = toks[i].clone();
        if truth_toks.len() >= 4 {
            let context: Vec<Vec<String>> = toks[i - CONTEXT..i].to_vec();
            let mut chosen: Vec<Vec<String>> = Vec::with_capacity(CANDIDATES);
            let mut r = fnv1a(&i.to_string()) ^ seed;
            let mut seen = std::collections::HashSet::new();
            while chosen.len() + 1 < CANDIDATES {
                r ^= r << 13;
                r ^= r >> 7;
                r ^= r << 17;
                let idx = (r % toks.len() as u64) as usize;
                // Distractors from elsewhere in the corpus, never from this
                // item's own neighbourhood, which would make them plausible
                // continuations rather than distractors.
                if idx.abs_diff(i) < 16 || !seen.insert(idx) || toks[idx].len() < 4 {
                    continue;
                }
                chosen.push(toks[idx].clone());
            }
            let truth = (r % CANDIDATES as u64) as usize;
            chosen.insert(truth.min(chosen.len()), truth_toks);
            items.push(Item { context, candidates: chosen, truth });
        }
        // Stride, so the items are spread over the whole held-out half rather
        // than crowded into its opening.
        i += 7;
    }
    items
}

fn sentence_mrr(facet: &Facet, items: &[Item], lo: usize, hi: usize) -> f64 {
    let mut total = 0.0;
    for it in items {
        let mut ctx = vec![(0.0f64, 0.0f64); hi.saturating_sub(lo)];
        for sent in &it.context {
            let s = state_band(facet, sent, lo, hi);
            for (a, b) in ctx.iter_mut().zip(s.iter()) {
                a.0 += b.0;
                a.1 += b.1;
            }
        }
        let score =
            |c: &Vec<String>| similarity(&ctx, &state_band(facet, c, lo, hi));
        let truth_score = score(&it.candidates[it.truth]);
        let beating = it
            .candidates
            .iter()
            .enumerate()
            .filter(|(i, c)| *i != it.truth && score(c) > truth_score)
            .count();
        total += 1.0 / (beating + 1) as f64;
    }
    total / items.len().max(1) as f64
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/dictionary_corpus.txt".to_string());
    let seed: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(42);

    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();
    if corpus.is_empty() {
        eprintln!("empty corpus at {}", path);
        std::process::exit(1);
    }

    // Sentence order preserved in the held-out half: a next-sentence task needs
    // contiguity, and a shuffled corpus has none.
    let cut = corpus.len() * 80 / 100;
    let (train, held) = corpus.split_at(cut);

    let split = Harness::split(train.to_vec(), seed);
    let trainer = Trainer::new(LEARNING_RATE).with_seed(seed);
    let base = Harness::train_ranking_only(&split, &trainer, 4);

    let train_toks: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let mut descended = base.clone();
    let dr = Descent::cycle(&mut descended, &train_toks, Up::Bag, 4, 0.25, true);

    let items = build_items(held, seed);
    println!(
        "vocabulary {}, {} items of {} candidates, descent moved {} words{}",
        base.vocabulary_size(),
        items.len(),
        CANDIDATES,
        dr.words_moved,
        match dr.rejected {
            true => " (REJECTED — arm equals the control)",
            false => "",
        }
    );

    for (label, facet) in [("bottom-up", &base), ("after descent", &descended)] {
        // The distractor pool is the trained band, sorted: a HashMap's order is
        // not stable across runs and this vector is indexed by a hash.
        let mut pool: Vec<&String> = facet
            .lexicon
            .iter()
            .filter(|(_, p)| p.count >= COUNT_FLOOR)
            .map(|(w, _)| w)
            .collect();
        pool.sort();

        println!("\n=== {} ===", label);
        println!(
            "  {:>7} {:>10} {:>12} {:>14} {:>12}",
            "band", "channels", "kernel |z|", "word pair>rnd", "sentence MRR"
        );

        let mut word: Vec<f64> = Vec::new();
        let mut sent: Vec<f64> = Vec::new();
        for b in 0..(PHASE_CHANNELS / BAND) {
            let (lo, hi) = (b * BAND, (b + 1) * BAND);
            let w = pair_vs_random(facet, &pool, lo, hi);
            let s = sentence_mrr(facet, &items, lo, hi);
            // The band's own rate, as the kernel defines it: |z| is retention
            // per step, so a high value is a slow channel with long memory.
            let decay: f64 = (lo..hi)
                .map(|k| phiano::config::channel_kernel(k, PHASE_CHANNELS).norm())
                .sum::<f64>()
                / BAND as f64;
            println!(
                "  {:>7} {:>10} {:>12.4} {:>13.1}% {:>12.4}",
                b,
                format!("{lo}-{}", hi - 1),
                decay,
                w * 100.0,
                s
            );
            word.push(w);
            sent.push(s);
        }

        // A peak has to clear the spread of the other bands, or it is the
        // argmax of noise. Two flat profiles always put their maxima in
        // different bands, and reporting that as "the levels separate" is the
        // same false positive this project has now made three times.
        let peak = |v: &[f64]| -> (usize, f64) {
            let i = v
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
                .unwrap_or(0);
            let mean = v.iter().sum::<f64>() / v.len() as f64;
            let sd = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
                / (v.len() as f64 - 1.0).max(1.0))
            .sqrt();
            let z = match sd > 1e-12 {
                true => (v[i] - mean) / sd,
                false => 0.0,
            };
            (i, z)
        };

        let all_w = pair_vs_random(facet, &pool, 0, PHASE_CHANNELS);
        let all_s = sentence_mrr(facet, &items, 0, PHASE_CHANNELS);
        let chance_s: f64 =
            (1..=CANDIDATES).map(|r| 1.0 / r as f64).sum::<f64>() / CANDIDATES as f64;
        println!(
            "  {:>7} {:>10} {:>12} {:>13.1}% {:>12.4}",
            "all", "0-63", "-", all_w * 100.0, all_s
        );
        println!("  chance: word 50.0%, sentence {:.4}", chance_s);

        let (wi, wz) = peak(&word);
        let (si, sz) = peak(&sent);
        println!(
            "  word peak band {} at {:+.2} sd; sentence peak band {} at {:+.2} sd",
            wi, wz, si, sz
        );
        const SEPARATION: f64 = 2.0;
        println!(
            "  -> {}",
            match (wz >= SEPARATION, sz >= SEPARATION, wi != si) {
                (true, true, true) =>
                    "THE LEVELS SEPARATE BY RATE: both profiles peak, in different bands",
                (true, true, false) => "both peak, in the SAME band: no separation",
                _ =>
                    "FLAT: no band peaks beyond the spread of the others, so the \
                     channel index carries no hierarchy for these tasks",
            }
        );

        // What a fraction of the channels retains — the hologram property.
        // Bentov, ch. 1: information stored as an interference pattern is
        // recoverable from a piece of the film, at lower fidelity. If one band
        // of eight keeps most of the effect that all sixty-four carry, the
        // signal is distributed rather than localised, and losing channels
        // degrades gracefully instead of catastrophically.
        let band_mean = word.iter().sum::<f64>() / word.len() as f64;
        // Retention is a ratio to the full-channel effect, so it means nothing
        // when there is no full-channel effect to retain — dividing by a number
        // at chance produces a percentage in the hundreds of millions, which is
        // how this first printed.
        const MIN_EFFECT: f64 = 0.02;
        println!(
            "  holographic retention (word): {}",
            match all_w - 0.5 > MIN_EFFECT {
                true => format!(
                    "1/8 of the channels keeps {:.0}% of the effect over chance \
                     ({:.1} of {:.1} points)",
                    (band_mean - 0.5) / (all_w - 0.5) * 100.0,
                    (band_mean - 0.5) * 100.0,
                    (all_w - 0.5) * 100.0
                ),
                false => format!(
                    "undefined — all 64 channels are at chance ({:.1}%), so there \
                     is no effect for a subset to retain",
                    all_w * 100.0
                ),
            }
        );
    }

    println!(
        "\n  Low bands are slow with long memory, high bands fast with short\n\
         \x20 memory — that is what the kernel's |z| column shows. The prediction\n\
         \x20 under test is that sentences live low and words high, with overlapping\n\
         \x20 but distinctly peaked profiles. A flat profile, or two peaks in the\n\
         \x20 same band, says the channel index carries no hierarchy and the\n\
         \x20 per-channel kernel is decoration."
    );
}
