//! Tests for the candidate measurement: the Split_Rule test set, the four
//! verdict shapes, and Property 53 over the two thresholds and the weights
//! hash guard.

use super::*;
use phistd::gen::{for_all, Gen, Seeded, CASES};

/// A throwaway weights file under the temp directory, seeded by `tag` so two
/// tests never collide. Returns the path and leaves it holding `bytes`.
fn weights_file(tag: &str, bytes: &[u8]) -> PathBuf {
    let path = std::env::temp_dir().join(format!("phiano_curric_{tag}.weights"));
    std::fs::write(&path, bytes).expect("write weights fixture");
    path
}

/// A runner over a tiny Layout-cut test set and a fresh weights file.
fn runner(tag: &str) -> (CurriculumRunner, PathBuf) {
    let sentences: Vec<String> = (0..40).map(|i| format!("sentence number {i}")).collect();
    let test = TestSet::split(&sentences, CutId::Layout);
    let weights = weights_file(tag, b"the current learner weights, left alone");
    let runner = CurriculumRunner::new(test, &weights);
    (runner, weights)
}

/// An external candidate over an on-disk corpus, the measurement here needing
/// only the epoch/seed/channel fields the classifier reads.
fn external(epochs: usize, seed: u64, channels: usize) -> Candidate {
    Candidate::External {
        corpus: CorpusRef { name: "on-disk".into(), bytes: 1, licence: "n/a".into(), on_disk: true },
        epochs,
        seed,
        channels,
    }
}

/// A staged candidate honouring R4.5's order over `epochs` epochs.
fn staged(epochs: usize, seed: u64, channels: usize) -> Candidate {
    let stages = (1..=epochs)
        .map(|epoch| Stage {
            epoch,
            fed: if epoch > epochs - 2 { Fed::GuideTrain } else { Fed::Wider },
        })
        .collect();
    Candidate::Staged { stages, epochs, seed, channels }
}

/// The Reported numbers for a completed run on the test set's own cut.
fn completed(top1: f64, perplexity: f64, cut: CutId) -> Reported {
    Reported {
        top1,
        perplexity,
        scored: 4,
        cut,
        epochs_finished: 4,
        stop_reason: String::new(),
    }
}

#[test]
fn the_split_rule_takes_every_tenth_sentence() {
    let sentences: Vec<String> = (0..40).map(|i| format!("s{i}")).collect();
    let test = TestSet::split(&sentences, CutId::Layout);
    assert_eq!(test.indices, vec![0, 10, 20, 30]);
    assert_eq!(test.scored(), 4);
    assert_eq!(test.cut, CutId::Layout);
}

#[test]
fn a_run_above_both_learner_thresholds_is_measured() {
    let (runner, _w) = runner("measured");
    let c = external(4, 1, 8);
    let r = runner.classify(&c, &completed(30.0, 70.0, CutId::Layout));
    assert_eq!(r, CandidateResult::Measured { top1: 30.0, perplexity: 70.0, scored: 4 });
    assert_eq!(runner.beats_trigram(&r), Some(true));
}

#[test]
fn a_low_top1_or_a_high_perplexity_is_a_regression() {
    let (runner, _w) = runner("regress");
    let c = external(4, 1, 8);
    // top-1 exactly at the learner's 21.3 is a regression (at-or-below).
    let r = runner.classify(&c, &completed(21.3, 70.0, CutId::Layout));
    assert!(matches!(r, CandidateResult::Regression { .. }));
    assert_eq!(runner.beats_trigram(&r), None);
    // perplexity exactly at 101.2 is a regression (at-or-above).
    let r = runner.classify(&c, &completed(30.0, 101.2, CutId::Layout));
    assert!(matches!(r, CandidateResult::Regression { .. }));
}

#[test]
fn a_run_on_another_cut_is_not_comparable() {
    let (runner, _w) = runner("cut");
    let c = external(4, 1, 8);
    let r = runner.classify(&c, &completed(30.0, 70.0, CutId::Tokens));
    assert_eq!(r, CandidateResult::NotComparable { used: CutId::Tokens });
    assert_eq!(runner.beats_trigram(&r), None);
}

#[test]
fn a_short_run_is_incomplete_with_the_epochs_and_reason() {
    let (runner, _w) = runner("short");
    let c = staged(4, 1, 8);
    let mut rep = completed(30.0, 70.0, CutId::Layout);
    rep.epochs_finished = 2;
    rep.stop_reason = "wall-clock cap reached".to_string();
    let r = runner.classify(&c, &rep);
    assert_eq!(
        r,
        CandidateResult::Incomplete { epochs_finished: 2, reason: "wall-clock cap reached".to_string() }
    );
    assert_eq!(runner.beats_trigram(&r), None);
}

#[test]
fn clearing_only_one_trigram_threshold_does_not_beat_it() {
    let (runner, _w) = runner("one");
    let c = external(4, 1, 8);
    // top-1 above 24.8 but perplexity not below 84.7.
    let r = runner.classify(&c, &completed(26.0, 90.0, CutId::Layout));
    assert_eq!(runner.beats_trigram(&r), Some(false));
}

/// A `(top1, perplexity)` pair spanning the four quadrants around both
/// threshold pairs, with values placed exactly at each threshold.
#[derive(Debug, Clone)]
struct Pair {
    top1: f64,
    perplexity: f64,
}

impl Gen for Pair {
    fn draw(g: &mut Seeded) -> Self {
        // Candidate top-1 values straddling 21.3 and 24.8 exactly, plus a
        // spread either side; perplexity values straddling 101.2 and 84.7.
        const TOP1: [f64; 7] = [0.0, 21.3, 21.4, 24.8, 24.9, 30.0, 100.0];
        const PERP: [f64; 7] = [1.0, 50.0, 84.6, 84.7, 101.2, 101.3, 500.0];
        Pair { top1: *g.pick(&TOP1), perplexity: *g.pick(&PERP) }
    }
}

/// Property 53: The curriculum verdict follows both thresholds under their
/// own senses.
///
/// For any `(top1, perplexity)`, the candidate is a regression exactly when
/// top-1 <= 21.3 or perplexity >= 101.2; it beats the trigram exactly when
/// top-1 > 24.8 and perplexity < 84.7; and classifying a regression (or any
/// non-measured outcome) leaves the weights file byte-for-byte unchanged, the
/// hash checked before and after.
///
/// **Validates: Requirements 4.4, 4.8, 4.11, 4.12**
#[test]
fn property_53_the_curriculum_verdict_follows_both_thresholds() {
    let sentences: Vec<String> = (0..40).map(|i| format!("sentence {i}")).collect();
    let test = TestSet::split(&sentences, CutId::Layout);
    let weights = weights_file("prop53", b"frozen weights never rewritten by a measurement");
    let before = hash_weights(&weights);

    let result = for_all::<Pair>(0x5EED_C0DE, CASES, |p| {
        let runner = CurriculumRunner::new(test.clone(), &weights);
        let c = external(4, 7, 8);
        let r = runner.classify(&c, &completed(p.top1, p.perplexity, CutId::Layout));

        // The regression rule, under its own two senses.
        let want_regression = p.top1 <= LEARNER_TOP1 || p.perplexity >= LEARNER_PERPLEXITY;
        let is_regression = matches!(r, CandidateResult::Regression { .. });
        if want_regression != is_regression {
            return false;
        }

        // The beating rule needs both thresholds; a non-measured result never
        // reports a beating verdict (R4.11).
        let want_beats = p.top1 > TRIGRAM_TOP1 && p.perplexity < TRIGRAM_PERPLEXITY;
        match runner.beats_trigram(&r) {
            Some(b) if !want_regression => b == want_beats,
            None if want_regression => true,
            _ => false,
        }
    });
    assert!(result.is_ok(), "{}", result.unwrap_err());

    // The weights file was never rewritten across every case (R4.4, R4.12).
    assert_eq!(hash_weights(&weights), before, "weights must survive every measurement");
}
