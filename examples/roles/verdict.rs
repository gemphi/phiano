#[allow(unused_imports)]
use super::*;

/// Which of the two explanations — senses or training exposure — separates the
/// bands more.
pub(super) fn polysemy_verdict(ratios: &[(usize, f64)], fratios: &[f64]) {
    let mono = ratios.iter().find(|(i, _)| *i == 0).map(|(_, r)| *r);
    let poly = ratios.iter().filter(|(i, _)| *i >= 2).map(|(_, r)| *r).fold(f64::NAN, f64::min);

    println!("\n  VERDICT:");
    let sense_span = match (mono, poly.is_nan()) {
        (Some(m), false) => Some(poly / m.max(1e-9)),
        _ => None,
    };
    let freq_span = match (fratios.first(), fratios.last()) {
        (Some(lo), Some(hi)) if lo.is_finite() && hi.is_finite() => Some(hi / lo.max(1e-9)),
        _ => None,
    };

    match (sense_span, freq_span) {
        (Some(sp), Some(fp)) => {
            println!(
                "    Coherence rises {:.1}x from monosemous to polysemous heads, and\n\
                 \x20   {:.1}x from rare to frequent ones.",
                sp, fp
            );
            match fp >= sp {
                true => println!(
                    "    Frequency explains at least as much as polysemy, and polysemous\n\
                     \x20   words in a dictionary ARE the frequent ones — so this is a result\n\
                     \x20   about training exposure, not about senses. Splitting senses would\n\
                     \x20   not have fixed it. What the long tail needs is training, not types:\n\
                     \x20   a rare word's phases are still close to its hash seed."
                ),
                false => println!(
                    "    Polysemy separates the bands more than frequency does, so sense\n\
                     \x20   structure is carrying something frequency alone does not."
                ),
            }
        }
        _ => println!("    Not enough pairs in one band to compare."),
    }
    println!(
        "    Nothing above is fitted: sense counts come from the dictionary's own\n\
         \x20   numbering and the phasors are the ones already trained, so this cannot\n\
         \x20   be a result about the measurement adapting to the question."
    );
}
