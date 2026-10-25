#[allow(unused_imports)]
use super::*;

/// Reading two: C as a mode prior. Predict which of the 64 standing waves a
/// pair sits on from its frame class, held out, against the context-free
/// marginal — and then the control that matters most on this corpus, the same
/// pair in different frames.
pub(super) fn mode_prior_section(facet: &Facet, rows: &Rows) {
    let (pairs, context) = (&rows.pairs, &rows.context);
    println!("\n  === C as a mode prior ===");
    let table = OffsetTable::build(facet, pairs, OffsetEstimator::Raw);
    let modes = table.modes();
    let kept = table.pairs().to_vec();
    let index: HashMap<(&str, &str), Vec<usize>> = {
        let mut m: HashMap<(&str, &str), Vec<usize>> = HashMap::new();
        for (i, p) in pairs.iter().enumerate() {
            m.entry((p.0.as_str(), p.1.as_str())).or_default().push(i);
        }
        m
    };
    // `build` keeps rows in order and drops none here, but realign anyway rather
    // than assume: a silent misalignment would look like a result.
    let mut used_at: HashMap<(&str, &str), usize> = HashMap::new();
    let ctx_kept: Vec<String> = kept
        .iter()
        .map(|p| {
            let key = (p.0.as_str(), p.1.as_str());
            let n = used_at.entry(key).or_insert(0);
            let out = index
                .get(&key)
                .and_then(|v| v.get(*n))
                .map(|i| context[*i].clone())
                .unwrap_or_default();
            *n += 1;
            out
        })
        .collect();

    println!(
        "  {:<22} {:>8} {:>14} {:>14} {:>12}",
        "context C", "classes", "log-loss", "vs marginal", "vs shuffled"
    );
    for k in [8usize, 64] {
        let (classes, used) = collapse_small(&discover_classes(facet, &ctx_kept, k, 8));
        let real = predictive_gain(&classes, &modes);
        let sh = predictive_gain(&shuffle_classes(&classes, 0xC2B2_AE3D_27D4_EB4F), &modes);
        println!(
            "  {:<22} {:>8} {:>14.4} {:>+14.4} {:>+12.4}",
            format!("discovered, K={k}"),
            used,
            real.0,
            real.1,
            real.1 - sh.1
        );
    }

    // The control that matters most on this corpus: the *same pair* in different
    // frames. If C does anything, two occurrences of `cow` with different
    // neighbours should sit on different modes more often than two occurrences
    // with similar neighbours.
    let mut by_pair: HashMap<(&str, &str), Vec<usize>> = HashMap::new();
    for (i, p) in kept.iter().enumerate() {
        by_pair.entry((p.0.as_str(), p.1.as_str())).or_default().push(i);
    }
    let (mut same, mut n) = (0usize, 0usize);
    for occ in by_pair.values().filter(|v| v.len() >= 2) {
        for a in 0..occ.len() {
            for b in (a + 1)..occ.len() {
                if modes[occ[a]] == modes[occ[b]] {
                    same += 1;
                }
                n += 1;
            }
        }
    }
    println!(
        "\n  Two occurrences of the same pair land on the same mode {:.1}% of the\n\
         \x20 time ({} comparisons). The offset is a function of the two words alone,\n\
         \x20 so this is 100% by construction unless something varies it — and\n\
         \x20 nothing in the representation does. That number is the ceiling on what\n\
         \x20 any C can achieve here, and it is the finding, not the noise floor.",
        same as f64 / n.max(1) as f64 * 100.0,
        n
    );
    println!(
        "\n  Uniform chance over modes is ln(64) = {:.4} nats.",
        (PHASE_CHANNELS as f64).ln()
    );
}
