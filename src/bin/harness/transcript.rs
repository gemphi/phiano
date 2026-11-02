//! The Oracle transcript — one markdown page per quest, recording every ask
//! and every answer under the name the Oracle reports for itself (R3.12).
//!
//! The transcript is the record from which task 18.3 lifts its false claims
//! verbatim, so it lands before the frozen labelled set: every claim that set
//! calls false must be copied from a recorded answer rather than written for
//! the set. Because of that, the transcript records far more than the ask text
//! and the answer text. Per ask (R3.4, R3.10) it carries the index, the
//! held-row count the ask saw before any Oracle was consulted (zero for every
//! consulted ask by construction, R3.9), the rejection if the answer drew one,
//! the empty flag, the claims parsed, the claims refused with the class the
//! Checker gave each, and the rows the answer told.
//!
//! The page carries the Oracle's name as the Oracle reports it — `ollama run
//! phi4:14b --nowordwrap`, model name and tag and all — so every number later
//! produced from the page is attributed to that exact name (R3.12), and the
//! Checker's name beside it so the classification is attributed too.
//!
//! This module records; it does not consult. A [`Recording`] is driven by the
//! caller, which hands it each ask's text, the answer's [`Outcome`] from the
//! [`OracleSeam`], the held-row count, and a Checker to classify the parsed
//! claims. Nothing here spawns a child (R3.13): the tests drive it from a
//! `Scripted` Oracle through the seam.
#![allow(dead_code)]

use super::seam::{Outcome, Rejected};
use phiecs::quest::{parse_relations, Checker, Claim, Standing};
use phiecs::world::World;
use std::path::{Path, PathBuf};

mod page;

/// Where a quest's transcript pages are written, under `phinet/docs/`.
pub const TRANSCRIPTS_DIR: &str = "docs/transcripts";

/// One ask, recorded whatever its answer was. Every field a transcript must
/// carry per ask (R3.4, R3.10) sits here: the index within the quest, the
/// held-row count the ask saw, the rejection if the answer drew one, whether
/// the answer was empty after trimming, the claims parsed from it, the claims
/// refused with the class each drew, and the rows told.
#[derive(Clone, Debug, PartialEq)]
pub struct Recorded {
    pub index: u32,
    pub prompt: String,
    pub held: usize,
    pub rejection: Option<Rejected>,
    pub empty: bool,
    pub parsed: Vec<Claim>,
    pub refused: Vec<(Claim, Standing)>,
    pub told: Vec<Claim>,
}

impl Recorded {
    /// The count of claims parsed from the answer (R3.10).
    pub fn parsed_count(&self) -> usize {
        self.parsed.len()
    }

    /// The count of claims refused, each carrying the class that refused it.
    pub fn refused_count(&self) -> usize {
        self.refused.len()
    }

    /// The count of rows told — the claims that passed the Checker (R3.10).
    pub fn told_count(&self) -> usize {
        self.told.len()
    }
}

/// A quest's transcript as it is built: the Oracle's reported name, the
/// Checker's reported name, and the asks recorded so far in ask order. The
/// two names are read once at construction so the page attributes every
/// number to the name the Oracle gave for itself (R3.12).
pub struct Recording {
    oracle: String,
    checker: String,
    asks: Vec<Recorded>,
}

impl Recording {
    /// Begin a transcript under the two reported names. `oracle` is the name
    /// the Oracle reports for itself — `ollama run phi4:14b --nowordwrap`, or
    /// `scripted` in a test — and `checker` the name the Checker reports, so a
    /// bare-checker run is marked `bare` rather than passing silently.
    pub fn new(oracle: &str, checker: &str) -> Self {
        Recording { oracle: oracle.to_string(), checker: checker.to_string(), asks: Vec::new() }
    }

    /// The Oracle's reported name, as every number on the page is attributed.
    pub fn oracle(&self) -> &str {
        &self.oracle
    }

    /// The Checker's reported name.
    pub fn checker(&self) -> &str {
        &self.checker
    }

    /// Every ask recorded so far, in ask order.
    pub fn asks(&self) -> &[Recorded] {
        &self.asks
    }

    /// Record one ask's outcome. The claims are parsed from a `Told` answer
    /// and each is passed to `checker` against `world`: a claim the Checker
    /// lets tell is a told row, one it refuses is kept with the class it drew
    /// (R3.10). An empty, a rejection and an unreachable parse nothing and
    /// tell nothing. `held` is the row count the ask saw; `offered` are the
    /// kinds the prompt named, so parsing matches the quest's own.
    pub fn record(
        &mut self,
        prompt: &str,
        held: usize,
        outcome: &Outcome,
        offered: &[String],
        checker: &mut dyn Checker,
        world: &World,
    ) {
        let index = outcome_index(outcome);
        let (empty, rejection, answer) = classify(outcome);
        let parsed = match &answer {
            Some(text) => parse_relations(text, offered),
            None => Vec::new(),
        };
        let about = seed_word(prompt);
        let (told, refused) = split_claims(&parsed, &about, checker, world);
        self.asks.push(Recorded { index, prompt: prompt.to_string(), held, rejection, empty, parsed, refused, told });
    }

    /// Render the transcript as a markdown page: the two names in a header, a
    /// totals line, and one section per ask carrying every recorded field.
    pub fn render(&self) -> String {
        page::render(self)
    }

    /// Render and write the transcript to a new page under `dir`, returning
    /// its path. The name is the quest's seed word and the Oracle's name, so
    /// two quests under one Oracle do not collide; the directory is created if
    /// absent.
    pub fn write(&self, dir: &Path, seed: &str) -> std::io::Result<PathBuf> {
        std::fs::create_dir_all(dir)?;
        let path = dir.join(format!("{}_{}.md", page::slug(seed), page::slug(&self.oracle)));
        std::fs::write(&path, self.render())?;
        Ok(path)
    }

    /// The whole transcript's totals: claims parsed, claims refused, rows told.
    pub(super) fn totals(&self) -> (usize, usize, usize) {
        self.asks.iter().fold((0, 0, 0), |(p, r, t), a| {
            (p + a.parsed_count(), r + a.refused_count(), t + a.told_count())
        })
    }
}

/// The ask index an outcome carries, whatever its variant.
fn outcome_index(outcome: &Outcome) -> u32 {
    match outcome {
        Outcome::Told { index, .. }
        | Outcome::Empty { index }
        | Outcome::Rejected { index, .. }
        | Outcome::Unreachable { index, .. } => *index,
    }
}

/// An outcome read into the empty flag, the rejection if any, and the answer
/// text to parse (present only for a told answer).
fn classify(outcome: &Outcome) -> (bool, Option<Rejected>, Option<String>) {
    match outcome {
        Outcome::Told { answer, .. } => (false, None, Some(answer.clone())),
        Outcome::Empty { .. } => (true, None, None),
        Outcome::Rejected { why, .. } => (false, Some(why.clone()), None),
        Outcome::Unreachable { .. } => (false, None, None),
    }
}

/// Split the parsed claims into the rows told and the claims refused with the
/// class each drew, passing every claim to the Checker before any is told
/// (R3.10).
fn split_claims(
    parsed: &[Claim],
    about: &str,
    checker: &mut dyn Checker,
    world: &World,
) -> (Vec<Claim>, Vec<(Claim, Standing)>) {
    let mut told = Vec::new();
    let mut refused = Vec::new();
    for claim in parsed {
        let standing = checker.check(about, claim, world);
        match standing.tells() {
            true => told.push(claim.clone()),
            false => refused.push((claim.clone(), standing)),
        }
    }
    (told, refused)
}

/// The word a prompt is about: the quest asks about one word, so the record
/// reads it from the prompt's `Word: <w>` line, falling back to the whole
/// prompt trimmed when the prompt carries no such line.
fn seed_word(prompt: &str) -> String {
    prompt
        .lines()
        .find_map(|l| l.trim().strip_prefix("Word:"))
        .map(|w| w.trim().to_string())
        .unwrap_or_else(|| prompt.trim().to_string())
}

#[cfg(test)]
#[path = "transcript/tests.rs"]
mod tests;
