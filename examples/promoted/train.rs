#[allow(unused_imports)]
use super::*;

/// Trains the two arms that differ in exactly one thing — relations off versus
/// on — and reports their size, timing, and per-channel dispersion.
pub(super) fn train_arms(split: &Split) -> (Facet, Facet) {
    let t0 = std::time::Instant::now();
    let control = Harness::train_ranking_only(
        split,
        &Trainer::new(LEARNING_RATE).with_seed(0).without_relations(),
        PASSES,
    );
    let t_control = t0.elapsed();
    let t0 = std::time::Instant::now();
    let promoted =
        Harness::train_ranking_only(split, &Trainer::new(LEARNING_RATE).with_seed(0), PASSES);
    let t_promoted = t0.elapsed();

    println!(
        "control  {} words, trained in {:.1}s\npromoted {} words, {} with evidence, {} marker channels, trained in {:.1}s",
        control.vocabulary_size(),
        t_control.as_secs_f64(),
        promoted.vocabulary_size(),
        promoted.evidenced_words(),
        promoted.relations.markers(),
        t_promoted.as_secs_f64(),
    );
    println!(
        "  channels 0..12 are: {}",
        (0..12).map(|k| promoted.relation_name(k)).collect::<Vec<_>>().join(", ")
    );

    println!("\n  per-channel dispersion (1 = spread, 0 = collapsed), promoted vs control:");
    for k in [0, 1, 2, 3, 7, 8, 9, 10, 20, 40, 63] {
        println!(
            "    {:<10} {:.3}  {:.3}",
            promoted.relation_name(k),
            promoted.channel_dispersion(k),
            control.channel_dispersion(k)
        );
    }

    (control, promoted)
}
