/// Definition-grounded word initialization.
/// Replaces word.len()*PHI with centroid phases from word definitions.

use crate::chunker::ChunkStore;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use std::f64::consts::PI;

/// Re-seeds word phases from their dictionary definitions.
/// Each word's phase is moved toward the amplitude-weighted centroid
/// of its definition's token phases.
pub fn definition_ground_phases(facet: &mut Facet, chunk_store: &ChunkStore) -> usize {
    let entries = chunk_store.load_all();
    if entries.is_empty() { return 0; }

    println!("  [ground] Re-seeding phases from {} definitions...", entries.len());
    let mut grounded = 0usize;

    for (word, def) in &entries {
        if !facet.lexicon.contains_key(word) { continue; }
        let def_tokens = Tokenizer::tokenize(def);
        if def_tokens.is_empty() { continue; }

        let (mut sum_x, mut sum_y, mut count) = (0.0f64, 0.0f64, 0u32);
        for token in &def_tokens {
            if let Some(phasor) = facet.lexicon.get(token) {
                sum_x += phasor.phase.cos() * phasor.amplitude;
                sum_y += phasor.phase.sin() * phasor.amplitude;
                count += 1;
            }
        }

        if count > 0 {
            let centroid = sum_y.atan2(sum_x).rem_euclid(2.0 * PI);
            if let Some(phasor) = facet.lexicon.get_mut(word) {
                let current = phasor.phase;
                let mut diff = centroid - current;
                if diff > PI { diff -= 2.0 * PI; }
                if diff < -PI { diff += 2.0 * PI; }
                phasor.phase = (current + 0.5 * diff).rem_euclid(2.0 * PI);
                grounded += 1;
            }
        }
    }

    println!("  [ground] {} words re-seeded from definitions", grounded);
    grounded
}
