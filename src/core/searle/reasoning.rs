//! Multi-step reasoning chain for iterative cognitive refinement.
//!
//! Evaluates the cognitive core across iterative steps, feeding each step's
//! output as context into the next step to produce a chain-of-thought trace.
//! All public operations are encapsulated in [`ChainOfThought`], following the
//! Diem convention that all public symbols belong to named types.
//!
//! # Architecture
//!
//! ```text
//! Initial Prompt
//!   │
//!   ▼
//! Step 0: CognitiveCore::process() ──▶ Synthesis Step 0
//!   │
//!   ▼ (Feedback as next prompt)
//! Step 1: CognitiveCore::process() ──▶ Synthesis Step 1
//!   │
//!   ▼ (Check convergence / coherence > 0.85)
//! Chain<Thought> { problem, steps, final_answer, converged }
//! ```

use phiecs::phasor::facet::Facet;
use phient::generate::ContextWaveBuffer;
use serde::Serialize;

/// The chain itself is philog's (`philog::Chain`): one chain, two kinds
/// of step, these thoughts and the pathfinding's phase steps.
pub use philog::Chain;

/// One step of the chain of thought: what was asked and what the core said.
#[derive(Debug, Clone, Serialize)]
pub struct Thought {
    pub step: usize,
    pub prompt: String,
    pub output: String,
    pub speech_act: String,
    pub coherence: f64,
}

/// Engine for executing multi-step chain-of-thought cognitive reasoning.
pub struct ChainOfThought;

impl ChainOfThought {
    /// Runs the cognitive core multiple times, feeding each step's output
    /// as context into the next step.
    ///
    /// Produces a [`Chain`] of [`Thought`]s holding each intermediate
    /// reasoning step and a convergence indicator.
    pub fn reason_chain(
        cognitive_core: &super::CognitiveCore,
        facet: &Facet,
        context_buffer: &mut ContextWaveBuffer,
        prompt: &str,
        max_steps: usize,
    ) -> Chain<Thought> {
        let mut steps: Vec<Thought> = Vec::new();
        let mut current_prompt = prompt.to_string();

        for step_num in 0..max_steps {
            let result = cognitive_core.process(facet, context_buffer, &current_prompt);

            // Check if we've converged (output repeats)
            if step_num > 0 && steps[step_num - 1].output == result.synthesized_output {
                break;
            }

            steps.push(Thought {
                step: step_num,
                prompt: current_prompt.clone(),
                output: result.synthesized_output.clone(),
                speech_act: result.speech_act.clone(),
                coherence: result.coherence,
            });

            // Feed output back as next prompt (chain-of-thought)
            current_prompt = format!("{} relates to what?", result.synthesized_output);

            // Stop if coherence is high enough (we're confident)
            if result.coherence > 0.85 && step_num >= 1 {
                break;
            }
        }

        let final_answer = steps.last()
            .map(|s| s.output.clone())
            .unwrap_or_default();
        let converged = steps.len() < max_steps;

        Chain {
            problem: prompt.to_string(),
            steps,
            final_answer,
            converged,
        }
    }
}
