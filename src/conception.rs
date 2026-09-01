//! Definitions as compositions, across every channel, in order, both ways.
//!
//! [`crate::cognitive::grounding::DefinitionGrounder`] already moves a word
//! toward its definition. Measured, it halves phase dispersion and improves no
//! relation metric — and three properties of how it does it explain why:
//!
//! 1. **One channel.** It writes `theta(0)` only. The other 63 channels of the
//!    torus never see the definition, so 63/64 of the representation is
//!    untouched by meaning.
//! 2. **No order.** The target is the centroid of the definition's content
//!    words. A centroid is symmetric, so *a member of the board of directors*
//!    and *a board of the directors member* produce the same target. Anything
//!    carried by word order — argument roles, modification, "X of Y" versus
//!    "Y of X" — is discarded before it reaches the manifold.
//! 3. **One direction.** The headword moves toward its definers; the definers
//!    never move toward the headword. So *grandmother* learns from *mother* and
//!    *parent*, but *mother* learns nothing from appearing in *grandmother*'s
//!    definition, and the family of words that define each other never
//!    converges into a region.
//!
//! This module fixes all three. Composition runs on every channel; each
//! definition word is rotated by its position before it is summed, so the
//! superposition is a binding rather than a blur; and the pull is mutual, with
//! the definers drawn a smaller step toward the headword. Words that define
//! each other reinforce each other, which is what turns a definition graph into
//! a set of concept regions rather than a set of independent placements.
//!
//! Read and write passes stay separated (Jacobi, as in the grounder), so the
//! result does not depend on the order the dictionary is enumerated in.

use crate::config::{GOLDEN_ANGLE, PHASE_CHANNELS, TWO_PI};
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use std::collections::HashMap;
use std::f64::consts::PI;

/// How far the headword moves toward its composed definition each round.
pub const HEAD_STEP: f64 = 0.5;

/// How far each definition word moves back toward the headword, as a fraction
/// of `HEAD_STEP`.
///
/// Smaller than the forward step on purpose. A definer appears in many
/// entries; if it moved as far as the headword it would be dragged apart by
/// every entry it occurs in, and common words — which occur in the most
/// definitions — would move the most while carrying the least meaning.
pub const REINFORCE: f64 = 0.15;

/// Reinforcement for a **strong** pair: each word appears in the other's
/// definition.
///
/// Dict2vec (Tissier, Gravier & Habrard, EMNLP 2017) splits definitional pairs
/// exactly here, and the split is not cosmetic. *vehicle* is in the definition
/// of *car* and *car* is in the definition of *vehicle* — that reciprocity is
/// evidence of a semantic relation. *road* is in the definition of *car* but
/// *car* is not in the definition of *road*; the relation is real but weaker,
/// and treating the two alike gives *road* the same pull as *vehicle*.
///
/// The ratio here (0.8 : 0.45) is the one their grid search selected.
pub const BETA_STRONG: f64 = 0.8;

/// Reinforcement for a **weak** pair: one-way definitional membership.
pub const BETA_WEAK: f64 = 0.45;

/// Words too common to carry the meaning of an entry.
///
/// Not a stopword list for retrieval: these are the words that appear in a
/// large fraction of *all* definitions, so binding them into a composition adds
/// the same vector to every concept and washes the manifold out.
fn is_structural(w: &str) -> bool {
    matches!(
        w,
        "a" | "an" | "the" | "of" | "or" | "and" | "to" | "in" | "is" | "are" | "as" | "that"
            | "which" | "with" | "for" | "by" | "on" | "at" | "from" | "it" | "its" | "any"
            | "one" | "used" | "being" | "having" | "esp" | "especially" | "also"
    )
}

#[derive(Debug, Clone, Default)]
pub struct CompositionReport {
    pub entries_used: usize,
    pub heads_moved: usize,
    pub definers_reinforced: usize,
    pub rounds: usize,
    /// Mean absolute phase shift of a headword in the final round, in radians.
    /// Falling toward zero across rounds means the composition has converged.
    pub final_mean_shift: f64,
}

pub struct Conception;

impl Conception {
    /// Order-bound composition of a definition, per channel.
    ///
    /// Returns one target angle per channel, or `None` when the definition has
    /// no content words the facet knows.
    ///
    /// Position `i` rotates that word by `i · φ` (the golden angle) before it
    /// joins the sum. Rotation by an irrational multiple of a turn never
    /// repeats, so no two positions collide and the sum is invertible in
    /// principle rather than a bag of words. This is the same binding
    /// [`crate::wave::Wave::sentence_channels`] applies to sentences, used here
    /// on the definition that *constitutes* the word.
    pub fn compose(facet: &Facet, definition: &str) -> Option<Vec<f64>> {
        Self::compose_with(facet, definition, true)
    }

    /// [`Conception::compose`], with positional binding switchable.
    ///
    /// `bind = false` sums the definers with no rotation: a bag of words, where
    /// the same set of definers gives the same target however the entry is
    /// phrased. `bind = true` rotates each definer by its position. Which is
    /// better is an empirical question about the *source*: position is signal
    /// in a sentence and largely phrasing in a dictionary entry, and only the
    /// measurement can say which dominates.
    pub fn compose_with(facet: &Facet, definition: &str, bind: bool) -> Option<Vec<f64>> {
        let tokens: Vec<String> = Tokenizer::tokenize(definition)
            .into_iter()
            .filter(|w| !is_structural(w))
            .collect();

        let mut acc = vec![(0.0f64, 0.0f64); PHASE_CHANNELS];
        let mut used = 0usize;

        for (i, w) in tokens.iter().enumerate() {
            let p = match facet.lexicon.get(w) {
                Some(p) => p,
                None => continue,
            };
            used += 1;
            let roll = match bind {
                true => i as f64 * GOLDEN_ANGLE,
                false => 0.0,
            };
            // Amplitude is log-frequency, so a rare, specific definer already
            // counts for less than a common one. Inverting it here would let
            // hapax typos dominate an entry.
            let m = p.amplitude.max(1e-6);
            for k in 0..PHASE_CHANNELS {
                let t = p.theta(k) + roll;
                acc[k].0 += m * t.cos();
                acc[k].1 += m * t.sin();
            }
        }

        if used == 0 {
            return None;
        }

        Some(
            acc.iter()
                .map(|(x, y)| match x.hypot(*y) > 1e-12 {
                    true => y.atan2(*x),
                    // A channel whose definers cancelled exactly carries no
                    // information; leaving it at 0 would be a claim, so it is
                    // flagged with NaN and skipped by the writer.
                    false => f64::NAN,
                })
                .collect(),
        )
    }

    /// Shortest signed angle from `from` to `to`.
    #[inline]
    fn delta(from: f64, to: f64) -> f64 {
        let mut d = to - from;
        while d > PI {
            d -= TWO_PI;
        }
        while d < -PI {
            d += TWO_PI;
        }
        d
    }

    /// Composes every entry into the manifold, `rounds` times.
    ///
    /// Definitions are recursive — *grandmother* is defined through *mother*,
    /// which is defined through *parent* — so one pass propagates meaning
    /// exactly one hop and the process is iterated. The 0.5 step halves the
    /// remaining error each round, so this converges rather than oscillating.
    pub fn compose_all(
        facet: &mut Facet,
        entries: &[(String, String)],
        rounds: usize,
    ) -> CompositionReport {
        Self::compose_all_with(facet, entries, rounds, HEAD_STEP, REINFORCE)
    }

    /// [`Conception::compose_all`] with explicit step sizes.
    ///
    /// `reinforce = 0.0` turns the mutual pull off, which is the control that
    /// isolates what reinforcement contributes from what multi-channel
    /// order-bound composition contributes.
    pub fn compose_all_with(
        facet: &mut Facet,
        entries: &[(String, String)],
        rounds: usize,
        head_step: f64,
        reinforce: f64,
    ) -> CompositionReport {
        Self::compose_all_bound(facet, entries, rounds, head_step, reinforce, true)
    }

    /// [`Conception::compose_all_with`], with positional binding switchable.
    pub fn compose_all_bound(
        facet: &mut Facet,
        entries: &[(String, String)],
        rounds: usize,
        head_step: f64,
        reinforce: f64,
        bind: bool,
    ) -> CompositionReport {
        Self::compose_graded(facet, entries, rounds, head_step, reinforce, reinforce, bind, None)
    }

    /// [`Conception::compose_all_bound`] with the reinforcement pull graded by
    /// pair strength.
    ///
    /// `graph = Some(..)` splits each definer into a strong pair (each word in
    /// the other's definition) or a weak one (one-way), and pulls them at
    /// `strong` and `weak` respectively. `graph = None` applies `strong` to
    /// everything, which is the flat behaviour and the control the grading has
    /// to beat.
    #[allow(clippy::too_many_arguments)]
    pub fn compose_graded(
        facet: &mut Facet,
        entries: &[(String, String)],
        rounds: usize,
        head_step: f64,
        strong: f64,
        weak: f64,
        bind: bool,
        graph: Option<&DefinitionGraph>,
    ) -> CompositionReport {
        let mut report = CompositionReport { rounds, ..Default::default() };
        if entries.is_empty() {
            return report;
        }

        for _ in 0..rounds {
            // ---- READ: every target computed against frozen phases ----
            let mut head_targets: Vec<(String, Vec<f64>)> = Vec::with_capacity(entries.len());
            // Definer pulls accumulate as unit vectors per channel, so a word
            // appearing in many definitions ends up at their circular mean
            // rather than at whichever entry happened to be written last.
            let mut definer_pull: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
            let mut definer_hits: HashMap<String, u32> = HashMap::new();

            for (word, def) in entries {
                if !facet.lexicon.contains_key(word) {
                    continue;
                }
                let target = match Self::compose_with(facet, def, bind) {
                    Some(t) => t,
                    None => continue,
                };

                if strong > 0.0 || weak > 0.0 {
                    if let Some(head) = facet.lexicon.get(word) {
                        for t in Tokenizer::tokenize(def) {
                            if is_structural(&t) || t == *word {
                                continue;
                            }
                            if !facet.lexicon.contains_key(&t) {
                                continue;
                            }
                            // Pull weight is the pair's strength, and it enters
                            // as the vector's magnitude so a strong pair counts
                            // for more in the circular mean as well as moving
                            // further.
                            let w = match graph {
                                Some(g) if !g.is_strong(word, &t) => weak,
                                Some(_) => strong,
                                None => strong,
                            };
                            if w <= 0.0 {
                                continue;
                            }
                            *definer_hits.entry(t.clone()).or_insert(0) += 1;
                            let e = definer_pull
                                .entry(t)
                                .or_insert_with(|| vec![(0.0, 0.0); PHASE_CHANNELS]);
                            for k in 0..PHASE_CHANNELS {
                                let a = head.theta(k);
                                e[k].0 += w * a.cos();
                                e[k].1 += w * a.sin();
                            }
                        }
                    }
                }

                head_targets.push((word.clone(), target));
            }
            report.entries_used = head_targets.len();

            // ---- WRITE: heads first, then the reinforcement pull ----
            let mut moved = 0usize;
            let mut shift_sum = 0.0f64;
            let mut shift_n = 0usize;

            for (word, target) in &head_targets {
                if let Some(p) = facet.lexicon.get_mut(word) {
                    for k in 0..PHASE_CHANNELS {
                        if !target[k].is_finite() {
                            continue;
                        }
                        let cur = p.theta(k);
                        let step = head_step * Self::delta(cur, target[k]);
                        p.set_theta(k, cur + step);
                        shift_sum += step.abs();
                        shift_n += 1;
                    }
                    p.sync_phase();
                    moved += 1;
                }
            }
            report.heads_moved = moved;
            report.final_mean_shift = match shift_n {
                0 => 0.0,
                n => shift_sum / n as f64,
            };

            let mut reinforced = 0usize;
            for (word, pulls) in &definer_pull {
                let count = definer_hits.get(word).copied().unwrap_or(1) as f64;
                if let Some(p) = facet.lexicon.get_mut(word) {
                    for k in 0..PHASE_CHANNELS {
                        let (x, y) = pulls[k];
                        if x.hypot(y) < 1e-12 {
                            continue;
                        }
                        let cur = p.theta(k);
                        // The accumulated magnitude already carries the pair
                        // weights; the step is scaled by the mean weight so a
                        // word pulled only by weak pairs moves less than one
                        // pulled by strong ones.
                        let scale = (x.hypot(y) / count.max(1.0)).min(1.0);
                        let step = head_step * scale * Self::delta(cur, y.atan2(x));
                        p.set_theta(k, cur + step);
                    }
                    p.sync_phase();
                    reinforced += 1;
                }
            }
            report.definers_reinforced = reinforced;
        }

        report
    }
}

/// Which words define which, and how strongly.
///
/// A dictionary is a directed graph: an edge `a → b` means *b* occurs in the
/// definition of *a*. Two facts fall out of the direction, and both are used:
///
/// * A **strong pair** is a two-way edge. Each word occurs in the other's
///   definition, which is much stronger evidence of a semantic relation than
///   one-way membership — *car*/*vehicle*, not *car*/*road*.
/// * Any edge at all, in either direction, means the two words must **never be
///   drawn as a negative sample** for each other. Contrastive training pushes a
///   word away from its negatives; sampling uniformly from the vocabulary will
///   occasionally draw a word from the target's own definition and push apart
///   precisely the pair the definition says belongs together. Dict2vec calls
///   the fix controlled negative sampling and measures it discarding ~2% of
///   generated negatives — small in count, and it is undoing the training
///   signal every time it fires.
#[derive(Debug, Clone, Default)]
pub struct DefinitionGraph {
    edges: HashMap<String, std::collections::HashSet<String>>,
}

impl DefinitionGraph {
    pub fn build(entries: &[(String, String)]) -> Self {
        let mut edges: HashMap<String, std::collections::HashSet<String>> = HashMap::new();
        for (word, def) in entries {
            let set = edges.entry(word.clone()).or_default();
            for t in Tokenizer::tokenize(def) {
                if !is_structural(&t) && t != *word {
                    set.insert(t);
                }
            }
        }
        Self { edges }
    }

    /// Words appearing in `word`'s definition.
    pub fn definers(&self, word: &str) -> Option<&std::collections::HashSet<String>> {
        self.edges.get(word)
    }

    /// True when each word occurs in the other's definition.
    pub fn is_strong(&self, a: &str, b: &str) -> bool {
        self.contains(a, b) && self.contains(b, a)
    }

    /// True when the two are definitionally related in either direction — the
    /// test a negative sample has to fail.
    pub fn is_related(&self, a: &str, b: &str) -> bool {
        self.contains(a, b) || self.contains(b, a)
    }

    fn contains(&self, of: &str, word: &str) -> bool {
        self.edges.get(of).is_some_and(|s| s.contains(word))
    }

    /// `(strong, weak)` edge counts. Reported so the ratio can be sanity-checked
    /// against the source: dict2vec extracted 417K strong to 3.9M weak, roughly
    /// 1:9, from 200K definitions.
    pub fn counts(&self) -> (usize, usize) {
        let (mut strong, mut weak) = (0usize, 0usize);
        for (a, defs) in &self.edges {
            for b in defs {
                match self.contains(b, a) {
                    true => strong += 1,
                    false => weak += 1,
                }
            }
        }
        (strong / 2, weak)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phasor::SpectralPhasor;

    fn facet_with(words: &[&str]) -> Facet {
        let mut f = Facet::new();
        for w in words {
            f.lexicon
                .insert((*w).to_string(), SpectralPhasor::seeded(w, 1.0, 1));
        }
        f
    }

    /// The whole point of positional rotation: a definition is not a bag.
    #[test]
    fn test_composition_is_order_sensitive() {
        let f = facet_with(&["board", "directors", "member", "group"]);
        let a = Conception::compose(&f, "member board directors group").unwrap();
        let b = Conception::compose(&f, "group directors board member").unwrap();
        let differing = (0..PHASE_CHANNELS)
            .filter(|k| (a[*k] - b[*k]).abs() > 1e-6)
            .count();
        assert!(
            differing > PHASE_CHANNELS / 2,
            "reordering a definition must move most channels, moved {}",
            differing
        );
    }

    /// Every channel must carry the definition, not just channel 0.
    #[test]
    fn test_composition_writes_every_channel() {
        let mut f = facet_with(&["grandmother", "mother", "parent", "female"]);
        let before: Vec<f64> = (0..PHASE_CHANNELS)
            .map(|k| f.lexicon["grandmother"].theta(k))
            .collect();

        Conception::compose_all(
            &mut f,
            &[("grandmother".into(), "mother of a parent female".into())],
            1,
        );

        let after: Vec<f64> = (0..PHASE_CHANNELS)
            .map(|k| f.lexicon["grandmother"].theta(k))
            .collect();
        let moved = (0..PHASE_CHANNELS)
            .filter(|k| (before[*k] - after[*k]).abs() > 1e-6)
            .count();
        assert!(
            moved > PHASE_CHANNELS / 2,
            "composition touched only {} of {} channels — this is the \
             single-channel limitation it exists to remove",
            moved,
            PHASE_CHANNELS
        );
    }

    /// Reinforcement must move the definers, and must be the only thing that
    /// does — otherwise the control that isolates it is not a control.
    #[test]
    fn test_reinforcement_moves_definers_and_is_switchable() {
        let entries = vec![("grandmother".to_string(), "mother of a parent".to_string())];

        let mut off = facet_with(&["grandmother", "mother", "parent"]);
        let before = off.lexicon["mother"].theta(3);
        Conception::compose_all_with(&mut off, &entries, 1, HEAD_STEP, 0.0);
        assert!(
            (off.lexicon["mother"].theta(3) - before).abs() < 1e-9,
            "with reinforcement off, a definer must not move"
        );

        let mut on = facet_with(&["grandmother", "mother", "parent"]);
        Conception::compose_all_with(&mut on, &entries, 1, HEAD_STEP, REINFORCE);
        assert!(
            (on.lexicon["mother"].theta(3) - before).abs() > 1e-9,
            "with reinforcement on, a definer must move toward the headword"
        );
    }

    /// Strong and weak pairs must be told apart by reciprocity, not by
    /// co-occurrence, and both must be excluded from negative sampling.
    #[test]
    fn test_definition_graph_strong_weak_and_relatedness() {
        let entries = vec![
            ("car".to_string(), "road vehicle with an engine".to_string()),
            ("vehicle".to_string(), "a car or truck".to_string()),
            ("road".to_string(), "a wide way between places".to_string()),
        ];
        let g = DefinitionGraph::build(&entries);

        // car defines vehicle and vehicle defines car: reciprocal.
        assert!(g.is_strong("car", "vehicle"));
        // car defines road, road does not define car: one-way.
        assert!(!g.is_strong("car", "road"));
        // but still related, so still barred from being a negative sample.
        assert!(g.is_related("car", "road"));
        assert!(!g.is_related("car", "places"));
    }

    /// Grading by pair strength must actually move strong and weak definers by
    /// different amounts — otherwise it is the flat rule under another name.
    #[test]
    fn test_strong_pairs_move_further_than_weak() {
        let entries = vec![
            ("car".to_string(), "road vehicle".to_string()),
            ("vehicle".to_string(), "a car".to_string()),
            ("road".to_string(), "a wide way".to_string()),
        ];
        let g = DefinitionGraph::build(&entries);
        assert!(g.is_strong("car", "vehicle") && !g.is_strong("car", "road"));

        let start = facet_with(&["car", "vehicle", "road", "wide", "way"]);

        let mut graded = start.clone();
        Conception::compose_graded(
            &mut graded,
            &entries,
            1,
            HEAD_STEP,
            BETA_STRONG,
            BETA_WEAK,
            false,
            Some(&g),
        );

        let shift = |a: &Facet, b: &Facet, w: &str| {
            (0..PHASE_CHANNELS)
                .map(|k| Conception::delta(a.lexicon[w].theta(k), b.lexicon[w].theta(k)).abs())
                .sum::<f64>()
        };

        assert!(
            shift(&start, &graded, "vehicle") > shift(&start, &graded, "road"),
            "the strong definer must be pulled further than the weak one"
        );
    }

    /// Iterating must converge, not oscillate: the mean step has to shrink.
    #[test]
    fn test_composition_converges() {
        let mut f = facet_with(&["cat", "animal", "small", "domestic", "feline"]);
        let entries = vec![
            ("cat".to_string(), "small domestic feline animal".to_string()),
            ("feline".to_string(), "of the cat animal".to_string()),
        ];
        let first = Conception::compose_all(&mut f, &entries, 1).final_mean_shift;
        let later = Conception::compose_all(&mut f, &entries, 6).final_mean_shift;
        assert!(
            later < first,
            "mean shift must fall across rounds: {} then {}",
            first,
            later
        );
    }
}
