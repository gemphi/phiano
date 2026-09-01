use crate::chunker::ChunkStore;
use crate::config::TWO_PI;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use crate::wave::Wave;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Similarity below which a candidate is not worth suggesting.
const SUGGEST_THRESHOLD: f64 = 0.45;
/// How many suggestions to offer per unknown word.
const SUGGEST_K: usize = 5;
/// Stop asking about a word after this many unanswered attempts.
const MAX_ASKS: u32 = 3;
/// Spelling / context blend. Context carries more signal when it exists.
const ORTHOGRAPHIC_WEIGHT: f64 = 0.4;
const SEMANTIC_WEIGHT: f64 = 0.6;

/// Vision - a projection of what the model doesn't yet know.
#[derive(Debug, Clone)]
pub struct Vision {
    pub text: String,
    /// Words resolved automatically from a source, without asking.
    pub auto_learned: Vec<String>,
}

impl fmt::Display for Vision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  [envision] {}", self.text)
    }
}

/// A record of what the model has asked about and what it still lacks.
///
/// `detect_gaps` used to be stateless, so the same unknown word asked about in
/// ten consecutive turns produced the same question ten times, with no way to
/// prioritise, escalate, or stop. Curiosity without memory is a tic.
#[derive(Debug, Default)]
pub struct GapLedger {
    asked: HashMap<String, u32>,
    encountered: HashMap<String, u32>,
    resolved: HashSet<String>,
    first_seen: HashMap<String, u64>,
}

impl GapLedger {
    pub fn new() -> Self {
        Self::default()
    }

    fn now_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }

    /// Notes that an unknown word was encountered.
    pub fn encounter(&mut self, word: &str) {
        *self.encountered.entry(word.to_string()).or_insert(0) += 1;
        self.first_seen.entry(word.to_string()).or_insert_with(Self::now_ms);
    }

    /// Whether it is still worth asking the user about this word.
    pub fn should_ask(&self, word: &str) -> bool {
        !self.resolved.contains(word) && *self.asked.get(word).unwrap_or(&0) < MAX_ASKS
    }

    pub fn mark_asked(&mut self, word: &str) {
        *self.asked.entry(word.to_string()).or_insert(0) += 1;
    }

    pub fn mark_resolved(&mut self, word: &str) {
        self.resolved.insert(word.to_string());
    }

    pub fn unresolved_count(&self) -> usize {
        self.encountered.keys().filter(|w| !self.resolved.contains(*w)).count()
    }

    /// Unresolved gaps, most frequently encountered first — a learning agenda
    /// rather than a per-turn reflex.
    pub fn unresolved_ranked(&self) -> Vec<(String, u32)> {
        let mut v: Vec<(String, u32)> = self
            .encountered
            .iter()
            .filter(|(w, _)| !self.resolved.contains(*w))
            .map(|(w, c)| (w.clone(), *c))
            .collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        v
    }
}

/// Envision - detects knowledge gaps and projects what to learn next.
///
/// This is the model's one genuinely agentic behaviour: notice a hole in
/// itself, name it, propose a hypothesis, and try to fill it.
#[derive(Debug, Default)]
pub struct Envision {
    pub ledger: GapLedger,
}

impl Envision {
    pub fn new() -> Self {
        Self::default()
    }

    /// Detects knowledge gaps, resolving what it can before asking.
    ///
    /// Escalation order: the local dictionary first (instant, offline), and the
    /// user only when that fails. Suggestions blend spelling with **context**:
    /// an unknown word has no phase of its own, but the known words beside it
    /// do, and ray casting turns that wave into semantic neighbours. Comparing
    /// spellings alone offered `photograph` for `photosynthesis` while the
    /// manifold that could have offered `plant` went unread.
    pub fn detect_gaps(
        &mut self,
        facet: &mut Facet,
        chunk_store: Option<&ChunkStore>,
        text: &str,
    ) -> Option<Vision> {
        let tokens = Tokenizer::tokenize(text);
        let unknown: Vec<String> = tokens
            .iter()
            .filter(|t| !facet.contains_word(t))
            .cloned()
            .collect();

        if unknown.is_empty() {
            return None;
        }

        // Context wave from the *known* words of the same input.
        let context = Wave::sentence(facet, &tokens);

        let mut auto_learned = Vec::new();
        let mut still_unknown = Vec::new();

        for word in &unknown {
            self.ledger.encounter(word);

            // Escalate to the dictionary before spending the user's attention.
            let resolved = chunk_store
                .and_then(|cs| cs.load_definition(word))
                .map(|def| {
                    facet.get_or_init(word);
                    (word.clone(), def)
                });

            match resolved {
                Some((w, _def)) => {
                    self.ledger.mark_resolved(&w);
                    auto_learned.push(w);
                }
                None if self.ledger.should_ask(word) => {
                    self.ledger.mark_asked(word);
                    still_unknown.push(word.clone());
                }
                None => {}
            }
        }

        if auto_learned.is_empty() && still_unknown.is_empty() {
            return None;
        }

        let mut parts = Vec::new();
        if !auto_learned.is_empty() {
            parts.push(format!("Looked up {} from the dictionary.", auto_learned.join(", ")));
        }

        if !still_unknown.is_empty() {
            let list = still_unknown
                .iter()
                .map(|w| format!("'{}'", w))
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("I don't know {}. Can you define them?", list));

            for word in &still_unknown {
                let suggestions = Self::suggest(facet, context, word);
                if !suggestions.is_empty() {
                    let rendered = suggestions
                        .iter()
                        .map(|(w, s, kind)| format!("{} ({:.2}, {})", w, s, kind))
                        .collect::<Vec<_>>()
                        .join(", ");
                    parts.push(format!("  Is '{}' related to {}?", word, rendered));
                }
            }
        }

        Some(Vision { text: parts.join("\n"), auto_learned })
    }

    /// Blends spelling similarity with context resonance.
    fn suggest(
        facet: &Facet,
        context: crate::wave::c64,
        word: &str,
    ) -> Vec<(String, f64, &'static str)> {
        let mut scored: HashMap<String, (f64, &'static str)> = HashMap::new();

        // --- orthographic: borrow, prefilter by length, clone only survivors ---
        let wlen = word.chars().count() as i64;
        for kw in facet.lexicon.keys() {
            // A word more than three characters different in length shares too
            // little to clear the threshold; skipping it avoids the bigram work.
            if (kw.chars().count() as i64 - wlen).abs() > 3 {
                continue;
            }
            let s = Self::string_similarity(word, kw);
            if s > SUGGEST_THRESHOLD {
                scored.insert(kw.clone(), (ORTHOGRAPHIC_WEIGHT * s, "spelling"));
            }
        }

        // --- semantic: what the surrounding known words point at ---
        if context.norm() > 1e-9 {
            for (w, delta) in Wave::ray_cast(facet, context, SUGGEST_K * 2) {
                if Tokenizer::is_function_word(&w) || w.len() < 2 {
                    continue;
                }
                let sim = (1.0 - delta).clamp(0.0, 1.0);
                let entry = scored.entry(w).or_insert((0.0, "context"));
                entry.0 += SEMANTIC_WEIGHT * sim;
                if entry.1 == "spelling" {
                    entry.1 = "both";
                }
            }
        }

        let mut out: Vec<(String, f64, &'static str)> =
            scored.into_iter().map(|(w, (s, k))| (w, s, k)).collect();
        out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        out.truncate(SUGGEST_K);
        out
    }

    /// Spelling similarity: 40% shared prefix, 60% character-bigram Jaccard.
    fn string_similarity(a: &str, b: &str) -> f64 {
        if a == b {
            return 1.0;
        }
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }

        let ac: Vec<char> = a.chars().collect();
        let bc: Vec<char> = b.chars().collect();

        let prefix = ac.iter().zip(bc.iter()).take_while(|(x, y)| x == y).count();
        let shorter = ac.len().min(bc.len());
        let prefix_score = match shorter {
            0 => 0.0,
            n => prefix as f64 / n as f64,
        };

        let a_bigrams: HashSet<(char, char)> = ac.windows(2).map(|w| (w[0], w[1])).collect();
        let b_bigrams: HashSet<(char, char)> = bc.windows(2).map(|w| (w[0], w[1])).collect();
        if a_bigrams.is_empty() || b_bigrams.is_empty() {
            return prefix_score;
        }

        let intersection = a_bigrams.intersection(&b_bigrams).count();
        let union = a_bigrams.union(&b_bigrams).count();
        let bigram_score = match union {
            0 => 0.0,
            u => intersection as f64 / u as f64,
        };

        prefix_score * 0.4 + bigram_score * 0.6
    }

    /// Wraps a phase into [0, 2π). Kept for callers of the old helper.
    #[allow(dead_code)]
    fn wrap(p: f64) -> f64 {
        p.rem_euclid(TWO_PI)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trainer::Trainer;

    #[test]
    fn test_stops_asking_after_the_limit() {
        let mut e = Envision::new();
        let mut f = Facet::new();
        let t = Trainer::new(0.05);
        t.train_sentence(&mut f, "the parser reads code");

        for _ in 0..MAX_ASKS {
            assert!(e.detect_gaps(&mut f, None, "the tokenizer reads code").is_some());
        }
        // Beyond the limit it should stop pestering about the same word.
        assert!(
            e.detect_gaps(&mut f, None, "the tokenizer reads code").is_none(),
            "an unanswered gap must not be asked about forever"
        );
    }

    #[test]
    fn test_ledger_ranks_by_frequency() {
        let mut e = Envision::new();
        let mut f = Facet::new();
        let t = Trainer::new(0.05);
        t.train_sentence(&mut f, "known words only");

        let _ = e.detect_gaps(&mut f, None, "known words only zzz");
        let _ = e.detect_gaps(&mut f, None, "known words only zzz");
        let _ = e.detect_gaps(&mut f, None, "known words only qqq");

        let ranked = e.ledger.unresolved_ranked();
        assert_eq!(ranked[0].0, "zzz", "the most-encountered gap should lead the agenda");
        assert_eq!(e.ledger.unresolved_count(), 2);
    }

    #[test]
    fn test_suggestions_use_context_not_only_spelling() {
        let mut f = Facet::new();
        let t = Trainer::new(0.05);
        for _ in 0..5 {
            t.train_sentence(&mut f, "the parser reads source code tokens");
        }
        let ctx = Wave::text(&f, "the parser reads source code");
        let s = Envision::suggest(&f, ctx, "tokenizer");
        assert!(!s.is_empty());
        assert!(
            s.iter().any(|(_, _, kind)| *kind == "context" || *kind == "both"),
            "context must contribute, not spelling alone"
        );
    }

    #[test]
    fn test_known_input_produces_no_vision() {
        let mut e = Envision::new();
        let mut f = Facet::new();
        let t = Trainer::new(0.05);
        t.train_sentence(&mut f, "everything here is known");
        assert!(e.detect_gaps(&mut f, None, "everything here is known").is_none());
    }
}
