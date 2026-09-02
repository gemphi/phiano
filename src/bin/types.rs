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

use phiano::chunker::ChunkStore;
use phiano::config::LEARNING_RATE;
use phiano::metrics::harness::Harness;
use phiano::relation_type::{RelationTypes, MAX_RELATION_TYPES};
use phiano::sources::definition_core;
use phiano::tokenizer::Tokenizer;
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

    // Past the cap deliberately. A sweep that stops at the number under test
    // cannot find the number wrong.
    let ks: Vec<usize> = vec![1, 2, 4, 8, 16, 32, 48, 64, 96, 128];
    println!("\n=== does the type count saturate? ===");
    println!(
        "  {:>6} {:>11} {:>11} {:>10} {:>10} {:>7}",
        "k", "held-out", "shuffled", "gain", "noise", "time"
    );

    // `saturation` clusters uncapped on purpose: if it clamped to the cap, the
    // rows at 96 and 128 would silently repeat 64, the curve would look flat
    // past the cap, and the sweep would confirm the bound by construction.
    let mut curve: Vec<phiano::relation_type::SaturationPoint> = Vec::new();
    for &k in &ks {
        let started = std::time::Instant::now();
        let p = RelationTypes::saturation_at(&facet, &pairs, k, 10);
        let capped = match p.k > MAX_RELATION_TYPES {
            true => " past cap",
            false => "",
        };
        println!(
            "  {:>6} {:>11.4} {:>11.4} {:>10.4} {:>10.4} {:>6.0}s{}",
            p.k,
            p.held_out,
            p.shuffled,
            p.gain(),
            p.noise(),
            started.elapsed().as_secs_f64(),
            capped
        );
        curve.push(p);
    }

    // ---- the verdict, stated so it can be wrong ----
    let best = curve
        .iter()
        .max_by(|a, b| a.gain().partial_cmp(&b.gain()).unwrap_or(std::cmp::Ordering::Equal))
        .copied()
        .expect("curve non-empty");
    // The noise scale: how far two independent runs of the *null* land from
    // each other. A gain smaller than this is not a small effect, it is no
    // effect, and reporting it as a peak is how a sweep confirms whatever it was
    // asked about.
    let noise = curve.iter().map(|p| p.noise()).fold(0.0f64, f64::max);

    println!(
        "\n  best gain {:.4} at k={}; the null's own run-to-run spread is {:.4}",
        best.gain(),
        best.k,
        noise
    );

    if best.gain() < 3.0 * noise {
        println!(
            "\n  VERDICT: NO TYPE STRUCTURE AT ANY k. Held-out agreement climbs\n             \x20 steadily with the type count — and the shuffled null climbs with it,\n             \x20 point for point. Every one of those gains is smaller than the\n             \x20 distance between two runs of the null. Adding centroids improves the\n             \x20 fit to random offsets exactly as much as to real ones, which is what\n             \x20 k-means does to any cloud of points.\n\n             \x20 So the bounded-type question does not get an answer here, because its\n             \x20 premise fails: there are no discoverable relation types in these phase\n             \x20 offsets to be bounded. Whether the bound is 64 or 6 or 600 is moot\n             \x20 until there is structure for a type to be a type OF."
        );
    } else {
        let knee = curve
            .iter()
            .find(|p| p.gain() >= best.gain() * 0.98)
            .copied()
            .unwrap_or(best);
        println!("  flattens (within 2% of best) at k={}", knee.k);
        println!(
            "\n  VERDICT: {}",
            match knee.k <= MAX_RELATION_TYPES {
                true => format!(
                    "the bound holds — held-out structure clears its noise floor,\n                     \x20 peaks at {} types and is flat by {}, both inside the cap of {}.",
                    best.k, knee.k, MAX_RELATION_TYPES
                ),
                false => format!(
                    "the bound is WRONG — held-out structure is still improving at\n                     \x20 k={}, past the cap of {}.",
                    best.k, MAX_RELATION_TYPES
                ),
            }
        );
    }

    // ---- what the clusters are actually grouping by ----
    //
    // The first run's clusters looked like `adversary->against`,
    // `adversary->another`, `adversary->is`, `adversary->one` — four fillers of
    // one head, in one cluster. If clusters group by head rather than by
    // relation then the "types" are word identity in disguise, which would
    // explain a null that tracks the signal.
    {
        let types = RelationTypes::discover(&facet, &pairs, 64, 12);
        let mut shared = 0usize;
        let mut total = 0usize;
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
        // Under assignment that ignores the head, a pair shares its cluster with
        // another pair of the same head only as often as the head's own fan-out
        // and the cluster count allow.
        let mut fan: HashMap<&str, usize> = HashMap::new();
        for (h, _) in &pairs {
            *fan.entry(h.as_str()).or_insert(0) += 1;
        }
        let expected: f64 = fan
            .values()
            .map(|&f| {
                let f = f as f64;
                // P(at least one of the other f-1 pairs lands in the same one of
                // 64 clusters), times the f pairs.
                f * (1.0 - (1.0 - 1.0 / 64.0f64).powf(f - 1.0))
            })
            .sum::<f64>()
            / total.max(1) as f64;
        println!(
            "\n=== are the clusters grouping by relation, or by head? ===\n             \x20 {:.1}% of pairs share their cluster with another pair of the same head\n             \x20 ({:.1}% expected if clusters ignored the head entirely).",
            shared as f64 / total.max(1) as f64 * 100.0,
            expected * 100.0
        );
    }

    // ---- what the discovered types look like ----
    //
    // Only inspection, and only after the number is settled. Naming a cluster
    // after reading its members is how a discovered type gets an English word
    // attached to it, and doing it before the sweep would be choosing k by taste.
    let k = MAX_RELATION_TYPES;
    let types = RelationTypes::discover(&facet, &pairs, k, 20);
    println!("\n=== the {} types, by size ===", types.len());
    let mut ev: Vec<_> = types.all().into_iter().filter_map(|t| types.evidence(t)).collect();
    ev.sort_by_key(|e| std::cmp::Reverse(e.members.len()));
    for e in ev.iter().take(12) {
        let sample: Vec<String> = e
            .members
            .iter()
            .take(4)
            .map(|(h, f)| format!("{h}->{f}"))
            .collect();
        println!(
            "  {:<5} {:>6} pairs  coh {:>6.3}  {}",
            e.relation.label(),
            e.members.len(),
            e.coherence,
            sample.join(", ")
        );
    }
    println!(
        "\n  The labels are t00, t01 and so on because nothing named these. If a\n\
         \x20 cluster turns out to be the genus relation, that is a fact discovered\n\
         \x20 by reading its members, not a category the code was given in advance."
    );
}
