#[allow(unused_imports)]
use super::*;

/// Tests 1 and 2: does a family have a channel, and is it its own?
pub(super) fn report_family_channels(facet: &Facet, families: &[RelationFamily]) {
    println!(
        "  {:<14} {:>6} {:>10} {:>10} {:>10} {:>9}",
        "family", "pairs", "best chan", "agreement", "null best", "excess"
    );

    let mut peaks: Vec<(String, usize, f64)> = Vec::new();
    for fam in families {
        let pairs: Vec<(String, String)> =
            fam.pairs.iter().map(|p| (p.a.clone(), p.b.clone())).collect();
        let (prof, n) = channel_profile(facet, &pairs);
        if n < 5 {
            continue;
        }
        let (k, v) = argmax(&prof);

        // Two independent permutations, so the null's own spread is visible in
        // the excess rather than hidden inside it.
        let (n1, _) = channel_profile(facet, &permute(&pairs, 0x9E37_79B9_7F4A_7C15));
        let (n2, _) = channel_profile(facet, &permute(&pairs, 0xC2B2_AE3D_27D4_EB4F));
        let null_best = (argmax(&n1).1 + argmax(&n2).1) / 2.0;

        println!(
            "  {:<14} {:>6} {:>10} {:>10.4} {:>10.4} {:>+9.4}",
            fam.name,
            n,
            k,
            v,
            null_best,
            v - null_best
        );
        peaks.push((fam.name.clone(), k, v - null_best));
    }

    let distinct: std::collections::HashSet<usize> = peaks.iter().map(|p| p.1).collect();
    println!(
        "\n  {} families peak on {} distinct channels. Sharing one channel would\n\
         \x20 mean it is not a relation but simply the dimension with the most\n\
         \x20 variance; needing many would be the architecture working.",
        peaks.len(),
        distinct.len()
    );
}
