//! `harness` — the level-by-level evaluation harness (Requirement 1), a
//! binary and not a library so nothing can depend on it and no library target
//! names all ten crates (R1.11).
//!
//! This is the thin entry point: it declares the five sibling modules and,
//! when run, opens the Recorder over the declared Bars and walks the ten
//! levels in order through [`harness::Harness`] (task 17.5). The wiring, the
//! walk and the overall verdict live in `harness.rs`; the Bars reader, the
//! Recorder, the level table and the capped spawn in their own files.
//!
//! The actual walk shells out to `cargo test -p <level>` ten times, which is
//! slow, so `main` runs it only when asked (`--run`) and otherwise prints the
//! ten level names and what a run would do. `cargo test -p phiano --bin
//! harness` touches none of that: it runs the modules' own tests, which stay
//! fast and offline.
#![allow(dead_code)]

mod act;
mod bar;
mod curric;
mod curriculum;
mod harness;
mod ladder;
mod level;
mod record;
mod seam;
mod spawn;
mod transcript;

use std::path::Path;

/// Where the declared Bars are read from and where the run record is written.
const BARS_PATH: &str = "docs/bars.md";
const DOCS_DIR: &str = "docs";

fn main() {
    let run = std::env::args().any(|a| a == "--run");
    if !run {
        eprintln!(
            "harness: the ten levels are {}.\n\
             Pass --run to walk them with `cargo test -p <level>` and write a run record.",
            level::LEVELS.join(", ")
        );
        return;
    }
    if let Err(e) = run_walk() {
        eprintln!("harness: {e}");
        std::process::exit(1);
    }
}

/// Open the Recorder over the declared Bars and walk the ten levels, then
/// report the overall verdict and the record page's path. This is the heavy
/// path — ten cargo runs under the cap — kept off the ordinary test suite.
fn run_walk() -> Result<(), Box<dyn std::error::Error>> {
    let bars = bar::Bars::read(Path::new(BARS_PATH));
    let recorder = record::Recorder::open(Path::new(DOCS_DIR), &bars)?;
    let mut h = harness::Harness::new(bars, recorder, harness::Caps::default());

    let outcomes = h.run_all();
    let level_outcomes: Vec<level::Outcome> = outcomes.iter().map(|(_, o)| o.clone()).collect();
    for (name, outcome) in &outcomes {
        eprintln!("harness: {name} -> {outcome:?}");
    }

    // The ladder's eighteen numbers are measured by the heavy ladder run; a
    // walk with no ladder numbers judges the verdict on the levels alone.
    let mut numbers = ladder::run(h.bars());
    numbers.extend(curric::run(h.bars()));
    let bar_verdicts = h.record_ladder(&numbers);
    let verdict = harness::overall_verdict(&level_outcomes, &bar_verdicts);
    eprintln!("harness: overall verdict -> {verdict:?}");

    let page = h.close()?;
    eprintln!("harness: wrote run record {}", page.display());
    Ok(())
}
