pub mod api;
pub mod dialogue;
pub mod json;
pub mod local;
pub mod phi4;
pub mod wiktionary;

/// DictionarySource - a source of word definitions for bootstrapping the facet.
///
/// Implementations include local files, JSON dictionaries, API sources,
/// Wiktionary dumps, and Phi-4 references.
pub trait DictionarySource {
    /// Returns all (word, definition) pairs from this source.
    fn fetch_all(&self) -> Vec<(String, String)>;

    /// Returns all definitions for a single word from this source.
    fn fetch_definitions(&self, word: &str) -> Vec<String>;
}

/// Strips dictionary apparatus from a definition before it reaches the trainer.
///
/// Webster's entries carry part-of-speech markers, etymology brackets, sense
/// numbers and citation attributions. Trained as if they were content, those
/// tokens acquire positions in the manifold and then have to be blocked at
/// generation time by a hardcoded `boilerplate` list — which suppresses the
/// symptom while leaving the words wrongly *placed*. Cleaning at ingestion
/// fixes the cause.
pub fn clean_definition(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut depth_sq = 0i32;
    let mut depth_par = 0i32;

    for ch in raw.chars() {
        match ch {
            '[' => depth_sq += 1,
            ']' => depth_sq = (depth_sq - 1).max(0),
            '(' => depth_par += 1,
            ')' => depth_par = (depth_par - 1).max(0),
            _ if depth_sq == 0 && depth_par == 0 => out.push(ch),
            _ => {}
        }
    }

    // Drop leading sense numbers ("1.", "2.") and standalone apparatus tokens.
    let cleaned: Vec<String> = out
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
        .filter(|w| !w.is_empty())
        .filter(|w| !w.chars().all(|c| c.is_ascii_digit()))
        .filter(|w| !is_apparatus(w))
        .collect();

    cleaned.join(" ")
}

/// Dictionary metadata tokens — grammatical labels, editorial abbreviations and
/// citation attributions that are about the entry rather than part of it.
fn is_apparatus(word: &str) -> bool {
    matches!(
        word,
        "n" | "v" | "adj" | "adv" | "prep" | "conj" | "interj" | "pron"
            | "noun" | "verb" | "adjective" | "adverb" | "participle"
            | "plural" | "singular" | "pl" | "sing" | "imp" | "pp"
            | "obs" | "obsolete" | "archaic" | "rare" | "colloq" | "dial"
            | "cf" | "viz" | "ie" | "eg" | "etym" | "etymology"
            | "syn" | "opp" | "abbr" | "var" | "cap" | "usu"
            | "webster" | "unabridged" | "shak" | "milton" | "dryden"
            | "spenser" | "tennyson" | "chaucer" | "pope" | "bacon"
            | "see" | "sometimes" | "formerly"
    )
}

#[cfg(test)]
mod clean_tests {
    use super::*;

    #[test]
    fn test_brackets_and_apparatus_are_removed() {
        let raw = "1. (Zool.) A small [OE. catte] furry animal; n. -- Shak.";
        let c = clean_definition(raw);
        assert!(!c.contains("zool"), "parenthetical apparatus removed: {}", c);
        assert!(!c.contains("catte"), "etymology bracket removed: {}", c);
        assert!(!c.contains("shak"), "citation removed: {}", c);
        assert!(!c.split_whitespace().any(|w| w == "n"), "pos marker removed: {}", c);
        assert!(c.contains("furry") && c.contains("animal"), "content survives: {}", c);
    }

    #[test]
    fn test_plain_text_is_untouched_apart_from_case() {
        assert_eq!(clean_definition("An adult female person"), "an adult female person");
    }
}
