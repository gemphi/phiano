#[allow(unused_imports)]
use super::*;

use super::Parser;

/// The bug this replaced: `split_whitespace()[0]` on a quoted argument
/// silently kept one word, so `resonance "a large four-footed animal" 8`
/// queried `a`.
#[test]
fn test_quoted_text_survives_intact_with_its_count() {
    let (t, n) = Parser::text_and_count("\"a large four-footed animal\" 8", 5);
    assert_eq!(t, "a large four-footed animal");
    assert_eq!(n, 8);

    let (t, n) = Parser::text_and_count("'single quoted here'", 5);
    assert_eq!(t, "single quoted here");
    assert_eq!(n, 5, "no count given, so the default stands");
}

#[test]
fn test_unquoted_word_and_count() {
    assert_eq!(Parser::text_and_count("ownership 5", 3), ("ownership".into(), 5));
    assert_eq!(Parser::text_and_count("ownership", 3), ("ownership".into(), 3));
    // A lone numeric argument is the subject, not a count — `synonym 7`
    // asks about the word seven.
    assert_eq!(Parser::text_and_count("7", 3), ("7".into(), 3));
    // Unquoted multi-word input keeps every word.
    assert_eq!(
        Parser::text_and_count("large four footed animal", 5),
        ("large four footed animal".into(), 5)
    );
    assert_eq!(Parser::text_and_count("", 5), (String::new(), 5));
}
