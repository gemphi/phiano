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
use crate::conception::Conception;
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

/// Below this, the manifold is concentrating rather than organising.
///
/// Phase dispersion is 1.0 when phases spread uniformly and 0.0 when every word
/// sits at the same angle. Composition legitimately lowers it — concept regions
/// are concentration — but the anchor sweep showed dispersion running from 0.54
/// at no anchor up to 0.81 at a strong one, and the unanchored, unguarded rule
/// reached 0.305 on an earlier source. A floor is what keeps "creates concept
/// regions" from becoming "collapses" without anyone noticing.
pub const DISPERSION_FLOOR: f64 = 0.40;

/// The band the guard actually watches.
///
/// The global dispersion figure is a tail average and cannot see the failure the
/// guard exists to detect: on a 30k vocabulary, the 500 most frequent words can
/// collapse onto a single angle while the number still reads 0.98, because the
/// rare words keep their initialisation and dominate the mean. Every scored task
/// draws its candidates from the frequent band, so the guard is applied there —
/// and to the whole lexicon, since either collapsing is a failure.
pub const GUARD_BAND_TOP: usize = 2_000;

/// Global and frequent-band dispersion, in that order.
pub fn dispersion_pair(facet: &Facet) -> (f64, f64) {
    (facet.phase_dispersion(), facet.dispersion_top(GUARD_BAND_TOP))
}

/// Grounding engine for aligning lexical phases with source semantics.
pub struct DefinitionGrounder;

impl DefinitionGrounder {
    /// Relaxes every known word toward the centroid of its definition.
    pub fn ground_phases(facet: &mut Facet, chunk_store: &ChunkStore) -> usize {
        Self::ground_from(facet, chunk_store)
    }

    /// Grounds a facet by the rule the measurements selected.
    ///
    /// The centroid grounder above writes `theta(0)` and nothing else, and it
    /// was measured: it halves phase dispersion and moves no relation metric.
    /// [`Conception`] composes across all 64 channels, and across five seeds on
    /// a 296-pair benchmark takes analogy MRR from 0.0002 ± 0.0001 to
    /// 0.0270 ± 0.0031 — an effect that clears its own noise by a wide margin.
    ///
    /// Both paths stay reachable for one release. `GROUND_BY_COMPOSITION`
    /// selects; the old path is retired once this one has run in anger.
    ///
    /// # The guard
    ///
    /// Composition concentrates the manifold — that is how it creates concept
    /// regions — and past a point concentration *is* collapse, which is the
    /// failure the whole harness exists to detect. So the result is checked
    /// before it is kept: if phase dispersion falls below
    /// [`DISPERSION_FLOOR`], the composed facet is discarded and the caller
    /// keeps what it had. A relational gain bought with a collapsed manifold is
    /// not a gain, and shipping one silently would undo the measurement
    /// discipline that produced the rule.
    pub fn ground_best(facet: &mut Facet, chunk_store: &ChunkStore) -> usize {
        if !crate::config::GROUND_BY_COMPOSITION {
            return Self::ground_from(facet, chunk_store);
        }
        Self::ground_best_from(
            facet,
            chunk_store,
            crate::config::COMPOSITION_ANCHOR,
            crate::config::GROUNDING_ROUNDS,
        )
    }

    /// [`DefinitionGrounder::ground_best`] over any source, at explicit
    /// settings.
    ///
    /// Split out so the guard can be driven from a test: a rejection path that
    /// has never been observed to reject is not a guard, it is a comment.
    pub fn ground_best_from<G: Groundable>(
        facet: &mut Facet,
        source: &G,
        anchor: f64,
        rounds: usize,
    ) -> usize {
        let entries: Vec<(String, String)> = source
            .entries()
            .into_iter()
            .filter(|(w, _)| facet.lexicon.contains_key(w))
            .map(|(w, d)| (w, crate::sources::definition_core(&d)))
            .filter(|(_, d)| !d.split_whitespace().collect::<Vec<_>>().is_empty())
            .collect();

        if entries.is_empty() {
            return 0;
        }

        let (before, before_band) = dispersion_pair(facet);
        let mut candidate = facet.clone();
        let report = Conception::compose_anchored(
            &mut candidate,
            &entries,
            rounds,
            crate::conception::HEAD_STEP,
            crate::conception::BETA_STRONG,
            crate::conception::BETA_STRONG,
            false,
            None,
            anchor,
            None,
        );
        let (after, after_band) = dispersion_pair(&candidate);

        if after < DISPERSION_FLOOR || after_band < DISPERSION_FLOOR {
            let which = match after < DISPERSION_FLOOR {
                true => "global",
                false => "top-band",
            };
            println!(
                "  [compose] REJECTED on {}: dispersion {:.3} -> {:.3}, \
                 top-{} band {:.3} -> {:.3}, floor {:.2}. \
                 Keeping the pre-composition manifold.",
                which, before, after, GUARD_BAND_TOP, before_band, after_band, DISPERSION_FLOOR
            );
            return 0;
        }

        *facet = candidate;
        println!(
            "  [compose] {} of {} definitions composed, dispersion {:.3} -> {:.3}, \
             top-{} band {:.3} -> {:.3}",
            report.heads_moved,
            entries.len(),
            before,
            after,
            GUARD_BAND_TOP,
            before_band,
            after_band
        );
        report.heads_moved
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

#[cfg(test)]
mod ground_best_tests {
    use super::*;
    use crate::config::PHASE_CHANNELS;
    use crate::phasor::SpectralPhasor;

    struct Entries(Vec<(String, String)>);
    impl Groundable for Entries {
        fn entries(&self) -> Vec<(String, String)> {
            self.0.clone()
        }
    }

    fn facet_of(words: &[&str]) -> Facet {
        let mut f = Facet::new();
        for w in words {
            f.lexicon
                .insert((*w).to_string(), SpectralPhasor::seeded(w, 1.0, 1));
        }
        f
    }

    /// The floor must actually fire, and firing must leave the caller's facet
    /// untouched rather than partially composed.
    ///
    /// A guard that has never been observed to reject is not a guard. This
    /// drives dispersion down deliberately — every word composed toward one
    /// shared definition — and asserts the rejection path both triggers and
    /// preserves.
    #[test]
    fn test_dispersion_floor_rejects_and_preserves() {
        let words = ["alpha", "beta", "gamma", "delta", "epsilon", "core"];
        let mut f = facet_of(&words);

        // Every word defined by the same single term collapses them together.
        let entries: Vec<(String, String)> = words
            .iter()
            .filter(|w| **w != "core")
            .map(|w| ((*w).to_string(), "core".to_string()))
            .collect();

        let before: Vec<Vec<f64>> = words
            .iter()
            .map(|w| (0..PHASE_CHANNELS).map(|k| f.lexicon[*w].theta(k)).collect())
            .collect();

        // Composed with no anchor and many rounds, this drives dispersion to
        // near zero, which is exactly what the floor exists to catch.
        let mut candidate = f.clone();
        Conception::compose_anchored(
            &mut candidate,
            &entries,
            12,
            crate::conception::HEAD_STEP,
            crate::conception::BETA_STRONG,
            crate::conception::BETA_STRONG,
            false,
            None,
            0.0,
            None,
        );
        assert!(
            candidate.phase_dispersion() < DISPERSION_FLOOR,
            "the fixture must actually collapse, got dispersion {}",
            candidate.phase_dispersion()
        );

        // Now through the guarded path: rejected, and nothing written back.
        let moved = DefinitionGrounder::ground_best_from(&mut f, &Entries(entries), 0.0, 12);
        assert_eq!(moved, 0, "a collapsing composition must be rejected");
        for (i, w) in words.iter().enumerate() {
            for k in 0..PHASE_CHANNELS {
                assert_eq!(
                    f.lexicon[*w].theta(k),
                    before[i][k],
                    "rejection must leave {} channel {} untouched",
                    w,
                    k
                );
            }
        }
    }

    /// A composition that keeps the manifold spread must be accepted.
    ///
    /// The vocabulary has to be large enough for dispersion to mean anything —
    /// on a handful of words the statistic is dominated by which few angles the
    /// seeds happened to land on, and the guard rejects healthy fixtures for
    /// reasons that have nothing to do with composition.
    #[test]
    fn test_healthy_composition_is_accepted() {
        // Real words, not generated names. `seeded` hashes the string, and
        // "head00".."head39" share a long prefix, which lands them in a narrow
        // band of channel 0 — a fixture that starts at dispersion 0.25 and
        // stays there tests the seeding, not the guard.
        let heads = [
            "cat", "dog", "horse", "eagle", "salmon", "oak", "rose", "copper", "granite",
            "wheat", "apple", "hammer", "chair", "violin", "wine", "bread", "cotton",
            "marble", "rifle", "sword",
        ];
        let fillers = [
            "animal", "plant", "metal", "stone", "food", "tool", "weapon", "bird", "fish",
            "tree", "flower", "grain", "fruit", "cloth", "drink", "wood", "iron", "seat",
            "music", "sharp", "heavy", "soft", "sweet", "bitter", "round", "long", "small",
            "large", "bright", "dark",
        ];
        let all: Vec<&str> = heads.iter().chain(fillers.iter()).copied().collect();
        let mut f = facet_of(&all);

        // Each head composed toward a different pair of fillers, so the
        // manifold organises without concentrating.
        let entries = Entries(
            heads
                .iter()
                .enumerate()
                .map(|(i, h)| {
                    (
                        (*h).to_string(),
                        format!("{} {}", fillers[i], fillers[(i + 11) % fillers.len()]),
                    )
                })
                .collect(),
        );

        let start_dispersion = f.phase_dispersion();
        let before = f.lexicon["cat"].theta(3);
        // A strong anchor, as the sweep measured it: alpha = 2.0 held dispersion at
        // 0.809 on the real dictionary. The accept path is what is under test
        // here, not the operating point.
        let moved = DefinitionGrounder::ground_best_from(&mut f, &entries, 2.0, 2);

        assert!(
            moved > 0,
            "a composition that keeps the manifold spread must be accepted \
             (dispersion {:.3} -> {:.3}, floor {:.2})",
            start_dispersion,
            f.phase_dispersion(),
            DISPERSION_FLOOR
        );
        assert!(f.phase_dispersion() >= DISPERSION_FLOOR);
        assert_ne!(
            f.lexicon["cat"].theta(3),
            before,
            "acceptance must write through to the caller's facet"
        );
    }
}
