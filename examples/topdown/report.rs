#[allow(unused_imports)]
use super::*;

/// What the downward pass did to the manifold.
pub(super) fn report_descent(arms: &[Arm]) {
    println!("\n=== the descent itself ===");
    println!(
        "  {:<26} {:>9} {:>9} {:>11} {:>11} {:>9}",
        "arm", "moved", "seen", "dispersion", "band", "kept"
    );
    for a in arms {
        match &a.report {
            None => println!(
                "  {:<26} {:>9} {:>9} {:>11.3} {:>11.3} {:>9}",
                a.label,
                "-",
                "-",
                a.facet.phase_dispersion(),
                a.facet.dispersion_top(2_000),
                "-"
            ),
            Some(r) => println!(
                "  {:<26} {:>9} {:>9} {:>11.3} {:>11.3} {:>9}",
                a.label,
                r.words_moved,
                r.words_seen,
                r.dispersion_after,
                r.band_after,
                match r.rejected {
                    true => "REJECTED",
                    false => "yes",
                }
            ),
        }
    }
    println!(
        "  A rejected arm was discarded by the collapse guard and is identical to\n\
         \x20 the control below — its scores are the control's, not a result."
    );
}

/// The load-bearing column: does the level below get better?
pub(super) fn report_word_level(arms: &[Arm], base: &Facet) {
    let families = RelationBenchmark::default_families();
    println!("\n=== word level, at a frequency floor of {} ===", FLOOR);
    let pool_n = RelationBenchmark::pool_size(base, FLOOR);
    println!(
        "  candidate pool {} words; chance analogy MRR ~{:.5}, chance nbr@10 {:.3}%",
        pool_n,
        1.0 / pool_n.max(1) as f64,
        1000.0 / pool_n.max(1) as f64
    );
    println!(
        "  {:<26} {:>10} {:>10} {:>10} {:>10}",
        "arm", "anlg MRR", "vs chance", "nbr@10", "pair>rnd"
    );
    for a in arms {
        let r = RelationBenchmark::evaluate_above(&a.facet, &families, FLOOR);
        let nbr: f64 = r.families.iter().map(|f| f.neighbour_top10).sum::<f64>()
            / r.families.len().max(1) as f64;
        let mrr: f64 = r.families.iter().map(|f| f.analogy_mrr).sum::<f64>()
            / r.families.len().max(1) as f64;
        println!(
            "  {:<26} {:>10.5} {:>9.2}x {:>9.2}% {:>9.1}%",
            a.label,
            mrr,
            mrr / (1.0 / pool_n.max(1) as f64),
            nbr * 100.0,
            r.overall_pair_vs_random * 100.0
        );
    }
}

/// The level the intervention was applied at.
pub(super) fn report_sentence_level(arms: &[Arm], held: &[String], seed: u64) {
    let docs = documents(held, 8);
    let pool: Vec<Vec<String>> = held.iter().map(|s| Tokenizer::tokenize(s)).collect();

    println!("\n=== sentence level, held out ===");
    println!(
        "  {:<26} {:>10} {:>10} {:>10}",
        "arm", "phase MRR", "lexical", "vs chance"
    );
    for a in arms {
        let r = SentenceBenchmark::evaluate(&a.facet, &docs, &pool, seed);
        let lexical = r.scorers.last().map(|s| s.mrr).unwrap_or(0.0);
        let best = r.scorers[..r.scorers.len().saturating_sub(1)]
            .iter()
            .map(|s| s.mrr)
            .fold(0.0f64, f64::max);
        println!(
            "  {:<26} {:>10.4} {:>10.4} {:>9.2}x",
            a.label,
            best,
            lexical,
            best / r.chance_mrr.max(1e-12)
        );
    }
}

/// How the two columns above relate, and why the null row is the only baseline.
pub(super) fn report_footer() {
    println!(
        "\n  How to read this. The sentence column is the level the rule was applied\n\
         \x20 at, so a gain there is partly circular by construction. The word column\n\
         \x20 is the claim: the downward pass never saw a relation probe, so an\n\
         \x20 improvement there is constraint from above genuinely improving the level\n\
         \x20 below. And the SHUFFLED row is the floor under both — it applies the same\n\
         \x20 update rule to sentences that are word bags with no co-occurrence\n\
         \x20 structure, so whatever it gains is the rule, not the hierarchy. Only the\n\
         \x20 gap between a real arm and that row is evidence of anything."
    );
}
