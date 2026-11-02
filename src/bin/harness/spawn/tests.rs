//! Tests for the capped spawn's parser and counts.
//!
//! The OS paths — below-normal priority, the `taskkill` tree kill and the
//! 900-second cap — test Windows and its tools, not this code, so they get a
//! single example apiece under `os`. The parser is pinned by Property 56 and
//! by captured fixture output, including a fixture crate with a deliberate
//! syntax error.

use super::*;
use phistd::gen::{for_all, Gen, Seeded, CASES};

/// A libtest summary with its five count kinds, drawn in a plausible range.
#[derive(Debug, Clone)]
struct Summary {
    passed: usize,
    failed: usize,
    ignored: usize,
    measured: usize,
    filtered: usize,
    /// The order the five kinds are emitted in, and which spelling of the
    /// result verb the line opens with.
    order: usize,
    ok: bool,
}

impl Gen for Summary {
    fn draw(g: &mut Seeded) -> Self {
        Summary {
            passed: g.below(500) as usize,
            failed: g.below(50) as usize,
            ignored: g.below(20) as usize,
            measured: g.below(10) as usize,
            filtered: g.below(30) as usize,
            order: g.below(6) as usize,
            ok: bool::draw(g),
        }
    }
}

impl Summary {
    /// Render the line the way cargo does, permuting the five fields so the
    /// parser is not pinned to one order, and appending the `finished in`
    /// suffix cargo adds half the time.
    fn render(&self) -> String {
        let parts = [
            format!("{} passed", self.passed),
            format!("{} failed", self.failed),
            format!("{} ignored", self.ignored),
            format!("{} measured", self.measured),
            format!("{} filtered out", self.filtered),
        ];
        // Six rotations give six orders; passed and failed always present.
        let n = parts.len();
        let mut ordered: Vec<&String> = Vec::with_capacity(n);
        for i in 0..n {
            ordered.push(&parts[(i + self.order) % n]);
        }
        let verb = if self.ok { "ok" } else { "FAILED" };
        let body = ordered
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("; ");
        if self.order % 2 == 0 {
            format!("test result: {verb}. {body}; finished in 1.23s")
        } else {
            format!("test result: {verb}. {body}")
        }
    }
}

/// **Property 56: A level's test counts conserve**
///
/// For any libtest summary the parser accepts, in any order and spelling
/// cargo emits including ignored, measured and filtered counts, the parsed
/// passed plus failed equals the parsed run, and each kind equals what the
/// summary stated.
///
/// **Validates: Requirements 1.2**
#[test]
fn property_56_a_levels_test_counts_conserve() {
    let r = for_all::<Summary>(0x5C_0057, CASES, |s| {
        let rendered = s.render();
        let c = match parse_counts(&rendered) {
            Some(c) => c,
            None => return false,
        };
        c.conserves()
            && c.passed == s.passed
            && c.failed == s.failed
            && c.ignored == s.ignored
            && c.measured == s.measured
            && c.filtered == s.filtered
            && c.run == s.passed + s.failed
    });
    if let Err(f) = r {
        panic!("{f}");
    }
}

/// The captured output of a real passing run, parsed back.
const FIXTURE_PASS: &str = "\
running 3 tests
test a ... ok
test b ... ok
test c ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
";

/// The captured output of a run with failures, two `failures:` blocks as
/// libtest emits them.
const FIXTURE_FAIL: &str = "\
running 4 tests
test keeps ... ok
test drops ... FAILED
test burns ... FAILED
test holds ... ok

failures:

---- drops stdout ----
thread 'drops' panicked at src/lib.rs:4:5:
assertion failed
---- burns stdout ----
thread 'burns' panicked at src/lib.rs:9:5:
left != right

failures:
    burns
    drops

test result: FAILED. 2 passed; 2 failed; 1 ignored; 0 measured; 3 filtered out
";

#[test]
fn fixture_passing_run_parses_and_conserves() {
    let c = parse_counts(FIXTURE_PASS).expect("a summary line");
    assert_eq!(c.run, 3);
    assert_eq!(c.passed, 3);
    assert_eq!(c.failed, 0);
    assert!(c.conserves());
}

#[test]
fn fixture_failing_run_parses_counts_and_names() {
    let c = parse_counts(FIXTURE_FAIL).expect("a summary line");
    assert_eq!(c.passed, 2);
    assert_eq!(c.failed, 2);
    assert_eq!(c.ignored, 1);
    assert_eq!(c.filtered, 3);
    assert_eq!(c.run, 4);
    assert!(c.conserves());
    let names = failing_names(FIXTURE_FAIL);
    assert_eq!(names, vec!["burns".to_string(), "drops".to_string()]);
}

#[test]
fn no_summary_line_parses_to_none() {
    assert_eq!(parse_counts("cargo: nothing ran here"), None);
}

#[test]
fn first_build_error_takes_the_first_error_line() {
    let stderr = "\
   Compiling fixture v0.1.0
error[E0425]: cannot find value `nonesuch` in this scope
 --> src/lib.rs:2:5
error: aborting due to 1 previous error
";
    let e = first_build_error(stderr).expect("an error line");
    assert!(e.starts_with("error[E0425]"));
}

#[test]
fn cap_truncates_to_the_declared_prefix() {
    let long = "x".repeat(MESSAGE_CAP + 500);
    let capped = cap(&long);
    assert_eq!(capped.chars().count(), MESSAGE_CAP);
    assert!(long.starts_with(&capped));
}

#[test]
fn a_killed_level_records_the_cap_exactly() {
    let started = Instant::now();
    assert_eq!(elapsed_or_cap(started, true, CAP_SECONDS), 900.0);
}

/// A fixture crate holding a deliberate syntax error, compiled with
/// `cargo test --no-run` so the build failure is distinct from a test
/// failure (R1.9). Gated behind `--ignored` so the ordinary suite stays fast
/// and offline; run it with `cargo test -p phiano --bin harness -- --ignored`.
#[test]
#[ignore = "spawns cargo on a fixture crate; run explicitly on Windows"]
fn os_build_failure_is_distinguished_from_a_test_failure() {
    let manifest = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/broken/Cargo.toml"
    );
    let r = run_capped(
        &["test", "--no-run", "--manifest-path", manifest],
        CAP_SECONDS,
    )
    .expect("spawn cargo");
    match r {
        Capped::Finished { ok, err, .. } => {
            assert!(!ok, "the broken fixture must not compile");
            let e = first_build_error(&err).expect("a build error line");
            assert!(e.to_lowercase().contains("error"));
        }
        Capped::Killed => panic!("a tiny build should not hit the cap"),
    }
}
