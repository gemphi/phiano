//! The level table — the ten crates the Harness walks, the four outcomes one
//! level's run can come to, and `compiled_for`, the crates
//! `cargo test -p <level>` draws in (R1.1, R1.7, R1.8, R11.11).
//!
//! The Harness is a binary, not a library, so it links none of the ten crates
//! it polices for the sake of naming them, and it cannot link the Structure_-
//! Check's integration-test target either. The permitted-edge graph is
//! therefore written out here over the same ten names in the same order as
//! `phinet/tests/structural/mod.rs`'s `PERMITTED` — phiano now including
//! phisor — rather than linked. `the_level_graph_matches_the_structural_table`
//! guards against the two copies drifting apart by holding this table to the
//! shape the structural test pins.
//!
//! What this file owns is only the table, the outcomes and the closure. The
//! `Harness` struct, `run_one`, `run_all` and the capped spawn live in their
//! own files (tasks 17.4 and 17.5); the one thing they borrow from here is
//! `HarnessError`, the error an unaccepted level name reports.

/// The ten phinet crates in dependency order: the order a level's tests
/// compile in, and the order `run_all` starts them in (R1.1). phistd is the
/// bottom, phiano the top.
pub const LEVELS: [&str; 10] = [
    "phistd", "phifno", "phiton", "phical", "phiecs", "phiclm", "phient", "philog", "phisor",
    "phiano",
];

/// The permitted normal edges between the ten crates (R11.2), written out
/// here because the binary links no crate it polices. Each crate's list is
/// the crates it may declare as a normal dependency, which is the set whose
/// transitive closure `compiled_for` walks. This is a copy of the structural
/// `PERMITTED` table — phiano on phisor included — and must stay equal to it.
const PERMITTED: &[(&str, &[&str])] = &[
    ("phistd", &[]),
    ("phifno", &["phistd"]),
    ("phiton", &["phistd"]),
    ("phical", &["phistd", "phifno"]),
    ("phiecs", &["phistd", "phiton", "phical"]),
    ("phiclm", &["phiecs"]),
    ("phient", &["phistd", "phical", "phiton", "phiecs", "phiclm"]),
    ("philog", &["phient", "phiecs", "phiton"]),
    ("phisor", &["phiecs", "phiton", "phical", "phistd", "phiclm"]),
    (
        "phiano",
        &["phistd", "phiton", "phical", "phifno", "phient", "philog", "phiecs", "phiclm", "phisor"],
    ),
];

/// What one level's run came to (R1.2, R1.3, R1.6, R1.9). The four shapes are
/// exclusive: a level that passed carries its three counts and its seconds, a
/// level that failed carries those plus the name and message of every failing
/// test, a build failure carries the first compiler error and no test counts,
/// and a timeout carries only the elapsed cap.
///
/// The capped spawn (task 17.4) builds these from one cargo run and the
/// Harness (task 17.5) reports them; until those land the variants have no
/// in-crate caller, so the dead-code warning is allowed here rather than
/// faked away by a throwaway use.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    /// Every test passed: `passed + failed == run` and `failed == 0`.
    Passed { run: usize, passed: usize, failed: usize, seconds: f64 },
    /// At least one test failed; each failure names the test and its message
    /// (R1.3). `passed + failed == run`.
    Failed { run: usize, passed: usize, failed: usize, seconds: f64, failures: Vec<(String, String)> },
    /// The sources did not compile; the first compiler error is kept and no
    /// test counts are recorded (R1.9), distinct from `Failed`.
    BuildFailed { seconds: f64, first_error: String },
    /// The run passed the wall-clock cap and was killed; the elapsed time is
    /// the cap and no test counts are recorded (R1.6).
    TimedOut { seconds: f64 },
}

/// The error an unaccepted level name reports: the value given and all ten
/// accepted names, so the message is complete on its own (R1.8). An
/// unaccepted name runs no level and writes no record — the caller in
/// `run_one` (task 17.5) returns this without touching the Recorder.
#[derive(Clone, Debug, PartialEq)]
pub enum HarnessError {
    UnknownLevel { given: String, accepted: [&'static str; 10] },
}

impl std::fmt::Display for HarnessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HarnessError::UnknownLevel { given, accepted } => write!(
                f,
                "unknown level {given:?}; the ten accepted level names are {}",
                accepted.join(", ")
            ),
        }
    }
}

impl std::error::Error for HarnessError {}

/// Is `name` one of the ten declared level names (R1.1, R1.8)? The match is
/// exact: a case variant, a padded name, a prefix or a suffix is not accepted.
pub fn is_level(name: &str) -> bool {
    LEVELS.contains(&name)
}

/// Accept a level name or report it unaccepted, naming the given value and all
/// ten accepted names (R1.8). This runs no level and writes no record; it only
/// answers whether the name is one the Harness knows.
pub fn accept(given: &str) -> Result<&'static str, HarnessError> {
    LEVELS
        .iter()
        .copied()
        .find(|&l| l == given)
        .ok_or_else(|| HarnessError::UnknownLevel { given: given.to_string(), accepted: LEVELS })
}

/// The crates `cargo test -p <level>` compiles: the transitive closure of the
/// level's dependencies in the permitted-edge graph together with the level's
/// own crate, and nothing above it (R11.11). The result is in dependency
/// order — every crate appears before any crate that depends on it, so the
/// level itself is last — which is also `LEVELS` order restricted to the
/// closure. An unaccepted name returns an empty list, naming nothing to
/// compile.
pub fn compiled_for(level: &str) -> Vec<&'static str> {
    closure_in(PERMITTED, level)
}

/// The transitive closure of `level`'s dependencies in `graph`, together with
/// `level` itself, in dependency order. Shared by `compiled_for` and by
/// Property 60's exhaustive check over generated graphs, so one closure walk
/// is tested rather than two. A name absent from `graph` closes to nothing.
pub(crate) fn closure_in(graph: &[(&'static str, &'static [&'static str])], level: &str) -> Vec<&'static str> {
    if !graph.iter().any(|(c, _)| *c == level) {
        return Vec::new();
    }
    let mut reached: Vec<&'static str> = Vec::new();
    let mut stack: Vec<&'static str> = vec![graph.iter().find(|(c, _)| *c == level).map(|(c, _)| *c).unwrap()];
    while let Some(c) = stack.pop() {
        if reached.contains(&c) {
            continue;
        }
        reached.push(c);
        if let Some((_, deps)) = graph.iter().find(|(name, _)| *name == c) {
            for &d in *deps {
                if !reached.contains(&d) {
                    stack.push(d);
                }
            }
        }
    }
    // Dependency order: a crate before anything that depends on it. The graph
    // is acyclic and topologically ordered as written, so ordering by each
    // crate's index in `graph` is a topological order, and the level — which
    // depends on every other member of its own closure — lands last.
    reached.sort_by_key(|c| graph.iter().position(|(name, _)| name == c).unwrap_or(usize::MAX));
    reached
}

#[cfg(test)]
mod tests;
