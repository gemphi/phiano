//! Speech act agent - Searle's theory of illocutionary force.
//! The classifier is phiton's `SpeechAct` (read from the markers in
//! data/searle_markers.json); this agent reports it for a prompt.

use super::marker;
use super::shape::*;

/// 4. SpeechActAgent - classifies illocutionary force with felicity conditions.
pub struct SpeechActAgent;

impl SpeechActAgent {
    /// Data-driven classification using the loaded markers.
    pub fn classify(prompt: &str) -> SpeechAct {
        SpeechAct::classify_marked(prompt, marker::load())
    }

    /// Extracts propositional content separately from illocutionary force.
    pub fn extract_propositional_content(prompt: &str) -> String {
        marker::load().propositional_content(prompt)
    }

    /// Felicity conditions per speech act type (Searle's conditions).
    pub fn felicity_conditions(act: SpeechAct, _prompt: &str) -> FelicityCondition {
        act.felicity()
    }

    /// Perlocutionary effect - the effect on the hearer.
    pub fn perlocutionary_effect(act: SpeechAct) -> &'static str {
        act.perlocutionary_effect()
    }

    /// Detects literal vs speaker meaning divergence.
    pub fn speaker_vs_literal_meaning(prompt: &str) -> (String, String) {
        marker::load().literal_and_speaker(prompt)
    }

    pub fn process(prompt: &str) -> AgentContribution {
        let act = Self::classify(prompt);
        let felicity = Self::felicity_conditions(act, prompt);
        let prop = Self::extract_propositional_content(prompt);
        let perloc = Self::perlocutionary_effect(act);
        let (literal, speaker) = Self::speaker_vs_literal_meaning(prompt);

        AgentContribution {
            agent_name: "SpeechAct",
            agent_role: "Illocutionary force + felicity conditions",
            confidence: 0.9,
            output: format!(
                "Act: {} | Content: \"{}\" | Felicity: {} | Perlocutionary: {} | Literal: \"{}\" | Speaker: \"{}\"",
                act.label(), prop, match felicity.satisfied {
                    true => "met",
                    false => "unmet",
                },
                perloc, literal, speaker
            ),
            phase_contribution: 0.0,
        }
    }
}
