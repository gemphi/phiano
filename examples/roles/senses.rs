#[allow(unused_imports)]
use super::*;

use phiano::roles::LexicalRules;
use phiano::sources::definition_senses;

/// Sense counts straight from the dictionary's own numbering, and the
/// per-sense glosses they came from.
pub(super) struct Senses {
    pub(super) counts: HashMap<String, usize>,
    pub(super) glosses: HashMap<String, Vec<String>>,
}

/// Count the senses the dictionary itself numbers, and report the scale of the
/// collapse one phasor per word imposes.
pub(super) fn load_senses(chunks_path: &str) -> Senses {
    let entries = ChunkStore::new(chunks_path).load_all();
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut glosses: HashMap<String, Vec<String>> = HashMap::new();
    for (w, raw) in &entries {
        let g = definition_senses(raw);
        if !g.is_empty() {
            counts.insert(w.clone(), g.len());
            glosses.insert(w.clone(), g);
        }
    }

    let polysemous = counts.values().filter(|n| **n > 1).count();
    let mean: f64 = counts.values().sum::<usize>() as f64 / counts.len().max(1) as f64;
    let max = counts.values().copied().max().unwrap_or(0);
    println!("\n=== what one phasor per word costs ===");
    println!(
        "  {} entries, {:.2} senses each on average, {} with more than one, max {}",
        counts.len(),
        mean,
        polysemous,
        max
    );

    Senses { counts, glosses }
}

/// Relations named by the language itself, extracted per sense.
pub(super) fn lexical_rules(facet: &Facet, sense_glosses: &HashMap<String, Vec<String>>) -> LexicalRules {
    let known: std::collections::HashSet<&str> = facet.lexicon.keys().map(|s| s.as_str()).collect();
    let is_content = |w: &str| w.len() > 2 && !is_function_word(w) && !is_modifier(w) && known.contains(w);

    let mut lex = LexicalRules::new();
    for (w, gs) in sense_glosses {
        if facet.lexicon.contains_key(w) {
            for g in gs {
                lex.extract(w, g, is_content);
            }
        }
    }
    lex
}
