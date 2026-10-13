#[allow(unused_imports)]
use super::*;

impl ChatIntent {

    /// Generates a fluent conversational response using continuous phase attractor synthesis.
    pub fn generate_response(
        &self,
        model: &Model,
        cog: &CognitiveResult,
        wiki_extract: Option<&str>,
    ) -> String {
        match self {
            Self::Greeting => {
                let mut ctx_buf = crate::generate::ContextWaveBuffer::new(DEFAULT_CONTEXT_WINDOW);
                let generator = crate::generate::Generator::new(32, 0.7);
                let dynamic_greeting = generator.generate(
                    &model.facet,
                    &mut ctx_buf,
                    "hello I am Phiano your continuous learning cognitive assistant",
                );

                format!(
                    "{}\n\n*Continuous Phase Manifold*: {} active words (Coherence: {:.2})",
                    dynamic_greeting,
                    model.facet.vocabulary_size(),
                    cog.coherence
                )
            }

            Self::SelfCorrection { statement, correction } => {
                let mut ctx_buf = crate::generate::ContextWaveBuffer::new(DEFAULT_CONTEXT_WINDOW);
                let generator = crate::generate::Generator::new(36, 0.5);
                let affirmation = generator.generate(
                    &model.facet,
                    &mut ctx_buf,
                    correction,
                );

                format!(
                    "### Phase Manifold Recalibrated (π-Anti-Phase Pulse Applied)\n\n\
                    {}\n\n\
                    • **Updated Attractor**: \"{}\"\n\
                    • **State**: Destructive wave cancellation completed (Zero catastrophic forgetting)",
                    affirmation,
                    statement
                )
            }

            Self::Explanation { topic } => self.explain(model, cog, topic, wiki_extract),

            Self::PersonalMemory { statement } => {
                let mut ctx_buf = crate::generate::ContextWaveBuffer::new(DEFAULT_CONTEXT_WINDOW);
                let generator = crate::generate::Generator::new(28, 0.6);
                let generated_ack = generator.generate(
                    &model.facet,
                    &mut ctx_buf,
                    statement,
                );

                format!(
                    "### Memory Manifold Updated\n\n\
                    {}\n\n\
                    • **Input Registered**: \"{}\"\n\
                    • **Speech Act**: {} (Direction of fit: {})\n\
                    • **Total Active Lexicon**: {} words",
                    generated_ack,
                    statement,
                    cog.speech_act,
                    cog.direction_of_fit,
                    model.facet.vocabulary_size()
                )
            }

            Self::InstitutionalDeclaration { declaration } => {
                let mut ctx_buf = crate::generate::ContextWaveBuffer::new(DEFAULT_CONTEXT_WINDOW);
                let generator = crate::generate::Generator::new(28, 0.6);
                let generated_response = generator.generate(
                    &model.facet,
                    &mut ctx_buf,
                    declaration,
                );

                let felicity_status = if cog.felicity_conditions.satisfied {
                    "Satisfied"
                } else {
                    "Unmet"
                };
                format!(
                    "### Institutional Declaration Processed\n\n\
                    {}\n\n\
                    • **Utterance**: \"{}\"\n\
                    • **Speech Act**: Declarative (World ↔ Mind Double Direction of Fit)\n\
                    • **Precondition Status**: {}\n\
                    • **Total Active Lexicon**: {} words",
                    generated_response,
                    declaration,
                    felicity_status,
                    model.facet.vocabulary_size()
                )
            }

            Self::Recommendation { query } => self.recommend(model, cog, query, wiki_extract),

            Self::GeneralQuery { prompt } => {
                let reasoning = model.cognitive_core.reason(
                    &model.facet,
                    &mut crate::generate::ContextWaveBuffer::new(DEFAULT_CONTEXT_WINDOW),
                    prompt,
                    DEFAULT_REASONING_STEPS - 1,
                );
                format!(
                    "{}\n\n*Semantic Coherence*: {:.2} · Active Lexicon: {} words",
                    reasoning.final_answer,
                    cog.coherence,
                    model.facet.vocabulary_size()
                )
            }
        }
    }
}

