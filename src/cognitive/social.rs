/// Social ontology agents — Searle's theory of social reality.
/// All markers loaded from data/searle_markers.json (no hardcoded arrays).

use super::types::*;
use super::markers::SearleMarkers;
use crate::facet::Facet;

/// 13. SocialOntologyAgent — classifies brute vs institutional facts.
pub struct SocialOntologyAgent;

impl SocialOntologyAgent {
    /// Applies constitutive rules: "X counts as Y in context C".
    pub fn counts_as_rules(prompt: &str, markers: &SearleMarkers) -> Vec<String> {
        let p = prompt.to_lowercase();
        markers.counts_as_rules.iter()
            .filter(|(k, _)| p.contains(k.as_str()))
            .map(|(_, v)| v.clone())
            .collect()
    }

    pub fn process(prompt: &str) -> AgentContribution {
        let markers = SearleMarkers::load();
        let institutional = SearleMarkers::count_matches(prompt, &markers.institutional_markers);
        let brute = SearleMarkers::count_matches(prompt, &markers.brute_markers);

        let (category, confidence) = match (institutional > brute, brute > institutional) {
            (true, _) => ("institutional fact (exists by collective acceptance)", 0.85),
            (_, true) => ("brute fact (exists independently of minds)", 0.85),
            _ => ("mixed/ambiguous (requires further analysis)", 0.5),
        };

        let rules = Self::counts_as_rules(prompt, &markers);
        let rules_str = if rules.is_empty() {
            "No constitutive rules triggered".to_string()
        } else {
            format!("Constitutive rules: {}", rules.join("; "))
        };

        AgentContribution {
            agent_name: "SocialOntology",
            agent_role: "Brute vs institutional facts + counts-as rules",
            confidence,
            output: format!("{} (institutional={}, brute={}). {}", category, institutional, brute, rules_str),
            phase_contribution: 0.0,
        }
    }
}

/// 14. ObserverRelativityAgent — detects perspective-dependent meaning.
pub struct ObserverRelativityAgent;

impl ObserverRelativityAgent {
    pub fn process(_facet: &Facet, prompt: &str) -> AgentContribution {
        let markers = SearleMarkers::load();
        let p = prompt.to_lowercase();
        let perspectives = [
            ("first_person (I)", &["i", "me", "my", "mine", "myself"][..]),
            ("second_person (you)", &["you", "your", "yours", "yourself"][..]),
            ("collective (we)", &["we", "us", "our", "ours", "ourselves"][..]),
            ("third_party (he/she/they)", &["he", "she", "they", "it", "them", "their"][..]),
        ];

        let tokens: std::collections::HashSet<&str> = p.split_whitespace().collect();
        let mut best = "neutral (no perspective)";
        let mut best_count = 0;
        for (name, marker_list) in &perspectives {
            let count = marker_list.iter().filter(|m| tokens.contains(*m)).count();
            if count > best_count {
                best_count = count;
                best = name;
            }
        }

        let observer_relative = SearleMarkers::contains_any(prompt, &markers.observer_relative_markers);
        let observer_note = if observer_relative {
            " — contains observer-relative features (good/bad/useful depend on perspective)"
        } else {
            ""
        };

        AgentContribution {
            agent_name: "ObserverRelativity",
            agent_role: "Perspective detection + observer-relative features",
            confidence: if best_count > 0 { 0.75 } else { 0.4 },
            output: format!("Perspective: {} (markers={}){}", best, best_count, observer_note),
            phase_contribution: 0.0,
        }
    }
}

/// 15. CollectiveIntentionAgent — aggregates agent perspectives.
pub struct CollectiveIntentionAgent;

impl CollectiveIntentionAgent {
    pub fn process(contributions: &[AgentContribution]) -> AgentContribution {
        let avg_confidence = contributions.iter()
            .map(|c| c.confidence)
            .sum::<f64>() / contributions.len().max(1) as f64;

        let collective_phase = contributions.iter()
            .map(|c| c.phase_contribution)
            .sum::<f64>().rem_euclid(2.0 * std::f64::consts::PI);

        AgentContribution {
            agent_name: "CollectiveIntention",
            agent_role: "Irreducible 'we intend' aggregation",
            confidence: avg_confidence,
            output: format!(
                "Collective intentionality: {:.0}% across {} agents, phase={:.3}",
                avg_confidence * 100.0, contributions.len(), collective_phase
            ),
            phase_contribution: collective_phase,
        }
    }
}
