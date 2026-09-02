//! How many relation types does the language actually hold?
//! `cargo run --release --bin types [chunks] [corpus]`
//!
//! The claim: a language holds a bounded number of relation types — at most 64 —
//! so they fit in a fixed-width type and the whole system can be built on
//! discovered relations rather than hardcoded ones.
//!
//! A cap does not test itself. `.take(64)` yields at most 64 of anything, and
//! `MAX_WORD_SENSES = 64` in this codebase is exactly that: a truncation wearing
//! a claim's clothes. What would make the bound real is *saturation* — held-out
//! structure ceasing to improve before the cap, so a 65th type would have
//! nothing left to explain.
//!
//! So: fit relation types on half the extracted pairs, score the other half, and
//! sweep the type count past the cap. Two columns decide it.
//!
//! * **held-out** — agreement between a held-out pair's phase offset and its
//!   nearest fitted type. In-sample tightness rises with `k` mechanically and
//!   would confirm any bound put to it, so it is not what is reported.
//! * **shuffled** — the same, on pairs whose fillers have been permuted. This
//!   preserves the vocabulary, the offset marginals and the type count, and
//!   destroys only which head goes with which filler. A held-out curve its null
//!   tracks is measuring k-means, not language.
//!
//! The bound is supported if the gain flattens below 64 and the flattening is
//! not the null flattening too. It is refuted if the gain is still climbing at
//! 128 — and that is an outcome this is set up to be able to report.
//!
//! # Two manifolds
//!
//! The sweep runs on both the bottom-up manifold and the one the downward pass
//! in [`phiano::topdown`] produced, because those two disagree about whether
//! this project has relational structure at all. The descent tripled analogy
//! MRR at the same frequency floor — 0.00247 to 0.00814, four times its own
//! null — while the first run of this sweep found no relation types in the
//! bottom-up manifold whatsoever.
//!
//! Both cannot be the whole story, and the disagreement is informative either
//! way. If the descended manifold's types clear their null, then the analogy
//! gain *is* shared offsets and the descent built the structure the bounded-type
//! claim needs. If it does not, the analogy gain is something other than a
//! shared step between word pairs, and the bounded-type claim has no
//! representation to stand on in either manifold.

use phiano::chunker::ChunkStore;
use phiano::config::LEARNING_RATE;
use phiano::metrics::harness::Harness;
use phiano::relation_type::{OffsetEstimator, RelationTypes};
use phiano::sources::definition_core;
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;
use std::collections::HashMap;

/// Pairs are drawn only from words the model has actually seen this often.
///
/// A word seen four times sits at its hash seed, so its offset to anything is
/// noise, and a cluster of noise is still a cluster. The floor sweep in
/// `bin/roles` showed what happens when the untrained tail is left in.
const COUNT_FLOOR: u32 = 25;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let chunks = args.get(1).cloned().unwrap_or_else(|| "data/chunks".to_string());
    let corpus_path = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "data/dictionary_corpus.txt".to_string());

    let raw = std::fs::read_to_string(&corpus_path).unwrap_or_default();
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();
    if corpus.is_empty() {
        eprintln!("empty corpus at {}", corpus_path);
        std::process::exit(1);
    }

    let split = Harness::split(corpus, 42);
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);

    // Pairs straight out of the dictionary: a head and each content word of its
    // definitional core. Nothing here says which relation any pair stands in —
    // that is the whole point. No enum, no preposition list, no labels.
    let glosses: Vec<(String, String)> = ChunkStore::new(&chunks)
        .load_all()
        .into_iter()
        .map(|(w, d)| (w, definition_core(&d)))
        .collect();

    let above = |w: &str| facet.lexicon.get(w).is_some_and(|p| p.count >= COUNT_FLOOR);

    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut per_head: HashMap<String, usize> = HashMap::new();
    for (head, gloss) in &glosses {
        if !above(head) {
            continue;
        }
        for t in Tokenizer::tokenize(gloss) {
            if t == *head || !above(&t) {
                continue;
            }
            // Cap the fan-out per head so a long entry cannot dominate the
            // clustering by sheer weight of pairs.
            let n = per_head.entry(head.clone()).or_insert(0);
            if *n >= 8 {
                break;
            }
            *n += 1;
            pairs.push((head.clone(), t));
        }
    }
    pairs.sort();
    pairs.dedup();

    println!(
        "facet {} words ({} at or above count {}), {} pairs from {} glosses",
        facet.vocabulary_size(),
        facet.lexicon.values().filter(|p| p.count >= COUNT_FLOOR).count(),
        COUNT_FLOOR,
        pairs.len(),
        glosses.len()
    );
    if pairs.len() < 200 {
        eprintln!("too few pairs to say anything");
        std::process::exit(1);
    }

    // The downward pass, on the same training sentences the facet was fitted on.
    let train_toks: Vec<Vec<String>> = split
        .train
        .iter()
        .map(|s| Tokenizer::tokenize(s))
        .collect();
    let mut descended = facet.clone();
    let dr = Descent::cycle(&mut descended, &train_toks, Up::Bag, 4, 0.25, true);
    println!(
        "descent: {} of {} words moved, dispersion {:.3} -> {:.3}, band {:.3} -> {:.3}{}",
        dr.words_moved,
        dr.words_seen,
        dr.dispersion_before,
        dr.dispersion_after,
        dr.band_before,
        dr.band_after,
        match dr.rejected {
            true => "  REJECTED BY THE GUARD — this arm is the control",
            false => "",
        }
    );

    // Past the cap deliberately. A sweep that stops at the number under test
    // cannot find the number wrong. 128 was already shown flat on the bottom-up
    // manifold and costs more than the rest of the sweep together, so the
    // comparison runs to 64 and the earlier run stands for the tail.
    let ks: Vec<usize> = vec![16, 64];

    println!(
        "\n  {:<14} {:<16} {:>4} {:>10} {:>10} {:>10} {:>9} {:>10}",
        "manifold", "offset", "k", "held-out", "shuffled", "gain", "g/noise", "head-dom"
    );

    // Optional third argument restricts which manifold runs, so a long grid can
    // be finished in pieces after an out-of-memory kill without redoing the
    // rows that already landed.
    let only = args.get(3).cloned().unwrap_or_else(|| "both".to_string());
    let arms: Vec<(&str, &phiano::facet::Facet)> = match only.as_str() {
        "base" => vec![("bottom-up", &facet)],
        "descent" => vec![("after descent", &descended)],
        // The fixed basis costs nothing to evaluate, so it can be run on its own
        // without paying for the k-means grid — which has been killed for memory
        // twice at k=64.
        "harmonic" => Vec::new(),
        _ => vec![("bottom-up", &facet), ("after descent", &descended)],
    };

    for (label, f) in arms {
        for est in OffsetEstimator::ALL {
            // Head domination, measured once per (manifold, estimator) at the
            // full type count. Centering *mechanically* lowers this, so it is
            // not a verdict on its own — it says what the clusters are grouping
            // by, and the gain column says whether that grouping is worth
            // anything. Both are needed: a rule that destroyed the offsets
            // entirely would score 9.1% here and nothing at all there.
            let types = RelationTypes::discover_with(f, &pairs, 64, 12, est);
            let (mut shared, mut total) = (0usize, 0usize);
            for t in types.all() {
                if let Some(e) = types.evidence(t) {
                    let mut heads: HashMap<&str, usize> = HashMap::new();
                    for (h, _) in &e.members {
                        *heads.entry(h.as_str()).or_insert(0) += 1;
                    }
                    shared += heads.values().filter(|c| **c > 1).sum::<usize>();
                    total += e.members.len();
                }
            }
            let head_dom = shared as f64 / total.max(1) as f64 * 100.0;

            for &k in &ks {
                let p = RelationTypes::saturation_at_with(f, &pairs, k, 10, est);
                println!(
                    "  {:<14} {:<16} {:>4} {:>10.4} {:>10.4} {:>10.4} {:>8.1}x {:>9.1}%",
                    label,
                    est.label(),
                    p.k,
                    p.held_out,
                    p.shuffled,
                    p.gain(),
                    match p.noise() > 1e-9 {
                        true => p.gain() / p.noise(),
                        false => f64::INFINITY,
                    },
                    head_dom
                );
            }
        }
    }

    // ---- the fixed basis ----
    //
    // Bentov's standing waves: a bounded medium sustains only integer harmonics,
    // so a 64-channel torus has exactly 64 modes and the bound is the channel
    // count rather than a number anyone picked. The experimental point is that
    // this basis fits NOTHING — no centroids, no k-means, no seed, identical on
    // every corpus — while the rows above fit 64 rotations of 64 channels each,
    // 4,096 free numbers, to these very pairs.
    println!("\n  --- fixed harmonic basis: 0 fitted parameters vs 4096 above ---");
    println!(
        "  {:<14} {:<16} {:>10} {:>10} {:>10} {:>9}",
        "manifold", "offset", "held-out", "shuffled", "gain", "g/noise"
    );
    for (label, f) in [("bottom-up", &facet), ("after descent", &descended)] {
        for est in OffsetEstimator::ALL {
            let p = RelationTypes::harmonic_saturation(f, &pairs, est);
            println!(
                "  {:<14} {:<16} {:>10.4} {:>10.4} {:>10.4} {:>8.1}x",
                label,
                est.label(),
                p.held_out,
                p.shuffled,
                p.gain(),
                match p.noise() > 1e-9 {
                    true => p.gain() / p.noise(),
                    false => f64::INFINITY,
                }
            );
        }
    }

    // The chance level for the head-domination column: with 64 clusters and a
    // head taking up to 8 fillers, some collisions happen for free.
    let mut fan: HashMap<&str, usize> = HashMap::new();
    for (h, _) in &pairs {
        *fan.entry(h.as_str()).or_insert(0) += 1;
    }
    let n_pairs: usize = fan.values().sum();
    let expected: f64 = fan
        .values()
        .map(|&f| {
            let f = f as f64;
            f * (1.0 - (1.0 - 1.0 / 64.0f64).powf(f - 1.0))
        })
        .sum::<f64>()
        / n_pairs.max(1) as f64;

    println!(
        "\n  head-dom chance level: {:.1}%. A gain under 3x its noise is no effect.\n\
         \x20 Centering lowers head-dom by construction, so it only counts when the\n\
         \x20 gain survives — the two columns have to move the right way together.",
        expected * 100.0
    );
}
