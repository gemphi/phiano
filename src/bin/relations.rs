//! Relation-accuracy runner.
//!
//! `cargo run --release --bin relations -- [max_entries] [epochs]`
//!
//! Trains the manifold on dictionary definitions and asks whether it places
//! related words in related positions: is `woman` near `man`, and is the step
//! from `man` to `woman` the same step as from `grandfather` to `grandmother`?
//!
//! Every score is printed against its chance baseline, because a number without
//! one cannot be judged.

use phiano::chunker::ChunkStore;
use phiano::cognitive::DefinitionGrounder;
use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::metrics::relation::{RelationBenchmark, RelationReport};
use phiano::trainer::Trainer;

fn print_report(label: &str, r: &RelationReport) {
    println!("\n=== {} ===", label);
    println!("  vocabulary: {}", r.vocabulary_size);
    println!(
        "  {:<10} {:>6} {:>12} {:>10} {:>10} {:>10} {:>10} {:>8}",
        "family", "pairs", "pair>random", "near@10", "near@50", "analog@1", "analog@5", "MRR"
    );
    for f in &r.families {
        println!(
            "  {:<10} {:>6} {:>11.0}% {:>9.0}% {:>9.0}% {:>9.1}% {:>9.1}% {:>8.3}",
            f.name,
            f.usable_pairs,
            f.pair_vs_random * 100.0,
            f.neighbour_top10 * 100.0,
            f.neighbour_top50 * 100.0,
            f.analogy_top1 * 100.0,
            f.analogy_top5 * 100.0,
            f.analogy_mrr
        );
    }
    println!(
        "  {:<10} {:>6} {:>11.0}% {:>9}  {:>9}  {:>9.1}%",
        "CHANCE", "-", 50.0,
        format!("{:.2}%", r.chance_neighbour_top10 * 100.0),
        format!("{:.2}%", r.chance_neighbour_top10 * 5.0 * 100.0),
        r.chance_analogy_top1 * 100.0
    );
    println!(
        "\n  overall: pair>random {:.0}% (chance 50%), analogy@1 {:.2}% (chance {:.4}%)",
        r.overall_pair_vs_random * 100.0,
        r.overall_analogy_top1 * 100.0,
        r.chance_analogy_top1 * 100.0
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let max_entries: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(20_000);
    let epochs: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);

    let store = ChunkStore::new("data/chunks");
    let entries = store.load_all();
    if entries.is_empty() {
        eprintln!("no dictionary entries found under data/chunks");
        std::process::exit(1);
    }

    let families = RelationBenchmark::default_families();
    let probe_words: Vec<String> = families
        .iter()
        .flat_map(|f| f.pairs.iter().flat_map(|p| [p.a.clone(), p.b.clone()]))
        .collect();

    // Keep every probe word, then fill up to the budget with the rest, so the
    // benchmark is never scored on a vocabulary that happens to exclude it.
    let mut selected: Vec<(String, String)> = entries
        .iter()
        .filter(|(w, _)| probe_words.contains(w))
        .cloned()
        .collect();
    let probe_count = selected.len();
    for (w, d) in &entries {
        if selected.len() >= max_entries {
            break;
        }
        if !probe_words.contains(w) {
            selected.push((w.clone(), d.clone()));
        }
    }

    println!(
        "dictionary: {} entries available, training on {} ({} probe words present), {} epoch(s)",
        entries.len(),
        selected.len(),
        probe_count,
        epochs
    );

    let trainer = Trainer::new(LEARNING_RATE);

    // Train the same data under different objectives.
    //
    // The mixing sweep in RESULTS §3 found that training the manifold on
    // next-word ranking rather than centroid attraction moved its recovered
    // predictive signal from 0.9% to 24.3% — 27x, from the objective alone. If
    // relational structure is going to appear anywhere, it should appear under
    // the objective that already demonstrated it carries more information.
    let mut trained: Vec<(String, Facet)> = Vec::new();

    for regime in ["co-occurrence", "ranking", "both"] {
        let mut facet = Facet::new();
        let start = std::time::Instant::now();

        for _ in 0..epochs {
            for (word, def) in &selected {
                let text = format!("{} {}", word, def);
                match regime {
                    "co-occurrence" => {
                        trainer.train_sentence(&mut facet, &text);
                    }
                    "ranking" => {
                        // One structural pass to populate the lexicon and the
                        // n-gram tables, then the ranking objective alone.
                        let seed = Trainer { learning_rate: 0.0, neg_samples: 0, definitions: None };
                        seed.train_sentence(&mut facet, &text);
                        trainer.train_predictive(&mut facet, &text);
                        trainer.train_predictive(&mut facet, &text);
                    }
                    _ => {
                        trainer.train_full(&mut facet, &text);
                    }
                }
            }
        }

        println!(
            "  [{}] {:?}  vocabulary {}  dispersion {:.4}",
            regime,
            start.elapsed(),
            facet.vocabulary_size(),
            facet.phase_dispersion()
        );
        trained.push((regime.to_string(), facet));
    }

    println!("\n{:<16} {:>12} {:>9} {:>9} {:>10} {:>10} {:>8}",
        "objective", "pair>random", "near@10", "near@50", "analog@1", "analog@5", "MRR");
    let mut best_report: Option<RelationReport> = None;
    for (name, facet) in &trained {
        let r = RelationBenchmark::evaluate(facet, &families);
        let near10: f64 = r.families.iter().map(|f| f.neighbour_top10).sum::<f64>()
            / r.families.len().max(1) as f64;
        let near50: f64 = r.families.iter().map(|f| f.neighbour_top50).sum::<f64>()
            / r.families.len().max(1) as f64;
        let a5: f64 = r.families.iter().map(|f| f.analogy_top5).sum::<f64>()
            / r.families.len().max(1) as f64;
        let mrr: f64 = r.families.iter().map(|f| f.analogy_mrr).sum::<f64>()
            / r.families.len().max(1) as f64;
        println!(
            "{:<16} {:>11.0}% {:>8.0}% {:>8.0}% {:>9.2}% {:>9.2}% {:>8.4}",
            name,
            r.overall_pair_vs_random * 100.0,
            near10 * 100.0,
            near50 * 100.0,
            r.overall_analogy_top1 * 100.0,
            a5 * 100.0,
            mrr
        );
        if best_report.is_none() {
            best_report = Some(r);
        }
    }
    let chance = best_report.as_ref().map(|r| r.chance_analogy_top1).unwrap_or(0.0);
    println!(
        "{:<16} {:>11.0}% {:>8.2}% {:>8.2}% {:>9.4}% {:>9.4}%",
        "CHANCE", 50.0,
        1000.0 / trained[0].1.vocabulary_size().max(1) as f64,
        5000.0 / trained[0].1.vocabulary_size().max(1) as f64,
        chance * 100.0, chance * 5.0 * 100.0
    );

    // Detailed per-family view for the strongest regime, plus the grounding
    // ablation, using the co-occurrence model for continuity with earlier runs.
    let mut facet = trained.remove(0).1;
    let before = RelationBenchmark::evaluate(&facet, &families);
    print_report("per-family, co-occurrence training", &before);

    DefinitionGrounder::ground_phases(&mut facet, &store);
    println!("dispersion after grounding: {:.4}", facet.phase_dispersion());
    let after = RelationBenchmark::evaluate(&facet, &families);
    print_report("per-family, after definition grounding", &after);

    println!("\n--- verdict ---");
    let best = before.overall_analogy_top1.max(after.overall_analogy_top1) * 100.0;
    println!(
        "analogy: best {:.2}% against {:.4}% chance — {}",
        best,
        before.chance_analogy_top1 * 100.0,
        match best > before.chance_analogy_top1 * 100.0 * 10.0 {
            true => "well above chance; the manifold encodes the relation",
            false => "at or near chance; the manifold does not encode the relation",
        }
    );
    let pair = before.overall_pair_vs_random.max(after.overall_pair_vs_random) * 100.0;
    println!(
        "similarity: best {:.0}% against 50% chance — {}",
        pair,
        match pair > 60.0 {
            true => "related words are grouped",
            false => "related words are no closer than random ones",
        }
    );

    // Footprint of the interned on-disk format, on a dictionary-scale model.
    let path = std::env::temp_dir().join("phiano_relations_model.chroma");
    let path = path.to_str().unwrap_or("model.chroma");
    if phiano::storage::Storage::save(&facet, path).is_ok() {
        if let Ok(meta) = std::fs::metadata(path) {
            let bytes = meta.len();
            // What the previous string-keyed layout would have cost: every
            // n-gram follower stored its word as an owned String, trigram keys
            // stored two, and phase_lags duplicated the bigram key set again.
            let avg_word: f64 = facet
                .lexicon
                .keys()
                .map(|w| w.len() as f64)
                .sum::<f64>()
                / facet.vocabulary_size().max(1) as f64;
            let entries = facet.ngram_entries() as f64;
            // 8 bytes of length prefix + the word, per stored key, per table.
            let v2_ngrams = entries * (8.0 + avg_word) * 2.0 + entries * 4.0;
            let v2_est = bytes as f64 + v2_ngrams;

            println!("\n=== footprint (interned v3 format) ===");
            println!("  vocabulary      : {}", facet.vocabulary_size());
            println!("  n-gram entries  : {}", facet.ngram_entries());
            println!("  mean word length: {:.1} chars", avg_word);
            println!("  on disk (v3)    : {:.1} MB", bytes as f64 / 1_048_576.0);
            println!("  string-keyed est: {:.1} MB", v2_est / 1_048_576.0);
            println!("  reduction       : {:.0}%", 100.0 * (1.0 - bytes as f64 / v2_est));
        }
    }

    if let Ok(j) = serde_json::to_string_pretty(&(&before, &after)) {
        let _ = std::fs::write("data/relations.json", j);
        println!("\nfull report written to data/relations.json");
    }
}
