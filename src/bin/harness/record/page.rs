//! The run record page's name and rendered body (Requirement 2 criteria 1, 3
//! and 4). The parser that reads a page back — Property 5's oracle, and the
//! re-barred lookup of R2.4 — lives in the sibling `parse` module.
//!
//! The body is a stable, machine-parseable table. Each number is one row
//! carrying, in a fixed order, the name, value, unit, Bar (value and sense, or
//! `-`), control name and value (or `-`), verdict, and the re-barred mark when
//! the Bar drifts from the most recent earlier page.

use super::parse::leading_number;
use super::{sense_word, unit_word, Bar, Judged, Measured, SERIES_START};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The slug every run record carries: fixed, because R2 criterion 3 fixes the
/// name to the run's date and the directory's numbered convention.
pub(super) const SLUG: &str = "harness";

/// The next unused page path under `dir`: `<nnn>_<yyyy-mm-dd>_harness.md`,
/// continuing the numbered series from 208 (R2.3). The number is one past the
/// highest `<nnn>_` already present, or `SERIES_START` when none is.
pub fn next_page(dir: &Path) -> PathBuf {
    let next = highest_number(dir).map(|n| n + 1).unwrap_or(SERIES_START);
    dir.join(format!("{next:03}_{}_{SLUG}.md", today()))
}

/// The highest leading three-digit number among the markdown pages in `dir`,
/// or `None` when the directory holds none.
fn highest_number(dir: &Path) -> Option<u32> {
    let entries = std::fs::read_dir(dir).ok()?;
    let mut best: Option<u32> = None;
    for e in entries.flatten() {
        let name = e.file_name();
        if let Some(n) = leading_number(&name.to_string_lossy()) {
            best = Some(best.map_or(n, |b| b.max(n)));
        }
    }
    best
}

/// Today's date as `yyyy-mm-dd` in UTC, from the system clock with no outside
/// crate: the civil date from the Unix day count.
fn today() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (y, m, d) = civil_from_days((secs / 86_400) as i64);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Howard Hinnant's days-to-civil conversion: a Unix day count to (year,
/// month, day), pure integer arithmetic over the Gregorian calendar.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// Render the whole page: the git stamp, the count header with the verdict
/// partition, and one row per number (R2.1, R2.3). `previous` gives the most
/// recent earlier page's Bar per name for the re-barred mark (R2.4).
pub fn render(stamp: &str, numbers: &[Measured], previous: &BTreeMap<String, Bar>) -> String {
    let (pass, miss, unjudged) = partition(numbers);
    let mut s = String::new();
    s.push_str(&format!("# {SLUG} run record\n\n## Working trees\n\n"));
    s.push_str(stamp);
    s.push_str("\n\n## Totals\n\n");
    s.push_str(&format!(
        "recorded {}; passes {pass}; misses {miss}; unjudged {unjudged}\n\n",
        numbers.len()
    ));
    s.push_str("## Numbers\n\n");
    s.push_str("| name | value | unit | bar | control | verdict | rebarred |\n");
    s.push_str("|---|---|---|---|---|---|---|\n");
    for m in numbers {
        s.push_str(&row(m, previous));
        s.push('\n');
    }
    s
}

/// The verdict partition: passes, Misses and unjudged (R2.3).
fn partition(numbers: &[Measured]) -> (usize, usize, usize) {
    let (mut p, mut mi, mut u) = (0, 0, 0);
    for m in numbers {
        match m.verdict {
            Judged::Pass => p += 1,
            Judged::Miss { .. } => mi += 1,
            Judged::Unjudged => u += 1,
        }
    }
    (p, mi, u)
}

/// One number's row, every cell escaped so a stray `|` cannot split it.
fn row(m: &Measured, previous: &BTreeMap<String, Bar>) -> String {
    let bar = match &m.bar {
        Some(b) => format!("{} {}", fmt(b.value), sense_word(b.sense)),
        None => "-".to_string(),
    };
    let control = match &m.control {
        Some((n, v)) => format!("{} {}", esc(n), fmt(*v)),
        None => "-".to_string(),
    };
    let verdict = match m.verdict {
        Judged::Pass => "pass".to_string(),
        Judged::Miss { shortfall } => format!("miss {}", fmt(shortfall)),
        Judged::Unjudged => "unjudged".to_string(),
    };
    let rebarred = match (previous.get(&m.name), &m.bar) {
        (Some(was), Some(now)) if was != now => format!("{} -> {}", fmt(was.value), fmt(now.value)),
        _ => "-".to_string(),
    };
    format!(
        "| `{}` | {} | {} | {} | {} | {} | {} |",
        esc(&super::cap(&m.name, 200)),
        fmt(m.value),
        unit_word(m.unit),
        bar,
        control,
        verdict,
        rebarred
    )
}

/// Format a float stably: a finite value with no trailing noise, the three
/// non-finite values by name so the parser reads them back.
pub(super) fn fmt(v: f64) -> String {
    if v.is_nan() {
        "nan".to_string()
    } else if v.is_infinite() {
        if v > 0.0 {
            "inf".to_string()
        } else {
            "-inf".to_string()
        }
    } else {
        format!("{v}")
    }
}

/// Escape a cell's pipe and newline so a value never splits a row.
fn esc(s: &str) -> String {
    s.replace('|', "/").replace('\n', " ")
}
