//! The live Ladder run with an honest settings sweep.
//!
//! `run_walk` once handed the Recorder an empty slice, so every learning
//! number came back unjudged. This module loads The_Guide, sweeps a declared
//! grid of the exchange routine's settings, keeps the settings that score the
//! highest function purity on the dev part (ties go to the higher dev
//! terminator purity, then the earlier setting), and reports that run's numbers as
//! `record::Measured` judged against the declared Bars.
//!
//! The sweep is honest hyperparameter search, not fitting the yardstick: the
//! Bar (56.7%), the Split_Rule, the test labels and the scorer are all fixed.
//! Only the learner's settings vary. The metric optimised is function purity
//! on `Part::Dev`; the Held_Out_Slice (`Part::Test`) is read once, after the
//! choice, for the reported figure the Bar judges. If no setting clears the
//! Bar, the record shows an honest miss.
//!
//! The act-height numbers stay out, since the English reader (task 13.7) is
//! not built, so they remain unjudged exactly as the design says.

use super::bars::{Bars, Unit};
use super::record::Measured;
use phisor::guide::holdout::HoldoutGuard;
use phisor::guide::{self, Part, Split};
use phisor::ladder::{Ladder, Seeds};
use phical::classes::Settings;
use phiton::p64::variants::Level;
use std::path::Path;
use gold::{decidable_figure, gold_figures, realisable_figure};
use tune::{tune, value};

/// The Guide and its Key, relative to the workspace root.
const GUIDE: &str = "crates/phiano/data/person/murphy.txt";
const KEY: &str = "crates/phiano/data/person/murphy_key.txt";
/// The foundation's mined part-of-speech lexicon, primed into the reader so
/// its word classes come from what was learned over the corpora.
const POS: &str = "crates/phiano/data/lex/pos.tsv";
/// The foundation's closed-class function words, primed so the reader never
/// inflects a pronoun, preposition or determiner.
const FUNCTION: &str = "crates/phiano/data/lex/function.txt";

/// The Mode the ladder numbers are reported at (R5.6 declared the purity Bar
/// over Mode 16).
const MODE: Level = Level::P16;

/// Run the Ladder on the Guide, sweeping settings, and return the best run's
/// numbers as Recorder `Measured`. An unreadable corpus yields no numbers, so
/// the walk records every ladder Bar unjudged rather than failing.
pub fn run(bars: &Bars) -> Vec<Measured> {
    let (Ok(text), Ok(key)) = (
        std::fs::read_to_string(Path::new(GUIDE)),
        std::fs::read_to_string(Path::new(KEY)),
    ) else {
        eprintln!("harness: the Guide at {GUIDE} is unreadable; ladder numbers are unjudged");
        return Vec::new();
    };
    // The Guide's own table of irregular verbs (Appendix 1.4), so the reader
    // and realiser take their irregular forms from the book being read.
    phisor::lexicon::prime_irregular(&text);
    let split = Split::new(guide::read(&text, &key));
    let (best, purity) = tune(&split);
    eprintln!(
        "harness: tuned settings max_passes={} stop_fraction={} min_unit_frequency={} \
         split_cost_penalty={} -> dev function purity {purity:.1}%",
        best.max_passes, best.stop_fraction, best.min_unit_frequency, best.split_cost_penalty
    );
    let out = measure(&split, bars, &best);
    if let Some(held) = out.iter().find(|m| m.name == "ladder.function_purity.learned") {
        eprintln!("harness: held-out function purity {:.1}% (read once, after the choice)", held.value);
    }
    out
}


/// Compute the built ladder measures over `split` at the chosen `settings` and
/// judge each against the declared Bars.
fn measure(split: &Split, bars: &Bars, settings: &Settings) -> Vec<Measured> {
    let ladder = Ladder::new(split, Seeds::DECLARED);
    let [hash, _given, learned] = ladder.all_kinds(MODE, settings);
    let mut out = Vec::new();

    let mut guard = HoldoutGuard::new(vec![]);
    if let Ok(report) = ladder.function_purity(&learned, &mut guard) {
        out.push(Measured::judged("ladder.function_purity.learned", value(&report.purity), Unit::Percentage, bars));
    }
    if let Ok(gap) = ladder.function_gap(&learned, &hash, &mut guard) {
        out.push(Measured::judged("ladder.function_purity.gap", value(&gap.difference), Unit::Percentage, bars));
    }
    if let Ok(term) = ladder.terminator_purity(&learned, &mut guard) {
        out.push(Measured::judged("ladder.terminator_purity.learned", value(&term.purity), Unit::Percentage, bars));
    }
    if let Ok(river) = ladder.river_z(&learned, &hash) {
        out.push(Measured::judged("ladder.river_z.learned", value(&river.against_three), Unit::ZScore, bars));
        out.push(Measured::judged("ladder.river_z.above_hash", value(&river.against_hash), Unit::ZScore, bars));
    }

    // The act height, from the honest first English reader (task 13.7). The
    // round-trip and force purity are measured over the frozen gold set; a
    // first reader clears neither Bar, so these record honest misses rather
    // than the unjudged they were before. Coverage and per-field need a
    // baseline and stay unjudged.
    // Prime the reader with the foundation's mined POS lexicon, so its act
    // numbers reflect word classes learned over the corpora rather than the
    // reader's built-in closed sets. Absent file: the reader falls back.
    if let Ok(pos) = std::fs::read_to_string(Path::new(POS)) {
        phisor::lexicon::prime(&pos);
    }
    if let Ok(func) = std::fs::read_to_string(Path::new(FUNCTION)) {
        phisor::lexicon::prime_function(&func);
    }

    if let Ok(gold) = phisor::gold::read() {
        // All 150 frames: the headline round-trip and force purity the task
        // 13.7 reader already reports.
        let all: Vec<_> = gold.pairs().collect();
        out.extend(gold_figures("", &all, bars));
        // The same two figures over the reachable subset only - the 150 minus
        // the Unreachable_Frames no text-only reader can label (R2.2). The
        // exclusion lives only in this figure; the gold file is untouched. The
        // reachable figures are the before baseline the Realiser tasks (7.1 to
        // 7.4) compare their after figures against (R4.5).
        let reachable: Vec<_> = gold
            .entries
            .iter()
            .enumerate()
            .filter(|(i, _)| phisor::gold::unreachable(*i).is_none())
            .map(|(_, e)| (&e.act, e.sentence.as_str()))
            .collect();
        out.extend(gold_figures("_reachable", &reachable, bars));
        out.extend(realisable_figure(&gold, bars));
        out.extend(decidable_figure(&gold, bars));
    }

    // The reader on the Held_Out_Slice: four measures, regression flag and
    // abstention kinds (see ct).
    out.extend(super::act::measure(split, bars, Path::new(super::DOCS_DIR)));
    out
}


/// One decimal place, matching the Recorder's reporting.
pub(super) fn one_decimal(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

mod gold;
mod tune;

#[cfg(test)]
mod tests;
