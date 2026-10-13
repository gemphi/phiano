#[allow(unused_imports)]
use super::super::*;

impl InstructionEngine {

    /// Generates a chat response using the full cognitive pipeline.
    ///
    /// # Pipeline
    ///
    /// 1. Online-trains the manifold with the user prompt
    /// 2. Classifies the instruction kind
    /// 3. Looks up grounded definitions from the chunk store
    /// 4. Runs cognitive synthesis (intentional states)
    /// 5. Composes the reply from definitions or torus attractor generation
    /// 6. Pushes both prompt and reply into the context buffer
    ///
    /// # Returns
    ///
    /// A [`ChatResponse`] containing the reply text, cognitive metadata,
    /// speech act classification, and phase trace.
    pub fn generate_response(
        facet: &mut Facet,
        trainer: &Trainer,
        cognitive_core: &CognitiveCore,
        context_buffer: &mut ContextWaveBuffer,
        generator: &Generator,
        prompt: &str,
    ) -> ChatResponse {
        trainer.train_online(facet, prompt);

        let kind = InstructionKind::parse(prompt);
        let definitions = lookup_definitions(cognitive_core, prompt);
        for (word, def) in &definitions {
            trainer.train_definition(facet, word, def);
        }

        let cognitive_result = cognitive_core.process(facet, context_buffer, prompt);
        let text = compose_reply(facet, context_buffer, generator, prompt, &definitions);
        let phase_trace = match definitions.is_empty() {
            true => text.clone(),
            false => generator.generate(facet, context_buffer, prompt),
        };

        context_buffer.push_turn(facet, prompt);
        context_buffer.push_turn(facet, &text);

        ChatResponse {
            text,
            cognitive_synthesis: cognitive_result.synthesized_output,
            intent: kind,
            speech_act: cognitive_result.speech_act,
            direction_of_fit: cognitive_result.direction_of_fit,
            satisfaction: cognitive_result.satisfaction,
            phase_trace,
        }
    }

    /// Executes an incoming instruction using shared cognitive core + context buffer.
    ///
    /// Builds the hierarchical phase field first, then delegates to
    /// [`Self::generate_response`].
    pub fn execute_instruction(
        &mut self,
        facet: &mut Facet,
        _trainer: &Trainer,
        cognitive_core: &CognitiveCore,
        context_buffer: &mut ContextWaveBuffer,
        prompt: &str,
    ) -> ChatResponse {
        self.phase_field.build_hierarchy(facet);
        Self::generate_response(facet, _trainer, cognitive_core, context_buffer, &self.generator, prompt)
    }
}
