#[allow(unused_imports)]
use super::*;

/// The saturation sweep: fit relation types on half the pairs, score the other
/// half, and read the gain against the shuffled null at each type count.
pub(super) fn sweep(facet: &Facet, descended: &Facet, pairs: &[(String, String)]) {
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
    let args: Vec<String> = std::env::args().collect();
    let only = args.get(3).cloned().unwrap_or_else(|| "both".to_string());
    let arms: Vec<(&str, &phiano::facet::Facet)> = match only.as_str() {
        "base" => vec![("bottom-up", facet)],
        "descent" => vec![("after descent", descended)],
        // The fixed basis costs nothing to evaluate, so it can be run on its own
        // without paying for the k-means grid — which has been killed for memory
        // twice at k=64.
        "harmonic" => Vec::new(),
        _ => vec![("bottom-up", facet), ("after descent", descended)],
    };

    for (label, f) in arms {
        for est in OffsetEstimator::ALL {
            let head_dom = head_domination(f, pairs, est);

            for &k in &ks {
                let p = KindTable::saturation_at_with(f, pairs, k, 10, est);
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
}

/// Head domination, measured once per (manifold, estimator) at the full type
/// count. Centering *mechanically* lowers this, so it is not a verdict on its
/// own — it says what the clusters are grouping by, and the gain column says
/// whether that grouping is worth anything. Both are needed: a rule that
/// destroyed the offsets entirely would score 9.1% here and nothing at all
/// there.
fn head_domination(f: &Facet, pairs: &[(String, String)], est: OffsetEstimator) -> f64 {
    let types = KindTable::discover_with(f, pairs, 64, 12, est);
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
    shared as f64 / total.max(1) as f64 * 100.0
}
