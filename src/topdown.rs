//! The downward path: sentences constraining the words they are made of.
//!
//! Phiano is bottom-up in its entirety. Words are trained by next-word ranking;
//! a sentence is a sum of its words; a paragraph is a sum of its sentences. At
//! no point does a level above constrain the level below. That is the whole
//! architecture in one line, and it is why "a word is a hair, the sentence is
//! the coat" is currently a claim the code does not implement — the coat here
//! is defined as the sum of its hairs and has no say in where they lie.
//!
//! This module adds the missing direction. One *cycle* is:
//!
//! ```text
//! UP    sentence state  s_j = Σ  e^{i(θ_k(w) + pos·ρ_k)}   over words w in sentence j
//! DOWN  each word w  ←  rotated toward the mean of the sentences containing it
//! ```
//!
//! and the cycle is iterated, because a word moved on round one changes the
//! sentence states that move it on round two.
//!
//! # Three things that would make this measure nothing, and what is done about them
//!
//! **Self-reinforcement.** A word is part of the state it is being pulled
//! toward. Left alone, every word is partly pulled toward itself, the objective
//! improves, and nothing has been learned. Every target here is computed
//! *leave-one-out*: the word's own contribution is subtracted from its
//! sentence's state before the state is used to move it. This is why the
//! encoding is a superposition rather than the recurrence — subtraction has to
//! be exact.
//!
//! **Collapse.** "Pull each word toward the mean of its contexts" is an
//! attraction rule with a known fixed point: every word at one angle. Function
//! words are the accelerant, since `the` occurs in nearly every sentence and so
//! its target *is* the global mean. Two defences. First, [`Descent::contrast`]
//! subtracts the corpus-mean sentence state, so a word is pulled toward what its
//! contexts have *distinctively* rather than toward what all contexts share;
//! under contrast a word that occurs everywhere has a target of near-zero
//! magnitude and does not move. This is inverse document frequency, written in
//! phase. Second, the result is checked against the band-aware dispersion guard
//! and discarded if the frequent band has concentrated.
//!
//! **Measuring at the level you intervened on.** Constraining words from
//! sentences and then reporting a sentence score is close to circular. The
//! question worth asking is whether the downward path improves the level
//! *below* it — whether word-level relational structure gets better when
//! sentences are allowed to have an opinion. Both are reported; the word-level
//! one is the claim.
//!
//! # What it measured
//!
//! Three seeds on the 120,000-sentence dictionary corpus, word-level relational
//! scores at a frequency floor of 25 (pool 3,654 words, chance analogy MRR
//! 0.00027), mean +/- sd:
//!
//! ```text
//! arm                     analogy MRR        nbr@10   pair>random
//! bottom-up only      0.00247 +/- 0.00018      1.95%        60.7%
//! descent (bag)       0.00814 +/- 0.00072      5.22%        72.5%
//! descent (bound)     0.00558 +/- 0.00099      1.43%        65.7%
//! descent SHUFFLED    0.00206 +/- 0.00008      0.28%        55.7%
//! ```
//!
//! The downward pass is 3.3x the bottom-up control and 4.0x its own null, with
//! non-overlapping intervals on every column. The null is the load-bearing one:
//! it lands *below* the control, so the same update rule applied to
//! structureless sentences makes the manifold worse. The gain is attributable to
//! which words occur together, not to the fact that something moved.
//!
//! Two things this did not do. It did not help at the sentence level — every arm
//! sits at 1.01x chance on held-out next-sentence selection and loses to lexical
//! overlap, including the arms that improved the words. Constraint from above
//! improved the level below without improving the level it came from, which is
//! not the direction the hypothesis predicted. And the order-sensitive encoding
//! (`bound`) is consistently worse than the order-free one, matching every
//! earlier result in this project where positional rotation was tried.
//!
//! The no-contrast arm was rejected by the collapse guard on all three seeds, at
//! band dispersion 0.109-0.126 with *global* dispersion 0.534 — above the 0.40
//! floor. The global-only guard this project shipped until today would have
//! accepted a collapsed manifold three times out of three.

use crate::config::{PHASE_CHANNELS, TWO_PI};
use crate::facet::Facet;
use crate::phasor::SpectralPhasor;
use crate::wave::c64;
use std::collections::HashMap;

/// Fraction of the way to its target a word moves per round.
///
/// Full steps make the cycle a fixed-point iteration on an attraction map,
/// which is the collapse the guard exists to catch. A partial step is the
/// anchor: the word keeps most of what the bottom-up pass gave it.
pub const DEFAULT_STEP: f64 = 0.25;

/// Rounds of the up-down cycle.
pub const DEFAULT_ROUNDS: usize = 4;

/// Minimum mean resultant length of a word's contexts for it to move.
///
/// A word whose sentences point in every direction has a short resultant, and
/// its argument is the angle of a rounding error. Words below this keep the
/// position bottom-up training gave them, which is the honest default: no
/// evidence, no move.
pub const DEFAULT_CONCENTRATION: f64 = 0.05;

/// Multiplier on the chance resultant, `sqrt(pi) / (2 * sqrt(n))`.
///
/// `n` unit vectors drawn uniformly at random have that expected resultant
/// length — 0.44 at four contexts, 0.09 at a hundred. A word must beat its own
/// chance level by this factor to move, so "agreement" means agreement above
/// what its context count produces for free. Without it the threshold is
/// backwards: rare words clear a fixed bar on noise and frequent ones, whose
/// evidence is real, fall under it.
pub const RAYLEIGH_C: f64 = 1.3;

/// How far a word's contexts must differ from the average context to move it.
///
/// Expressed as a fraction of the average context's own magnitude, so it is
/// scale-free. Below it, the word's contexts *are* the corpus average and the
/// residual's argument is a rounding error — which is the case for exactly the
/// words that would drag the manifold to a point.
pub const CONTRAST_MIN: f64 = 0.10;

/// Positional rotation per channel, for the order-sensitive encoding.
///
/// Shared across words, so position is a direction rather than a scramble — the
/// same reasoning as role rotation in [`crate::roles`].
#[inline]
fn position_rotation(k: usize) -> f64 {
    let p = crate::config::CHANNEL_PRIMES[k % crate::config::CHANNEL_PRIMES.len()] as f64;
    TWO_PI * p / crate::config::PRIME_MODULUS
}

/// How a sentence is summed from its words during the upward pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Up {
    /// Plain superposition. Order-free; the control.
    Bag,
    /// Each word rotated by its position. Order-sensitive, and still exactly
    /// subtractive, which leave-one-out requires.
    Bound,
}

impl Up {
    pub fn label(self) -> &'static str {
        match self {
            Up::Bag => "bag",
            Up::Bound => "bound",
        }
    }
}

/// What one cycle did, in the terms needed to decide whether to keep it.
#[derive(Debug, Clone)]
pub struct DescentReport {
    pub rounds: usize,
    pub sentences: usize,
    /// Words that had enough contextual agreement to move, on the last round.
    pub words_moved: usize,
    /// Words in the lexicon that occur in at least one sentence.
    pub words_seen: usize,
    pub dispersion_before: f64,
    pub dispersion_after: f64,
    pub band_before: f64,
    pub band_after: f64,
    /// True when the guard rejected the result and the facet was left alone.
    pub rejected: bool,
}

pub struct Descent;

impl Descent {
    /// Contribution of one word at one position to its sentence's state.
    #[inline]
    fn contribution(p: &SpectralPhasor, pos: usize, k: usize, up: Up) -> c64 {
        let rot = match up {
            Up::Bag => 0.0,
            Up::Bound => pos as f64 * position_rotation(k),
        };
        c64::from_polar(p.amplitude, p.theta(k) + rot)
    }

    /// Runs the up-down cycle and keeps the result only if the guard allows it.
    ///
    /// `contrast` subtracts the corpus-mean sentence state from every target.
    /// It is a parameter rather than a constant because "does contrast prevent
    /// the collapse" is the question this module exists to answer, and an
    /// answer needs both arms.
    pub fn cycle(
        facet: &mut Facet,
        sentences: &[Vec<String>],
        up: Up,
        rounds: usize,
        step: f64,
        contrast: bool,
    ) -> DescentReport {
        Self::cycle_with(facet, sentences, up, rounds, step, contrast, DEFAULT_CONCENTRATION)
    }

    /// [`Descent::cycle`] with the concentration threshold exposed.
    pub fn cycle_with(
        facet: &mut Facet,
        sentences: &[Vec<String>],
        up: Up,
        rounds: usize,
        step: f64,
        contrast: bool,
        concentration: f64,
    ) -> DescentReport {
        let (dispersion_before, band_before) = crate::cognitive::grounding::dispersion_pair(facet);

        // Only sentences whose words the lexicon knows contribute anything, and
        // a sentence of one known word has an empty leave-one-out state.
        let usable: Vec<Vec<(String, usize)>> = sentences
            .iter()
            .map(|s| {
                s.iter()
                    .enumerate()
                    .filter(|(_, w)| facet.lexicon.contains_key(*w))
                    .map(|(i, w)| (w.clone(), i))
                    .collect::<Vec<_>>()
            })
            .filter(|s: &Vec<(String, usize)>| s.len() >= 2)
            .collect();

        let words_seen = {
            let mut set = std::collections::HashSet::new();
            for s in &usable {
                for (w, _) in s {
                    set.insert(w.as_str());
                }
            }
            set.len()
        };

        let mut candidate = facet.clone();
        let mut words_moved = 0usize;

        for _ in 0..rounds.max(1) {
            // ---- UP ----
            let states: Vec<Vec<c64>> = usable
                .iter()
                .map(|sent| {
                    let mut h = vec![c64::new(0.0, 0.0); PHASE_CHANNELS];
                    for (w, pos) in sent {
                        let p = &candidate.lexicon[w];
                        for (k, z) in h.iter_mut().enumerate() {
                            *z += Self::contribution(p, *pos, k, up);
                        }
                    }
                    h
                })
                .collect();

            // ---- DOWN ----
            //
            // Leave-one-out is per *type*, not per token: all occurrences of the
            // word are subtracted from its sentence's state. Subtracting only
            // the token being scored leaves the word's other occurrences in the
            // thing that moves it, which is the self-reinforcement this is
            // supposed to rule out — `the cat sat on the mat` would pull `the`
            // toward `the`.
            //
            // Everything below is accumulated linearly and length-normalised, so
            // that a word's own phase can be removed from the *corpus* average
            // exactly rather than approximately. That matters more than it
            // sounds: `the` sits inside the leave-one-out state of every other
            // word in its sentences, so any corpus average built without
            // removing it contains it, and subtracting such an average pushes
            // the word away from itself — a spreading force in semantic
            // costume. Two earlier formulations of the contrast failed exactly
            // there, each moving `the` further than a topical word instead of
            // less.
            let mut total = vec![c64::new(0.0, 0.0); PHASE_CHANNELS];
            // Per word: Σ its own contribution, Σ its leave-one-out contexts,
            // Σ those contexts unit-normalised (for the agreement test), and the
            // number of sentences it appeared in.
            let mut own_sum: HashMap<&str, Vec<c64>> = HashMap::new();
            let mut loo_sum: HashMap<&str, Vec<c64>> = HashMap::new();
            let mut unit_sum: HashMap<&str, Vec<c64>> = HashMap::new();
            let mut counts: HashMap<&str, usize> = HashMap::new();

            let zeros = || vec![c64::new(0.0, 0.0); PHASE_CHANNELS];

            for (sent, st) in usable.iter().zip(states.iter()) {
                let inv = 1.0 / sent.len() as f64;
                for (k, z) in st.iter().enumerate() {
                    total[k] += *z * inv;
                }

                let mut by_word: Vec<(&str, Vec<usize>)> = Vec::new();
                for (w, pos) in sent {
                    match by_word.iter_mut().find(|(k, _)| *k == w.as_str()) {
                        Some((_, v)) => v.push(*pos),
                        None => by_word.push((w.as_str(), vec![*pos])),
                    }
                }

                for (w, positions) in by_word {
                    let p = &candidate.lexicon[w];
                    let own: Vec<c64> = (0..PHASE_CHANNELS)
                        .map(|k| {
                            positions
                                .iter()
                                .map(|pos| Self::contribution(p, *pos, k, up))
                                .sum::<c64>()
                                * inv
                        })
                        .collect();

                    let o = own_sum.entry(w).or_insert_with(zeros);
                    for k in 0..PHASE_CHANNELS {
                        o[k] += own[k];
                    }

                    // A sentence made only of this word says nothing about it
                    // once the word is removed — its own contribution still has
                    // to come out of the corpus total above, but it casts no
                    // vote below.
                    let loo: Vec<c64> =
                        (0..PHASE_CHANNELS).map(|k| st[k] * inv - own[k]).collect();
                    if !loo.iter().any(|z| z.norm() > 1e-12) {
                        continue;
                    }

                    let l = loo_sum.entry(w).or_insert_with(zeros);
                    for k in 0..PHASE_CHANNELS {
                        l[k] += loo[k];
                    }
                    let u = unit_sum.entry(w).or_insert_with(zeros);
                    for k in 0..PHASE_CHANNELS {
                        if loo[k].norm() > 1e-12 {
                            u[k] += loo[k] / loo[k].norm();
                        }
                    }
                    *counts.entry(w).or_insert(0) += 1;
                }
            }

            let n_sent = usable.len().max(1) as f64;

            // Applied in sorted order so the result does not depend on the
            // hash map's enumeration. The updates are independent, but the
            // reproducibility test is cheap to keep true.
            let mut keys: Vec<&str> = counts.keys().copied().collect();
            keys.sort_unstable();

            words_moved = 0;
            let mut updates: Vec<(String, Vec<f64>)> = Vec::new();
            for w in keys {
                let n = counts[w] as f64;
                let loo = &loo_sum[w];
                let unit = &unit_sum[w];
                let own = &own_sum[w];
                let p = &candidate.lexicon[w];
                let mut moved = false;
                let mut thetas: Vec<f64> = (0..PHASE_CHANNELS).map(|k| p.theta(k)).collect();

                // A word with few contexts has a high resultant by accident:
                // n unit vectors drawn uniformly have expected resultant
                // sqrt(pi)/(2*sqrt(n)), which is 0.44 at n = 4. A fixed
                // threshold therefore lets rare words move on noise while
                // holding frequent ones still — exactly backwards. The floor is
                // the chance resultant for that word's own context count.
                let agree_floor = concentration.max(RAYLEIGH_C / n.max(1.0).sqrt());

                for k in 0..PHASE_CHANNELS {
                    // Do this word's contexts agree with each other at all?
                    if unit[k].norm() / n < agree_floor {
                        continue;
                    }

                    // Where its contexts point, and where *every* context
                    // points with this word taken out of them. Both are averages
                    // of the same quantity — a length-normalised sentence state
                    // minus this word — so the difference is exactly "what this
                    // word's contexts have that the average context does not".
                    // A word present in every sentence has the two averages
                    // equal by construction and a residual of zero.
                    let d = loo[k] / n;
                    let m = (total[k] - own[k]) / n_sent;
                    let t = match contrast {
                        true => d - m,
                        false => d,
                    };

                    // Under contrast, a word whose contexts are the corpus
                    // average has nothing left to move toward, and the argument
                    // of a residual that small is a rounding error.
                    if contrast && t.norm() < CONTRAST_MIN * m.norm() {
                        continue;
                    }
                    if t.norm() < 1e-12 {
                        continue;
                    }

                    let delta = (t.arg() - thetas[k] + std::f64::consts::PI).rem_euclid(TWO_PI)
                        - std::f64::consts::PI;
                    thetas[k] = (thetas[k] + step * delta).rem_euclid(TWO_PI);
                    moved = true;
                }
                if moved {
                    words_moved += 1;
                    updates.push((w.to_string(), thetas));
                }
            }

            for (w, thetas) in updates {
                if let Some(p) = candidate.lexicon.get_mut(&w) {
                    for (k, t) in thetas.iter().enumerate() {
                        p.set_theta(k, *t);
                    }
                    p.sync_phase();
                }
            }
        }

        let (dispersion_after, band_after) = crate::cognitive::grounding::dispersion_pair(&candidate);
        let floor = crate::cognitive::grounding::DISPERSION_FLOOR;
        let rejected = dispersion_after < floor || band_after < floor;

        if !rejected {
            *facet = candidate;
        }

        DescentReport {
            rounds: rounds.max(1),
            sentences: usable.len(),
            words_moved,
            words_seen,
            dispersion_before,
            dispersion_after,
            band_before,
            band_after,
            rejected,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sentences(raw: &[&str]) -> Vec<Vec<String>> {
        raw.iter()
            .map(|s| s.split_whitespace().map(|w| w.to_string()).collect())
            .collect()
    }

    fn facet_over(sents: &[Vec<String>]) -> Facet {
        let mut f = Facet::new();
        for s in sents {
            for w in s {
                f.get_or_init(w);
            }
        }
        f
    }

    /// Two disjoint topics should separate: words that co-occur move together,
    /// words that never co-occur do not.
    #[test]
    fn test_descent_pulls_co_occurring_words_together() {
        let sents = sentences(&[
            "cat dog animal fur",
            "dog animal fur paw",
            "cat animal paw fur",
            "engine piston fuel valve",
            "piston fuel valve shaft",
            "engine fuel shaft valve",
        ]);
        let mut f = facet_over(&sents);

        let before = f.resonance("cat", "dog");
        let before_cross = f.resonance("cat", "engine");

        let r = Descent::cycle(&mut f, &sents, Up::Bag, 6, 0.4, false);
        assert!(!r.rejected, "guard rejected: band {:.3}", r.band_after);
        assert!(r.words_moved > 0, "nothing moved");

        let after = f.resonance("cat", "dog");
        let after_cross = f.resonance("cat", "engine");

        assert!(
            after - before > after_cross - before_cross,
            "within-topic resonance should rise more than across-topic: \
             cat/dog {before:.3}->{after:.3}, cat/engine {before_cross:.3}->{after_cross:.3}"
        );
    }

    /// The word must not be part of what moves it — including its *other*
    /// occurrences.
    ///
    /// `the cat sat on the mat` contains `the` twice. Subtracting only the token
    /// being scored leaves the second `the` inside the state that moves the
    /// first, so the word is pulled toward itself and the cycle reports progress
    /// on a corpus that says nothing about it. This fixture is the pure case:
    /// every sentence is one word repeated, so a correct leave-one-out has
    /// nothing left and nothing may move.
    #[test]
    fn test_leave_one_out_is_per_type_not_per_token() {
        let sents = sentences(&["alpha alpha alpha", "alpha alpha", "alpha alpha alpha alpha"]);
        let mut f = facet_over(&sents);
        let before: Vec<f64> = (0..PHASE_CHANNELS).map(|k| f.lexicon["alpha"].theta(k)).collect();
        let r = Descent::cycle(&mut f, &sents, Up::Bag, 4, 0.5, false);
        assert_eq!(r.words_moved, 0, "a corpus of one repeated word says nothing");
        for k in 0..PHASE_CHANNELS {
            assert!((f.lexicon["alpha"].theta(k) - before[k]).abs() < 1e-12);
        }
    }

    /// Contrast holds still the word that is everywhere, and lets the word that
    /// is somewhere move.
    ///
    /// Two disjoint topics; `the` occurs in every sentence of both, so its
    /// contexts average to the corpus mean and subtracting that leaves nothing.
    /// A topic word's contexts average to *its topic's* mean, which the corpus
    /// mean does not cancel. Asserting only that `the` moves less would pass for
    /// a rule that froze everything, so both halves are checked.
    #[test]
    fn test_contrast_holds_the_ubiquitous_word_and_frees_the_topical_one() {
        let mut sents: Vec<Vec<String>> = Vec::new();
        for i in 0..30 {
            let a = ["a1", "a2", "a3", "a4"];
            let b = ["b1", "b2", "b3", "b4"];
            sents.push(
                ["the", a[i % 4], a[(i + 1) % 4], a[(i + 2) % 4]]
                    .iter()
                    .map(|w| w.to_string())
                    .collect(),
            );
            sents.push(
                ["the", b[i % 4], b[(i + 1) % 4], b[(i + 2) % 4]]
                    .iter()
                    .map(|w| w.to_string())
                    .collect(),
            );
        }

        let base = facet_over(&sents);
        let moved = |f: &Facet, w: &str| -> f64 {
            (0..PHASE_CHANNELS)
                .map(|k| angular_distance(f.lexicon[w].theta(k), base.lexicon[w].theta(k)))
                .sum::<f64>()
                / PHASE_CHANNELS as f64
        };

        let mut plain = base.clone();
        let r_plain = Descent::cycle(&mut plain, &sents, Up::Bag, 4, 0.5, false);

        let mut contrasted = base.clone();
        let r_contrast = Descent::cycle(&mut contrasted, &sents, Up::Bag, 4, 0.5, true);

        // Without contrast this corpus collapses and the guard throws the
        // result away — which is the finding, not a test artefact. `the` is in
        // every sentence, so every word's target is dominated by the one thing
        // they all share.
        assert!(
            r_plain.rejected,
            "expected collapse without contrast; band {:.3} -> {:.3}",
            r_plain.band_before, r_plain.band_after
        );
        assert!(
            !r_contrast.rejected,
            "contrast should survive the guard; band {:.3} -> {:.3}",
            r_contrast.band_before, r_contrast.band_after
        );

        // And what survives has the right shape: the word that is everywhere
        // stays put, the word that belongs to one topic moves.
        assert!(
            moved(&contrasted, "a1") > 4.0 * moved(&contrasted, "the"),
            "a topic word should move and the ubiquitous one should not: \
             a1 {:.4}, the {:.4}",
            moved(&contrasted, "a1"),
            moved(&contrasted, "the")
        );
    }

    fn angular_distance(a: f64, b: f64) -> f64 {
        ((a - b + std::f64::consts::PI).rem_euclid(TWO_PI) - std::f64::consts::PI).abs()
    }

    /// The guard has to be able to reject, or it is a comment.
    ///
    /// A closed vocabulary where every word co-occurs with every other is the
    /// pure collapse case: each word's contexts average to the same global mean,
    /// so a full-step attraction cycle takes the whole lexicon to one angle.
    /// This is not a contrived corner — it is the fixed point the downward pass
    /// has by construction, and the reason it needs a guard at all.
    #[test]
    fn test_the_guard_rejects_a_collapsing_descent() {
        let vocab: Vec<String> = (0..40).map(|i| format!("w{i}")).collect();
        let mut sents: Vec<Vec<String>> = Vec::new();
        let mut r: u64 = 0x243F_6A88_85A3_08D3;
        for _ in 0..400 {
            let mut s: Vec<String> = Vec::new();
            while s.len() < 6 {
                r ^= r << 13;
                r ^= r >> 7;
                r ^= r << 17;
                let w = vocab[(r % vocab.len() as u64) as usize].clone();
                if !s.contains(&w) {
                    s.push(w);
                }
            }
            sents.push(s);
        }

        let mut f = facet_over(&sents);
        let before = f.clone();

        let r = Descent::cycle_with(&mut f, &sents, Up::Bag, 40, 1.0, false, 0.0);
        assert!(
            r.rejected,
            "a full-step attraction cycle on a fully-connected vocabulary should \
             collapse: dispersion {:.3} -> {:.3}, band {:.3} -> {:.3}",
            r.dispersion_before, r.dispersion_after, r.band_before, r.band_after
        );
        // Rejection must leave the facet untouched, not partly moved.
        for (w, p) in &before.lexicon {
            for k in 0..PHASE_CHANNELS {
                assert!((f.lexicon[w].theta(k) - p.theta(k)).abs() < 1e-12, "{w} moved");
            }
        }
    }

    #[test]
    fn test_descent_is_reproducible() {
        let sents = sentences(&[
            "alpha beta gamma delta",
            "beta gamma epsilon zeta",
            "gamma delta eta theta",
            "alpha epsilon theta iota",
        ]);
        let run = || {
            let mut f = facet_over(&sents);
            Descent::cycle(&mut f, &sents, Up::Bound, 5, 0.3, true);
            f
        };
        let a = run();
        let b = run();
        for (w, p) in &a.lexicon {
            for k in 0..PHASE_CHANNELS {
                assert!((p.theta(k) - b.lexicon[w].theta(k)).abs() < 1e-12, "{w} channel {k}");
            }
        }
    }
}
