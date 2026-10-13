#[allow(unused_imports)]
use super::*;

/// Every relational headline in this project was averaged over 70k words, most
/// of them seen fewer than five times.
///
/// A word seen four times has phases essentially equal to its hash seed, so it
/// contributes noise to every ranking. §12 measured the effect on prepositional
/// coherence — 1.27x noise for rare heads against 14.14x for frequent ones — and
/// this asks the same question of the benchmark that produced the project's
/// headline numbers.
///
/// **The pool shrinks as the floor rises, which makes the task easier.** So
/// chance is recomputed at every floor and the comparison is the ratio to it.
/// Reading the raw column across rows would credit the restriction for removing
/// distractors.
pub(super) fn floor_experiment(facet: &Facet) {
    let families = RelationBenchmark::default_families();

    println!("\n=== does the headline survive the untrained tail being removed? ===");
    println!(
        "  {:>6} {:>8} {:>7} {:>9} {:>10} {:>9} {:>10} {:>9}",
        "floor", "pool", "pairs", "anlg MRR", "chance MRR", "ratio", "nbr@10", "vs chance"
    );

    for floor in [0u32, 2, 5, 25, 100, 200] {
        let r = RelationBenchmark::evaluate_above(facet, &families, floor);
        let usable: usize = r.families.iter().map(|f| f.usable_pairs).sum();
        if usable < 20 {
            println!(
                "  {:>6} {:>8} {:>7} {:>9} {:>10} {:>9} {:>10} {:>9}",
                floor, r.vocabulary_size, usable, "-", "-", "-", "-", "too few pairs"
            );
            continue;
        }

        let mrr: f64 = r.families.iter().map(|f| f.analogy_mrr).sum::<f64>()
            / r.families.len().max(1) as f64;
        let nbr: f64 = r.families.iter().map(|f| f.neighbour_top10).sum::<f64>()
            / r.families.len().max(1) as f64;

        // Chance MRR for a uniformly random ranking over the pool: the mean of
        // 1/rank across the pool, which is ln(N)/N to a good approximation.
        let n = r.vocabulary_size.max(1) as f64;
        let chance_mrr = (n.ln() + 0.5772) / n;
        let chance_nbr = r.chance_neighbour_top10;

        println!(
            "  {:>6} {:>8} {:>7} {:>9.4} {:>10.5} {:>8.1}x {:>9.1}% {:>8.1}x",
            floor,
            r.vocabulary_size,
            usable,
            mrr,
            chance_mrr,
            mrr / chance_mrr.max(1e-12),
            nbr * 100.0,
            nbr / chance_nbr.max(1e-12)
        );
    }

    // ---- reconciling this with the coherence result ----
    //
    // §12 found frequent heads at 14.14x noise coherence and read it as strong
    // relational structure. The sweep above says the opposite: restricting to
    // frequent words makes ranking WORSE relative to chance. Both cannot be
    // descriptions of the same thing.
    //
    // Coherence measures whether pairs share an offset, and there is a
    // degenerate way to score well on it: if the frequent words are all
    // clustered together, every offset between them is small and similar.
    // Consistent near-zero offsets are perfect coherence and useless for
    // ranking, because everything is near everything. That is the same
    // degenerate optimum the dispersion guard exists for, and §12 did not check
    // it.
    println!("\n  --- is the frequent-word coherence real, or local collapse? ---");
    println!("  {:>10} {:>8} {:>12}", "floor", "words", "dispersion");
    for floor in [0u32, 5, 25, 100, 200] {
        let mut sub = facet.clone();
        sub.lexicon.retain(|_, p| p.count >= floor.max(1));
        let n = sub.lexicon.len();
        if n < 20 {
            println!("  {:>10} {:>8} {:>12}", floor, n, "too few");
            continue;
        }
        println!("  {:>10} {:>8} {:>12.3}", floor, n, sub.phase_dispersion());
    }
    println!(
        "  Dispersion near 1.0 means the words are spread around the circle;\n\
         \x20 falling sharply with the floor would mean the frequent words sit in one\n\
         \x20 region, and that their offsets agree because they are all close together\n\
         \x20 rather than because they encode a relation."
    );

    println!(
        "\n  The pool shrinks as the floor rises, so the raw columns are NOT\n\
         \x20 comparable across rows — a smaller pool has fewer distractors and every\n\
         \x20 raw score goes up for free. Only the ratio-to-chance columns compare,\n\
         \x20 and only rows with enough surviving probe pairs mean anything at all."
    );
}
