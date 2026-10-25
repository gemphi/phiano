#[allow(unused_imports)]
use super::*;

pub(super) fn print_report(label: &str, r: &RelationReport) {
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
