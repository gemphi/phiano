#[allow(unused_imports)]
use super::*;

/// One row per training objective, with the chance line underneath.
pub(super) fn print_objective_table(trained: &[(String, Facet)], families: &[RelationFamily]) {
    println!("\n{:<16} {:>12} {:>9} {:>9} {:>10} {:>10} {:>8}",
        "objective", "pair>random", "near@10", "near@50", "analog@1", "analog@5", "MRR");
    let mut best_report: Option<RelationReport> = None;
    for (name, facet) in trained {
        let r = RelationBenchmark::evaluate(facet, families);
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
}
