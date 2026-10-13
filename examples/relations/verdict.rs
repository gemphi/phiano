#[allow(unused_imports)]
use super::*;

/// The two headline questions, each against its chance baseline.
pub(super) fn print_verdict(before: &RelationReport, after: &RelationReport) {
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
}
