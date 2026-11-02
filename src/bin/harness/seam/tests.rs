//! The Oracle_Seam's tests, split past the 250-line cap into single-word
//! files under `tests/`: `props` holds Property 35 (the arguments a spawn
//! names) and Property 36 (the rejection conditions, with an ANSI escape
//! spliced at a random offset and lengths from 65,530 to 65,545); `units`
//! holds the facts that pin the budget, Overflow, the unreachable stop, the
//! empty answer and the structural check-before-tell rule.
//!
//! None of these spawn a child: Property 35 reads a `Spawned`'s declared
//! `name` rather than running it, and every other test drives the seam with a
//! `Scripted` Oracle or the tiny `One` stub below, as R3.13 requires of a
//! test run. The shared harness — the run seed and the one-answer stub — sits
//! here so both files reach it through `super`.

use super::*;
use phiecs::quest::OracleError;

#[path = "tests/props.rs"]
mod props;
#[path = "tests/units.rs"]
mod units;

/// The run's declared seed, matching the sibling modules' convention.
const SEED: u64 = 0x5747_4f4c_4341_494d;

/// An Oracle that answers one crafted string once, then nothing. Used to feed
/// the seam an exact answer and read back how it was classified.
struct One {
    answer: Option<Result<String, OracleError>>,
}

impl One {
    fn text(s: &str) -> Self {
        One { answer: Some(Ok(s.to_string())) }
    }
    fn err(e: OracleError) -> Self {
        One { answer: Some(Err(e)) }
    }
}

impl Oracle for One {
    fn ask(&mut self, _prompt: &str) -> Result<String, OracleError> {
        self.answer.take().unwrap_or(Err(OracleError::Empty))
    }
}

/// A seam over the one-answer stub, a [`Bare`](phiecs::quest::Bare) checker
/// and the given budget.
fn seam_over(oracle: One, budget: u32) -> OracleSeam {
    OracleSeam::new(Box::new(oracle), Box::new(phiecs::quest::Bare), budget)
}
