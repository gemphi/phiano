/// Searle markers loaded from data/searle_markers.json.
/// No hardcoded word lists - all markers are data-driven.

use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

static MARKERS: OnceLock<SearleMarkers> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
pub struct SearleMarkers {
    pub indirect_patterns: Vec<String>,
    pub commissive_markers: Vec<String>,
    pub expressive_markers: Vec<String>,
    pub declarative_markers: Vec<String>,
    pub directive_question_markers: Vec<String>,
    pub directive_command_markers: Vec<String>,
    pub rhetorical_markers: Vec<String>,
    pub institutional_markers: Vec<String>,
    pub brute_markers: Vec<String>,
    pub counts_as_rules: HashMap<String, String>,
    pub observer_relative_markers: Vec<String>,
}

impl SearleMarkers {
    /// Loads from data/searle_markers.json (cached via OnceLock).
    pub fn load() -> &'static SearleMarkers {
        MARKERS.get_or_init(|| {
            let path = std::path::Path::new("data/searle_markers.json");
            match std::fs::read_to_string(path) {
                Ok(content) => serde_json::from_str(&content).unwrap_or_else(|e| {
                    eprintln!("[searle] Failed to parse markers JSON: {}", e);
                    SearleMarkers::default_inner()
                }),
                Err(e) => {
                    eprintln!("[searle] Could not load data/searle_markers.json: {}", e);
                    SearleMarkers::default_inner()
                }
            }
        })
    }

    fn default_inner() -> Self {
        Self {
            indirect_patterns: vec![],
            commissive_markers: vec![],
            expressive_markers: vec![],
            declarative_markers: vec![],
            directive_question_markers: vec![],
            directive_command_markers: vec![],
            rhetorical_markers: vec![],
            institutional_markers: vec![],
            brute_markers: vec![],
            counts_as_rules: HashMap::new(),
            observer_relative_markers: vec![],
        }
    }

    /// Checks if text contains any marker from a list.
    pub fn contains_any(text: &str, markers: &[String]) -> bool {
        let p = text.to_lowercase();
        markers.iter().any(|m| p.contains(m.as_str()))
    }

    /// Checks if text starts with any marker from a list.
    pub fn starts_with_any(text: &str, markers: &[String]) -> bool {
        let p = text.to_lowercase();
        markers.iter().any(|m| p.starts_with(m.as_str()))
    }

    /// Counts how many markers from a list appear in text.
    pub fn count_matches(text: &str, markers: &[String]) -> usize {
        let p = text.to_lowercase();
        markers.iter().filter(|m| p.contains(m.as_str())).count()
    }
}
