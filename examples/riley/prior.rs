#[allow(unused_imports)]
use super::*;

/// C as a mode prior.
///
/// The mean-shift reading of "in C" failed. The other reading fits the
/// geometry better: the medium sustains 64 standing waves, and C says *which
/// one you are in* rather than translating it. `cow counts as animal` in a
/// taxonomy and `cow counts as livestock` in a market are two modes, not one
/// mode plus an offset.
///
/// That is a prediction test with no free parameters on either side. The mode
/// labels come from the fixed harmonic basis, which no training produced and
/// no context ever saw. If a context class predicts them on held-out pairs
/// better than the marginal does, C carries information about which relation
/// holds.
///
/// The confound to kill first: the gloss contains the head and the filler, so
/// a context built from it trivially knows the pair, and the shuffled-C null
/// does not control for that — shuffling destroys the link entirely rather
/// than isolating it. So the context state is the gloss **with the pair's own
/// two words removed**, the same leave-one-out discipline the downward pass
/// uses. What is left is the rest of the sentence, and any predictive power
/// is not the sentence quoting the answer back.
pub(super) fn mode_prior_table(d: &Data) {
    let (facet, pairs, sources) = (&d.facet, &d.pairs, &d.sources);
    println!("\n  --- C as a mode prior (leave-the-pair-out context) ---");

    let loo_context: Vec<String> = pairs
        .iter()
        .zip(sources.iter())
        .map(|((h, f), gloss)| {
            Tokenizer::tokenize(gloss)
                .into_iter()
                .filter(|w| w != h && w != f)
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();

    let table = OffsetTable::build(facet, pairs, phient::train::kinds::OffsetEstimator::Raw);
    let modes = table.modes();
    let kept: Vec<(String, String)> = table.pairs().to_vec();

    // Realign the leave-one-out contexts onto the pairs the table kept.
    let index: HashMap<(&str, &str), usize> = pairs
        .iter()
        .enumerate()
        .map(|(i, p)| ((p.0.as_str(), p.1.as_str()), i))
        .collect();
    let ctx_for_kept: Vec<String> = kept
        .iter()
        .map(|p| {
            index
                .get(&(p.0.as_str(), p.1.as_str()))
                .map(|i| loo_context[*i].clone())
                .unwrap_or_default()
        })
        .collect();

    println!(
        "  {:<22} {:>8} {:>14} {:>14} {:>12}",
        "context C", "classes", "log-loss", "vs marginal", "vs shuffled"
    );

    for k in [8usize, 64] {
        let classes = discover_classes(facet, &ctx_for_kept, k, 8);
        let (classes, used) = collapse_small(&classes);
        let real = predictive_gain(&classes, &modes);
        let shuffled = predictive_gain(&shuffle_classes(&classes, 0xC2B2_AE3D_27D4_EB4F), &modes);
        println!(
            "  {:<22} {:>8} {:>14.4} {:>+14.4} {:>+12.4}",
            format!("discovered, K={k}"),
            used,
            real.0,
            real.1,
            real.1 - shuffled.1
        );
    }

    let syn: Vec<u16> = kept
        .iter()
        .zip(ctx_for_kept.iter())
        .map(|(_, g)| phiton::p64::topology::SentenceType::classify(g).base as u16)
        .collect();
    let (syn, used) = collapse_small(&syn);
    let real = predictive_gain(&syn, &modes);
    let shuffled = predictive_gain(&shuffle_classes(&syn, 0xC2B2_AE3D_27D4_EB4F), &modes);
    println!(
        "  {:<22} {:>8} {:>14.4} {:>+14.4} {:>+12.4}",
        "syntactic (p64)", used, real.0, real.1, real.1 - shuffled.1
    );

    println!(
        "\n  Log-loss is in nats per pair over 64 modes; the marginal is what you\n\
         \x20 pay knowing only how common each mode is. A positive 'vs marginal' means\n\
         \x20 the context helped, and a positive 'vs shuffled' means it helped for a\n\
         \x20 reason other than being a partition. Uniform chance is ln(64) = {:.4}.",
        (PHASE_CHANNELS as f64).ln()
    );
}
