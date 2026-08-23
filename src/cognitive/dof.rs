/// Direction of fit and satisfaction agents.
///
/// Split from speech_acts.rs to keep files under 200 lines.
/// These agents depend on SpeechActAgent's classification.

use super::types::*;
use super::speech_acts::SpeechActAgent;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;

/// 5. DirectionOfFitAgent — maps speech act to mind-world alignment.
/// Searle's direction of fit:
/// - Assertives: mind→world (belief should match reality)
/// - Directives: world→mind (reality should change to match desire)
/// - Commissives: world→mind (speaker commits to changing reality)
/// - Expressives: none (no fit — just expressing a state)
/// - Declaratives: both (declaration creates the fact it represents)
pub struct DirectionOfFitAgent;

impl DirectionOfFitAgent {
    pub fn process(prompt: &str) -> AgentContribution {
        let act = SpeechActAgent::classify(prompt);
        let dof = act.direction_of_fit();
        let desc = match dof {
            DirectionOfFit::MindToWorld => "The mind should match the world (belief/assertion)",
            DirectionOfFit::WorldToMind => "The world should change to match the mind (desire/command)",
            DirectionOfFit::None => "No direction of fit — pure expression of psychological state",
            DirectionOfFit::Both => "Both directions — declaration creates the fact it represents",
        };

        AgentContribution {
            agent_name: "DirectionOfFit",
            agent_role: "Mind-world alignment (Searle's direction of fit)",
            confidence: 0.85,
            output: format!("{}: {}", dof.as_str(), desc),
            phase_contribution: 0.0,
        }
    }
}

/// 6. SatisfactionAgent — checks if intentional state satisfaction conditions can be met.
/// For beliefs: satisfied when the world matches the content.
/// For desires: satisfied when the world changes to match the content.
/// For intentions: satisfied when the action is completed.
pub struct SatisfactionAgent;

impl SatisfactionAgent {
    pub fn process(facet: &Facet, prompt: &str) -> AgentContribution {
        let tokens = Tokenizer::tokenize(prompt);
        let known_count = tokens.iter().filter(|t| facet.lexicon.contains_key(*t)).count();
        let total = tokens.len().max(1);
        let satisfaction = known_count as f64 / total as f64;

        let act = SpeechActAgent::classify(prompt);
        let sat_desc = match act {
            SpeechActType::Assertive => "Satisfied when the proposition is true",
            SpeechActType::Directive => "Satisfied when the hearer performs the requested act",
            SpeechActType::Commissive => "Satisfied when the speaker fulfills the commitment",
            SpeechActType::Expressive => "Satisfied by the expression itself",
            SpeechActType::Declarative => "Satisfied by the successful performance of the declaration",
        };

        AgentContribution {
            agent_name: "Satisfaction",
            agent_role: "Intentional state satisfaction conditions",
            confidence: satisfaction,
            output: format!(
                "Satisfaction potential: {:.0}% ({} of {} words known) — {}",
                satisfaction * 100.0, known_count, total, sat_desc
            ),
            phase_contribution: 0.0,
        }
    }
}
