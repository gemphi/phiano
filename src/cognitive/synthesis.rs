/// Mental causation agent - Searle's theory of how mental states cause action.
///
/// Searle's key insight: intentional states (beliefs, desires, intentions)
/// have causal power - they drive behavior. This is NOT epiphenomenalism.
///
/// The model:
/// 1. Beliefs (mind→world) - represent how the world is
/// 2. Desires (world→mind) - represent how the world should be
/// 3. Intentions (world→mind) - commitments to making the world match
///
/// Practical reasoning (Searle's adaptation of Aristotle):
///   Belief: "X is the case" + Desire: "I want Y" → Intention: "I will do A to get Y"
///
/// The MentalCausation agent constructs intentional states from the prompt,
/// then uses them to drive word selection and sentence construction.

use super::types::*;
use super::speech_acts::SpeechActAgent;
use super::word_selection;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;

/// 16. MentalCausationAgent - drives output from intentional states.
pub struct MentalCausationAgent;

impl MentalCausationAgent {
    pub fn process(
        facet: &Facet,
        prompt: &str,
        contributions: &[AgentContribution],
    ) -> (AgentContribution, Vec<IntentionalState>) {
        let act = SpeechActAgent::classify(prompt);
        let prop = SpeechActAgent::extract_propositional_content(prompt);

        // Construct intentional states from the prompt
        let states = Self::construct_intentional_states(facet, prompt, act, &prop);

        // Use collective phase to guide word selection
        let collective_phase = contributions.iter()
            .filter(|c| c.phase_contribution != 0.0)
            .map(|c| c.phase_contribution)
            .sum::<f64>()
            .rem_euclid(2.0 * std::f64::consts::PI);

        // Select words using intentional states as guides
        let synthesis_words = word_selection::WordSelector::select_words(facet, prompt, &states, collective_phase);

        // Synthesize output using intentional state-driven construction
        let output = word_selection::WordSelector::synthesize(&synthesis_words, act, &states, prompt);

        let contrib = AgentContribution {
            agent_name: "MentalCausation",
            agent_role: "Intentional states cause output (belief→desire→intention)",
            confidence: match synthesis_words.len() >= 3 {
                true => 0.75,
                false => 0.3,
            },
            output,
            phase_contribution: collective_phase,
        };

        (contrib, states)
    }

    /// Constructs intentional states from the prompt.
    /// Different speech acts produce different constellations of states.
    fn construct_intentional_states(
        facet: &Facet,
        prompt: &str,
        act: SpeechActType,
        proposition: &str,
    ) -> Vec<IntentionalState> {
        let tokens = Tokenizer::tokenize(prompt);
        let known: Vec<String> = tokens.iter()
            .filter(|t| facet.lexicon.contains_key(*t))
            .cloned()
            .collect();
        let content = match known.is_empty() {
            true => proposition.to_string(),
            false => known.iter().take(5).cloned().collect::<Vec<_>>().join(" "),
        };

        match act {
            SpeechActType::Assertive => vec![
                IntentionalState {
                    mode: PsychologicalMode::Belief,
                    content: format!("the world is such that {}", content),
                    direction_of_fit: DirectionOfFit::MindToWorld,
                    satisfaction_condition: "satisfied if the proposition is true".to_string(),
                    sincerity: 0.8,
                },
            ],
            SpeechActType::Directive => vec![
                IntentionalState {
                    mode: PsychologicalMode::Desire,
                    content: format!("the hearer performs action related to {}", content),
                    direction_of_fit: DirectionOfFit::WorldToMind,
                    satisfaction_condition: "satisfied when hearer complies".to_string(),
                    sincerity: 0.9,
                },
                IntentionalState {
                    mode: PsychologicalMode::Belief,
                    content: format!("the hearer is able to respond about {}", content),
                    direction_of_fit: DirectionOfFit::MindToWorld,
                    satisfaction_condition: "satisfied if hearer has the capacity".to_string(),
                    sincerity: 0.7,
                },
            ],
            SpeechActType::Commissive => vec![
                IntentionalState {
                    mode: PsychologicalMode::Intention,
                    content: format!("I will do action related to {}", content),
                    direction_of_fit: DirectionOfFit::WorldToMind,
                    satisfaction_condition: "satisfied when I fulfill the commitment".to_string(),
                    sincerity: 0.9,
                },
            ],
            SpeechActType::Expressive => vec![
                IntentionalState {
                    mode: PsychologicalMode::Hope,
                    content: format!("expression about {}", content),
                    direction_of_fit: DirectionOfFit::None,
                    satisfaction_condition: "satisfied by the expression itself".to_string(),
                    sincerity: 0.85,
                },
            ],
            SpeechActType::Declarative => vec![
                IntentionalState {
                    mode: PsychologicalMode::Intention,
                    content: format!("I bring about the state of affairs: {}", content),
                    direction_of_fit: DirectionOfFit::Both,
                    satisfaction_condition: "satisfied by successful declaration".to_string(),
                    sincerity: 0.95,
                },
                IntentionalState {
                    mode: PsychologicalMode::Belief,
                    content: "I have the institutional authority to declare".to_string(),
                    direction_of_fit: DirectionOfFit::MindToWorld,
                    satisfaction_condition: "satisfied if I have the authority".to_string(),
                    sincerity: 0.8,
                },
            ],
        }
    }
}
