//! Property 7: the figures `docs/english_reader.md` quotes match the harness
//! record it names (Requirement 6.5).
//!
//! The doc's first paragraph names one run record. Both files are read at test
//! time by path from `CARGO_MANIFEST_DIR`; the record is read back with the
//! Recorder's own page parser. Every table row and every prose figure the doc
//! quotes is looked up by name in the record and compared after the rounding
//! the record uses: one decimal for a percentage, none for a count, two for a
//! perplexity or z-score. A failure names the metric and both values.
use crate::bars::{Bar, Judged, Sense, Unit};
use crate::record::{parse::parse, Measured};
use std::path::{Path, PathBuf};
const DOC: &str = "docs/english_reader.md";
/// The workspace root (`phinet`) from this crate's manifest directory.
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
/// `v` rounded the way the record reports it for `unit`.
fn rounded(v: f64, unit: Unit) -> f64 {
    let places = match unit {
        Unit::Percentage => 1,
        Unit::Count => 0,
        _ => 2,
    };
    let k = 10f64.powi(places);
    (v * k).round() / k
}
fn close(a: f64, b: f64) -> bool {
    (a.is_nan() && b.is_nan()) || (a - b).abs() < 1e-9
}
/// The record named in the doc's first paragraph: the first backticked
/// `*_harness.md` token. Not the newest page.
fn named_record(doc: &str) -> String {
    // The paragraph under the title: lines up to the first blank line or table.
    let first = doc
        .lines()
        .filter(|l| !l.starts_with('#'))
        .skip_while(|l| l.trim().is_empty())
        .take_while(|l| !l.trim().is_empty() && !l.starts_with('|'))
        .collect::<Vec<_>>()
        .join(" ");
    first
        .split('`')
        .skip(1)
        .step_by(2)
        .find(|t| t.ends_with("_harness.md"))
        .expect("the doc's opening names no `*_harness.md` record")
        .to_string()
}
struct Check<'a> {
    rec: &'a [Measured],
    checked: usize,
    fails: Vec<String>,
}
impl Check<'_> {
    /// The record entry for a doc name, bare or full.
    fn find(&mut self, name: &str) -> Option<&Measured> {
        let name = name.trim_matches('`');
        let hit = self
            .rec
            .iter()
            .find(|m| m.name == name)
            .or_else(|| self.rec.iter().find(|m| m.name == format!("ladder.act.{name}")))
            .or_else(|| self.rec.iter().find(|m| m.name == format!("ladder.{name}")));
        if hit.is_none() {
            self.fails.push(format!("{name}: the doc quotes it, the record has no such metric"));
        }
        hit
    }
    fn value(&mut self, name: &str, cell: &str) {
        let Some(m) = self.find(name).cloned() else { return };
        self.checked += 1;
        let cell = cell.trim().trim_matches('`');
        let want = rounded(m.value, m.unit);
        let ok = if cell == "absent" {
            m.value.is_nan()
        } else {
            cell.parse::<f64>().is_ok_and(|d| close(d, want))
        };
        if !ok {
            self.fails.push(format!("{}: doc says {cell}, record says {want} (raw {})", m.name, m.value));
        }
    }
    fn bar(&mut self, name: &str, cell: &str) {
        let Some(m) = self.find(name).cloned() else { return };
        self.checked += 1;
        let cell = cell.trim();
        let ok = match (m.bar, cell) {
            (None, "none declared") => true,
            (Some(Bar { value, sense }), c) => {
                let word = if sense == Sense::AtLeast { "at least" } else { "at most" };
                c.strip_suffix(word)
                    .and_then(|n| n.trim().parse::<f64>().ok())
                    .is_some_and(|n| close(n, value))
            }
            _ => false,
        };
        if !ok {
            self.fails.push(format!("{}: doc bar `{cell}`, record bar {:?}", m.name, m.bar));
        }
    }
    /// The bar's value only, as the closing prose gives it without a sense.
    fn bar_value(&mut self, name: &str, cell: &str) {
        let Some(m) = self.find(name).cloned() else { return };
        self.checked += 1;
        let ok = matches!((m.bar, cell.parse::<f64>()), (Some(b), Ok(d)) if close(b.value, d));
        if !ok {
            self.fails.push(format!("{}: doc bar `{cell}`, record bar {:?}", m.name, m.bar));
        }
    }
    fn verdict(&mut self, name: &str, cell: &str) {
        let Some(m) = self.find(name).cloned() else { return };
        self.checked += 1;
        let cell = cell.trim();
        let ok = match m.verdict {
            Judged::Pass => cell == "pass",
            Judged::Unjudged => cell == "unjudged",
            Judged::Miss { shortfall } => cell
                .strip_prefix("miss by ")
                .and_then(|r| r.split(|c: char| !(c.is_ascii_digit() || c == '.')).next())
                .and_then(|n| n.parse::<f64>().ok())
                .is_some_and(|n| close(n, rounded(shortfall.abs(), Unit::Percentage))),
        };
        if !ok {
            self.fails.push(format!("{}: doc verdict `{cell}`, record verdict {:?}", m.name, m.verdict));
        }
    }
}
fn cells(line: &str) -> Vec<&str> {
    line.trim().trim_matches('|').split('|').map(str::trim).collect()
}
/// Check every figure the doc quotes against the record's page. Returns the
/// number of comparisons made and one message per mismatch.
fn check(doc: &str, record: &str) -> (usize, Vec<String>) {
    let rec = parse(record);
    let mut c = Check { rec: &rec, checked: 0, fails: Vec::new() };
    let mut head: Vec<String> = Vec::new();
    let mut in_table = false;
    for line in doc.lines() {
        if !line.trim_start().starts_with('|') {
            in_table = false;
            continue;
        }
        let cs = cells(line);
        if cs[0].starts_with("---") {
            continue;
        }
        if !in_table {
            // The first row of a table is its header.
            head = cs.iter().map(|s| s.to_string()).collect();
            in_table = true;
        } else {
            table_row(&mut c, &head, &cs);
        }
    }
    prose(&mut c, &doc.split_whitespace().collect::<Vec<_>>().join(" "));
    (c.checked, c.fails)
}
/// The number that starts `text`, as written.
fn leading(text: &str) -> String {
    let s: String = text.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
    s.trim_end_matches('.').to_string()
}
/// Figures quoted in prose, each found by a fixed anchor phrase. A missing
/// anchor is a failure, so reworded prose cannot silently escape the check.
fn prose(c: &mut Check, text: &str) {
    let after = |anchor: &str| text.split_once(anchor).map(|(_, r)| leading(r));
    let before = |anchor: &str| {
        let (l, _) = text.split_once(anchor)?;
        let w = l.rsplit(' ').next()?;
        Some(w.to_string())
    };
    let facts = [
        ("clean_kept", before(" kept,")),
        ("clean_removed", before(" removed. `clean_soundness_regression`")),
        ("clean_soundness_regression", after("`clean_soundness_regression` is ")),
        ("abstract_learned_predicted", after("The layer predicted ")),
        ("abstract_learned_abstained", after("abstained on ")),
        ("abstract_gate_enabled", after("`abstract_gate_enabled` = ")),
        ("abstract_gate_beats_shuffled", after("`abstract_gate_beats_shuffled` = ")),
        ("abstract_gate_beats_frequency", after("`abstract_gate_beats_frequency` = ")),
        ("force_purity_reachable", after("Reachable-only force purity (")),
        ("round_trip_reachable", after("Reachable-only round-trip (")),
        ("round_trip", after("the all-150 figure of ")),
    ];
    for (name, found) in facts {
        match found {
            Some(v) => c.value(name, &v),
            None => c.fails.push(format!("{name}: the prose anchor for it is gone from the doc")),
        }
    }
    // "(<value> against <bar>" in the closing list of missed bars.
    for name in ["force_purity", "round_trip", "function_purity.learned", "curriculum.perplexity"] {
        let anchor = format!("`{name}` (");
        let Some(rest) = after_raw(text, &anchor) else {
            c.fails.push(format!("{name}: the missed-bar sentence is gone from the doc"));
            continue;
        };
        let v = leading(rest);
        let bar = rest[v.len()..].strip_prefix(" against ").map(leading).unwrap_or_default();
        c.value(name, &v);
        c.bar_value(name, &bar);
    }
}
fn after_raw<'t>(text: &'t str, anchor: &str) -> Option<&'t str> {
    text.split_once(anchor).map(|(_, r)| r)
}

fn table_row(c: &mut Check, head: &[String], cs: &[&str]) {
    let h: Vec<&str> = head.iter().map(String::as_str).collect();
    let name = cs[0].trim_matches('`');
    match h.as_slice() {
        ["measure", "record 234", "record 235", ..] => {
            c.value(name, cs[2]);
            c.bar(name, cs[3]);
            c.verdict(name, cs[4]);
        }
        ["measure", "all 150", ..] => {
            c.value(name, cs[1]);
            c.value(&format!("{name}_reachable"), cs[2]);
            c.bar(name, cs[3]);
            c.verdict(name, cs[4]);
        }
        ["measure", "value", "bar", ..] => {
            c.value(name, cs[1]);
            c.bar(name, cs[2]);
            c.verdict(name, cs[3]);
        }
        ["operator", .., "removed"] => c.value(&format!("operator.{name}"), cs[2]),
        ["kind", "count"] => c.value(&format!("abstain.{name}"), cs[1]),
        ["list", "covers", ..] => {
            c.value(&format!("wordlist.{name}.covers"), cs[1]);
            c.value(&format!("wordlist.{name}.would_change"), cs[2]);
        }
        ["level", "arm", ..] => {
            let arm = cs[1].trim_end_matches(" control");
            for (i, kind) in [(2, "top1"), (3, "perplexity"), (4, "predicted")] {
                c.value(&format!("{}_{arm}_{kind}", cs[0]), cs[i]);
            }
        }
        _ => {}
    }
}
#[test]
fn documented_figures_match_the_harness_record() {
    let doc = std::fs::read_to_string(root().join(DOC)).expect("docs/english_reader.md is readable");
    let name = named_record(&doc);
    let record = std::fs::read_to_string(root().join("docs").join(&name))
        .unwrap_or_else(|e| panic!("the record {name} the doc names is unreadable: {e}"));
    let (checked, fails) = check(&doc, &record);
    assert!(checked >= 90, "only {checked} figures were compared; the doc tables were not read");
    assert!(fails.is_empty(), "{} doc figure(s) drifted from {name}:\n{}", fails.len(), fails.join("\n"));
}
#[test]
fn a_drifted_figure_fails_and_names_the_metric_and_both_values() {
    let doc = std::fs::read_to_string(root().join(DOC)).unwrap();
    let record = std::fs::read_to_string(root().join("docs").join(named_record(&doc))).unwrap();
    let drifted = doc.replace("| `foundation_coverage` | 29.5 | 31.1 |", "| `foundation_coverage` | 29.5 | 31.2 |");
    assert_ne!(drifted, doc, "the row to drift is gone from the doc");
    let (_, fails) = check(&drifted, &record);
    assert_eq!(fails.len(), 1, "{fails:?}");
    assert!(fails[0].contains("foundation_coverage") && fails[0].contains("31.2") && fails[0].contains("31.1"));
}
