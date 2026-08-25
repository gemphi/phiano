/// Text composition from river flow banks.
///
/// Words are woven in the order the river visits them.
/// The source opens, tension provides contrast, resolution closes.

use crate::facet::Facet;

/// Composes text from the river flow path and word banks into cohesive narrative prose.
pub fn compose(_path: &[u16], banks: &[Vec<String>], resonant: &[String]) -> String {
    let mut story = Vec::new();

    // 1. Opening Chapter
    if let Some(source_words) = banks.first() {
        if !source_words.is_empty() {
            let w = source_words.iter().take(3).cloned().collect::<Vec<_>>();
            if w.len() >= 2 {
                story.push(format!("In the beginning, the narrative opens with {} and {}.", w[0], w[1]));
            } else if !w.is_empty() {
                story.push(format!("In the beginning, the narrative centers upon {}.", w[0]));
            }
        }
    }

    // 2. Progression through chromatic phase sectors
    for (i, bank) in banks.iter().enumerate().skip(1) {
        if bank.is_empty() { continue; }
        let clean_words: Vec<String> = bank.iter()
            .filter(|w| w.len() > 2 && w.chars().all(|c| c.is_alphabetic()))
            .take(3)
            .cloned()
            .collect();

        if clean_words.is_empty() { continue; }

        if i % 3 == 0 {
            story.push(format!("As tension shifts across the manifold, the current evokes {}.", clean_words.join(" and ")));
        } else if i % 2 == 0 {
            story.push(format!("Through continuous resonance, the structure deepens with {}.", clean_words.join(" and ")));
        } else {
            story.push(format!("Moving through neighboring sectors, we encounter {}.", clean_words.join(" and ")));
        }
    }

    // 3. Harmonic Resolution
    if !resonant.is_empty() {
        let res_words: Vec<String> = resonant.iter()
            .filter(|w| w.len() > 2 && w.chars().all(|c| c.is_alphabetic()))
            .take(3)
            .cloned()
            .collect();
        if !res_words.is_empty() {
            story.push(format!("Finally, the composition achieves harmonic equilibrium, resting in {}.", res_words.join(" and ")));
        }
    }

    if story.is_empty() {
        return "The harmonic phase waves converge into a stable attractor state.".to_string();
    }

    story.join(" ")
}

/// Reorders words in a bank using bigram transition probabilities.
pub fn reorder_with_bigrams(facet: &Facet, words: &[String]) -> Vec<String> {
    if words.len() <= 1 {
        return words.to_vec();
    }

    let mut remaining: Vec<String> = words.to_vec();
    let mut ordered: Vec<String> = Vec::new();

    ordered.push(remaining.remove(0));

    while !remaining.is_empty() {
        let current = ordered.last().unwrap();
        let mut best_idx = 0;
        let mut best_score = -1.0f64;

        for (i, candidate) in remaining.iter().enumerate() {
            let score = facet.bigram_probability(current, candidate);
            if score > best_score {
                best_score = score;
                best_idx = i;
            }
        }

        ordered.push(remaining.remove(best_idx));
    }

    ordered
}
