//! The page parser — Property 5's oracle — and the re-barred lookup of R2.4.
//!
//! `parse` reads a rendered page's Numbers table back into its `Measured`
//! rows, so a test can confirm each number appears exactly once with its name,
//! value, unit, Bar, control and verdict, and that the verdict counts
//! partition the total. `previous_bars` reads the most recent earlier page's
//! Bar per name, so a Bar that drifts is marked re-barred.

use super::{sense_of, unit_of, Bar, Judged, Measured};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Parse a rendered page back into its numbers, Property 5's oracle.
pub fn parse(body: &str) -> Vec<Measured> {
    let mut out = Vec::new();
    let mut in_numbers = false;
    for line in body.lines() {
        if line.trim_start().starts_with("## Numbers") {
            in_numbers = true;
            continue;
        }
        if !in_numbers || !line.trim_start().starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line.trim().trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() < 7 || cells[0] == "name" || cells[0].starts_with("---") {
            continue;
        }
        if let Some(m) = parse_row(&cells) {
            out.push(m);
        }
    }
    out
}

/// Reconstruct one `Measured` from a parsed row's seven cells.
fn parse_row(c: &[&str]) -> Option<Measured> {
    let name = c[0].strip_prefix('`')?.strip_suffix('`')?.to_string();
    Some(Measured {
        name,
        value: num(c[1])?,
        unit: unit_of(c[2])?,
        bar: parse_bar(c[3]),
        control: parse_control(c[4]),
        verdict: parse_verdict(c[5])?,
    })
}

/// Parse the Bar cell: `-` for none, else `<value> <sense>`.
fn parse_bar(s: &str) -> Option<Bar> {
    if s.trim() == "-" {
        return None;
    }
    let mut it = s.split_whitespace();
    Some(Bar { value: num(it.next()?)?, sense: sense_of(it.next()?)? })
}

/// Parse the control cell: `-` for none, else `<name> <value>`.
fn parse_control(s: &str) -> Option<(String, f64)> {
    if s.trim() == "-" {
        return None;
    }
    let (name, val) = s.trim().rsplit_once(' ')?;
    Some((name.to_string(), num(val)?))
}

/// Parse the verdict cell: `pass`, `unjudged`, or `miss <shortfall>`.
fn parse_verdict(s: &str) -> Option<Judged> {
    let s = s.trim();
    match s {
        "pass" => Some(Judged::Pass),
        "unjudged" => Some(Judged::Unjudged),
        _ => s.strip_prefix("miss ").and_then(num).map(|shortfall| Judged::Miss { shortfall }),
    }
}

/// Parse a float the way `page::fmt` wrote it, including the three non-finite
/// names.
fn num(s: &str) -> Option<f64> {
    match s.trim() {
        "nan" => Some(f64::NAN),
        "inf" => Some(f64::INFINITY),
        "-inf" => Some(f64::NEG_INFINITY),
        t => t.parse().ok(),
    }
}

/// The three-digit number at the start of a `<nnn>_...` page name, or `None`.
pub(super) fn leading_number(name: &str) -> Option<u32> {
    let digits: String = name.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.len() == 3 && name.as_bytes().get(3) == Some(&b'_') {
        digits.parse().ok()
    } else {
        None
    }
}

/// The most recent earlier page's Bar per number name, for the re-barred
/// comparison of R2.4. Reads the highest-numbered page in `dir` and parses its
/// Bars; an empty directory yields no previous Bars.
pub fn previous_bars(dir: &Path) -> BTreeMap<String, Bar> {
    let mut out = BTreeMap::new();
    let path = match highest(dir) {
        Some((_, p)) => p,
        None => return out,
    };
    let body = match std::fs::read_to_string(&path) {
        Ok(b) => b,
        Err(_) => return out,
    };
    for m in parse(&body) {
        if let Some(bar) = m.bar {
            out.insert(m.name, bar);
        }
    }
    out
}

/// The value the most recent earlier page recorded for `name`, or `None` when
/// there is no earlier page or it holds no finite value under that name.
pub fn previous_value(dir: &Path, name: &str) -> Option<f64> {
    let body = std::fs::read_to_string(highest(dir)?.1).ok()?;
    parse(&body).into_iter().find(|m| m.name == name && m.value.is_finite()).map(|m| m.value)
}

/// The numbered page in `dir` with the greatest leading number, as
/// `(number, path)`, or `None` when it holds none. The one scan: the next
/// page's number is read from it (`page::next_page`) and so is the most
/// recent earlier page's body.
pub(super) fn highest(dir: &Path) -> Option<(u32, PathBuf)> {
    let mut best: Option<(u32, PathBuf)> = None;
    for e in std::fs::read_dir(dir).ok()?.flatten() {
        let Some(n) = leading_number(&e.file_name().to_string_lossy()) else { continue };
        match best.as_ref() {
            Some((b, _)) if n <= *b => {}
            _ => best = Some((n, e.path())),
        }
    }
    best
}
