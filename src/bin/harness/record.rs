//! The Recorder — one markdown run record per walk (Requirement 2).
//!
//! The Recorder writes each measured number with its name, value, unit, Bar,
//! control arm and verdict (R2.1), opens the page with the git stamp of the
//! workspace and of each submodule (R2.5), keeps a missed Bar on the record
//! unaltered (R2.2), marks a re-barred number against the most recent earlier
//! page (R2.4), and leaves every earlier page byte-identical (R2.3, R2.9).
//!
//! The four small judged types — `Unit`, `Sense`, `Bar` and `Judged` — are
//! the Bars reader's (task 17.1) and are reused from `super::bars` rather than
//! declared a second time, which the no-duplicate-types rule forbids. The
//! Recorder owns `Measured`, `Recorder` and `RecordError`.
//!
//! The page is `phinet/docs/<nnn>_<yyyy-mm-dd>_harness.md`, continuing the
//! numbered series from 208 — the one path the one-word naming rule does not
//! reach, because Requirement 2 criterion 3 fixes this name to the run's date
//! and the directory's numbered convention.
#![allow(dead_code)]

use super::bar::{Bar, Bars, Judged, Sense, Unit};
use std::path::{Path, PathBuf};

mod page;
mod parse;
mod stamp;

#[cfg(test)]
mod tests;

/// The series continues from 208; `phinet/docs/` holds no numbered page yet,
/// so an empty directory starts here (design, Decisions Needing the Ruling).
pub const SERIES_START: u32 = 208;

/// The cap on how many modified-path names a dirty working tree lists (R2.5).
pub const DIRTY_NAMES_CAP: usize = 20;

/// One measured number, with everything R2.1 requires beside it: the name, the
/// value, the unit from the closed list, the Bar judged against (absent when
/// `unjudged`), the control arm's name and value, and the verdict.
#[derive(Clone, Debug, PartialEq)]
pub struct Measured {
    pub name: String,
    pub value: f64,
    pub unit: Unit,
    pub bar: Option<Bar>,
    pub control: Option<(String, f64)>,
    pub verdict: Judged,
}

impl Measured {
    /// A number judged against the Bars input, with no control arm. The Bar
    /// and verdict are both drawn from `bars`, so an absent name is `Unjudged`
    /// with no Bar (R2.6) and an unreadable input makes every number
    /// `Unjudged` (R2.8).
    pub fn judged(name: &str, value: f64, unit: Unit, bars: &Bars) -> Measured {
        Measured {
            name: name.to_string(),
            value,
            unit,
            bar: bars.bar(name),
            control: None,
            verdict: bars.judge(name, value),
        }
    }

    /// The same, carrying the control arm's name and value for the record
    /// (R2.1): the number beside which this one is compared.
    pub fn against(mut self, control: &str, value: f64) -> Measured {
        self.control = Some((control.to_string(), value));
        self
    }
}

/// The error a Recorder reports at a component boundary. An unwritable page
/// names the path and the reason the write failed (R2.9); a missing Bars input
/// names the path (R2.8), though the Recorder continues recording in that case
/// rather than failing the open.
#[derive(Clone, Debug, PartialEq)]
pub enum RecordError {
    Unwritable { path: PathBuf, reason: String },
    NoBars { path: PathBuf },
}

impl std::fmt::Display for RecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordError::Unwritable { path, reason } => {
                write!(f, "cannot write record page {}: {reason}", path.display())
            }
            RecordError::NoBars { path } => {
                write!(f, "cannot read declared-Bars input {}", path.display())
            }
        }
    }
}

impl std::error::Error for RecordError {}

/// The Recorder accumulates the run's numbers and renders one markdown page at
/// `close`. It holds the target directory, the git stamp written at `open`,
/// the re-bar comparison against the most recent earlier page, and the numbers
/// written so far.
pub struct Recorder {
    dir: PathBuf,
    stamp: String,
    previous: std::collections::BTreeMap<String, Bar>,
    numbers: Vec<Measured>,
}

impl Recorder {
    /// Open the run's record page in `dir`, writing the git stamp of the
    /// workspace and of each submodule, clean or dirty, with up to 20 modified
    /// paths each when dirty (R2.5). The page is rendered at `close`; `open`
    /// gathers the stamp and the most recent earlier page's Bars so the
    /// re-barred comparison of R2.4 can run, and reports whether the Bars
    /// input was readable so R2.8 can surface the missing path.
    pub fn open(dir: &Path, bars: &Bars) -> Result<Recorder, RecordError> {
        let stamp = stamp::git_stamp(dir);
        let previous = parse::previous_bars(dir);
        if let Some(path) = &bars.unreadable {
            // R2.8: the missing input is reported, but recording continues, so
            // this is a surfaced note rather than a fatal error. The caller
            // (task 17.5) prints it; the Recorder carries on with every number
            // unjudged, which the empty Bars already guarantee.
            eprintln!("harness: declared-Bars input unreadable: {path}");
        }
        Ok(Recorder { dir: dir.to_path_buf(), stamp, previous, numbers: Vec::new() })
    }

    /// Write one measured number, flushing it before the next level starts
    /// (R2.1). The number is held in order and rendered at `close`; a missed
    /// Bar is kept unaltered for the rest of the run (R2.2) because `write`
    /// never revisits an earlier entry.
    pub fn write(&mut self, m: Measured) -> Result<(), RecordError> {
        self.numbers.push(m);
        Ok(())
    }

    /// The counts of passes, Misses and unjudged entries, and the total (R2.3).
    /// The three always partition the total: every `Measured` carries exactly
    /// one of the three verdicts.
    pub fn tally(&self) -> (usize, usize, usize, usize) {
        let mut pass = 0;
        let mut miss = 0;
        let mut unjudged = 0;
        for m in &self.numbers {
            match m.verdict {
                Judged::Pass => pass += 1,
                Judged::Miss { .. } => miss += 1,
                Judged::Unjudged => unjudged += 1,
            }
        }
        (pass, miss, unjudged, self.numbers.len())
    }

    /// The previous page's Bar for `name` and the new Bar, when they differ
    /// (R2.4). `None` when no earlier page declared this name, the new number
    /// carries no Bar, or the two are equal.
    pub fn rebarred(&self, name: &str, now: Option<Bar>) -> Option<(Bar, Bar)> {
        let now = now?;
        let was = self.previous.get(name)?;
        if *was == now {
            None
        } else {
            Some((*was, now))
        }
    }

    /// Render and write the run's record page, returning its path. Every
    /// earlier page is left byte-identical (R2.3): a new file is written at the
    /// next unused number and no earlier page is opened for writing. An
    /// unwritable page reports its path and the reason (R2.9), and because the
    /// write touches only the new file, every earlier page stays byte-identical
    /// on failure too.
    pub fn close(self) -> Result<PathBuf, RecordError> {
        let path = page::next_page(&self.dir);
        let body = page::render(&self.stamp, &self.numbers, &self.previous);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| RecordError::Unwritable {
                path: path.clone(),
                reason: e.to_string(),
            })?;
        }
        std::fs::write(&path, body).map_err(|e| RecordError::Unwritable {
            path: path.clone(),
            reason: e.to_string(),
        })?;
        Ok(path)
    }

    /// The numbers written so far, for the wiring in task 17.5 and the tests.
    pub fn numbers(&self) -> &[Measured] {
        &self.numbers
    }
}

/// The value the most recent earlier page recorded for `name`, if any.
pub fn previous_value(dir: &Path, name: &str) -> Option<f64> {
    parse::previous_value(dir, name)
}

/// Truncate `s` to at most `cap` characters, taking a prefix of the original
/// (R2.5's 20-name list, and the shared cap rule Property 4 pins over all
/// four declared caps). Counting is by `char`, so a multi-byte character is
/// never split.
pub fn cap(s: &str, cap: usize) -> String {
    s.chars().take(cap).collect()
}

/// Render one number's unit as the word the page and its parser agree on, one
/// of the six of `Unit`.
pub(crate) fn unit_word(u: Unit) -> &'static str {
    match u {
        Unit::Percentage => "percentage",
        Unit::Perplexity => "perplexity",
        Unit::Count => "count",
        Unit::ZScore => "zscore",
        Unit::F1 => "f1",
        Unit::Slope => "slope",
    }
}

/// Parse a unit word back, the inverse of `unit_word`, for the page parser.
pub(crate) fn unit_of(s: &str) -> Option<Unit> {
    match s.trim() {
        "percentage" => Some(Unit::Percentage),
        "perplexity" => Some(Unit::Perplexity),
        "count" => Some(Unit::Count),
        "zscore" => Some(Unit::ZScore),
        "f1" => Some(Unit::F1),
        "slope" => Some(Unit::Slope),
        _ => None,
    }
}

/// Render a Bar's sense as the word the page carries.
pub(crate) fn sense_word(s: Sense) -> &'static str {
    match s {
        Sense::AtLeast => "atleast",
        Sense::AtMost => "atmost",
    }
}

/// Parse a sense word back.
pub(crate) fn sense_of(s: &str) -> Option<Sense> {
    match s.trim() {
        "atleast" => Some(Sense::AtLeast),
        "atmost" => Some(Sense::AtMost),
        _ => None,
    }
}
