//! The unit facts: the budget (R3.11), Overflow (R3.9), the unreachable stop
//! (R3.3), the empty answer (R3.5), a non-zero exit (R3.8) and the structural
//! check-before-tell rule (R8.10).

use super::super::*;
use super::{seam_over, One};
use phiecs::quest::{Bare, OracleError};

// --- Budget (R3.11) --------------------------------------------------------

#[test]
fn a_budget_is_clamped_into_one_to_four_thousand_ninety_six() {
    assert_eq!(seam_over(One::text("a"), 0).budget(), MIN_BUDGET);
    assert_eq!(seam_over(One::text("a"), 9_000).budget(), MAX_BUDGET);
    assert_eq!(seam_over(One::text("a"), 64).budget(), 64);
    assert_eq!(DEFAULT_BUDGET, 64);
}

#[test]
fn an_exhausted_budget_makes_no_further_ask() {
    let mut seam = seam_over(One::text("is-a | mammal | 0.9"), 1);
    assert!(seam.ask("one", 0).is_some(), "the first ask is within budget");
    assert!(seam.exhausted(), "a one-ask budget is spent after one ask");
    assert!(seam.ask("two", 0).is_none(), "a second ask is refused");
    assert_eq!(seam.asked(), 1, "exactly one ask was made");
}

// --- Overflow (R3.9) -------------------------------------------------------

#[test]
fn a_held_row_consults_no_oracle() {
    let mut seam = seam_over(One::text("is-a | mammal | 0.9"), 64);
    assert!(seam.ask("prompt", 3).is_none(), "three rows already answer this ask");
    assert_eq!(seam.asked(), 0, "no ask was made against the budget");
    assert!(seam.outcomes().is_empty(), "nothing is recorded for an unconsulted ask");
}

#[test]
fn a_consulted_ask_records_the_claim_count() {
    let said = "is-a | mammal | 0.9\nhas | fur | 0.8";
    let mut seam = seam_over(One::text(said), 64);
    match seam.ask("prompt", 0).expect("consulted") {
        Outcome::Told { claims, index, .. } => {
            assert_eq!(*claims, 2, "both relation lines parse to a claim");
            assert_eq!(*index, 0, "the first ask is index 0");
        }
        other => panic!("expected a told answer, got {other:?}"),
    }
}

// --- The unreachable stop (R3.3) -------------------------------------------

#[test]
fn an_unreachable_program_stops_before_a_second_ask() {
    let mut seam = seam_over(One::err(OracleError::Unreachable("no-such-program".into())), 64);
    match seam.ask("one", 0).expect("the first ask runs").clone() {
        Outcome::Unreachable { program, index } => {
            assert_eq!(program, "no-such-program", "the error names the program");
            assert_eq!(index, 0);
        }
        other => panic!("expected unreachable, got {other:?}"),
    }
    assert_eq!(seam.asked(), 1, "the ask count is 1 (R3.3)");
    assert!(seam.stopped().unwrap_or("").contains("unreachable"), "the stop reason is recorded");
    assert!(seam.ask("two", 0).is_none(), "the quest stops before a second ask");
}

// --- The empty answer (R3.5) -----------------------------------------------

#[test]
fn an_empty_answer_is_counted_but_not_rejected() {
    let mut seam = seam_over(One::text("   \n\t  "), 64);
    match seam.ask("prompt", 0).expect("consulted").clone() {
        Outcome::Empty { index } => assert_eq!(index, 0),
        other => panic!("expected an empty answer, got {other:?}"),
    }
    assert_eq!(seam.asked(), 1, "the empty ask is counted against the budget (R3.5)");
    assert!(!seam.outcomes()[0].tells(), "an empty answer tells no row");
}

#[test]
fn an_empty_oracle_error_is_an_empty_answer_not_a_rejection() {
    let mut seam = seam_over(One::err(OracleError::Empty), 64);
    assert!(matches!(seam.ask("prompt", 0), Some(Outcome::Empty { .. })));
}

// --- A non-zero exit (R3.8) ------------------------------------------------

#[test]
fn a_failed_child_is_rejected_with_capped_stderr() {
    let long = "e".repeat(STDERR_CAP + 500);
    let mut seam = seam_over(One::err(OracleError::Failed(long)), 64);
    match seam.ask("prompt", 0).expect("consulted").clone() {
        Outcome::Rejected { why: Rejected::Exited { stderr, .. }, .. } => {
            assert_eq!(stderr.chars().count(), STDERR_CAP, "stderr is capped at 2,000 chars (R3.8)");
        }
        other => panic!("expected an exited rejection, got {other:?}"),
    }
    assert!(!seam.outcomes()[0].tells(), "a failed child tells no row");
}

// --- The structural check-before-tell rule (R8.10) -------------------------

/// The seam cannot be built without a Checker: `new` and `scripted` both take
/// one by value, so there is no path to a seam that tells a row unchecked.
/// This is a compile-time fact; the test exercises the one constructor that
/// spawns no child and confirms it holds a Checker to check against.
#[test]
fn a_seam_is_built_with_a_checker_and_scripted_spawns_no_child() {
    let mut seam = OracleSeam::scripted(&[("cat", "is-a | mammal | 0.9")], Box::new(Bare), 64);
    // The Checker is present and usable — the seam could not exist without it.
    let claim = phiecs::quest::Claim { kind: "is-a".into(), other: "mammal".into(), weight: 0.9 };
    let standing = seam.checker().check("cat", &claim, &phiecs::world::World::new());
    assert!(standing.tells(), "a sound claim is tellable, and it was checked");
    // The Scripted oracle answers from its table, spawning nothing (R3.13).
    assert!(matches!(seam.ask("Word: cat", 0), Some(Outcome::Told { .. })));
}
