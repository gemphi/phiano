//! The Curriculum_Runner's candidate measurement (Requirement 4 criteria 1,
//! 2, 3, 4, 8, 11 and 12).
//!
//! One `TestSet` is built by the Split_Rule and used unchanged for every
//! candidate (R4.1). Each is measured on that one set, reporting its top-1
//! next-word percentage and perplexity to one decimal place beside the
//! learner's 21.3% / 101.2, the trigram's 24.8% / 84.7 and the count of
//! sentences scored (R4.2, R4.3).
//! The verdict follows two thresholds under their own senses (R4.4, R4.8): a
//! top-1 at or below 21.3% OR a perplexity at or above 101.2 is a
//! [`CandidateResult::Regression`]; beating the trigram needs BOTH a top-1
//! above 24.8% AND a perplexity below 84.7. A candidate measured on any other
//! test set is [`CandidateResult::NotComparable`] with no beating verdict
//! (R4.11); a run stopping before its declared epoch count is
//! [`CandidateResult::Incomplete`] with the epochs finished and reason (R4.12).
//! In the three non-`Measured` cases the weights file's hash is taken before
//! any candidate runs and asserted unchanged after (R4.4, R4.12) — a
//! `phistd::mix64` fold over the bytes, an absent file folding to a sentinel.
//! The logic is pure; Property 53 drives the thresholds and the guard. Task
//! 19.2's additions — `CorpusRef`, the staged order's `Stage` rule, the
//! run-start record, the per-corpus shares (Property 54) and the `Comparison`
//! — live in the single-word sibling [`provenance`] module this file
//! re-exports, keeping curriculum.rs under the 250-line cap.

use phisor::guide::holdout::{role_of, CutId, Role};
use std::path::{Path, PathBuf};

pub mod provenance;
// Re-exports the full 19.2 surface; some is consumed only by the heavy run.
#[allow(unused_imports)]
pub use provenance::{present, Comparison, Corpus, CorpusRef, Fed, RunStart, Stage};

/// The current numbers every candidate is written beside (R4.3).
pub const LEARNER_TOP1: f64 = 21.3;
pub const LEARNER_PERPLEXITY: f64 = 101.2;
pub const TRIGRAM_TOP1: f64 = 24.8;
pub const TRIGRAM_PERPLEXITY: f64 = 84.7;

/// The one split every candidate is measured on (R4.1). Built once by the
/// Split_Rule over The_Guide's sentences and reused unchanged, so two
/// candidates are never compared across two different test sets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestSet {
    pub indices: Vec<usize>,
    pub sentences: Vec<String>,
    pub cut: CutId,
}

impl TestSet {
    /// Build the test set by the Split_Rule (`i % 10 == 0`) over the guide's
    /// sentences in index order, carrying the cut the indices belong to so a
    /// candidate measured on a different cut can be caught as not comparable.
    pub fn split(sentences: &[String], cut: CutId) -> TestSet {
        let mut indices = Vec::new();
        let mut kept = Vec::new();
        for (i, s) in sentences.iter().enumerate() {
            if role_of(i) == Role::Test {
                indices.push(i);
                kept.push(s.clone());
            }
        }
        TestSet { indices, sentences: kept, cut }
    }

    /// The number of test sentences, written beside every candidate (R4.3).
    pub fn scored(&self) -> usize {
        self.sentences.len()
    }
}

/// The two candidate shapes (R4.5). The fields each carries are read by the
/// heavy run and by [`provenance`]; the measurement here only needs the test
/// set and the numbers the run produced. [`CorpusRef`], [`Stage`] and the
/// staged-order rule (`seed`, `channels`, `staging_valid`) are defined in the
/// [`provenance`] module, which carries the rest of the 19.2 additions.
#[derive(Clone, Debug)]
pub enum Candidate {
    /// Trains on an external corpus (R4.5), whose name/size/licence are
    /// recorded before the first step when it is not on disk (R4.6).
    External { corpus: CorpusRef, epochs: usize, seed: u64, channels: usize },
    /// The_Guide alone in a staged order: at least 4 epochs, the wider text in
    /// the earlier ones, only the guide's training sentences in at least the
    /// final 2 (R4.5).
    Staged { stages: Vec<Stage>, epochs: usize, seed: u64, channels: usize },
}

impl Candidate {
    /// The declared epoch count, against which a short run is incomplete.
    pub fn epochs(&self) -> usize {
        match self {
            Candidate::External { epochs, .. } | Candidate::Staged { epochs, .. } => *epochs,
        }
    }
}

/// What a candidate run produced, before any threshold is applied: the numbers
/// to one decimal place, the cut they were measured on, and how far the run
/// got. Training lives in the heavy run; the pure verdict reads this.
#[derive(Clone, Debug, PartialEq)]
pub struct Reported {
    pub top1: f64,
    pub perplexity: f64,
    pub scored: usize,
    /// The cut the candidate was actually measured on. When it differs from
    /// the `TestSet`'s cut the result is `NotComparable` (R4.11).
    pub cut: CutId,
    /// The epochs the run finished; when below the candidate's declared count
    /// the result is `Incomplete` (R4.12).
    pub epochs_finished: usize,
    /// The reason the run stopped early, for an `Incomplete` result.
    pub stop_reason: String,
}

/// The verdict for one candidate (R4.2, R4.4, R4.11, R4.12). Exactly one of
/// four outcomes; only `Measured` carries numbers forward to a beating verdict.
#[derive(Clone, Debug, PartialEq)]
pub enum CandidateResult {
    /// Completed its epochs on R4.1's test set with numbers above the
    /// regression thresholds.
    Measured { top1: f64, perplexity: f64, scored: usize },
    /// top1 <= 21.3 or perplexity >= 101.2; weights left byte-for-byte
    /// unchanged and no new weights written (R4.4).
    Regression { top1: f64, perplexity: f64 },
    /// Measured on any test set other than R4.1's; no beating-the-trigram
    /// verdict is reported (R4.11).
    NotComparable { used: CutId },
    /// Stopped before the declared epoch count (R4.12).
    Incomplete { epochs_finished: usize, reason: String },
}

/// A rounded-to-one-decimal view of a number, so the reported top-1 and
/// perplexity match what is written on the record (R4.2).
fn one_decimal(x: f64) -> f64 {
    (x * 10.0).round() / 10.0
}

/// True when a top-1 / perplexity pair is a regression under R4.4: a top-1 at
/// or below 21.3%, OR a perplexity at or above 101.2. The senses differ — a
/// top-1 is a floor, a perplexity a ceiling — which is why neither can be
/// inferred from the other.
pub fn is_regression(top1: f64, perplexity: f64) -> bool {
    top1 <= LEARNER_TOP1 || perplexity >= LEARNER_PERPLEXITY
}

/// True when a top-1 / perplexity pair beats the trigram under R4.8: BOTH a
/// top-1 above 24.8% AND a perplexity below 84.7. A pair meeting one alone
/// does not beat it.
pub fn beats_trigram_numbers(top1: f64, perplexity: f64) -> bool {
    top1 > TRIGRAM_TOP1 && perplexity < TRIGRAM_PERPLEXITY
}

/// The `phistd::mix64` fold over a file's bytes — the weights hash taken
/// before any candidate runs and asserted unchanged after (R4.4, R4.12). An
/// absent file folds to a fixed sentinel, so a run that writes nothing still
/// compares equal to itself.
pub fn hash_weights(path: &Path) -> u64 {
    match std::fs::read(path) {
        Ok(bytes) => {
            let mut h: u64 = 0xC0FF_EE00_1234_5678;
            for b in bytes {
                h = phistd::mix64(h ^ b as u64);
            }
            h
        }
        Err(_) => 0xDEAD_BEEF_DEAD_BEEF,
    }
}

/// The Curriculum_Runner's candidate measurement. Holds the one test set every
/// candidate is measured on (R4.1) and the weights file's hash taken before
/// any candidate runs (R4.4, R4.12).
pub struct CurriculumRunner {
    test: TestSet,
    weights: PathBuf,
    weights_before: u64,
}

impl CurriculumRunner {
    /// Open the runner over the one test set and the weights file, folding the
    /// weights' bytes once up front so a later run can be asserted to have
    /// written nothing.
    pub fn new(test: TestSet, weights: &Path) -> CurriculumRunner {
        CurriculumRunner {
            test,
            weights: weights.to_path_buf(),
            weights_before: hash_weights(weights),
        }
    }

    /// The one test set, so a caller can write its count beside every
    /// candidate (R4.3).
    pub fn test(&self) -> &TestSet {
        &self.test
    }

    /// Classify one candidate's reported numbers into a [`CandidateResult`]
    /// (R4.2, R4.4, R4.11, R4.12). The order is incomplete, then not
    /// comparable, then regression, then measured: a run that stopped early
    /// has no trustworthy numbers to compare, and a run on the wrong cut has
    /// no comparable numbers either, so both are caught before any threshold.
    ///
    /// For every outcome but `Measured` the weights file is asserted unchanged
    /// against the hash taken at `new`, so no new weights were written.
    pub fn classify(&self, c: &Candidate, r: &Reported) -> CandidateResult {
        let result = if r.epochs_finished < c.epochs() {
            CandidateResult::Incomplete {
                epochs_finished: r.epochs_finished,
                reason: r.stop_reason.clone(),
            }
        } else if r.cut != self.test.cut {
            CandidateResult::NotComparable { used: r.cut }
        } else if is_regression(r.top1, r.perplexity) {
            CandidateResult::Regression {
                top1: one_decimal(r.top1),
                perplexity: one_decimal(r.perplexity),
            }
        } else {
            CandidateResult::Measured {
                top1: one_decimal(r.top1),
                perplexity: one_decimal(r.perplexity),
                scored: self.test.scored(),
            }
        };
        if !matches!(result, CandidateResult::Measured { .. }) {
            assert_eq!(
                hash_weights(&self.weights),
                self.weights_before,
                "a non-measured candidate must leave the weights byte-for-byte unchanged",
            );
        }
        result
    }

    /// Whether a result beats the trigram (R4.8): `Some(true)` when a measured
    /// candidate clears both thresholds, `Some(false)` when a measured
    /// candidate clears neither or one alone, and `None` when there is no
    /// comparable measured number — a regression, a not-comparable run or an
    /// incomplete run reports no beating verdict (R4.11).
    pub fn beats_trigram(&self, r: &CandidateResult) -> Option<bool> {
        match r {
            CandidateResult::Measured { top1, perplexity, .. } => {
                Some(beats_trigram_numbers(*top1, *perplexity))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests;
