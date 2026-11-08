//! The Harness's wiring tests: Property 55 over synthetic outcome and
//! Bar-verdict vectors, and the unit facts that pin `run_one`, `new` and the
//! verdict's edges.
//!
//! Property 55 is pure — it exercises `overall_verdict`, the one function
//! R1.11 pins — so it draws ten-outcome vectors across all four outcome shapes
//! and Bar-verdict sets with 0 to 20 Misses, never spawning cargo. The one
//! test that would shell out to cargo is `#[ignore]`, like `spawn.rs`'s OS
//! test, so `cargo test -p phiano --bin harness` stays fast and offline.

use super::*;
use crate::bar::{Bar, Bars, Judged, Sense};
use crate::level::Outcome;
use crate::record::{Measured, Recorder};
use phistd::gen::Seeded;
use std::collections::BTreeMap;

/// The run's declared seed, matching the sibling modules' convention.
const SEED: u64 = 0x5747_4f4c_4341_494d;

// --- Property 55 -----------------------------------------------------------

/// Draw one of the four outcome shapes. The `clean` flag asks for the only
/// shape that can clear the sweep — a `Passed` with zero failures — so the
/// caller can build a known-clean sweep and then perturb exactly one slot.
fn draw_outcome(g: &mut Seeded, clean: bool) -> Outcome {
    let seconds = g.unit() * 100.0;
    if clean {
        let passed = g.below(500) as usize;
        return Outcome::Passed { run: passed, passed, failed: 0, seconds };
    }
    match g.below(4) {
        0 => {
            // A pass that still carries a failure count is not clean.
            let passed = g.below(500) as usize;
            let failed = 1 + g.below(20) as usize;
            Outcome::Passed { run: passed + failed, passed, failed, seconds }
        }
        1 => {
            let passed = g.below(500) as usize;
            let failed = 1 + g.below(20) as usize;
            Outcome::Failed {
                run: passed + failed,
                passed,
                failed,
                seconds,
                failures: vec![("a_test".to_string(), "boom".to_string())],
            }
        }
        2 => Outcome::BuildFailed { seconds, first_error: "error: nope".to_string() },
        _ => Outcome::TimedOut { seconds: 900.0 },
    }
}

/// Draw a Bar verdict, with `misses` of the ten set to a non-passing verdict
/// (a Miss or an Unjudged), the rest a Pass. The count of non-passing verdicts
/// is exactly `misses`, so the property knows by construction whether the Bar
/// set clears.
fn draw_bars(g: &mut Seeded, misses: usize) -> Vec<Judged> {
    let n = 18; // the eighteen ladder numbers
    let mut v: Vec<Judged> = vec![Judged::Pass; n];
    let misses = misses.min(n);
    // Place the misses at the first `misses` positions after a shuffle-free
    // pick, so the count is exact.
    for slot in v.iter_mut().take(misses) {
        *slot = if g.below(2) == 0 {
            Judged::Miss { shortfall: -(1.0 + g.unit() * 10.0) }
        } else {
            Judged::Unjudged
        };
    }
    v
}

/// **Property 55: The Harness's overall verdict is pass only on a clean
/// sweep.**
///
/// Over ten-outcome vectors across all four outcome shapes and Bar-verdict
/// sets with 0 to 20 Misses, `overall_verdict` returns `Pass` exactly when
/// every level passed with zero failures and every Bar passed; any other
/// vector is a `Miss`. The expected verdict is known by construction, not
/// recomputed by a second copy of the rule.
///
/// **Validates: Requirements 1.11**
#[test]
fn property_55_the_overall_verdict_is_pass_only_on_a_clean_sweep() {
    for case in 0..256u64 {
        let mut g = Seeded::for_case(SEED, case as usize);

        // How many of the ten levels are dirty, and how many of the eighteen
        // Bars miss (0 to 20, clamped to 18).
        let dirty_levels = g.below(11) as usize; // 0..=10
        let bar_misses = g.below(21) as usize; // 0..=20

        let mut outcomes: Vec<Outcome> = Vec::with_capacity(10);
        for i in 0..10usize {
            outcomes.push(draw_outcome(&mut g, i >= dirty_levels));
        }
        // The first `dirty_levels` slots are dirty; shuffle which slots are
        // dirty by rotating, so the dirty slot is not always the first.
        let rot = g.below(10) as usize;
        outcomes.rotate_left(rot.min(9));

        let bars = draw_bars(&mut g, bar_misses);

        let expected_clean = dirty_levels == 0 && bar_misses == 0;
        let got = overall_verdict(&outcomes, &bars);
        assert_eq!(
            got == Verdict::Pass,
            expected_clean,
            "case {case}: dirty_levels={dirty_levels} bar_misses={bar_misses} got {got:?}"
        );
    }
}

// --- Unit facts about the verdict edges ------------------------------------

/// Ten clean passes and eighteen passed Bars: the only clean sweep.
fn clean_run() -> (Vec<Outcome>, Vec<Judged>) {
    let outcomes = vec![
        Outcome::Passed { run: 3, passed: 3, failed: 0, seconds: 1.0 };
        10
    ];
    let bars = vec![Judged::Pass; 18];
    (outcomes, bars)
}

#[test]
fn a_clean_sweep_passes() {
    let (o, b) = clean_run();
    assert_eq!(overall_verdict(&o, &b), Verdict::Pass);
}

#[test]
fn one_failed_level_misses() {
    let (mut o, b) = clean_run();
    o[4] = Outcome::Failed {
        run: 2,
        passed: 1,
        failed: 1,
        seconds: 1.0,
        failures: vec![("t".to_string(), "m".to_string())],
    };
    match overall_verdict(&o, &b) {
        Verdict::Miss { reason } => assert!(reason.contains("phiecs"), "{reason}"),
        v => panic!("expected a miss, got {v:?}"),
    }
}

#[test]
fn each_dirty_outcome_shape_and_a_missed_bar_miss() {
    // A build failure, a timeout, a pass carrying failures, and a missed Bar
    // each spoil an otherwise clean sweep.
    let (mut o, b) = clean_run();
    o[0] = Outcome::BuildFailed { seconds: 1.0, first_error: "error".to_string() };
    assert!(matches!(overall_verdict(&o, &b), Verdict::Miss { .. }));

    let (mut o, b) = clean_run();
    o[9] = Outcome::TimedOut { seconds: 900.0 };
    assert!(matches!(overall_verdict(&o, &b), Verdict::Miss { .. }));

    let (mut o, b) = clean_run();
    o[2] = Outcome::Passed { run: 5, passed: 4, failed: 1, seconds: 1.0 };
    assert!(matches!(overall_verdict(&o, &b), Verdict::Miss { .. }));

    let (o, mut b) = clean_run();
    b[7] = Judged::Miss { shortfall: -2.0 };
    assert!(matches!(overall_verdict(&o, &b), Verdict::Miss { .. }));
}

#[test]
fn an_unjudged_bar_does_not_clear_the_sweep() {
    let (o, mut b) = clean_run();
    b[0] = Judged::Unjudged;
    match overall_verdict(&o, &b) {
        Verdict::Miss { reason } => assert!(reason.contains("unjudged"), "{reason}"),
        v => panic!("expected a miss, got {v:?}"),
    }
}

// --- run_one and the Harness's construction --------------------------------

/// A Harness over an empty Bars and a Recorder opened on a fresh temp dir, for
/// the construction and `run_one` name-check tests that never spawn cargo.
fn harness_in(dir: &std::path::Path) -> Harness {
    let mut map = BTreeMap::new();
    map.insert("curriculum.perplexity".to_string(), Bar { value: 200.0, sense: Sense::AtMost });
    let bars = Bars::from_map(map);
    let recorder = Recorder::open(dir, &bars).expect("open recorder");
    Harness::new(bars, recorder, Caps { seconds: 1 })
}

#[test]
fn run_one_rejects_an_unaccepted_level_without_spawning() {
    let dir = std::env::temp_dir().join(format!("phiano_harness_55_{SEED:x}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    let before = std::fs::read_dir(&dir).unwrap().count();

    let mut h = harness_in(&dir);
    let got = h.run_one("not-a-level");
    assert!(got.is_err(), "an unaccepted name must not run a level");
    assert_eq!(
        std::fs::read_dir(&dir).unwrap().count(),
        before,
        "a rejected name left a file on disk"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn record_ladder_writes_every_number_and_returns_its_verdicts() {
    let dir = std::env::temp_dir().join(format!("phiano_harness_ladder_{SEED:x}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    let mut h = harness_in(&dir);

    let numbers = vec![
        Measured::judged("mode.learned.function_purity", 60.0, crate::bar::Unit::Percentage, h.bars()),
        Measured::judged("curriculum.perplexity", 250.0, crate::bar::Unit::Perplexity, h.bars()),
    ];
    let verdicts = h.record_ladder(&numbers);
    assert_eq!(verdicts.len(), 2);
    // The perplexity 250 against an at-most-200 Bar is a Miss; the first name
    // has no Bar, so it is Unjudged.
    assert!(matches!(verdicts[0], Judged::Unjudged));
    assert!(matches!(verdicts[1], Judged::Miss { .. }));
    let _ = std::fs::remove_dir_all(&dir);
}

/// The actual ten-level walk shells out to cargo ten times, so it is gated
/// behind `--ignored`, like `spawn.rs`'s OS test. Run it explicitly with
/// `cargo test -p phiano --bin harness -- --ignored run_all`.
#[test]
#[ignore = "spawns cargo ten times; run explicitly"]
fn os_run_all_walks_the_ten_levels() {
    let dir = std::env::temp_dir().join("phiano_harness_walk");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    let mut h = harness_in(&dir);
    let outcomes = h.run_all();
    assert_eq!(outcomes.len(), 10, "every level reports exactly one outcome");
    for (i, (name, _)) in outcomes.iter().enumerate() {
        assert_eq!(*name, crate::level::LEVELS[i], "levels run in LEVELS order");
    }
    let _ = std::fs::remove_dir_all(&dir);
}
