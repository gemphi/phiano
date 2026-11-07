//! The English reader's held-out measures (`ladder.act.*`).
//!
//! The reader is measured on the Guide's Held_Out_Slice only (its Test part,
//! one sentence in ten, never tuned on). Two views: the foundation view (every
//! Test sentence) and the clean view (the same sentences after the text-only
//! Well_Formed operators). Each view gives coverage (share read) and
//! soundness (share of the read that round-trips under Canonical_Form).
//!
//! Bars for the clean view are declared here in code, so they hold even when
//! `docs/bars.md` is silent; a Bar in `bars.md` for the same name wins. A
//! clean-soundness drop of more than [`REGRESSION_POINTS`] from the previous
//! record is flagged as a regression and names the change under test.
//!
//! A metric that cannot be computed is written as a named absence: the same
//! name with a NaN value and no verdict, never a zero.

use super::ladder::one_decimal;
use super::bars::{Bar, Bars, Judged, Sense, Unit};
use super::record::{self, Measured};
use phisor::guide::{Part, Split};
use std::path::Path;
use std::process::Command;

/// Declared Bar: clean coverage, percent of clean held-out sentences read.
pub const CLEAN_COVERAGE_BAR: f64 = 32.5;
/// Declared Bar: clean soundness, percent of the read that round-trips.
pub const CLEAN_SOUNDNESS_BAR: f64 = 76.8;
/// A clean-soundness drop larger than this many points is a regression.
pub const REGRESSION_POINTS: f64 = 1.0;

/// Measure the reader on the Held_Out_Slice and return every `ladder.act`
/// held-out record. `docs` is where the previous run record lives.
pub fn measure(split: &Split, bars: &Bars, docs: &Path) -> Vec<Measured> {
    let mut out = Vec::new();
    let heldout = split.strings(Part::Test);
    let clean = split.clean_strings(Part::Test);

    out.extend(round_trip_canonical(bars));
    out.extend(view("foundation", &heldout, bars, None));
    // What the filter did, so the cleaned figures can be read honestly.
    out.push(count("ladder.act.clean_kept", clean.len(), bars));
    out.push(count("ladder.act.clean_removed", heldout.len() - clean.len(), bars));
    for (name, n) in split.removed_by(Part::Test) {
        let rule = phisor::guide::OPERATORS.iter().find(|o| o.name == name).map_or("", |o| o.rule);
        eprintln!("ladder.act.operator.{name}: {n} removed ({rule})");
        out.push(count(&format!("ladder.act.operator.{name}"), n, bars));
    }
    let clean_view = view("clean", &clean, bars, Some((CLEAN_COVERAGE_BAR, CLEAN_SOUNDNESS_BAR)));
    out.extend(regression(&clean_view, docs, bars));
    out.extend(clean_view);

    // Why the reader still abstains on the clean sentences, by named kind.
    let mut kinds = vec![0usize; phisor::ladder::ABSTENTION_KINDS.len()];
    for s in clean.iter().filter(|s| phisor::ladder::read(s).is_none()) {
        let kind = phisor::ladder::abstention_kind(s);
        if let Some(i) = phisor::ladder::ABSTENTION_KINDS.iter().position(|k| *k == kind) {
            kinds[i] += 1;
        }
    }
    for (kind, n) in phisor::ladder::ABSTENTION_KINDS.iter().zip(kinds) {
        out.push(count(&format!("ladder.act.abstain.{kind}"), n, bars));
    }

    // Audit each hand-authored Reader word list against the Held_Out_Slice:
    // how many held-out words the list covers, and how many would change if
    // the list were decided by the foundation lexicon instead (task 5.1,
    // R5.2, R5.3). Counts only; no held-out word reaches a Reader decision.
    out.extend(word_lists(&heldout, bars));
    // Abstract and surface prediction with control arms, and the gate (task 9.6).
    out.extend(predict::measure(split, bars));
    out
}

/// `ladder.act.round_trip_canonical`: the gold realisation round trip with
/// Canonical_Form on both sides, reported beside the strict
/// `ladder.act.round_trip` (which stays strict and is emitted by `ladder.rs`).
/// Unjudged unless a Bar names it. A gold set that cannot load is a named
/// absence, never a zero.
fn round_trip_canonical(bars: &Bars) -> Vec<Measured> {
    const NAME: &str = "ladder.act.round_trip_canonical";
    let rate = phisor::gold::read().ok().and_then(|gold| {
        let (matched, total, _) = phisor::ladder::round_trip_canonical(gold.pairs());
        (total > 0).then(|| one_decimal(100.0 * matched as f64 / total as f64))
    });
    vec![percent(NAME, rate, bars, None)]
}

/// The distinct lower-case tokens of the Held_Out_Slice, the vocabulary the
/// word-list audit measures each list's reach over. Terminating marks are
/// already off the reconstructed strings.
fn heldout_vocabulary(sentences: &[String]) -> std::collections::BTreeSet<String> {
    sentences
        .iter()
        .flat_map(|s| s.trim_end_matches(['.', '?']).split_whitespace())
        .map(str::to_lowercase)
        .collect()
}

/// Whether the foundation lexicon - the learned source - would place `word` in
/// the list category `name`. This is the "if the list were learned instead"
/// arm of R5.3: a verb list is decided by verb-capability, the adverb list by
/// an adverbial tag, the conjunction list by the closed-class function words.
/// An unprimed or unknown word is not placed, so the arm reports a real learned
/// verdict and never borrows the hand list's answer.
fn learned_in_list(name: &str, word: &str) -> bool {
    match name {
        "control_verbs" | "complement_verbs" | "performatives" => phisor::lexicon::verb_capable(word),
        "set_aside_adverbs" => phisor::lexicon::tag(word).is_some_and(|t| t.contains('r')),
        "clause_conjunctions" => phisor::lexicon::function_word(word),
        _ => false,
    }
}

/// Per hand-authored Reader word list, two counts over the Held_Out_Slice's
/// vocabulary: `ladder.act.wordlist.<name>.covers`, the held-out words the
/// hand list contains, and `ladder.act.wordlist.<name>.would_change`, the
/// held-out words whose membership the foundation lexicon would decide
/// differently (task 5.1, R5.2, R5.3). Both are plain counts, judged only when
/// a Bar names them; they measure a list, they do not move the Reader.
fn word_lists(heldout: &[String], bars: &Bars) -> Vec<Measured> {
    let vocab = heldout_vocabulary(heldout);
    let mut out = Vec::new();
    for (name, words) in phisor::ladder::word_lists() {
        let in_hand: std::collections::BTreeSet<&str> = words.iter().map(String::as_str).collect();
        let covers = vocab.iter().filter(|w| in_hand.contains(w.as_str())).count();
        let would_change =
            vocab.iter().filter(|w| in_hand.contains(w.as_str()) != learned_in_list(name, w)).count();
        eprintln!("ladder.act.wordlist.{name}: covers {covers} held-out words, {would_change} would change if learned");
        out.push(count(&format!("ladder.act.wordlist.{name}.covers"), covers, bars));
        out.push(count(&format!("ladder.act.wordlist.{name}.would_change"), would_change, bars));
    }
    out
}

/// Coverage and soundness of the reader over `sentences`, named
/// `ladder.act.<view>_coverage` and `..._soundness`. `declared` carries the
/// in-code Bars. An uncomputable figure is a named absence.
fn view(name: &str, sentences: &[String], bars: &Bars, declared: Option<(f64, f64)>) -> Vec<Measured> {
    let (matched, covered, total) =
        phisor::ladder::read_realise_rate(sentences.iter().map(String::as_str));
    let coverage = (total > 0).then(|| one_decimal(100.0 * covered as f64 / total as f64));
    let soundness = (covered > 0).then(|| one_decimal(100.0 * matched as f64 / covered as f64));
    vec![
        percent(&format!("ladder.act.{name}_coverage"), coverage, bars, declared.map(|d| d.0)),
        percent(&format!("ladder.act.{name}_soundness"), soundness, bars, declared.map(|d| d.1)),
    ]
}

/// A flag record: 1 when clean soundness fell by more than the allowed points
/// against the previous run record, else 0. Absent when either side is.
fn regression(clean_view: &[Measured], docs: &Path, bars: &Bars) -> Vec<Measured> {
    const NAME: &str = "ladder.act.clean_soundness";
    let now = clean_view.iter().find(|m| m.name == NAME).map(|m| m.value).filter(|v| v.is_finite());
    let before = record::previous_value(docs, NAME);
    let flag = "ladder.act.clean_soundness_regression";
    let (Some(now), Some(before)) = (now, before) else {
        eprintln!("{flag}: absent (no comparable clean soundness in this or the previous record)");
        return vec![absent(flag, Unit::Count)];
    };
    let regressed = before - now > REGRESSION_POINTS;
    if regressed {
        eprintln!(
            "{flag}: REGRESSION clean soundness {before:.1} -> {now:.1} (more than {REGRESSION_POINTS} points) by change: {}",
            change_under_test()
        );
    }
    vec![count(flag, usize::from(regressed), bars)]
}

/// The commit under test, named by short hash and subject; a fixed note when
/// git cannot say.
fn change_under_test() -> String {
    Command::new("git")
        .args(["log", "-1", "--format=%h %s"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown (git log unavailable)".to_string())
}

/// A count record judged by `bars` (unjudged when no Bar names it).
fn count(name: &str, n: usize, bars: &Bars) -> Measured {
    Measured::judged(name, n as f64, Unit::Count, bars)
}

/// A percentage, judged by `bars.md` if it names the figure, else by the
/// in-code `declared` Bar (at least), else unjudged. `None` is a named absence.
fn percent(name: &str, value: Option<f64>, bars: &Bars, declared: Option<f64>) -> Measured {
    let Some(value) = value else {
        eprintln!("{name}: absent (nothing to compute it from)");
        return absent(name, Unit::Percentage);
    };
    let measured = Measured::judged(name, value, Unit::Percentage, bars);
    match (measured.bar, declared) {
        (None, Some(floor)) => {
            let verdict =
                if value >= floor { Judged::Pass } else { Judged::Miss { shortfall: value - floor } };
            let bar = Bar { value: floor, sense: Sense::AtLeast };
            Measured { bar: Some(bar), verdict, ..measured }
        }
        _ => measured,
    }
}

/// A named absence: the metric's name with a NaN value and no verdict.
fn absent(name: &str, unit: Unit) -> Measured {
    Measured {
        name: name.to_string(),
        value: f64::NAN,
        unit,
        bar: None,
        control: None,
        verdict: Judged::Unjudged,
    }
}

mod predict;
#[cfg(test)]
mod tests;
