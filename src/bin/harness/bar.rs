//! The declared-Bars reader (Requirement 2 criteria 2, 6, 7 and 8).
//!
//! The Bars are read once from `phinet/docs/bars.md` before the first level
//! runs (R2.7) and judge every measured number thereafter. A name absent from
//! the input is `Unjudged`, never a pass (R2.6); an input that cannot be read
//! makes every number in the run `Unjudged` while recording continues (R2.8).
//!
//! This file owns the four small types the Recorder (task 17.2) also needs —
//! `Unit`, `Sense`, `Bar` and `Judged` — so the Recorder `use`s them from here
//! rather than declaring a second copy, which the no-duplicate-types rule
//! forbids. The reader lands before task 17.5, so no number is ever judged
//! against a Bar that was not declared first.
//!
//! The format is a markdown table whose data rows begin with a backtick-quoted
//! name cell carrying, in order, the name, the Bar value, the `Sense`
//! (`AtLeast`/`AtMost`) and the unit (one of the six of `Unit`); further cells
//! (the human-facing control column) are ignored, and a row whose first cell
//! is not backtick-quoted is skipped, so the table reads as plain docs.

use std::collections::BTreeMap;
use std::path::Path;

/// The closed unit list of R2.1. Shared with the Recorder (task 17.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Percentage,
    Perplexity,
    Count,
    ZScore,
    F1,
    Slope,
}

impl Unit {
    /// Parse a unit cell, case-insensitively over the six declared names.
    fn parse(s: &str) -> Option<Unit> {
        match s.trim().to_ascii_lowercase().as_str() {
            "percentage" => Some(Unit::Percentage),
            "perplexity" => Some(Unit::Perplexity),
            "count" => Some(Unit::Count),
            "zscore" | "z-score" => Some(Unit::ZScore),
            "f1" => Some(Unit::F1),
            "slope" => Some(Unit::Slope),
            _ => None,
        }
    }
}

/// Which way a Bar faces. A purity is met by rising to or above its value; a
/// perplexity is met by falling to or below it. The sense is declared with the
/// Bar in `bars.md`, never inferred from the unit, because a judge that
/// assumed every Bar is a floor would pass a perplexity of 500.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sense {
    AtLeast,
    AtMost,
}

impl Sense {
    /// Parse a sense cell, case-insensitively.
    fn parse(s: &str) -> Option<Sense> {
        match s.trim().to_ascii_lowercase().as_str() {
            "atleast" | "at least" | "at-least" => Some(Sense::AtLeast),
            "atmost" | "at most" | "at-most" => Some(Sense::AtMost),
            _ => None,
        }
    }

    /// Is `value` on the met side of `bar` under this sense? The boundary is
    /// inclusive both ways: a value exactly at the Bar passes.
    fn met(self, value: f64, bar: f64) -> bool {
        match self {
            Sense::AtLeast => value >= bar,
            Sense::AtMost => value <= bar,
        }
    }
}

/// One declared Bar: the value and the sense it faces.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bar {
    pub value: f64,
    pub sense: Sense,
}

/// The verdict a Bar returns for a measured number (R2.2, R2.6, R2.8). Exactly
/// three outcomes: a pass, a Miss carrying the signed shortfall, or an
/// unjudged number whose name no Bar declared.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Judged {
    Pass,
    /// The signed distance from the Bar in the number's own unit (R2.2):
    /// `value - bar`, which is negative when an `AtLeast` Bar is missed from
    /// below and positive when an `AtMost` Bar is missed from above.
    Miss {
        shortfall: f64,
    },
    /// No Bar for this name in the declared-Bars input, or the input could not
    /// be read (R2.6, R2.8).
    Unjudged,
}

/// The Bars, read once before the first level runs (R2.7). Holds the current
/// page's declared Bars and, for the re-barred comparison of R2.4, the Bar
/// each name carried on the most recent earlier record page.
#[derive(Clone, Debug, Default)]
pub struct Bars {
    bars: BTreeMap<String, Bar>,
    /// The most recent earlier record page's Bar per name, for R2.4. The
    /// Recorder (task 17.2) fills this from the previous page; the reader
    /// leaves it empty.
    previous: BTreeMap<String, Bar>,
    /// Set to the path when the input could not be read: every number is then
    /// `Unjudged` and recording continues (R2.8).
    pub unreadable: Option<String>,
}

impl Bars {
    /// Read the declared Bars from `path`, once, before the first level runs
    /// (R2.7). An unreadable input reports the missing path in `unreadable`
    /// and leaves the table empty, so every `judge` answers `Unjudged` while
    /// recording continues (R2.8).
    pub fn read(path: &Path) -> Bars {
        match std::fs::read_to_string(path) {
            Ok(text) => Bars { bars: parse(&text), previous: BTreeMap::new(), unreadable: None },
            Err(e) => Bars {
                bars: BTreeMap::new(),
                previous: BTreeMap::new(),
                unreadable: Some(format!("{}: {e}", path.display())),
            },
        }
    }

    /// Build Bars directly from a name-to-Bar map, for the Recorder and tests.
    pub fn from_map(bars: BTreeMap<String, Bar>) -> Bars {
        Bars { bars, previous: BTreeMap::new(), unreadable: None }
    }

    /// The declared Bar for `name`, or `None` when the name is absent or the
    /// input was unreadable.
    pub fn bar(&self, name: &str) -> Option<Bar> {
        if self.unreadable.is_some() {
            return None;
        }
        self.bars.get(name).copied()
    }

    /// The number of declared Bars read.
    pub fn len(&self) -> usize {
        self.bars.len()
    }

    /// Whether any Bar was declared.
    pub fn is_empty(&self) -> bool {
        self.bars.is_empty()
    }

    /// Judge `value` against the Bar declared for `name` (R2.2, R2.6, R2.8).
    /// An absent name — or any name when the input was unreadable — is
    /// `Unjudged`, never a pass. A met Bar is a `Pass`; a missed Bar is a
    /// `Miss` whose shortfall is `value - bar`, the signed distance in the
    /// number's own unit.
    pub fn judge(&self, name: &str, value: f64) -> Judged {
        match self.bar(name) {
            None => Judged::Unjudged,
            Some(bar) => {
                if bar.sense.met(value, bar.value) {
                    Judged::Pass
                } else {
                    Judged::Miss { shortfall: value - bar.value }
                }
            }
        }
    }

    /// The previous and new Bar for `name` when they differ, for R2.4's
    /// re-barred mark. `None` when no earlier page declared this name or the
    /// two are equal; the reader leaves `previous` empty, so a freshly read
    /// `Bars` reports no re-barring until the Recorder seeds the prior page.
    pub fn rebarred(&self, name: &str) -> Option<(Bar, Bar)> {
        let now = self.bars.get(name)?;
        let was = self.previous.get(name)?;
        if was == now { None } else { Some((*was, *now)) }
    }

    /// Seed the previous-page Bars for the re-barred comparison (R2.4), used by
    /// the Recorder once it has read the most recent earlier page.
    pub fn with_previous(mut self, previous: BTreeMap<String, Bar>) -> Bars {
        self.previous = previous;
        self
    }
}

/// Parse the data rows of `bars.md`: every row whose first cell is a
/// backtick-quoted name. A row missing a well-formed value, sense or unit is
/// skipped rather than crashing the read — a malformed Bar must not silently
/// judge, so its number is simply absent and thus `Unjudged`.
fn parse(text: &str) -> BTreeMap<String, Bar> {
    let mut out = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() < 4 {
            continue;
        }
        let name = match backtick(cells[0]) {
            Some(n) => n,
            None => continue,
        };
        let value = match cells[1].parse::<f64>() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let sense = match Sense::parse(cells[2]) {
            Some(s) => s,
            None => continue,
        };
        if Unit::parse(cells[3]).is_none() {
            continue;
        }
        out.insert(name.to_string(), Bar { value, sense });
    }
    out
}

/// The content of a single backtick-quoted cell, or `None` when the cell is
/// not exactly one backtick-quoted token.
fn backtick(cell: &str) -> Option<&str> {
    let inner = cell.strip_prefix('`')?.strip_suffix('`')?;
    if inner.is_empty() || inner.contains('`') {
        None
    } else {
        Some(inner)
    }
}

#[cfg(test)]
mod tests;
