#[allow(unused_imports)]
use super::*;

/// Test 3: does the best dimension beat the average of all of them?
pub(super) fn report_similarity(facet: &Facet, families: &[RelationFamily]) {
    let mut pool: Vec<&String> = facet
        .lexicon
        .iter()
        .filter(|(_, p)| p.count >= COUNT_FLOOR)
        .map(|(w, _)| w)
        .collect();
    pool.sort();

    println!("\n  {:<28} {:>14}", "similarity", "pair>random");
    for (label, f) in [
        ("mean of 64 (as shipped)", &mean_sim as &dyn Fn(&Facet, &str, &str) -> f64),
        ("best single dimension", &max_sim),
        ("mean of best 8", &top8_sim),
    ] {
        let s = pair_vs_random(facet, &pool, families, f);
        println!("  {:<28} {:>13.1}%", label, s * 100.0);
    }

    println!(
        "\n  Chance is 50.0%. If relations lived in single dimensions, the best\n\
         \x20 dimension would beat the average over all 64 — that is the whole\n\
         \x20 practical content of the claim, and the cheapest thing to act on if it\n\
         \x20 holds. A flat result does not refute the architecture; it says nothing\n\
         \x20 in the training objective ever asked a channel to specialise, so there\n\
         \x20 is no reason one would have."
    );
}
