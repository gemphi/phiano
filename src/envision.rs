use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use std::cmp::Ordering;
use std::fmt;

/// Vision - a projection of what the model doesn't yet know.
///
/// Produced by the envision phase of the recursive learning cycle.
/// Contains the list of unknown words and, for each, a list of
/// related known words with similarity scores.
#[derive(Debug, Clone)]
pub struct Vision {
    pub text: String,
}

impl fmt::Display for Vision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  [envision] {}", self.text)
    }
}

/// Envision - detects knowledge gaps and projects what to learn next.
///
/// This is the "envision" phase of the recursive learning cycle:
/// `envision → apply → eval → iterate → scale`.
///
/// After each user input, the envisioner checks for unknown words
/// and suggests related known words that might help define them.
pub struct Envision;

impl Envision {
    /// Creates a new envisioner.
    pub fn new() -> Self {
        Self
    }

    /// Detects knowledge gaps in the given text.
    ///
    /// Tokenizes the input and checks which words are missing from the facet.
    /// For each unknown word, finds the top 5 most similar known words
    /// using a string similarity heuristic (prefix + bigram overlap).
    ///
    /// Returns `None` if all words are already known.
    pub fn detect_gaps(&self, facet: &Facet, text: &str) -> Option<Vision> {
        let tokens = Tokenizer::tokenize(text);

        let unknown: Vec<String> = tokens
            .iter()
            .filter(|t| !facet.contains_word(t))
            .cloned()
            .collect();

        if unknown.is_empty() {
            return None;
        }

        let mut related = Vec::new();
        for word in &unknown {
            let mut candidates: Vec<(String, f64)> = facet
                .lexicon
                .keys()
                .map(|kw| (kw.clone(), Self::string_similarity(word, kw)))
                .filter(|(_, score)| *score > 0.5)
                .collect();

            candidates.sort_by(|a, b| {
                b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal)
            });
            candidates.truncate(5);
            related.push((word.clone(), candidates));
        }

        let word_list = unknown
            .iter()
            .map(|w| format!("'{}'", w))
            .collect::<Vec<_>>()
            .join(", ");

        let mut parts = vec![
            format!("I don't know {}. Can you define them?", word_list),
        ];

        for (word, rel) in &related {
            if !rel.is_empty() {
                let suggestions = rel
                    .iter()
                    .map(|(w, s)| format!("{} ({:.2})", w, s))
                    .collect::<Vec<_>>()
                    .join(", ");
                parts.push(format!("  Is '{}' related to {}?", word, suggestions));
            }
        }

        Some(Vision {
            text: parts.join("\n"),
        })
    }

    /// Computes a string similarity score between two words.
    ///
    /// Combines two signals:
    /// - **Prefix overlap** (40% weight): fraction of shared leading characters
    /// - **Bigram Jaccard** (60% weight): intersection over union of character bigrams
    ///
    /// Returns 1.0 for identical strings, 0.0 for empty strings.
    fn string_similarity(a: &str, b: &str) -> f64 {
        if a == b {
            return 1.0;
        }
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }

        let ac: Vec<char> = a.chars().collect();
        let bc: Vec<char> = b.chars().collect();

        let prefix = ac
            .iter()
            .zip(bc.iter())
            .take_while(|(x, y)| x == y)
            .count();

        let shorter = ac.len().min(bc.len());
        let prefix_score = if shorter > 0 {
            prefix as f64 / shorter as f64
        } else {
            0.0
        };

        let a_bigrams: Vec<String> = ac
            .windows(2)
            .map(|w| w.iter().collect())
            .collect();
        let b_bigrams: Vec<String> = bc
            .windows(2)
            .map(|w| w.iter().collect())
            .collect();

        if a_bigrams.is_empty() || b_bigrams.is_empty() {
            return prefix_score;
        }

        let intersection = a_bigrams.iter().filter(|bg| b_bigrams.contains(bg)).count();
        let union = a_bigrams.len() + b_bigrams.len() - intersection;
        let bigram_score = if union > 0 {
            intersection as f64 / union as f64
        } else {
            0.0
        };

        prefix_score * 0.4 + bigram_score * 0.6
    }
}

impl Default for Envision {
    fn default() -> Self {
        Self::new()
    }
}
