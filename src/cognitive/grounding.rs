//! Definition-grounded word initialization.
//!
//! A word's position should be the centre of mass of what the word *means*, not
//! an artefact of how it is spelled. Dictionary definitions make that computable
//! without labels: a dictionary is a closed system in which every word is
//! defined by other words, so meaning is recoverable from structure alone.
//!
//! # Architecture
//!
//! ```text
//! Dictionary Definitions (ChunkStore)
//!   │
//!   ▼
//! DefinitionGrounder::ground_phases()   ── repeated GROUNDING_ROUNDS times
//!   ├─▶ READ pass:  centroid of each definition's content words (all words frozen)
//!   └─▶ WRITE pass: relax each word halfway toward its centroid
//! ```
//!
//! The read and write passes are separated (Jacobi rather than Gauss-Seidel) so
//! that the result does not depend on the order the dictionary happens to be
//! enumerated in, and the whole thing is iterated because definitions are
//! recursive — `cat` → `animal` → `organism` — and one pass propagates meaning
//! exactly one hop.

use crate::chunker::ChunkStore;
use crate::config::{GROUNDING_ROUNDS, TWO_PI};
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use std::f64::consts::PI;

/// Any source that can describe a symbol in terms of other symbols.
///
/// Grounding is not dictionary-specific: the mechanism takes a mapping from a
/// symbol to a bag of symbols and places the symbol at the bag's centre of
/// mass. A function body, a lead paragraph, a docstring or a window of
/// mentions all have that shape, so any of them can ground a manifold.
pub trait Groundable {
    /// `(symbol, description)` pairs.
    fn entries(&self) -> Vec<(String, String)>;
    /// A label for logging.
    fn source_name(&self) -> &str {
        "source"
    }
}

impl Groundable for ChunkStore {
    fn entries(&self) -> Vec<(String, String)> {
        self.load_all()
    }
    fn source_name(&self) -> &str {
        "dictionary"
    }
}

/// Grounding engine for aligning lexical phases with source semantics.
pub struct DefinitionGrounder;

impl DefinitionGrounder {
    /// Relaxes every known word toward the centroid of its definition.
    pub fn ground_phases(facet: &mut Facet, chunk_store: &ChunkStore) -> usize {
        Self::ground_from(facet, chunk_store)
    }

    /// Grounds a facet from any [`Groundable`] source.
    pub fn ground_from<G: Groundable>(facet: &mut Facet, source: &G) -> usize {
        let entries = source.entries();
        if entries.is_empty() {
            return 0;
        }

        println!(
            "  [ground] Re-seeding phases from {} {} entries...",
            entries.len(),
            source.source_name()
        );
        let mut grounded = 0usize;

        for round in 0..GROUNDING_ROUNDS {
            // ---- READ pass: compute every target against frozen phases ----
            let mut updates: Vec<(String, f64)> = Vec::with_capacity(entries.len());
            for (word, def) in &entries {
                if !facet.lexicon.contains_key(word) {
                    continue;
                }
                if let Some(target) = Self::definition_centroid(facet, def) {
                    updates.push((word.clone(), target));
                }
            }

            // ---- WRITE pass: relax halfway toward each target ----
            let mut moved = 0usize;
            let mut total_shift = 0.0f64;
            for (word, target) in &updates {
                if let Some(phasor) = facet.lexicon.get_mut(word) {
                    let current = phasor.theta(0);
                    let mut diff = target - current;
                    if diff > PI {
                        diff -= TWO_PI;
                    }
                    if diff < -PI {
                        diff += TWO_PI;
                    }
                    phasor.set_theta(0, current + 0.5 * diff);
                    phasor.sync_phase();
                    total_shift += (0.5 * diff).abs();
                    moved += 1;
                }
            }

            grounded = moved;
            let mean_shift = if moved > 0 { total_shift / moved as f64 } else { 0.0 };
            println!(
                "  [ground] round {}/{}: {} phases, mean shift {:.5} rad",
                round + 1,
                GROUNDING_ROUNDS,
                moved,
                mean_shift
            );

            // 0.5 damping halves the remaining error each round, so this
            // converges geometrically; stop once it stops mattering.
            if mean_shift < 0.001 {
                break;
            }
        }

        facet.grounded_version = crate::config::GROUNDING_VERSION;
        println!("  [ground] Grounded {} word phases from definitions.", grounded);
        grounded
    }

    /// Amplitude-weighted circular centroid of a definition's *content* words.
    ///
    /// Function words are excluded. They appear in nearly every definition, so
    /// at full weight they pull every word in the dictionary toward one shared
    /// point — grounding would then re-create the collapse it is meant to fix.
    fn definition_centroid(facet: &Facet, definition: &str) -> Option<f64> {
        let tokens = Tokenizer::tokenize(definition);
        if tokens.is_empty() {
            return None;
        }

        let (mut sum_x, mut sum_y, mut count) = (0.0f64, 0.0f64, 0u32);
        for token in &tokens {
            if Tokenizer::is_function_word(token) {
                continue;
            }
            if let Some(phasor) = facet.lexicon.get(token) {
                let th = phasor.theta(0);
                sum_x += th.cos() * phasor.amplitude;
                sum_y += th.sin() * phasor.amplitude;
                count += 1;
            }
        }

        match count > 0 {
            true => Some(sum_y.atan2(sum_x).rem_euclid(TWO_PI)),
            false => None,
        }
    }
}
