#[allow(unused_imports)]
use super::*;

/// Tests 1 and 2: does a relation family have a channel beyond what a
/// permuted-filler null reaches on its own best channel, and do different
/// families claim different channels?
pub(super) fn family_channels(
    rel: &RelationalFacet,
    vocab: &RelationVocab,
    families: &[RelationFamily],
) {
    println!(
        "\n  {:<14} {:>6} {:>16} {:>6} {:>10} {:>10} {:>9}",
        "family", "pairs", "best channel", "cov", "agreement", "null best", "excess"
    );
    let mut peaks: Vec<usize> = Vec::new();
    let mut excesses: Vec<f64> = Vec::new();
    for fam in families {
        let pairs: Vec<(String, String)> =
            fam.pairs.iter().map(|p| (p.a.clone(), p.b.clone())).collect();
        let (prof, cov, n) = profile(rel, &pairs);
        if n < 5 {
            continue;
        }
        let (k, v) = argmax(&prof);
        if !v.is_finite() {
            println!("  {:<14} {:>6} {:>16}", fam.name, n, "no covered channel");
            continue;
        }
        let (n1, _, _) = profile(rel, &permute(&pairs, 0x9E37_79B9_7F4A_7C15));
        let (n2, _, _) = profile(rel, &permute(&pairs, 0xC2B2_AE3D_27D4_EB4F));
        let nb1 = argmax(&n1).1;
        let nb2 = argmax(&n2).1;
        let null_best = match nb1.is_finite() && nb2.is_finite() {
            true => (nb1 + nb2) / 2.0,
            false => f64::NAN,
        };
        println!(
            "  {:<14} {:>6} {:>16} {:>6} {:>10.4} {:>10.4} {:>+9.4}",
            fam.name,
            n,
            format!("{} ({})", k, vocab.name(k)),
            cov[k],
            v,
            null_best,
            v - null_best
        );
        peaks.push(k);
        excesses.push(v - null_best);
    }
    let distinct: std::collections::HashSet<usize> = peaks.iter().copied().collect();
    let worst = excesses.iter().cloned().fold(f64::INFINITY, f64::min);
    println!(
        "\n  {} families on {} distinct channels. Worst excess {:+.4} — that is the\n\
         \x20 noise scale, and no positive row counts unless it clears it.",
        peaks.len(),
        distinct.len(),
        worst
    );
}
