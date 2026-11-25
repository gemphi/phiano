//! The words of a line a person typed, a quoted run kept whole, and the
//! command word a line opens with.
//!
//! `compose` and `persona` both took an argument apart this way, each with
//! its own copy of the scan (2026-11-24); the scan lives here once. So did
//! the first cut of a line - the command word and the rest - which the
//! dispatcher, the driver dispatcher, the prompt loop and the server's
//! command route each wrote out (2026-11-25).

/// A line read as a command: the word it opens with, and the rest of it
/// trimmed. Both are empty for an empty line, and the rest is empty for a
/// line of one word.
pub fn asked(line: &str) -> (&str, &str) {
    let mut part = line.splitn(2, char::is_whitespace);
    (part.next().unwrap_or(""), part.next().map(str::trim).unwrap_or(""))
}

/// The words of `line`: a run between a pair of quotes is one word, whichever
/// quote opened it, and whitespace parts the rest. What was built before an
/// opening quote is dropped, so `add "a name" rest` reads the name whole.
pub fn quoted(line: &str) -> Vec<String> {
    let mut said = Vec::new();
    let mut word = String::new();
    let mut quote: Option<char> = None;
    for ch in line.chars() {
        match quote {
            None if ch == '"' || ch == '\'' => {
                quote = Some(ch);
                word.clear();
            }
            Some(q) if ch == q => {
                quote = None;
                say(&mut word, &mut said);
            }
            None if ch.is_whitespace() => say(&mut word, &mut said),
            _ => word.push(ch),
        }
    }
    say(&mut word, &mut said);
    said
}

/// The word built so far, if there is one, and nothing in its place.
fn say(word: &mut String, said: &mut Vec<String>) {
    match word.is_empty() {
        true => {}
        false => said.push(std::mem::take(word)),
    }
}

#[cfg(test)]
mod tests {
    use super::asked;

    /// What the dispatchers read off a line, and what the argument of a
    /// command is before the command sees it.
    #[test]
    fn a_line_is_a_command_word_and_the_rest() {
        assert_eq!(asked("define cat"), ("define", "cat"));
        assert_eq!(asked("compose   \"a prompt\" 8"), ("compose", "\"a prompt\" 8"));
        assert_eq!(asked("stats"), ("stats", ""));
        assert_eq!(asked(""), ("", ""));
        assert_eq!(asked("  leading"), ("", "leading"), "a line is cut where it is");
    }

    use super::quoted;

    #[test]
    fn a_quoted_run_is_one_word() {
        assert_eq!(quoted("add \"a long name\" warm"), ["add", "a long name", "warm"]);
        assert_eq!(quoted("'single quotes' too"), ["single quotes", "too"]);
        assert_eq!(quoted("  spaced   out  "), ["spaced", "out"]);
        assert_eq!(quoted(""), [] as [String; 0]);
    }

    #[test]
    fn a_quote_opening_mid_word_drops_what_stood_before_it() {
        // The scan has read a line this way since it was written: a quote
        // opens a word rather than joining one, so an apostrophe inside a
        // word takes the word with it. Commands are typed with the quotes
        // around a whole argument, where this does not show.
        assert_eq!(quoted("it's fine"), ["s fine"]);
    }
}
