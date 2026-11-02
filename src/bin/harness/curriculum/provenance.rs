//! The run-start record, the per-corpus word shares, and the side-by-side
//! comparison the Curriculum_Runner presents (Requirement 4 criteria 7, 9
//! and 10).
//!
//! Three fixed-shape records sit here, split out of `curriculum.rs` so each
//! source file stays under the 250-line cap:
//!
//!   - [`RunStart`] is written when a candidate run starts (R4.10): the
//!     random seed, the declared epoch count, the channel count and the
//!     combined training vocabulary size in words, plus the channel-to-
//!     vocabulary ratio read straight off those last two, so a thinner
//!     channel can be seen rather than inferred.
//!   - [`provenance`] reports the words taken from each corpus and the
//!     guide's share of training words to one decimal place (R4.9). The
//!     shares sum to the whole — the invariant Property 54 drives.
//!   - [`present`] puts both measured candidates' numbers side by side
//!     (R4.7). It makes no change to the default curriculum and registers
//!     nothing: it returns a [`Comparison`] the caller acts on only once the
//!     user names the winner, so the wait is in the caller, not here.

use super::{Candidate, CandidateResult};

/// A rounded-to-one-decimal view, matching what is written on the record.
fn one_decimal(x: f64) -> f64 {
    (x * 10.0).round() / 10.0
}

/// A corpus a candidate requires, recorded before the first training step
/// when it is not already on disk (R4.6). An on-disk corpus needs no fetch
/// and so no size-and-licence record before the run; an off-disk one carries
/// its name, byte size and licence so the fetch is attributable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorpusRef {
    pub name: String,
    pub bytes: u64,
    pub licence: String,
    pub on_disk: bool,
}

/// What one epoch of a staged run is fed (R4.5). `Wider` is the external text
/// the earlier epochs tell; `GuideTrain` is The_Guide's training sentences
/// alone (Split_Rule train = not test, not dev), which at least the final two
/// epochs must tell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fed {
    Wider,
    GuideTrain,
}

/// One epoch of the staged candidate's order, numbered from the start of the
/// run and fed from one of the two sources (R4.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stage {
    pub epoch: usize,
    pub fed: Fed,
}

impl Candidate {
    /// The candidate's declared random seed (R4.10).
    pub fn seed(&self) -> u64 {
        match self {
            Candidate::External { seed, .. } | Candidate::Staged { seed, .. } => *seed,
        }
    }

    /// The candidate's declared channel count (R4.10).
    pub fn channels(&self) -> usize {
        match self {
            Candidate::External { channels, .. } | Candidate::Staged { channels, .. } => *channels,
        }
    }

    /// Whether a staged candidate honours R4.5's order: at least 4 epochs
    /// declared, every declared epoch present once in `1..=epochs`, the wider
    /// text told in the earlier epochs, and only the guide's training
    /// sentences told in at least the final 2 epochs. An `External` candidate
    /// has no staged order and is vacuously valid.
    pub fn staging_valid(&self) -> bool {
        let (stages, epochs) = match self {
            Candidate::External { .. } => return true,
            Candidate::Staged { stages, epochs, .. } => (stages, *epochs),
        };
        if epochs < 4 || stages.len() != epochs {
            return false;
        }
        // Every epoch 1..=epochs appears exactly once.
        let mut seen: Vec<usize> = stages.iter().map(|s| s.epoch).collect();
        seen.sort_unstable();
        if seen != (1..=epochs).collect::<Vec<_>>() {
            return false;
        }
        // The final 2 epochs feed only the guide's training sentences; at
        // least one earlier epoch feeds the wider text; no wider epoch sits in
        // the final 2.
        let guide_tail = stages
            .iter()
            .filter(|s| s.epoch > epochs - 2)
            .all(|s| s.fed == Fed::GuideTrain);
        let wider_early = stages
            .iter()
            .any(|s| s.fed == Fed::Wider && s.epoch <= epochs - 2);
        let no_late_wider = stages
            .iter()
            .all(|s| !(s.fed == Fed::Wider && s.epoch > epochs - 2));
        guide_tail && wider_early && no_late_wider
    }
}

/// One corpus and the number of training words taken from it (R4.9). The
/// guide's own training sentences are one such corpus, flagged so the guide's
/// share can be summed out of the whole.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Corpus {
    pub name: String,
    pub words: usize,
    /// True for The_Guide's own training sentences, whose share R4.9 names.
    pub is_guide: bool,
}

impl Corpus {
    pub fn wider(name: &str, words: usize) -> Corpus {
        Corpus { name: name.to_string(), words, is_guide: false }
    }
    pub fn guide(name: &str, words: usize) -> Corpus {
        Corpus { name: name.to_string(), words, is_guide: true }
    }
}

/// Each corpus's name, its training-word count and its share of the whole as
/// a percentage to one decimal place (R4.9). The shares sum to 100.0 within
/// rounding whenever any words were taken; an all-zero input reports every
/// share as 0.0 rather than dividing by zero.
pub fn provenance(corpora: &[Corpus]) -> Vec<(String, usize, f64)> {
    let total: usize = corpora.iter().map(|c| c.words).sum();
    corpora
        .iter()
        .map(|c| {
            let share = if total == 0 {
                0.0
            } else {
                one_decimal(100.0 * c.words as f64 / total as f64)
            };
            (c.name.clone(), c.words, share)
        })
        .collect()
}

/// The guide's share of training words as a percentage to one decimal place
/// (R4.9): the guide corpora's words over the whole. Zero when no words were
/// taken, so the quotient is never undefined.
pub fn guide_share(corpora: &[Corpus]) -> f64 {
    let total: usize = corpora.iter().map(|c| c.words).sum();
    if total == 0 {
        return 0.0;
    }
    let guide: usize = corpora.iter().filter(|c| c.is_guide).map(|c| c.words).sum();
    one_decimal(100.0 * guide as f64 / total as f64)
}

/// The run-start record (R4.10): the seed, the declared epoch count, the
/// channel count and the combined training vocabulary size in words, with the
/// channel-to-vocabulary ratio derived from the last two so a thinner channel
/// is legible on the record itself.
#[derive(Clone, Debug, PartialEq)]
pub struct RunStart {
    pub seed: u64,
    pub epochs: usize,
    pub channels: usize,
    pub vocabulary: usize,
}

impl RunStart {
    /// Read the seed, epoch count and channel count off the candidate and
    /// pair them with the combined training vocabulary size (R4.10).
    pub fn of(candidate: &Candidate, vocabulary: usize) -> RunStart {
        RunStart {
            seed: candidate.seed(),
            epochs: candidate.epochs(),
            channels: candidate.channels(),
            vocabulary,
        }
    }

    /// The channel-to-vocabulary ratio read straight off the record (R4.10):
    /// channels over vocabulary words, to four decimal places. A zero
    /// vocabulary reports `None` — undefined, not a thin-looking zero.
    pub fn channel_ratio(&self) -> Option<f64> {
        if self.vocabulary == 0 {
            None
        } else {
            Some((self.channels as f64 / self.vocabulary as f64 * 10_000.0).round() / 10_000.0)
        }
    }
}

/// The two candidates' numbers placed side by side (R4.7). This is a record,
/// not a decision: it holds both results and flags whether either was
/// actually measured, and it names no default curriculum. The caller presents
/// it and waits for the user to name the winner before registering anything.
#[derive(Clone, Debug, PartialEq)]
pub struct Comparison {
    pub external: CandidateResult,
    pub staged: CandidateResult,
}

impl Comparison {
    /// The measured top-1 and perplexity of each candidate, `None` for a
    /// candidate that did not produce a comparable measured number.
    pub fn numbers(&self) -> (Option<(f64, f64)>, Option<(f64, f64)>) {
        (measured(&self.external), measured(&self.staged))
    }
}

/// The measured top-1/perplexity pair of a result, or `None` when the result
/// is not a `Measured` one.
fn measured(r: &CandidateResult) -> Option<(f64, f64)> {
    match r {
        CandidateResult::Measured { top1, perplexity, .. } => Some((*top1, *perplexity)),
        _ => None,
    }
}

/// Put both candidates' numbers side by side (R4.7). This makes no change to
/// the default curriculum and registers nothing: it returns the
/// [`Comparison`] for the caller to show the user, who alone names the winner.
/// Modelling the wait as a returned value keeps the no-auto-register rule a
/// property of the type, not a convention the caller might forget.
pub fn present(external: &CandidateResult, staged: &CandidateResult) -> Comparison {
    Comparison { external: external.clone(), staged: staged.clone() }
}

#[cfg(test)]
mod tests;
