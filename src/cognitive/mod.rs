/// Cognitive core - Searle-inspired 16-agent cognitive architecture.
///
/// Module structure:
/// - types.rs: Core data structures (IntentionalState, FelicityConditions, etc.)
/// - markers.rs: Data-driven Searle markers (loaded from JSON, cached)
/// - intentionality.rs: Intentionality, Aboutness, Background agents
/// - speech_acts.rs: SpeechAct agent (illocutionary force, felicity conditions)
/// - dof.rs: DirectionOfFit + Satisfaction agents
/// - reference.rs: Reference, Network, TruthCondition agents
/// - semantics.rs: Semantics, Syntax, Awareness agents
/// - social.rs: SocialOntology, ObserverRelativity, CollectiveIntention agents
/// - synthesis.rs: MentalCausation agent (intentional states -> output)
/// - word_selection.rs: word selection + synthesis templates
/// - reasoning.rs: multi-step reasoning chain
/// - grounding.rs: definition-grounded phase initialization
/// - mod.rs: CognitiveCore coordinator

pub mod types;
pub mod markers;
pub mod intentionality;
pub mod speech_acts;
pub mod dof;
pub mod reference;
pub mod semantics;
pub mod social;
pub mod synthesis;
pub mod word_selection;
pub mod reasoning;
pub mod grounding;

#[allow(unused_imports)]
pub use types::{AgentContribution, CognitiveResult, IntentionalState, FelicityConditions};
#[allow(unused_imports)]
pub use types::{PsychologicalMode, DirectionOfFit, SpeechActType};
pub use speech_acts::SpeechActAgent;
#[allow(unused_imports)]
pub use reasoning::{ReasoningResult, ReasoningStep, ReasoningChain};
pub use grounding::DefinitionGrounder;
#[allow(unused_imports)]
pub use word_selection::WordSelector;

use crate::chunker::ChunkStore;
use crate::facet::Facet;
use crate::generate::ContextWaveBuffer;

/// CognitiveCore - coordinates all 16 agents and synthesizes output.
pub struct CognitiveCore {
    pub chunk_store: ChunkStore,
}

impl CognitiveCore {
    pub fn new(chunk_store: ChunkStore) -> Self {
        Self { chunk_store }
    }

    /// Processes a prompt through all 16 cognitive agents.
    pub fn process(
        &self,
        facet: &Facet,
        context_buffer: &mut ContextWaveBuffer,
        prompt: &str,
    ) -> CognitiveResult {
        // Agents 1-3: Intentionality cluster
        let a1 = intentionality::IntentionalityAgent::process(facet, prompt);
        let a2 = intentionality::AboutnessAgent::process(facet, prompt);
        let a3 = reference::ReferenceAgent::process(facet, prompt, &self.chunk_store);

        // Agents 4-6: Speech act cluster
        let a4 = speech_acts::SpeechActAgent::process(prompt);
        let a5 = dof::DirectionOfFitAgent::process(prompt);
        let a6 = dof::SatisfactionAgent::process(facet, prompt);

        // Agent 7: Background
        let a7 = intentionality::BackgroundAgent::process(context_buffer);

        // Agent 8: Network
        let a8 = reference::NetworkAgent::process(facet, prompt);

        // Agents 10-12: Semantics cluster
        let a10 = semantics::AwarenessAgent::process(facet, prompt);
        let a10b = semantics::SemanticsAgent::process(facet, prompt);
        let a11 = reference::TruthConditionAgent::process(facet, prompt);
        let a12 = semantics::SyntaxAgent::process(facet, prompt);

        // Agents 13-15: Social cluster
        let a13 = social::SocialOntologyAgent::process(prompt);
        let a15 = social::ObserverRelativityAgent::process(facet, prompt);

        let mut contributions = vec![
            a1.clone(), a2, a3, a4.clone(), a5, a6.clone(), a7, a8,
            a10, a10b, a11, a12, a13, a15,
        ];

        // Agent 9: Collective intention (needs all prior contributions)
        let a9 = social::CollectiveIntentionAgent::process(&contributions);
        contributions.push(a9);

        // Agent 16: Mental causation (needs all contributions, produces output + states)
        let (a16, intentional_states) = synthesis::MentalCausationAgent::process(
            facet, prompt, &contributions,
        );
        let synthesized_output = a16.output.clone();
        contributions.push(a16);

        // Compute Searle-level metadata
        let act_type = SpeechActAgent::classify(prompt);
        let felicity = SpeechActAgent::felicity_conditions(act_type, prompt);
        let perlocutionary = SpeechActAgent::perlocutionary_effect(act_type).to_string();
        let prop_content = SpeechActAgent::extract_propositional_content(prompt);
        let (literal, speaker) = SpeechActAgent::speaker_vs_literal_meaning(prompt);

        let coherence = contributions.iter()
            .map(|c| c.confidence)
            .sum::<f64>() / contributions.len() as f64;

        CognitiveResult {
            prompt: prompt.to_string(),
            agent_outputs: contributions,
            synthesized_output,
            coherence,
            intentionality_phase: a1.phase_contribution,
            speech_act: act_type.as_str().to_string(),
            direction_of_fit: act_type.direction_of_fit().as_str().to_string(),
            satisfaction: a6.confidence,
            intentional_states,
            felicity_conditions: felicity,
            perlocutionary_effect: perlocutionary,
            propositional_content: prop_content,
            speaker_meaning: speaker,
            literal_meaning: literal,
        }
    }

    /// Multi-step reasoning chain (delegates to reasoning module).
    pub fn reason(
        &self,
        facet: &Facet,
        context_buffer: &mut ContextWaveBuffer,
        prompt: &str,
        max_steps: usize,
    ) -> ReasoningResult {
        ReasoningChain::reason_chain(self, facet, context_buffer, prompt, max_steps)
    }
}
