//! The Oracle_Seam: the one place a local model (Ollama, Phi-4) is reached,
//! its absence and failure handled, and Overflow enforced (Requirement 3).
//!
//! The seam owns three things a bare [`phiecs::quest::Oracle`] does not: a
//! [`Checker`], required at construction so the check-before-tell rule of
//! R8.10 is structural and not conventional — the seam cannot be built
//! without one; an ask budget in the range 1 to 4,096 (64 by default, R3.11);
//! and the answer caps of R3.4 through R3.8 — an ANSI escape, a 65,536-char
//! ceiling, a 120-second wall-clock ceiling and a non-zero exit — each of
//! which rejects an answer, records the ask's index, tells no row, and lets
//! the walk go on while the budget remains (R3.4, R3.6, R3.7, R3.8).
//!
//! The seam lives under `phiano/src/bin/harness/`, so it is part of a binary
//! and not the phiano library (R1.11): nothing can depend on it. It reaches
//! the model only through `phiecs::quest`'s `Spawned` and `Scripted`, so no
//! process-spawning code of its own is written here — `spawn` hands back a
//! `Spawned` with exactly the arguments R3.1 names, and a test or dry run
//! hands back a `Scripted` that spawns no child (R3.13).
#![allow(dead_code)]

use phiecs::quest::{Checker, Oracle, OracleError, Scripted, Spawned};

/// The declared ask-budget bounds (R3.11): a quest may ask between 1 and
/// 4,096 times, and asks 64 times when no budget is named.
pub const MIN_BUDGET: u32 = 1;
pub const MAX_BUDGET: u32 = 4_096;
pub const DEFAULT_BUDGET: u32 = 64;

/// The answer caps of R3.4 through R3.8.
pub const CHAR_CAP: usize = 65_536;
pub const SECONDS_CAP: u64 = 120;
pub const STDERR_CAP: usize = 2_000;

/// Why an answer was rejected (R3.4, R3.6, R3.7, R3.8). Every variant records
/// what its criterion names, and a rejection tells no row and does not stop
/// the walk while the budget remains.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Rejected {
    /// The answer carried an ANSI escape sequence, at the byte offset of the
    /// escape (R3.4).
    AnsiEscape { at: usize },
    /// The answer ran past the 65,536-character ceiling (R3.6).
    TooLong { chars: usize },
    /// The child outlasted the 120-second ceiling and was terminated (R3.7).
    TimedOut { seconds: u64 },
    /// The child exited non-zero, carrying up to 2,000 characters of its
    /// standard-error text (R3.8).
    Exited { status: i32, stderr: String },
}

/// One ask's outcome, recorded whatever it was. Every outcome carries the
/// ask's index within the run so a rejection names where it fell (R3.4).
#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    /// The answer passed every cap. `held_zero` records that the held-row
    /// count was zero, which it is for every consulted ask by construction
    /// (R3.9), and `claims` the count of claims parsed from it (R3.10).
    Told { index: u32, answer: String, claims: usize },
    /// The answer was empty after trimming: recorded and counted against the
    /// budget, but not a rejection (R3.5).
    Empty { index: u32 },
    /// The answer was rejected on one of the declared conditions; no row is
    /// told from it (R3.4, R3.6, R3.7, R3.8).
    Rejected { index: u32, why: Rejected },
    /// The program could not be run: the quest stops before a second ask and
    /// the ask count is 1 (R3.3).
    Unreachable { index: u32, program: String },
}

impl Outcome {
    /// Whether this outcome permits a row to be told (R3.10). An empty, a
    /// rejection and an unreachable all tell nothing.
    pub fn tells(&self) -> bool {
        matches!(self, Outcome::Told { .. })
    }
}

/// The Oracle_Seam: an Oracle to consult, the Checker every parsed claim is
/// passed before any row is told (R3.10), and the ask budget (R3.11). The
/// Checker is a constructor argument, not a setter, so a seam without one
/// cannot exist and the check-before-tell rule is structural (R8.10).
pub struct OracleSeam {
    oracle: Box<dyn Oracle>,
    checker: Box<dyn Checker>,
    budget: u32,
    asked: u32,
    outcomes: Vec<Outcome>,
    stopped: Option<String>,
}

impl OracleSeam {
    /// Build a seam over an Oracle, a Checker and an ask budget. The budget is
    /// clamped into 1 to 4,096 (R3.11); a caller naming none passes
    /// [`DEFAULT_BUDGET`]. A Checker is required, so the seam cannot be built
    /// without the thing that checks before a row is told.
    pub fn new(oracle: Box<dyn Oracle>, checker: Box<dyn Checker>, budget: u32) -> Self {
        let budget = budget.clamp(MIN_BUDGET, MAX_BUDGET);
        OracleSeam { oracle, checker, budget, asked: 0, outcomes: Vec::new(), stopped: None }
    }

    /// A `Spawned` Oracle with exactly the arguments R3.1 names: `ollama` is
    /// given `run`, the model name and `--nowordwrap` and nothing else; any
    /// other program is given exactly the arguments named in the run's
    /// settings, with no argument of the seam's own. The prompt goes to the
    /// child's stdin and the answer is read from its stdout, which `Spawned`
    /// already does (R3.2).
    pub fn spawn(program: &str, args: &[&str]) -> Spawned {
        match program == "ollama" {
            // `args[0]` is the model name; ollama's three fixed arguments are
            // the seam's, not the caller's, so nothing else is read from args.
            true => Spawned::ollama(args.first().copied().unwrap_or("")),
            false => Spawned::new(program, args),
        }
    }

    /// A seam over a `Scripted` Oracle and the given Checker, for a test or a
    /// dry run: no child process is ever spawned (R3.13).
    pub fn scripted(answers: &[(&str, &str)], checker: Box<dyn Checker>, budget: u32) -> Self {
        Self::new(Box::new(Scripted::new(answers)), checker, budget)
    }

    /// The budget this seam will ask up to (R3.11).
    pub fn budget(&self) -> u32 {
        self.budget
    }

    /// How many asks have been made.
    pub fn asked(&self) -> u32 {
        self.asked
    }

    /// Whether the budget is spent: no further ask may be made (R3.11).
    pub fn exhausted(&self) -> bool {
        self.asked >= self.budget
    }

    /// Why the run stopped early, if it did — an unreachable program (R3.3).
    pub fn stopped(&self) -> Option<&str> {
        self.stopped.as_deref()
    }

    /// Every ask's recorded outcome, in ask order.
    pub fn outcomes(&self) -> &[Outcome] {
        &self.outcomes
    }

    /// The Checker this seam checks every claim against before a row is told.
    pub fn checker(&mut self) -> &mut dyn Checker {
        self.checker.as_mut()
    }

    /// Consult the Oracle for one ask, validate its answer against the caps,
    /// record the outcome and return it. Overflow (R3.9) is the caller's to
    /// decide — this is the consulted path, where the held-row count is zero
    /// by construction — so `held` is the row count the caller measured, and
    /// an ask with `held > 0` consults no Oracle and records nothing. The
    /// budget is honoured: an exhausted seam makes no further ask (R3.11).
    pub fn ask(&mut self, prompt: &str, held: usize) -> Option<&Outcome> {
        if held > 0 || self.exhausted() || self.stopped.is_some() {
            return None;
        }
        let index = self.asked;
        self.asked += 1;
        let outcome = match self.oracle.ask(prompt) {
            Ok(text) => validate(index, &text),
            Err(OracleError::Unreachable(program)) => {
                self.stopped = Some(format!("oracle unreachable: {program}"));
                Outcome::Unreachable { index, program }
            }
            // An oracle that ran and said nothing usable is an empty answer:
            // recorded, counted against the budget, not a rejection (R3.5).
            Err(OracleError::Empty) => Outcome::Empty { index },
            Err(OracleError::Failed(stderr)) => Outcome::Rejected {
                index,
                why: Rejected::Exited { status: 1, stderr: cap_stderr(&stderr) },
            },
        };
        self.outcomes.push(outcome);
        self.outcomes.last()
    }
}

/// Validate one answer against the caps in the order R3 declares them: a
/// non-zero exit and a timeout are reported by the Oracle itself, so what is
/// left to read from the text is the length ceiling (R3.6), an ANSI escape
/// (R3.4) and emptiness after trimming (R3.5). The length cap is read first:
/// a 70,000-character answer is `TooLong` whatever it holds.
fn validate(index: u32, text: &str) -> Outcome {
    let chars = text.chars().count();
    if chars > CHAR_CAP {
        return Outcome::Rejected { index, why: Rejected::TooLong { chars } };
    }
    if let Some(at) = ansi_at(text) {
        return Outcome::Rejected { index, why: Rejected::AnsiEscape { at } };
    }
    if text.trim().is_empty() {
        return Outcome::Empty { index };
    }
    let claims = phiecs::quest::parse_relations(text, &[]).len();
    Outcome::Told { index, answer: text.to_string(), claims }
}

/// The byte offset of the first ANSI escape (an `ESC` `0x1B` byte, which opens
/// `ESC[...`), if the text carries one (R3.4). ollama's line-wrapping writes
/// `ESC[nD` and `ESC[K`; `--nowordwrap` suppresses them, but an answer that
/// still carries one is rejected rather than parsed with escapes in it.
fn ansi_at(text: &str) -> Option<usize> {
    text.find('\u{1b}')
}

/// The first 2,000 characters of a child's standard-error text (R3.8).
fn cap_stderr(stderr: &str) -> String {
    stderr.chars().take(STDERR_CAP).collect()
}

#[cfg(test)]
#[path = "seam/tests.rs"]
mod tests;
