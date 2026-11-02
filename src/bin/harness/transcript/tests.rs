//! The transcript's tests: that every per-ask field is recorded (R3.4,
//! R3.10), that a refused claim is kept with the class it drew and tells no
//! row, that the page carries the Oracle's reported name (R3.12), and that a
//! `Scripted` oracle drives the whole thing with no child spawned (R3.13).

use super::*;
use crate::seam::OracleSeam;
use phiecs::quest::{Bare, Checker, Oracle, Scripted, Standing};
use phiecs::world::World;

/// The kinds a new World offers, which the quest parses against.
fn offered() -> Vec<String> {
    phiecs::quest::base_kinds()
}

/// Drive one ask through a seam over a `Scripted` oracle and record it. The
/// seam spawns no child (R3.13); the recording reads the seam's reported
/// Oracle name so the page attributes its numbers to it.
fn record_one(answers: &[(&str, &str)], prompt: &str, held: usize) -> (Recording, String) {
    let scripted = Scripted::new(answers);
    let name = Scripted::new(answers).name();
    let mut seam = OracleSeam::new(Box::new(scripted), Box::new(Bare), 64);
    let offered = offered();
    let mut rec = Recording::new(&name, &Bare.name());
    let mut checker = Bare;
    let world = World::new();
    match seam.ask(prompt, held) {
        Some(outcome) => {
            let outcome = outcome.clone();
            rec.record(prompt, held, &outcome, &offered, &mut checker, &world);
        }
        None => {} // Overflow or an exhausted budget: nothing to record.
    }
    (rec, name)
}

#[test]
fn a_told_answer_records_the_parsed_claims_and_the_rows_told() {
    let said = "is-a | mammal | 0.9\nhas | fur | 0.8";
    let (rec, _) = record_one(&[("cat", said)], "Word: cat", 0);
    let ask = &rec.asks()[0];
    assert_eq!(ask.index, 0, "the first ask is index 0");
    assert_eq!(ask.held, 0, "a consulted ask saw zero held rows (R3.9)");
    assert!(!ask.empty, "a told answer is not empty");
    assert_eq!(ask.rejection, None, "a told answer drew no rejection");
    assert_eq!(ask.parsed_count(), 2, "both relation lines parsed");
    assert_eq!(ask.told_count(), 2, "both sound claims told a row");
    assert_eq!(ask.refused_count(), 0, "nothing was refused");
}

#[test]
fn a_degenerate_claim_is_refused_with_its_class_and_tells_no_row() {
    // `cat | cat | 1.0` when asked about cat is a self-edge: the Bare checker
    // refuses it as degenerate, so it is kept with its class, not told.
    let said = "is-a | mammal | 0.9\nis-a | cat | 1.0";
    let (rec, _) = record_one(&[("cat", said)], "Word: cat", 0);
    let ask = &rec.asks()[0];
    assert_eq!(ask.parsed_count(), 2, "both lines parsed");
    assert_eq!(ask.told_count(), 1, "only the sound claim told a row");
    assert_eq!(ask.refused_count(), 1, "the self-edge was refused");
    let (claim, standing) = &ask.refused[0];
    assert_eq!(claim.other, "cat", "the refused claim is the self-edge");
    assert!(matches!(standing, Standing::Degenerate { .. }), "it drew the degenerate class");
}

#[test]
fn an_empty_answer_sets_the_empty_flag_and_parses_nothing() {
    let (rec, _) = record_one(&[("cat", "   \n\t ")], "Word: cat", 0);
    let ask = &rec.asks()[0];
    assert!(ask.empty, "an answer empty after trimming sets the flag (R3.5)");
    assert_eq!(ask.parsed_count(), 0, "an empty answer parses no claim");
    assert_eq!(ask.told_count(), 0, "and tells no row");
    assert_eq!(ask.rejection, None, "an empty answer is not a rejection");
}

#[test]
fn an_overflowing_ask_consults_no_oracle_and_records_nothing() {
    // A held-row count above zero is Overflow: the seam consults no Oracle,
    // so the recording has no ask to show (R3.9).
    let (rec, _) = record_one(&[("cat", "is-a | mammal | 0.9")], "Word: cat", 3);
    assert!(rec.asks().is_empty(), "an Overflow ask records nothing");
}

#[test]
fn the_page_carries_the_oracle_name_it_reports() {
    let said = "is-a | mammal | 0.9";
    let (rec, name) = record_one(&[("cat", said)], "Word: cat", 0);
    assert_eq!(name, "scripted", "the Scripted oracle names itself");
    let page = rec.render();
    assert!(page.contains("oracle: `scripted`"), "the page attributes to the Oracle's name (R3.12)");
    assert!(page.contains("checker: `bare`"), "and names the Checker that classified");
}

#[test]
fn the_page_carries_a_refused_claim_verbatim_for_the_frozen_set() {
    // Task 18.3 copies a false claim from this page, so the refused claim must
    // appear on it as the row-shaped line it was.
    let said = "is-a | mammal | 0.9\nis-a | cat | 1.0";
    let (rec, _) = record_one(&[("cat", said)], "Word: cat", 0);
    let page = rec.render();
    assert!(page.contains("refused (degenerate)"), "the refused claim is on the page with its class");
    assert!(page.contains("is-a | cat | 1"), "and it is copied verbatim for task 18.3");
}

#[test]
fn a_model_name_and_tag_survive_into_the_page() {
    // An Oracle that names a model and tag (as `ollama run phi4:14b
    // --nowordwrap` does) must have that whole name on the page (R3.12).
    let mut rec = Recording::new("ollama run phi4:14b --nowordwrap", "phisor::truth v1");
    let world = World::new();
    let mut checker = Bare;
    let told = crate::seam::Outcome::Told {
        index: 0,
        answer: "is-a | mammal | 0.9".into(),
        claims: 1,
    };
    rec.record("Word: cat", 0, &told, &offered(), &mut checker, &world);
    let page = rec.render();
    assert!(page.contains("phi4:14b"), "the model name and tag are attributed (R3.12)");
    assert!(page.contains("--nowordwrap"), "the whole reported name survives");
    assert!(page.contains("phisor::truth v1"), "the Checker's name survives too");
}

#[test]
fn writing_a_page_creates_the_transcripts_directory_and_a_named_file() {
    let said = "is-a | mammal | 0.9";
    let (rec, _) = record_one(&[("cat", said)], "Word: cat", 0);
    let dir = std::env::temp_dir().join(format!("phiano-transcript-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let path = rec.write(&dir, "cat").expect("the page is written");
    assert!(path.exists(), "the page exists on disk");
    let name = path.file_name().unwrap().to_string_lossy().into_owned();
    assert!(name.starts_with("cat_"), "the file is named by the seed word: {name}");
    assert!(name.contains("scripted"), "and by the Oracle's name: {name}");
    let body = std::fs::read_to_string(&path).expect("the page reads back");
    assert!(body.contains("oracle: `scripted`"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_rejected_answer_records_its_rejection_and_tells_no_row() {
    let why = Rejected::AnsiEscape { at: 3 };
    let outcome = crate::seam::Outcome::Rejected { index: 0, why: why.clone() };
    let mut rec = Recording::new("scripted", "bare");
    let world = World::new();
    let mut checker = Bare;
    rec.record("Word: cat", 0, &outcome, &offered(), &mut checker, &world);
    let ask = &rec.asks()[0];
    assert_eq!(ask.rejection, Some(why), "the rejection is recorded (R3.4)");
    assert_eq!(ask.parsed_count(), 0, "a rejected answer parses nothing");
    assert_eq!(ask.told_count(), 0, "and tells no row");
    assert!(rec.render().contains("rejected: ansi-escape"), "the page names the rejection");
}
