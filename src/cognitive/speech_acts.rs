/// Speech act agent — Searle's theory of illocutionary force.
/// Classification is data-driven via data/searle_markers.json.
/// Uses match-based dispatch, not if-else chains.

use super::types::*;
use super::markers::SearleMarkers;
use crate::tokenizer::Tokenizer;

/// 4. SpeechActAgent — classifies illocutionary force with felicity conditions.
pub struct SpeechActAgent;

impl SpeechActAgent {
    /// Data-driven classification using loaded markers.
    pub fn classify(prompt: &str) -> SpeechActType {
        let markers = SearleMarkers::load();
        Self::classify_with_markers(prompt, &markers)
    }

    /// Classification with injected markers.
    fn classify_with_markers(prompt: &str, m: &SearleMarkers) -> SpeechActType {
        // Indirect speech acts: "Can you X?" is literally a question,
        // but functions as a request (Searle 1975).
        if SearleMarkers::starts_with_any(prompt, &m.indirect_patterns) {
            return SpeechActType::Directive;
        }

        // Match on first matching category in Searle's priority order.
        // Commissive/expressive/declarative checked before directive because
        // "i will explain" is commissive, not directive.
        match () {
            _ if SearleMarkers::contains_any(prompt, &m.commissive_markers) => SpeechActType::Commissive,
            _ if SearleMarkers::contains_any(prompt, &m.expressive_markers) => SpeechActType::Expressive,
            _ if SearleMarkers::contains_any(prompt, &m.declarative_markers) => SpeechActType::Declarative,
            _ if SearleMarkers::contains_any(prompt, &m.directive_question_markers) => SpeechActType::Directive,
            _ if SearleMarkers::contains_any(prompt, &m.directive_command_markers) => SpeechActType::Directive,
            _ => SpeechActType::Assertive,
        }
    }

    /// Extracts propositional content separately from illocutionary force.
    pub fn extract_propositional_content(prompt: &str) -> String {
        let markers = SearleMarkers::load();
        let p = prompt.to_lowercase();
        let tokens = Tokenizer::tokenize(prompt);

        let all_markers: Vec<&[String]> = vec![
            &markers.directive_question_markers,
            &markers.directive_command_markers,
            &markers.indirect_patterns,
            &markers.commissive_markers,
            &markers.declarative_markers,
            &markers.expressive_markers,
        ];

        let mut content = p.clone();
        for marker_list in &all_markers {
            for marker in marker_list.iter() {
                if content.starts_with(marker.as_str()) {
                    content = content[marker.len()..].trim().to_string();
                    break;
                }
            }
            if content != p { break; }
        }

        if content.is_empty() {
            tokens.iter().take(5).cloned().collect::<Vec<_>>().join(" ")
        } else {
            content
        }
    }

    /// Felicity conditions per speech act type (Searle's conditions).
    pub fn felicity_conditions(act: SpeechActType, _prompt: &str) -> FelicityConditions {
        match act {
            SpeechActType::Assertive => FelicityConditions {
                propositional_content_rule: "Must express a proposition that can be true or false".into(),
                preparatory_condition: "Speaker has evidence for the truth of P".into(),
                sincerity_condition: "Speaker believes P is true".into(),
                essential_condition: "Counts as an undertaking that P represents an actual state of affairs".into(),
                satisfied: true,
            },
            SpeechActType::Directive => FelicityConditions {
                propositional_content_rule: "Must express a future act A of the hearer".into(),
                preparatory_condition: "Hearer is able to do A; speaker has authority to request".into(),
                sincerity_condition: "Speaker wants hearer to do A".into(),
                essential_condition: "Counts as an attempt to get hearer to do A".into(),
                satisfied: true,
            },
            SpeechActType::Commissive => FelicityConditions {
                propositional_content_rule: "Must express a future act A of the speaker".into(),
                preparatory_condition: "Speaker is able to do A".into(),
                sincerity_condition: "Speaker intends to do A".into(),
                essential_condition: "Counts as an undertaking of an obligation to do A".into(),
                satisfied: true,
            },
            SpeechActType::Expressive => FelicityConditions {
                propositional_content_rule: "Expresses a psychological state about a state of affairs".into(),
                preparatory_condition: "The state of affairs must be relevant to speaker and hearer".into(),
                sincerity_condition: "Speaker genuinely has the expressed psychological state".into(),
                essential_condition: "Counts as an expression of the psychological state".into(),
                satisfied: true,
            },
            SpeechActType::Declarative => FelicityConditions {
                propositional_content_rule: "Must express a state of affairs S that the declaration brings about".into(),
                preparatory_condition: "Speaker must have the institutional authority to bring about S".into(),
                sincerity_condition: "Speaker intends to bring about S".into(),
                essential_condition: "Counts as bringing about S by declaration".into(),
                satisfied: true,
            },
        }
    }

    /// Perlocutionary effect — the effect on the hearer.
    pub fn perlocutionary_effect(act: SpeechActType) -> &'static str {
        match act {
            SpeechActType::Assertive => "convince/persuade — hearer comes to believe P",
            SpeechActType::Directive => "compliance — hearer performs the requested act",
            SpeechActType::Commissive => "trust — hearer relies on speaker's commitment",
            SpeechActType::Expressive => "rapport — hearer feels acknowledged",
            SpeechActType::Declarative => "institutional change — the world is altered by the declaration",
        }
    }

    /// Detects literal vs speaker meaning divergence.
    pub fn speaker_vs_literal_meaning(prompt: &str) -> (String, String) {
        let markers = SearleMarkers::load();
        let p = prompt.to_lowercase();
        let literal = prompt.to_string();

        if SearleMarkers::starts_with_any(prompt, &markers.indirect_patterns) {
            let speaker = format!("Do the requested action (indirect request): {}", prompt);
            return (literal, speaker);
        }

        if SearleMarkers::contains_any(prompt, &markers.rhetorical_markers) {
            let speaker = format!("Nobody knows/cares — rhetorical (not a real question): {}", prompt);
            return (literal, speaker);
        }

        (literal.clone(), literal)
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
                act.as_str(), prop, if felicity.satisfied { "met" } else { "unmet" },
                perloc, literal, speaker
            ),
            phase_contribution: 0.0,
        }
    }
}
