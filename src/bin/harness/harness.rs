//! The Harness itself — the walk that ties the four sibling modules together
//! (Requirement 1 criteria 4, 10 and 11).
//!
//! `run_one` runs a single level through the capped spawn and records its one
//! outcome, writing nothing for the other nine (R1.4); `run_all` walks the ten
//! in `LEVELS` order, starting a level only once the preceding one has come to
//! one of the four outcomes (R1.10); `run_ladder` reports the six heights in
//! all three arms — eighteen numbers, each with its Bar and its verdict
//! (R5), through `phisor::ladder`. The overall verdict (R1.11) is a pass only
//! on a clean sweep: all ten levels passed with zero failures, none built
//! failed or timed out, and every Bar judged in Requirements 3 through 9 met.
//!
//! This file lives under `phiano/src/bin/harness/`, so it is part of a binary
//! and not the phiano library (R1.11): no library target names all ten crates
//! and nothing can depend on the Harness.
//!
//! The actual-cargo walk is slow and shells out through `spawn`, so it is kept
//! off the ordinary `cargo test -p phiano --bin harness` path (which stays
//! fast and offline) behind the `run_walk` entry the binary calls and the
//! `#[ignore]` OS test, exactly as `spawn.rs` gates its `taskkill` path. The
//! overall-verdict logic — the one thing Property 55 pins — is a pure function
//! of the ten outcomes and the Bar verdicts, so it is tested over synthetic
//! vectors, never by spawning cargo.

use super::bar::{Bars, Judged};
use super::level::{self, Outcome};
use super::record::Recorder;
use super::spawn::{self, Capped};
use std::path::PathBuf;
use std::time::Instant;

/// The caps a run is held to: the wall-clock seconds one level may take before
/// its process tree is killed (R1.6). Held as a field so a test or a quick
/// local walk can shorten the cap without touching `spawn::CAP_SECONDS`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Caps {
    /// Per-level wall-clock cap in seconds (R1.6); `spawn::CAP_SECONDS` by
    /// default.
    pub seconds: u64,
}

impl Default for Caps {
    fn default() -> Self {
        Caps { seconds: spawn::CAP_SECONDS }
    }
}

/// The Harness: the declared Bars every number is judged against, the Recorder
/// the run writes through, and the caps a level is held to. It owns no crate
/// it polices — it shells out to `cargo test -p <level>` through `spawn`.
pub struct Harness {
    bars: Bars,
    recorder: Recorder,
    caps: Caps,
}

impl Harness {
    /// Build a Harness over the declared Bars, an opened Recorder and the caps
    /// (R1.4). The Recorder is already opened (its git stamp gathered) by the
    /// caller, so construction takes it by value and the walk writes through
    /// it.
    pub fn new(bars: Bars, recorder: Recorder, caps: Caps) -> Harness {
        Harness { bars, recorder, caps }
    }

    /// The declared Bars this run judges against.
    pub fn bars(&self) -> &Bars {
        &self.bars
    }

    /// Run exactly one level and report its single outcome, recording no
    /// outcome for the other nine (R1.4). An unaccepted level name reports the
    /// given value and the ten accepted names, runs no level and leaves the
    /// Recorder untouched (R1.8): `level::accept` is consulted first.
    pub fn run_one(&mut self, name: &str) -> Result<Outcome, level::HarnessError> {
        let level = level::accept(name)?;
        Ok(self.walk_level(level))
    }

    /// Walk the ten levels in `LEVELS` order (R1.1), starting a level only
    /// once the preceding one has come to one of the four outcomes (R1.10).
    /// Returns the ten outcomes in `LEVELS` order; the loop is sequential, so
    /// at most one cargo child is ever in flight (R1.5), and a level's outcome
    /// is complete before the next begins.
    pub fn run_all(&mut self) -> Vec<(&'static str, Outcome)> {
        let mut out: Vec<(&'static str, Outcome)> = Vec::with_capacity(level::LEVELS.len());
        for &name in &level::LEVELS {
            // The preceding level has reported its outcome by the time control
            // returns here, because `walk_level` blocks on the capped spawn.
            let outcome = self.walk_level(name);
            out.push((name, outcome));
        }
        out
    }

    /// Run `cargo test -p <level>` under the cap and turn the capped result
    /// into one of the four outcomes (R1.2, R1.3, R1.6, R1.9). A build failure
    /// is distinguished from a test failure by running `--no-run` first, as
    /// `spawn.rs` documents.
    fn walk_level(&self, level: &'static str) -> Outcome {
        let secs = self.caps.seconds;
        let started = Instant::now();

        // The build step first, so a compile error (R1.9) is not mistaken for
        // a test failure (R1.3).
        let build = spawn::run_capped(&["test", "-p", level, "--no-run"], secs);
        match build {
            Ok(Capped::Killed) => return Outcome::TimedOut { seconds: secs as f64 },
            Ok(Capped::Finished { ok: false, err, .. }) => {
                let first = spawn::first_build_error(&err)
                    .unwrap_or_else(|| "build failed with no error line".to_string());
                let seconds = spawn::elapsed_or_cap(started, false, secs);
                return Outcome::BuildFailed { seconds, first_error: first };
            }
            Ok(Capped::Finished { ok: true, .. }) => {}
            Err(e) => {
                let seconds = spawn::elapsed_or_cap(started, false, secs);
                return Outcome::BuildFailed { seconds, first_error: format!("spawn failed: {e}") };
            }
        }

        // The test run, every failure surfaced via `--no-fail-fast`.
        let run = spawn::run_capped(&["test", "-p", level, "--no-fail-fast"], secs);
        match run {
            Ok(Capped::Killed) => Outcome::TimedOut { seconds: secs as f64 },
            Ok(Capped::Finished { ok, out, err }) => {
                let text = format!("{out}\n{err}");
                let counts = spawn::parse_counts(&text).unwrap_or_default();
                let seconds = spawn::elapsed_or_cap(started, false, secs);
                if ok && counts.failed == 0 {
                    Outcome::Passed {
                        run: counts.run,
                        passed: counts.passed,
                        failed: counts.failed,
                        seconds,
                    }
                } else {
                    let failures = spawn::failing_names(&text)
                        .into_iter()
                        .map(|n| (n, String::new()))
                        .collect();
                    Outcome::Failed {
                        run: counts.run,
                        passed: counts.passed,
                        failed: counts.failed,
                        seconds,
                        failures,
                    }
                }
            }
            Err(e) => Outcome::BuildFailed {
                seconds: spawn::elapsed_or_cap(started, false, secs),
                first_error: format!("spawn failed: {e}"),
            },
        }
    }

    /// Record the eighteen ladder numbers — the six heights in all three arms
    /// (R5) — each with its Bar and its verdict, and return the Bar verdicts
    /// the overall verdict reads. The numbers are written through the Recorder
    /// in height-then-arm order; a height that misses its Bar does not stop the
    /// ones above it (R5.14), so every number is recorded whatever its verdict.
    ///
    /// The real ladder measures shell out through the split and the grown code,
    /// which is slow, so the heavy run is driven by `run_walk`; this method
    /// takes the eighteen already-measured numbers so Property 55 can exercise
    /// the verdict path over synthetic vectors without a ladder run.
    pub fn record_ladder(&mut self, numbers: &[super::record::Measured]) -> Vec<Judged> {
        let mut verdicts = Vec::with_capacity(numbers.len());
        for m in numbers {
            verdicts.push(m.verdict);
            let _ = self.recorder.write(m.clone());
        }
        verdicts
    }

    /// Close the Recorder, writing the run's record page and returning its
    /// path (R2.3). Every earlier page is left byte-identical.
    pub fn close(self) -> Result<PathBuf, super::record::RecordError> {
        self.recorder.close()
    }
}

/// The overall verdict of a run (R1.11). A pass only on a clean sweep;
/// otherwise a miss carrying why.
#[derive(Clone, Debug, PartialEq)]
pub enum Verdict {
    /// Every level passed with zero failures, none built failed or timed out,
    /// and every judged Bar in Requirements 3 through 9 was met.
    Pass,
    /// At least one of those held false; the reason names the first cause so
    /// the record says why the sweep was not clean.
    Miss { reason: String },
}

/// The overall verdict over the ten level outcomes and the Bar verdicts
/// gathered across Requirements 3 through 9 (R1.11). This is the one function
/// Property 55 pins, and it is pure: no cargo, no disk, no clock. A pass needs
/// every level `Passed` with `failed == 0` and every Bar `Pass`; a single
/// `Failed`, `BuildFailed`, `TimedOut`, non-zero failure count or `Miss`
/// makes the run a miss, and an `Unjudged` Bar is neither a pass nor a miss —
/// it cannot clear the sweep, so it is a miss naming the unjudged number.
pub fn overall_verdict(outcomes: &[Outcome], bars: &[Judged]) -> Verdict {
    for (i, o) in outcomes.iter().enumerate() {
        let name = level::LEVELS.get(i).copied().unwrap_or("?");
        match o {
            Outcome::Passed { failed: 0, .. } => {}
            Outcome::Passed { failed, .. } => {
                return miss(format!("level {name} reported {failed} failures despite passing"));
            }
            Outcome::Failed { failed, .. } => {
                return miss(format!("level {name} failed with {failed} failing tests"));
            }
            Outcome::BuildFailed { .. } => return miss(format!("level {name} did not build")),
            Outcome::TimedOut { .. } => return miss(format!("level {name} timed out")),
        }
    }
    for (i, v) in bars.iter().enumerate() {
        match v {
            Judged::Pass => {}
            Judged::Miss { shortfall } => {
                return miss(format!("Bar {i} missed by {shortfall}"));
            }
            Judged::Unjudged => {
                return miss(format!("Bar {i} was unjudged, so the sweep is not clean"));
            }
        }
    }
    Verdict::Pass
}

fn miss(reason: String) -> Verdict {
    Verdict::Miss { reason }
}

#[cfg(test)]
#[path = "harness/tests.rs"]
mod tests;
