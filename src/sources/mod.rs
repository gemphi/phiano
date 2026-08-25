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
