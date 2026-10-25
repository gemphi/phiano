#[allow(unused_imports)]
use super::*;

/// The condition table: one row per condition, target metric first.
pub(super) fn print_table(rows: &[Row]) {
    println!("\n=== definitions as compositions ===");
    println!("usable relation pairs: {}", rows[0].usable_pairs);
    println!(
        "{:<32} {:>9} {:>9} {:>9} {:>9} {:>7} {:>9}",
        "condition", "pair/rnd", "nbr@10", "anlg@1", "anlg MRR", "disp", "valid ppl"
    );
    for r in rows {
        println!(
            "{:<32} {:>8.1}% {:>8.1}% {:>8.2}% {:>9.4} {:>7.3} {:>9.2}",
            r.name,
            r.pair_vs_random * 100.0,
            r.neighbour_top10 * 100.0,
            r.analogy_top1 * 100.0,
            r.analogy_mrr,
            r.dispersion,
            r.valid_ppl
        );
    }
}

/// A2: per-family, not just the aggregate. A total can be carried by one
/// family, and morphological families (plural, comparative, past tense) can
/// be learned from spelling alone while the semantic ones learn nothing —
/// which the aggregate would hide.
pub(super) fn print_families(best: &Facet) {
    let families = RelationBenchmark::default_families();
    let report = RelationBenchmark::evaluate(best, &families);
    println!("\n--- per family, best condition ---");
    println!(
        "{:<14} {:>7} {:>10} {:>10} {:>10}",
        "family", "usable", "pair/rnd", "nbr@10", "anlg MRR"
    );
    for f in &report.families {
        println!(
            "{:<14} {:>7} {:>9.1}% {:>9.1}% {:>10.4}",
            f.name,
            f.usable_pairs,
            f.pair_vs_random * 100.0,
            f.neighbour_top10 * 100.0,
            f.analogy_mrr
        );
    }
    let usable: usize = report.families.iter().map(|f| f.usable_pairs).sum();
    let covered = report.families.iter().filter(|f| f.usable_pairs > 0).count();
    println!(
        "  {} usable pairs across {} of {} families covered by this vocabulary",
        usable,
        covered,
        report.families.len()
    );
}

/// Each condition read against the baseline row, and the invariant that says
/// the no-phase path was not disturbed.
pub(super) fn print_reading(rows: &[Row]) {
    println!("\n--- reading ---");
    let b = &rows[0];
    for r in rows.iter().skip(1) {
        println!(
            "{:<32} pair {:+.1}pp, nbr@10 {:+.1}pp, MRR {:+.4}, dispersion {:+.3}",
            r.name,
            (r.pair_vs_random - b.pair_vs_random) * 100.0,
            (r.neighbour_top10 - b.neighbour_top10) * 100.0,
            r.analogy_mrr - b.analogy_mrr,
            r.dispersion - b.dispersion
        );
    }
    println!(
        "\nvalid perplexity is the no-phase path and must not move: {:.2} throughout.",
        b.valid_ppl
    );
}
