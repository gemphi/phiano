#[allow(unused_imports)]
use super::*;

/// Pairs straight out of the dictionary: a head and each content word of its
/// definitional core. Nothing here says which relation any pair stands in —
/// that is the whole point. No enum, no preposition list, no labels.
pub(super) fn gloss_pairs(facet: &Facet) -> Vec<(String, String)> {
    let args: Vec<String> = std::env::args().collect();
    let chunks = args.get(1).cloned().unwrap_or_else(|| "data/chunks".to_string());

    let glosses: Vec<(String, String)> = ChunkStore::new(&chunks)
        .load_all()
        .into_iter()
        .map(|(w, d)| (w, definition_core(&d)))
        .collect();

    let above = |w: &str| facet.lexicon.get(w).is_some_and(|p| p.count >= COUNT_FLOOR);

    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut per_head: HashMap<String, usize> = HashMap::new();
    for (head, gloss) in &glosses {
        if !above(head) {
            continue;
        }
        for t in Tokenizer::tokenize(gloss) {
            if t == *head || !above(&t) {
                continue;
            }
            // Cap the fan-out per head so a long entry cannot dominate the
            // clustering by sheer weight of pairs.
            let n = per_head.entry(head.clone()).or_insert(0);
            if *n >= 8 {
                break;
            }
            *n += 1;
            pairs.push((head.clone(), t));
        }
    }
    pairs.sort();
    pairs.dedup();

    println!(
        "facet {} words ({} at or above count {}), {} pairs from {} glosses",
        facet.vocabulary_size(),
        facet.lexicon.values().filter(|p| p.count >= COUNT_FLOOR).count(),
        COUNT_FLOOR,
        pairs.len(),
        glosses.len()
    );
    if pairs.len() < 200 {
        eprintln!("too few pairs to say anything");
        std::process::exit(1);
    }
    pairs
}

/// The chance level for the head-domination column: with 64 clusters and a
/// head taking up to 8 fillers, some collisions happen for free.
pub(super) fn head_dom_chance(pairs: &[(String, String)]) {
    let mut fan: HashMap<&str, usize> = HashMap::new();
    for (h, _) in pairs {
        *fan.entry(h.as_str()).or_insert(0) += 1;
    }
    let n_pairs: usize = fan.values().sum();
    let expected: f64 = fan
        .values()
        .map(|&f| {
            let f = f as f64;
            f * (1.0 - (1.0 - 1.0 / 64.0f64).powf(f - 1.0))
        })
        .sum::<f64>()
        / n_pairs.max(1) as f64;

    println!(
        "\n  head-dom chance level: {:.1}%. A gain under 3x its noise is no effect.\n\
         \x20 Centering lowers head-dom by construction, so it only counts when the\n\
         \x20 gain survives — the two columns have to move the right way together.",
        expected * 100.0
    );
}
