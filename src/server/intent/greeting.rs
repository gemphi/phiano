#[allow(unused_imports)]
use super::*;

impl ChatIntent {

    // ── PATTERN CLASSIFIER HELPERS ─────────────────────────────────────────────

    pub(super) fn is_greeting(p_lower: &str) -> bool {
    const GREETINGS: &[&str] = &[
        "hello", "hi", "hey", "good morning", "good afternoon", "good evening", "greetings", "howdy",
    ];
    let cleaned = p_lower.trim_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace());
    GREETINGS.iter().any(|&g| {
        cleaned.starts_with(g) || p_lower.starts_with(g)
    })
}

    pub(super) fn is_self_correction(p_lower: &str) -> bool {
    const CORRECTION_STARTS: &[&str] = &[
        "no, ", "no ", "that is wrong", "that's wrong", "incorrect", "correction:", "you are wrong", "you're wrong",
    ];
    CORRECTION_STARTS.iter().any(|&c| p_lower.starts_with(c) || p_lower.contains(c))
}

    /// Extracts the correction content by stripping the negation prefix.
    ///
    /// e.g. "no, that's wrong, dogs are mammals" → "dogs are mammals"
    pub(super) fn extract_correction_content(p_lower: &str) -> String {
    const CORRECTION_PREFIXES: &[&str] = &[
        "no, that's wrong, ", "no, that is wrong, ", "no, ",
        "that's wrong, ", "that is wrong, ",
        "incorrect, ", "correction: ",
        "you are wrong, ", "you're wrong, ",
        "no ", "incorrect ", "correction: ",
    ];
    for prefix in CORRECTION_PREFIXES {
        if let Some(rest) = p_lower.strip_prefix(prefix) {
            return rest.trim().to_string();
        }
    }
    p_lower.trim().to_string()
}

    pub(super) fn is_institutional_declaration(p_lower: &str, cog: &CognitiveResult) -> bool {
    cog.speech_act == "declarative"
        || p_lower.contains("i hereby")
        || p_lower.contains("i declare")
        || p_lower.contains("this meeting is")
}

    pub(super) fn is_personal_memory(p_lower: &str) -> bool {
    const PERSONAL_TRIGGERS: &[&str] = &[
        "my ", "i am ", "i love", "i live in", "we are", "allergic to", "my name is", "my daughter", "my son",
    ];
    PERSONAL_TRIGGERS.iter().any(|&t| p_lower.contains(t))
}

    pub(super) fn is_recommendation(p_lower: &str) -> bool {
    const RECOMMENDATION_TRIGGERS: &[&str] = &[
        "what books", "what topics", "what should", "recommend", "suggest", "where should", "how should i learn",
        "explore next", "books or topics",
    ];
    RECOMMENDATION_TRIGGERS.iter().any(|&r| p_lower.contains(r))
}

    pub(super) fn is_explanation_request(p_lower: &str) -> bool {
    const QUESTION_TRIGGERS: &[&str] = &[
        "what is", "define", "explain", "tell me about", "how does", "why do", "why does", "who is", "what are",
    ];
    QUESTION_TRIGGERS.iter().any(|&q| p_lower.contains(q)) || p_lower.ends_with('?')
}

    pub fn extract_topic_term(tokens: &[String], prompt: &str) -> String {
    const STOP_WORDS: &[&str] = &[
        "what", "is", "are", "were", "was", "be", "been", "being", "have", "has", "had", "a", "an", "the",
        "explain", "why", "do", "does", "did", "how", "can", "could", "will", "would", "shall", "should",
        "may", "might", "must", "you", "tell", "me", "about", "define", "who", "whom", "whose", "which",
        "where", "when", "she", "he", "they", "we", "i", "or", "and", "not", "next", "books", "recommend",
        "suggest", "for", "in", "on", "to", "with", "by", "at", "from", "into", "of", "considered", "between",
    ];

    for t in tokens {
        let t_clean = t.to_lowercase();
        let stripped = t_clean.trim_matches(|c: char| !c.is_alphabetic());
        if !STOP_WORDS.contains(&stripped) && stripped.len() >= 3 {
            return stripped.to_string();
        }
    }
        tokens.last().cloned().unwrap_or_else(|| prompt.to_string())
    }
}
