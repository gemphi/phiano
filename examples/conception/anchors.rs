#[allow(unused_imports)]
use super::*;

/// The retrofitting anchor. The best-scoring row above drops dispersion from
/// 0.986 to 0.327, which is a third of the way to collapse — the anchor is
/// the term that competes with the neighbour pull, and the sweep is what
/// says whether the relation gains survive keeping the manifold spread.
pub(super) fn anchor_conditions(
    s: &Setup,
    graph: &DefinitionGraph,
    kernel: &std::collections::HashSet<String>,
    rows: &mut Vec<Row>,
    best: &mut (f64, Facet),
) {
    let (base, entries, rounds, valid) = (&s.base, &s.entries, s.rounds, &s.split.valid);
    println!(
        "grounding kernel (pre-promotion): {} of {} nodes ({:.1}%)",
        kernel.len(),
        graph.nodes(),
        100.0 * kernel.len() as f64 / graph.nodes().max(1) as f64
    );
    for a in [0.25f64, 0.5, ANCHOR, 2.0] {
        let mut f = base.clone();
        Conception::compose_anchored(
            &mut f, entries, rounds, HEAD_STEP, BETA_STRONG, BETA_STRONG, false, None, a, None,
        );
        record(rows, best, &format!("  anchor α={:.2}", a), f, valid);
    }
    {
        // Kernel scheduling: hold the definitional core at a tenth of the rate
        // and compose the periphery against it, instead of relaxing all
        // entries simultaneously against neighbours that are themselves moving.
        let mut f = base.clone();
        Conception::compose_anchored(
            &mut f, entries, rounds, HEAD_STEP, BETA_STRONG, BETA_STRONG, false, None, ANCHOR,
            Some(kernel),
        );
        record(rows, best, "  anchor + held kernel", f, valid);
    }
}
