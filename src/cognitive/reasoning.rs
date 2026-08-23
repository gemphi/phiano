/// Multi-step reasoning chain — cognitive core runs multiple times,
/// feeding each step's output as context into the next step.

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

#[derive(Debug, Clone, Serialize)]
pub struct ReasoningStep {
    pub step: usize,
    pub prompt: String,
    pub output: String,
    pub speech_act: String,
    pub coherence: f64,
}

/// Runs the cognitive core multiple times, feeding each step's output
/// as context into the next step. This produces a chain-of-thought
/// style reasoning trace.
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
        if step_num > 0 {
            let prev_output = &steps[step_num - 1].output;
            if prev_output == &result.synthesized_output {
                break;
            }
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
        if result.coherence > 0.85 && step_num >= 1 {
            break;
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
