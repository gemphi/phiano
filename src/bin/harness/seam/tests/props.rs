//! Property 35 and Property 36.

use super::super::*;
use super::SEED;
use super::{seam_over, One};
use phistd::gen::{Gen, Seeded};

// --- Property 35 -----------------------------------------------------------

/// A program name and its named arguments: `ollama` is one draw, any other
/// program another, with 0 to 4 arguments of arbitrary short words.
#[derive(Debug, Clone)]
struct Call {
    program: String,
    args: Vec<String>,
}

impl Gen for Call {
    fn draw(g: &mut Seeded) -> Self {
        let words = ["phi4", "mistral", "llama3", "run", "model-x", "a", ""];
        let program = if g.below(2) == 0 {
            "ollama".to_string()
        } else {
            (*g.pick(&["llama-cli", "my-script", "a-program"])).to_string()
        };
        let n = g.below(5) as usize;
        let args = (0..n).map(|_| (*g.pick(&words)).to_string()).collect();
        Call { program, args }
    }
}

/// **Property 35: A spawned Oracle's arguments are exactly what was named.**
///
/// For any program and any named arguments, `OracleSeam::spawn` gives `ollama`
/// exactly `run`, the model name and `--nowordwrap` — three arguments, none of
/// them drawn from the caller's extra arguments — and gives any other program
/// exactly the arguments named, with no argument of the seam's own (R3.1). The
/// `Spawned`'s declared name is read rather than the child run, so no process
/// is spawned (R3.13).
///
/// **Validates: Requirements 3.1**
#[test]
fn property_35_a_spawned_oracles_arguments_are_exactly_what_was_named() {
    for case in 0..256usize {
        let call = phistd::gen::replay::<Call>(SEED, case);
        let args: Vec<&str> = call.args.iter().map(String::as_str).collect();
        let name = OracleSeam::spawn(&call.program, &args).name();
        if call.program == "ollama" {
            let model = call.args.first().map(String::as_str).unwrap_or("");
            assert_eq!(
                name,
                format!("ollama run {model} --nowordwrap"),
                "case {case}: ollama gets run, the model and --nowordwrap, nothing of the caller's extra args"
            );
        } else {
            // `Spawned::name` reports `program` then the args joined by a
            // space; the seam adds no argument of its own, so the args in the
            // name are exactly those named, in order.
            let want = format!("{} {}", call.program, args.join(" "));
            assert_eq!(name, want, "case {case}: another program gets exactly its named arguments");
            let in_name = name.strip_prefix(&call.program).unwrap().trim();
            assert_eq!(in_name, args.join(" ").trim(), "case {case}: no argument of the seam's own");
        }
    }
}

// --- Property 36 -----------------------------------------------------------

/// One answer to validate: a length between 65,530 and 65,545 characters, and
/// either clean text or text with an ANSI escape spliced at a random offset.
#[derive(Debug, Clone)]
struct Reply {
    len: usize,
    ansi_offset: Option<usize>,
}

impl Gen for Reply {
    fn draw(g: &mut Seeded) -> Self {
        // Lengths straddling the 65,536-character ceiling, so a case lands on
        // each side of it (R3.6).
        let len = 65_530 + g.below(16) as usize;
        let ansi_offset = match g.below(2) == 0 {
            true => Some(g.below(len.max(1) as u64) as usize),
            false => None,
        };
        Reply { len, ansi_offset }
    }
}

impl Reply {
    /// Render the answer: `len` ordinary characters, with an ANSI escape
    /// spliced in at `ansi_offset` when one is asked for.
    fn render(&self) -> String {
        let mut s: String = "x".repeat(self.len);
        if let Some(at) = self.ansi_offset {
            let at = at.min(s.len());
            s.insert_str(at, "\u{1b}[31m");
        }
        s
    }
}

/// **Property 36: An answer is rejected exactly on the declared conditions,
/// and a rejection tells no row.**
///
/// For an answer of 65,530 to 65,545 characters, with an ANSI escape spliced
/// at a random offset or none, the seam rejects it exactly when it runs past
/// the 65,536-character ceiling or carries an ANSI escape, rejects on the
/// length first, and tells no row on any rejection (R3.4, R3.6, R3.10). An
/// answer that is neither over length nor escaped is told.
///
/// **Validates: Requirements 3.4, 3.6, 3.10**
#[test]
fn property_36_an_answer_is_rejected_exactly_on_the_declared_conditions() {
    for case in 0..256usize {
        let spec = phistd::gen::replay::<Reply>(SEED ^ 0x36, case);
        let text = spec.render();
        let chars = text.chars().count();

        let mut seam = seam_over(One::text(&text), 64);
        let outcome = seam.ask("prompt", 0).expect("the budget allows one ask").clone();

        let over_length = chars > CHAR_CAP;
        let has_ansi = spec.ansi_offset.is_some();

        match &outcome {
            Outcome::Rejected { why: Rejected::TooLong { chars: c }, .. } => {
                assert!(over_length, "case {case}: TooLong but {chars} is within the cap");
                assert_eq!(*c, chars, "case {case}: the recorded count is the count received");
            }
            Outcome::Rejected { why: Rejected::AnsiEscape { at }, .. } => {
                // The length ceiling is read first, so an escaped answer that
                // is also over length is TooLong, never AnsiEscape.
                assert!(!over_length, "case {case}: an over-length answer must be TooLong first");
                assert!(has_ansi, "case {case}: AnsiEscape but no escape was spliced");
                assert_eq!(text.as_bytes()[*at], 0x1b, "case {case}: the offset names the escape byte");
            }
            Outcome::Told { .. } => {
                assert!(!over_length && !has_ansi, "case {case}: told, so within the cap and unescaped");
            }
            other => panic!("case {case}: unexpected outcome {other:?}"),
        }
        // A rejection tells no row (R3.10); a told answer does.
        assert_eq!(outcome.tells(), matches!(outcome, Outcome::Told { .. }), "case {case}");
    }
}
