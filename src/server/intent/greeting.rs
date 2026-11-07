#[allow(unused_imports)]
use super::*;


/// One of the intent classifier's phrase lists, read once from
/// `data/intent/<name>.txt` (the ruling of 2026-11-07: no data in code).
/// The whole line is the entry, so "no, " keeps its trailing space.
fn intent_list(name: &'static str) -> &'static [String] {
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};
    static LISTS: OnceLock<Mutex<HashMap<&'static str, &'static [String]>>> = OnceLock::new();
    let mut lists = LISTS.get_or_init(Default::default).lock().unwrap_or_else(|e| e.into_inner());
    lists.entry(name).or_insert_with(|| {
        let path = format!("{}/data/intent/{name}.txt", env!("CARGO_MANIFEST_DIR"));
        Box::leak(phistd::lexfile::read(path).into_boxed_slice())
    })
}

impl ChatIntent {

    // ── PATTERN CLASSIFIER HELPERS ─────────────────────────────────────────────

    pub(super) fn is_greeting(p_lower: &str) -> bool {
    let GREETINGS = intent_list("greetings");
    let cleaned = p_lower.trim_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace());
    GREETINGS.iter().any(|g| {
        cleaned.starts_with(g) || p_lower.starts_with(g)
    })
}

    pub(super) fn is_self_correction(p_lower: &str) -> bool {
    let CORRECTION_STARTS = intent_list("correction_starts");
    CORRECTION_STARTS.iter().any(|c| p_lower.starts_with(c.as_str()) || p_lower.contains(c.as_str()))
}

    /// Extracts the correction content by stripping the negation prefix.
    ///
    /// e.g. "no, that's wrong, dogs are mammals" → "dogs are mammals"
    pub(super) fn extract_correction_content(p_lower: &str) -> String {
    let CORRECTION_PREFIXES = intent_list("correction_prefixes");
    for prefix in CORRECTION_PREFIXES {
        if let Some(rest) = p_lower.strip_prefix(prefix.as_str()) {
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
    let PERSONAL_TRIGGERS = intent_list("personal_triggers");
    PERSONAL_TRIGGERS.iter().any(|t| p_lower.contains(t.as_str()))
}

    pub(super) fn is_recommendation(p_lower: &str) -> bool {
    let RECOMMENDATION_TRIGGERS = intent_list("recommendation_triggers");
    RECOMMENDATION_TRIGGERS.iter().any(|r| p_lower.contains(r.as_str()))
}

    pub(super) fn is_explanation_request(p_lower: &str) -> bool {
    let QUESTION_TRIGGERS = intent_list("question_triggers");
    QUESTION_TRIGGERS.iter().any(|q| p_lower.contains(q.as_str())) || p_lower.ends_with('?')
}

    pub fn extract_topic_term(tokens: &[String], prompt: &str) -> String {
    let STOP_WORDS = intent_list("stop_words");

    for t in tokens {
        let t_clean = t.to_lowercase();
        let stripped = t_clean.trim_matches(|c: char| !c.is_alphabetic());
        if !STOP_WORDS.iter().any(|w| w == stripped) && stripped.len() >= 3 {
            return stripped.to_string();
        }
    }
        tokens.last().cloned().unwrap_or_else(|| prompt.to_string())
    }
}
