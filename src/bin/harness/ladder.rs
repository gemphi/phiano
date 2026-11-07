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
use phisor::ladder::{Judged, Ladder, Seeds};
use phical::classes::{Settings, MAX_PASSES};
use phiton::p64::variants::Level;
use std::path::Path;

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

/// The declared settings grid. Each axis is a legitimate knob of the exchange
/// routine; the Bar, the split and the scorer are untouched. The grid is
/// deliberately small and fixed so a run reproduces.
fn grid() -> Vec<Settings> {
    let mut out = Vec::new();
    for &passes in &[1usize, 4, 8, MAX_PASSES] {
        for &floor in &[0usize, 2, 4] {
            for &penalty in &[0.0f64, 0.5, 1.0] {
                out.push(Settings {
                    max_passes: passes,
                    min_unit_frequency: floor,
                    split_cost_penalty: penalty,
                    ..Settings::default()
                });
            }
        }
    }
    out
}

/// Sweep the grid and keep the settings with the highest dev function
/// purity (never the Held_Out_Slice). Ties keep the earlier (simpler) settings, so a more complex setting
/// has to earn its place. Returns the best settings and the purity it scored.
fn tune(split: &Split) -> (Settings, f64) {
    let mut best = Settings { max_passes: 1, ..Settings::default() };
    let mut best_score = (f64::NEG_INFINITY, f64::NEG_INFINITY);
    for settings in grid() {
        let score = dev_scores_of(split, &settings);
        if score > best_score {
            best_score = score;
            best = settings;
        }
    }
    (best, best_score.0.max(0.0))
}

/// The dev scores a given settings earns, in the order the sweep compares
/// them: function purity first, then terminator purity.
///
/// Dev holds no answer sentences (the Split_Rule sends every Key answer to
/// Test), so dev function purity is the explain share for every setting and
/// never separates them. The terminator mark is read off the text, not the
/// Key, so terminator purity on dev still varies with the Kinds and breaks
/// that tie. It names no held-out label and reads only `Part::Dev`.
/// A setting that could not be scored is negative infinity on both, so it
/// never wins the sweep.
fn dev_scores_of(split: &Split, settings: &Settings) -> (f64, f64) {
    let ladder = Ladder::new(split, Seeds::DECLARED);
    let learned = ladder.kinds(phisor::ladder::Arm::Learned, MODE, settings);
    let mut guard = HoldoutGuard::new(vec![]);
    let function = ladder
        .function_purity_on(Part::Dev, &learned, &mut guard)
        .map_or(f64::NEG_INFINITY, |r| value(&r.purity));
    let terminator = ladder
        .terminator_purity_on(Part::Dev, &learned, &mut guard)
        .map_or(f64::NEG_INFINITY, |r| value(&r.purity));
    (function, terminator)
}

/// The number behind a ladder `Measured`, whichever verdict it carries, so
/// the Recorder judges the value against the declared Bar.
fn value(m: &phisor::ladder::Measured) -> f64 {
    match m.verdict {
        Judged::Pass => m.value,
        Judged::Miss { value, .. } => value,
    }
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

/// The round-trip and force-purity figures over a set of gold `(Act, text)`
/// pairs, named `ladder.act.round_trip{suffix}` and
/// `ladder.act.force_purity{suffix}`. `suffix` is empty for the all-150 cut
/// and `_reachable` for the subset that drops the Unreachable_Frames, so both
/// cuts are reported side by side (R2.2). A cut with nothing to score emits no
/// record, matching the all-150 behaviour it replaced.
fn gold_figures(suffix: &str, pairs: &[(&phisor::ir::Act, &str)], bars: &Bars) -> Vec<Measured> {
    let mut out = Vec::new();
    let (rt_matched, rt_total, _) =
        phisor::ladder::round_trip(pairs.iter().map(|&(a, s)| (a, s)));
    if rt_total > 0 {
        let rate = 100.0 * rt_matched as f64 / rt_total as f64;
        let name = format!("ladder.act.round_trip{suffix}");
        out.push(Measured::judged(&name, one_decimal(rate), Unit::Percentage, bars));
    }
    let (fp_matched, fp_covered) = phisor::ladder::force_purity(pairs.iter().map(|&(a, s)| (a, s)));
    if fp_covered > 0 {
        let purity = 100.0 * fp_matched as f64 / fp_covered as f64;
        let name = format!("ladder.act.force_purity{suffix}");
        out.push(Measured::judged(&name, one_decimal(purity), Unit::Percentage, bars));
    }
    out
}

/// `ladder.act.round_trip_realisable`: the strict gold round trip over the 150
/// minus the Unrealisable_Frames, the frames whose Act has no field for a word
/// of the sentence (R1.4, R1.8). The gold file is untouched; the exclusion
/// lives only in this figure.
fn realisable_figure(gold: &phisor::gold::Gold, bars: &Bars) -> Option<Measured> {
    let realisable: Vec<_> = gold
        .entries
        .iter()
        .enumerate()
        .filter(|(i, _)| phisor::gold::unrealisable(*i).is_none())
        .map(|(_, e)| (&e.act, e.sentence.as_str()))
        .collect();
    let (matched, total, _) = phisor::ladder::round_trip(realisable);
    (total > 0).then(|| {
        let rate = 100.0 * matched as f64 / total as f64;
        Measured::judged("ladder.act.round_trip_realisable", one_decimal(rate), Unit::Percentage, bars)
    })
}

/// `ladder.act.force_purity_decidable`: force purity with only the `BareWill`
/// frames removed. A bare `will` is a Commit or a prediction and the text does
/// not say which, so no Force can be read for it; the `NameCase` frames stay
/// in, since a proper name cannot change a Force (R2.1). The gold file is
/// untouched; the exclusion lives only in this figure.
fn decidable_figure(gold: &phisor::gold::Gold, bars: &Bars) -> Option<Measured> {
    use phisor::gold::Reason;
    let decidable: Vec<_> = gold
        .entries
        .iter()
        .enumerate()
        .filter(|(i, _)| phisor::gold::unreachable(*i) != Some(Reason::BareWill))
        .map(|(_, e)| (&e.act, e.sentence.as_str()))
        .collect();
    let (matched, covered) = phisor::ladder::force_purity(decidable);
    (covered > 0).then(|| {
        let purity = 100.0 * matched as f64 / covered as f64;
        Measured::judged("ladder.act.force_purity_decidable", one_decimal(purity), Unit::Percentage, bars)
    })
}

/// One decimal place, matching the Recorder's reporting.
pub(super) fn one_decimal(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use phisor::guide::{Function, Mark, Sentence};
    use phisor::ir::Force;

    /// A small deterministic text whose function follows its first word, in
    /// 240 sentences of three units, so every part of the split is occupied.
    fn synthetic(flip_test: bool) -> Split {
        let words = ["go", "see", "the", "a", "dog", "cat", "run", "is", "what", "now"];
        let mut state = 0x2545_f491u64;
        let sentences = (0..240usize)
            .map(|i| {
                let mut next = || {
                    state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    (state >> 33) as usize
                };
                let len = 4 + next() % 3;
                let mut ws: Vec<String> = (0..len).map(|_| words[next() % words.len()].to_string()).collect();
                let function = match i % 3 {
                    0 => Function::Explain,
                    1 => Function::Instruct,
                    _ => Function::Answer,
                };
                ws[0] = ["is", "go", "dog"][i % 3].to_string();
                let held_out = i % 10 == 0;
                let function = match (flip_test && held_out, function) {
                    (true, Function::Explain) => Function::Instruct,
                    (true, Function::Instruct) => Function::Answer,
                    (true, Function::Answer) => Function::Explain,
                    (false, f) => f,
                };
                Sentence { words: ws, function, mark: Mark::Stop, unit: i / 80 + 1, run: 0 }
            })
            .collect();
        Split::new(sentences)
    }

    /// **Property 3: Tuning never reads the Held_Out_Slice.**
    /// **Validates: Requirements 3.1**
    #[test]
    fn tuning_ignores_the_held_out_labels() {
        let (a, pa) = tune(&synthetic(false));
        let (b, pb) = tune(&synthetic(true));
        assert_eq!(pa, pb, "the tuned dev purity moved with the held-out labels");
        assert_eq!(
            (a.max_passes, a.min_unit_frequency, a.split_cost_penalty),
            (b.max_passes, b.min_unit_frequency, b.split_cost_penalty)
        );
    }

    /// The dev and held-out purity of every grid point, then the choice made
    /// on dev and the one the old held-out selection would have made. Run:
    /// `cargo test -p phiano --bin harness before_and_after -- --ignored --nocapture`
    /// from the workspace root.
    #[test]
    #[ignore = "diagnostic: reads the real Guide; run alone"]
    fn before_and_after_the_dev_choice() {
        let (Ok(text), Ok(key)) = (
            std::fs::read_to_string(Path::new(GUIDE)),
            std::fs::read_to_string(Path::new(KEY)),
        ) else {
            eprintln!("the Guide is unreadable from here");
            return;
        };
        phisor::lexicon::prime_irregular(&text);
        let split = Split::new(guide::read(&text, &key));
        let ladder = Ladder::new(&split, Seeds::DECLARED);
        let (mut by_dev, mut by_test) = ((f64::MIN, f64::MIN, 0usize), (f64::MIN, f64::MIN, 0usize));
        for (n, settings) in grid().iter().enumerate() {
            let learned = ladder.kinds(phisor::ladder::Arm::Learned, MODE, settings);
            let mut guard = HoldoutGuard::new(vec![]);
            let dev = value(&ladder.function_purity_on(Part::Dev, &learned, &mut guard).unwrap().purity);
            let test = value(&ladder.function_purity(&learned, &mut guard).unwrap().purity);
            eprintln!("grid {n}: dev {dev:.1} test {test:.1}");
            if dev > by_dev.0 {
                by_dev = (dev, test, n);
            }
            if test > by_test.1 {
                by_test = (dev, test, n);
            }
        }
        eprintln!("AFTER  (chosen on dev):  grid {} dev {:.1} held-out {:.1}", by_dev.2, by_dev.0, by_dev.1);
        eprintln!("BEFORE (chosen on test): grid {} dev {:.1} held-out {:.1}", by_test.2, by_test.0, by_test.1);
    }

    /// Per-function-class purity at the dev-chosen settings, on the dev part
    /// and on the Held_Out_Slice. A class is credited with the sentences that
    /// fall in Kinds whose majority label is that class: recall is that share
    /// of the class, precision is that share of those Kinds' sentences. Run:
    /// `cargo test -p phiano --bin harness per_function_class -- --ignored --nocapture`
    /// from the workspace root (the Guide path is relative to it).
    ///
    /// Recorded figures (dev-chosen settings, 9 Kinds):
    /// - dev: 95.4% over 822; explain 784 (recall 100.0), instruct 38 (0.0), answer 0.
    /// - held-out: 52.6% over 2994; explain 1558 (recall 86.0, precision 53.0),
    ///   instruct 96 (recall 0.0), answer 1340 (recall 17.6, precision 50.9).
    ///
    /// After 5.3 (dev ties broken by dev terminator purity; 13 Kinds): dev
    /// 95.4% unchanged; held-out 53.0% (explain recall 87.8, answer recall
    /// 16.4, instruct 0.0). The gain is finer Kinds, not better answer recall.
    ///
    /// 5.4 (second change, tried and reverted): `split_cost_penalty` was made to
    /// reach `settle` (added to `split_cost` before the keep decision). At the
    /// declared 0.5 and 1.0 nats, against a split cost of tens of nats, no
    /// split flipped: the 36 grid points scored as before and the dev-chosen
    /// held-out purity stayed 53.0% (13 Kinds), so the change did not raise it
    /// and was dropped. A larger penalty could only merge Kinds, and purity
    /// rises with the Kind count, so it cannot help. The gap against the
    /// hash-null is `UnequalKindCount` (learned 13, hash-null 16) already at
    /// 5.3, so it is unjudged, and the change could not have lowered it.
    /// Best held-out function purity after 5.3 and 5.4: 53.0%. The Bar stays
    /// 56.7%: an honest recorded miss, not a tuned pass.
    ///
    /// Weakest classes before 5.3: instruct (never the majority of any Kind) and answer
    /// (17.6% recall). Dev holds no answer sentences and 38 instruct, so its
    /// 95.4% is just the explain share; every setting scores it and dev cannot
    /// separate them. Held-out is 45% answer, so the purity there is near the
    /// 52% explain share plus a little.
    #[test]
    #[ignore = "diagnostic: reads the real Guide; run alone"]
    fn per_function_class_purity() {
        let (Ok(text), Ok(key)) = (
            std::fs::read_to_string(Path::new(GUIDE)),
            std::fs::read_to_string(Path::new(KEY)),
        ) else {
            eprintln!("the Guide is unreadable from here");
            return;
        };
        phisor::lexicon::prime_irregular(&text);
        let split = Split::new(guide::read(&text, &key));
        let (best, _) = tune(&split);
        let ladder = Ladder::new(&split, Seeds::DECLARED);
        let learned = ladder.kinds(phisor::ladder::Arm::Learned, MODE, &best);
        let [hash, _, paired] = ladder.all_kinds(MODE, &best);
        let mut gate = HoldoutGuard::new(vec![]);
        match ladder.function_gap(&paired, &hash, &mut gate) {
            Ok(gap) => eprintln!("gap vs hash-null {:.2}", value(&gap.difference)),
            Err(e) => eprintln!("gap vs hash-null unavailable: {e:?}"),
        }
        for part in [Part::Dev, Part::Test] {
            let mut guard = HoldoutGuard::new(vec![]);
            let report = ladder.function_purity_on(part, &learned, &mut guard).unwrap();
            eprintln!("{part:?}: purity {:.1}% over {} scored, {} unassigned, {} kinds",
                value(&report.purity), report.scored, report.unassigned, report.table.len());
            let majority = |row: &Vec<f64>| {
                (0..row.len()).max_by(|&a, &b| row[a].total_cmp(&row[b]).then(b.cmp(&a))).unwrap()
            };
            for (c, f) in guide::Function::ALL.iter().enumerate() {
                let total: f64 = report.table.iter().map(|r| r[c]).sum();
                let (mut caught, mut claimed) = (0.0, 0.0);
                for row in report.table.iter().filter(|r| majority(r) == c) {
                    caught += row[c];
                    claimed += row.iter().sum::<f64>();
                }
                let pct = |n: f64, d: f64| if d > 0.0 { 100.0 * n / d } else { 0.0 };
                eprintln!("  {:<9} n {total:>4} recall {:>5.1}% precision {:>5.1}%",
                    f.name(), pct(caught, total), pct(caught, claimed));
            }
        }
    }

    /// The reachable cut drops the Unreachable_Frames, so it reports the same
    /// two names with the `_reachable` suffix beside the all-150 pair, and
    /// scores no lower than the all-150 cut on either figure (R2.2).
    #[test]
    fn reachable_cut_sits_beside_the_all_150_cut() {
        let bars = Bars::default();
        // Prime the reader as the live walk does, so the before figures this
        // test prints match the harness record the Realiser tasks compare
        // against. Absent files leave the reader on its built-in classes.
        if let Ok(pos) = std::fs::read_to_string(Path::new(POS)) {
            phisor::lexicon::prime(&pos);
        }
        if let Ok(func) = std::fs::read_to_string(Path::new(FUNCTION)) {
            phisor::lexicon::prime_function(&func);
        }
        let gold = phisor::gold::read().expect("the frozen gold set loads");

        let all: Vec<_> = gold.pairs().collect();
        let reachable: Vec<_> = gold
            .entries
            .iter()
            .enumerate()
            .filter(|(i, _)| phisor::gold::unreachable(*i).is_none())
            .map(|(_, e)| (&e.act, e.sentence.as_str()))
            .collect();

        // The subset is the 150 minus the listed Unreachable_Frames.
        assert_eq!(reachable.len(), gold.len() - phisor::gold::UNREACHABLE.len());

        let all_figs = gold_figures("", &all, &bars);
        let reach_figs = gold_figures("_reachable", &reachable, &bars);
        let get = |figs: &[Measured], name: &str| {
            figs.iter().find(|m| m.name == name).map(|m| m.value)
        };

        let rt_all = get(&all_figs, "ladder.act.round_trip").expect("all round-trip");
        let fp_all = get(&all_figs, "ladder.act.force_purity").expect("all force purity");
        let rt_reach =
            get(&reach_figs, "ladder.act.round_trip_reachable").expect("reachable round-trip");
        let fp_reach =
            get(&reach_figs, "ladder.act.force_purity_reachable").expect("reachable force purity");

        // Every figure is a share, so it sits in [0, 100].
        for v in [rt_all, fp_all, rt_reach, fp_reach] {
            assert!((0.0..=100.0).contains(&v), "{v} is not a percentage");
        }
        // The Unreachable_Frames are the ones whose Force the surface text
        // cannot settle (BareWill, and the NameCase/Unaccusative that share a
        // surface shape), so dropping them lifts force purity. Round-trip is a
        // realiser check independent of the label, so it is not constrained to
        // move either way and is only checked to be in range above.
        assert!(fp_reach >= fp_all, "force purity {fp_reach} < all {fp_all}");

        // The realisable cut is the 150 minus the Unrealisable_Frames; every
        // frame left round-trips, so the figure is 100.
        let realisable = realisable_figure(&gold, &bars).expect("realisable round-trip");
        assert_eq!(realisable.name, "ladder.act.round_trip_realisable");
        assert_eq!(realisable.value, 100.0);
    }

    /// Every covered gold frame the Reader reads with a wrong Force, by gold
    /// index: `(index, gold Force, read Force, sentence)` (R2.2).
    fn wrong_force(gold: &phisor::gold::Gold) -> Vec<(usize, phisor::ir::Force, phisor::ir::Force, String)> {
        gold.entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| {
                let read = phisor::ladder::read(&e.sentence)?;
                (read.force != e.act.force).then(|| (i, e.act.force, read.force, e.sentence.clone()))
            })
            .collect()
    }

    /// Prime the reader as the live walk does (pos.tsv, function.txt and the
    /// Guide's irregular verbs), so the pinned table matches the harness record.
    fn prime_as_the_harness() {
        // The tests run from the package directory, not the workspace root the
        // harness consts are relative to, so resolve from the manifest.
        let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("data");
        let pos = std::fs::read_to_string(data.join("lex/pos.tsv")).expect("pos.tsv loads");
        phisor::lexicon::prime(&pos);
        let func = std::fs::read_to_string(data.join("lex/function.txt")).expect("function.txt loads");
        phisor::lexicon::prime_function(&func);
        let guide = std::fs::read_to_string(data.join("person/murphy.txt")).expect("the Guide text loads");
        phisor::lexicon::prime_irregular(&guide);
    }


    /// The gold frames the Reader reads with a wrong Force, by gold index:
    /// `(index, gold Force, read Force)`, measured with the harness priming
    /// (task 4.1). Gold is frozen, so this is the yardstick for tasks 4.2 to
    /// 4.4. Sentence beside the confusion:
    ///
    /// Commit read as Assert, the seven `BareWill` Unreachable_Frames (a bare
    /// `will` or `shall` is a Commit or a prediction, and is never mapped to
    /// Commit by rule, R2.5):
    ///
    /// - 117 `i will explain the rule.`
    /// - 119 `we shall not forget.`
    /// - 120 `i will send you the book.`
    /// - 121 `i am going to study french.`
    /// - 122 `we will meet at eight.`
    /// - 124 `i will have finished by friday.`
    /// - 126 `we will answer your letter.`
    ///
    /// Express read as Assert, six frames the text could settle:
    ///
    /// - 129 `congratulations on your success.`
    /// - 130 `i do not mind the noise.`
    /// - 131 `welcome to london.`
    /// - 132 `i have enjoyed the lesson.`
    /// - 133 `i was delighted with your letter.`
    /// - 134 `i am not complaining.`
    ///
    /// Direct read as Assert: 102 `please close the window.` Wish read as
    /// Assert: 143 `i wish we did not have to go.`
    const WRONG_FORCE: [(usize, Force, Force); 15] = [
        (102, Force::Direct, Force::Assert),
        (117, Force::Commit, Force::Assert),
        (119, Force::Commit, Force::Assert),
        (120, Force::Commit, Force::Assert),
        (121, Force::Commit, Force::Assert),
        (122, Force::Commit, Force::Assert),
        (124, Force::Commit, Force::Assert),
        (126, Force::Commit, Force::Assert),
        (129, Force::Express, Force::Assert),
        (130, Force::Express, Force::Assert),
        (131, Force::Express, Force::Assert),
        (132, Force::Express, Force::Assert),
        (133, Force::Express, Force::Assert),
        (134, Force::Express, Force::Assert),
        (143, Force::Wish, Force::Assert),
    ];

    /// Pins the wrong-Force gold frames by index with the confusion, and the
    /// decidable figure that follows from them (R2.1, R2.2).
    #[test]
    fn the_wrong_force_frames_are_pinned_by_index_and_confusion() {
        use phisor::gold::Reason;
        prime_as_the_harness();
        let gold = phisor::gold::read().expect("the frozen gold set loads");
        let wrong = wrong_force(&gold);
        let found: Vec<_> = wrong.iter().map(|(i, g, r, _)| (*i, *g, *r)).collect();
        assert_eq!(found, WRONG_FORCE, "the set of wrong-Force gold frames moved");
        assert!(found.iter().all(|(i, ..)| *i < phisor::gold::COUNT));

        // Confusion counts, gold Force to read Force.
        let count = |g: Force, r: Force| found.iter().filter(|f| f.1 == g && f.2 == r).count();
        assert_eq!(count(Force::Commit, Force::Assert), 7);
        assert_eq!(count(Force::Express, Force::Assert), 6);
        assert_eq!(count(Force::Direct, Force::Assert), 1);
        assert_eq!(count(Force::Wish, Force::Assert), 1);

        // The seven Commit frames are exactly the BareWill Unreachable_Frames.
        let bare: Vec<usize> = (0..phisor::gold::COUNT)
            .filter(|i| phisor::gold::unreachable(*i) == Some(Reason::BareWill))
            .collect();
        let commits: Vec<usize> =
            found.iter().filter(|f| f.1 == Force::Commit).map(|f| f.0).collect();
        assert_eq!(bare, commits, "the BareWill frames and the Commit confusions differ");

        // Removing only the BareWill frames leaves 8 wrong of 113 covered.
        let decidable = decidable_figure(&gold, &Bars::default()).expect("decidable figure");
        assert_eq!(decidable.name, "ladder.act.force_purity_decidable");
        assert_eq!(decidable.value, 92.9);
    }
}
