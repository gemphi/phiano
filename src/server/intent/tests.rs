#[allow(unused_imports)]
use super::*;

use crate::cognitive::CognitiveResult;

fn mock_cog(speech_act: &str) -> CognitiveResult {
    CognitiveResult {
        prompt: "test".to_string(),
        agent_outputs: Vec::new(),
        synthesized_output: "test output".to_string(),
        coherence: 0.85,
        intentionality_phase: 0.0,
        speech_act: speech_act.to_string(),
        direction_of_fit: "mind->world".to_string(),
        satisfaction: 0.9,
        intentional_states: Vec::new(),
        felicity_conditions: crate::cognitive::FelicityCondition {
            propositional_content_rule: "".to_string(),
            preparatory_condition: "".to_string(),
            sincerity_condition: "".to_string(),
            essential_condition: "".to_string(),
            satisfied: true,
        },
        perlocutionary_effect: "".to_string(),
        propositional_content: "".to_string(),
        speaker_meaning: "".to_string(),
        literal_meaning: "".to_string(),
    }
}

#[test]
fn test_intent_classification_greeting() {
    let cog = mock_cog("expressive");
    let intent = ChatIntent::classify("hello there", &["hello".into(), "there".into()], &cog);
    assert_eq!(intent, ChatIntent::Greeting);
}

#[test]
fn test_intent_classification_correction() {
    let cog = mock_cog("assertive");
    let intent = ChatIntent::classify("No, that's wrong dolphins are mammals", &["no".into()], &cog);
    assert!(matches!(intent, ChatIntent::SelfCorrection { .. }));
}

#[test]
fn test_intent_classification_explanation() {
    let cog = mock_cog("directive");
    let intent = ChatIntent::classify("what is entropy?", &["what".into(), "is".into(), "entropy".into()], &cog);
    assert_eq!(intent, ChatIntent::Explanation { topic: "entropy".to_string() });
}

#[test]
fn test_intent_classification_declaration() {
    let cog = mock_cog("declarative");
    let intent = ChatIntent::classify("I hereby declare this session open", &[], &cog);
    assert!(matches!(intent, ChatIntent::InstitutionalDeclaration { .. }));
}

#[test]
fn test_intent_classification_personal() {
    let cog = mock_cog("assertive");
    let intent = ChatIntent::classify("My daughter loves astronomy", &[], &cog);
    assert!(matches!(intent, ChatIntent::PersonalMemory { .. }));
}
