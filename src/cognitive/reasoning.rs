//! Multi-step reasoning chain for iterative cognitive refinement.
//!
//! Evaluates the cognitive core across iterative steps, feeding each step's
//! output as context into the next step to produce a chain-of-thought trace.
//! All public operations are encapsulated in [`ReasoningChain`], following the
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
//! ReasoningResult { steps, final_answer, converged }
//! ```

use crate::facet::Facet;
use crate::generate::ContextWaveBuffer;
use serde::Serialize;

/// Result of multi-step reasoning.
#[derive(Debug, Clone, Serialize)]
pub struct ReasoningResult {
    pub steps: Vec<ReasoningStep>,
    pub final_answer: String,
    pub converged: bool,
}

/// An individual reasoning step in the cognitive chain.
#[derive(Debug, Clone, Serialize)]
pub struct ReasoningStep {
    pub step: usize,
    pub prompt: String,
    pub output: String,
    pub speech_act: String,
    pub coherence: f64,
}

/// Engine for executing multi-step chain-of-thought cognitive reasoning.
pub struct ReasoningChain;

impl ReasoningChain {
    /// Runs the cognitive core multiple times, feeding each step's output
    /// as context into the next step.
    ///
    /// Produces a structured [`ReasoningResult`] containing each intermediate
    /// reasoning step and a convergence indicator.
    pub fn reason_chain(
        cognitive_core: &super::CognitiveCore,
        facet: &Facet,
        context_buffer: &mut ContextWaveBuffer,
        prompt: &str,
        max_steps: usize,
    ) -> ReasoningResult {
        let mut steps: Vec<ReasoningStep> = Vec::new();
        let mut current_prompt = prompt.to_string();

        for step_num in 0..max_steps {
            let result = cognitive_core.process(facet, context_buffer, &current_prompt);

            // Check if we've converged (output repeats)
            match step_num > 0 {
                true => {
                    let prev_output = &steps[step_num - 1].output;
                    match prev_output == &result.synthesized_output {
                        true => break,
                        false => {}
                    }
                }
                false => {}
            }

            steps.push(ReasoningStep {
                step: step_num,
                prompt: current_prompt.clone(),
                output: result.synthesized_output.clone(),
                speech_act: result.speech_act.clone(),
                coherence: result.coherence,
            });

            // Feed output back as next prompt (chain-of-thought)
            current_prompt = format!("{} relates to what?", result.synthesized_output);

            // Stop if coherence is high enough (we're confident)
            match result.coherence > 0.85 && step_num >= 1 {
                true => break,
                false => {}
            }
        }

        let final_answer = steps.last()
            .map(|s| s.output.clone())
            .unwrap_or_default();
        let converged = steps.len() < max_steps;

        ReasoningResult {
            steps,
            final_answer,
            converged,
        }
    }
}
