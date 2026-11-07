use super::gold::{decidable_figure, gold_figures, realisable_figure};
use super::tune::{grid, tune, value};

mod reachable;

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
