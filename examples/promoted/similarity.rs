#[allow(unused_imports)]
use super::*;

/// Questions 1 and 2: does the relation coupling cost the dense similarity the
/// engine depends on, and does the evidence-weighted comparison recover what
/// `bin/relational` saw? Every arm ranks the same pairs against the same pool.
pub(super) fn experiment_similarity(
    control: &Facet,
    promoted: &Facet,
    pool: &[String],
    families: &[RelationFamily],
    known: &dyn Fn(&str) -> bool,
    balance: &[f64],
) {
    println!("\n  {:<48} {:>12}", "similarity", "pair>random");
    let s_control = pair_vs_random(pool, families, known, |a, b| control.resonance(a, b));
    println!("  {:<48} {:>11.1}%", "control: dense resonance (relations off)", s_control * 100.0);
    let s_dense = pair_vs_random(pool, families, known, |a, b| promoted.resonance(a, b));
    println!("  {:<48} {:>11.1}%", "promoted: dense resonance", s_dense * 100.0);
    let s_weighted = pair_vs_random(pool, families, known, |a, b| {
        promoted.lexicon[a].similarity_with(&promoted.lexicon[b], Some(balance))
    });
    println!("  {:<48} {:>11.1}%", "promoted: evidence-weighted, balanced", s_weighted * 100.0);
    let s_raw = pair_vs_random(pool, families, known, |a, b| {
        promoted.lexicon[a].similarity(&promoted.lexicon[b])
    });
    println!("  {:<48} {:>11.1}%", "promoted: evidence-weighted, raw", s_raw * 100.0);
    let s_best = pair_vs_random(pool, families, known, |a, b| {
        (0..PHASE_CHANNELS).map(|k| promoted.on(a, b, k)).fold(f64::NEG_INFINITY, f64::max)
    });
    println!("  {:<48} {:>11.1}%", "promoted: best single relation", s_best * 100.0);
}
