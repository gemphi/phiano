#[allow(unused_imports)]
use super::*;

/// Scores one arm band by band on both tasks, then asks whether any band peaks
/// beyond the spread of the others and how much of the word-level effect a
/// single band retains.
pub(super) fn band_profile(label: &str, facet: &Facet, items: &[Item]) {
    // The distractor pool is the trained band, sorted: a HashMap's order is
    // not stable across runs and this vector is indexed by a hash.
    let mut pool: Vec<&String> = facet
        .lexicon
        .iter()
        .filter(|(_, p)| p.count >= COUNT_FLOOR)
        .map(|(w, _)| w)
        .collect();
    pool.sort();

    println!("\n=== {} ===", label);
    println!(
        "  {:>7} {:>10} {:>12} {:>14} {:>12}",
        "band", "channels", "kernel |z|", "word pair>rnd", "sentence MRR"
    );

    let mut word: Vec<f64> = Vec::new();
    let mut sent: Vec<f64> = Vec::new();
    for b in 0..(PHASE_CHANNELS / BAND) {
        let (lo, hi) = (b * BAND, (b + 1) * BAND);
        let w = pair_vs_random(facet, &pool, lo, hi);
        let s = sentence_mrr(facet, items, lo, hi);
        // The band's own rate, as the kernel defines it: |z| is retention
        // per step, so a high value is a slow channel with long memory.
        let decay: f64 = (lo..hi)
            .map(|k| phiano::config::channel_kernel(k, PHASE_CHANNELS).norm())
            .sum::<f64>()
            / BAND as f64;
        println!(
            "  {:>7} {:>10} {:>12.4} {:>13.1}% {:>12.4}",
            b,
            format!("{lo}-{}", hi - 1),
            decay,
            w * 100.0,
            s
        );
        word.push(w);
        sent.push(s);
    }

    let all_w = pair_vs_random(facet, &pool, 0, PHASE_CHANNELS);
    let all_s = sentence_mrr(facet, items, 0, PHASE_CHANNELS);
    let chance_s: f64 = (1..=CANDIDATES).map(|r| 1.0 / r as f64).sum::<f64>() / CANDIDATES as f64;
    println!(
        "  {:>7} {:>10} {:>12} {:>13.1}% {:>12.4}",
        "all", "0-63", "-", all_w * 100.0, all_s
    );
    println!("  chance: word 50.0%, sentence {:.4}", chance_s);

    let (wi, wz) = peak(&word);
    let (si, sz) = peak(&sent);
    println!(
        "  word peak band {} at {:+.2} sd; sentence peak band {} at {:+.2} sd",
        wi, wz, si, sz
    );
    const SEPARATION: f64 = 2.0;
    println!(
        "  -> {}",
        match (wz >= SEPARATION, sz >= SEPARATION, wi != si) {
            (true, true, true) =>
                "THE LEVELS SEPARATE BY RATE: both profiles peak, in different bands",
            (true, true, false) => "both peak, in the SAME band: no separation",
            _ =>
                "FLAT: no band peaks beyond the spread of the others, so the \
                 channel index carries no hierarchy for these tasks",
        }
    );

    // What a fraction of the channels retains — the hologram property.
    // Bentov, ch. 1: information stored as an interference pattern is
    // recoverable from a piece of the film, at lower fidelity. If one band
    // of eight keeps most of the effect that all sixty-four carry, the
    // signal is distributed rather than localised, and losing channels
    // degrades gracefully instead of catastrophically.
    let band_mean = word.iter().sum::<f64>() / word.len() as f64;
    // Retention is a ratio to the full-channel effect, so it means nothing
    // when there is no full-channel effect to retain — dividing by a number
    // at chance produces a percentage in the hundreds of millions, which is
    // how this first printed.
    const MIN_EFFECT: f64 = 0.02;
    println!(
        "  holographic retention (word): {}",
        match all_w - 0.5 > MIN_EFFECT {
            true => format!(
                "1/8 of the channels keeps {:.0}% of the effect over chance \
                 ({:.1} of {:.1} points)",
                (band_mean - 0.5) / (all_w - 0.5) * 100.0,
                (band_mean - 0.5) * 100.0,
                (all_w - 0.5) * 100.0
            ),
            false => format!(
                "undefined — all 64 channels are at chance ({:.1}%), so there \
                 is no effect for a subset to retain",
                all_w * 100.0
            ),
        }
    );
}
