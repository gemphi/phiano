#[allow(unused_imports)]
use super::*;

/// Test 3 on similarity: does the best single dimension beat the average of all
/// of them, and how does either compare against the shipped manifold on
/// identical probes?
pub(super) fn similarity_section(
    rel: &RelationalFacet,
    shipped: &Facet,
    pool: &[String],
    families: &[RelationFamily],
) {
    let known = |w: &str| known_word(rel, shipped, w);

    println!("\n  {:<40} {:>14}", "similarity", "pair>random");
    let shipped_score = pair_vs_random(pool, families, &known, |a, b| shipped.resonance(a, b));
    println!("  {:<40} {:>13.1}%", "shipped: mean of 64 channels", shipped_score * 100.0);

    let rel_all = pair_vs_random(pool, families, &known, |a, b| rel.similarity(a, b));
    println!("  {:<40} {:>13.1}%", "relational: evidence-weighted, all", rel_all * 100.0);

    let rel_max = pair_vs_random(pool, families, &known, |a, b| {
        (0..PHASE_CHANNELS).map(|k| rel.on(a, b, k)).fold(f64::NEG_INFINITY, f64::max)
    });
    println!("  {:<40} {:>13.1}%", "relational: best single dimension", rel_max * 100.0);

    let rel_top8 = pair_vs_random(pool, families, &known, |a, b| {
        let mut v: Vec<f64> = (0..PHASE_CHANNELS).map(|k| rel.on(a, b, k)).collect();
        v.sort_by(|p, q| q.partial_cmp(p).unwrap_or(std::cmp::Ordering::Equal));
        v[..8].iter().sum::<f64>() / 8.0
    });
    println!("  {:<40} {:>13.1}%", "relational: mean of best 8", rel_top8 * 100.0);
}
