/// Tokenizer — text normalization and tokenization utility.
///
/// All text is lowercased and stripped of non-alphanumeric characters
/// before being split into tokens. This provides a consistent input
/// representation across the trainer, evaluator, and envisioner.
pub struct Tokenizer;

impl Tokenizer {
    /// Tokenizes text into a list of lowercase alphanumeric words.
    ///
    /// Non-alphanumeric characters are stripped from each token,
    /// and empty tokens are filtered out.
    pub fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|w| {
                w.chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            })
            .filter(|w| !w.is_empty())
            .collect()
    }

    /// Splits text into sentences on `.`, `!`, or `?` delimiters.
    ///
    /// Empty sentences are filtered out.
    pub fn split_sentences(text: &str) -> Vec<String> {
        text.split(|c| c == '.' || c == '!' || c == '?')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let tokens = Tokenizer::tokenize("Hello world");
        assert_eq!(tokens, vec!["hello", "world"]);
    }

    #[test]
    fn test_tokenize_strips_punctuation() {
        let tokens = Tokenizer::tokenize("Hello, world! How's it going?");
        assert_eq!(tokens, vec!["hello", "world", "hows", "it", "going"]);
    }

    #[test]
    fn test_tokenize_empty() {
        let tokens = Tokenizer::tokenize("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_tokenize_whitespace_only() {
        let tokens = Tokenizer::tokenize("   ");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_split_sentences_basic() {
        let s = Tokenizer::split_sentences("Hello. World! How?");
        assert_eq!(s, vec!["Hello", "World", "How"]);
    }

    #[test]
    fn test_split_sentences_empty() {
        let s = Tokenizer::split_sentences("");
        assert!(s.is_empty());
    }

    #[test]
    fn test_split_sentences_no_terminator() {
        let s = Tokenizer::split_sentences("just words");
        assert_eq!(s, vec!["just words"]);
    }
}
