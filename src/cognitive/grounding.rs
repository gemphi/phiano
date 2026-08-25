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
    match entries.is_empty() {
        true => return 0,
        false => {}
    }

    println!("  [ground] Re-seeding phases from {} definitions...", entries.len());
    let mut grounded = 0usize;

    for (word, def) in &entries {
        match facet.lexicon.contains_key(word) {
            false => continue,
            true => {}
        }
        let def_tokens = Tokenizer::tokenize(def);
        match def_tokens.is_empty() {
            true => continue,
            false => {}
        }

        let (mut sum_x, mut sum_y, mut count) = (0.0f64, 0.0f64, 0u32);
        for token in &def_tokens {
            match facet.lexicon.get(token) {
                Some(phasor) => {
                    sum_x += phasor.phase.cos() * phasor.amplitude;
                    sum_y += phasor.phase.sin() * phasor.amplitude;
                    count += 1;
                }
                None => {}
            }
        }

        match count > 0 {
            true => {
                let centroid = sum_y.atan2(sum_x).rem_euclid(2.0 * PI);
                match facet.lexicon.get_mut(word) {
                    Some(phasor) => {
                        let current = phasor.phase;
                        let mut diff = centroid - current;
                        match diff > PI {
                            true => diff -= 2.0 * PI,
                            false => {}
                        }
                        match diff < -PI {
                            true => diff += 2.0 * PI,
                            false => {}
                        }
                        phasor.phase = (current + 0.5 * diff).rem_euclid(2.0 * PI);
                        grounded += 1;
                    }
                    None => {}
                }
            }
            false => {}
        }
    }

    println!("  [ground] {} words re-seeded from definitions", grounded);
    grounded
}
