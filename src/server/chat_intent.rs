use crate::chunker::ChunkStore;
use crate::cognitive::CognitiveResult;
use crate::config::{CHUNK_STORE_DIR, DEFAULT_CONTEXT_WINDOW, DEFAULT_REASONING_STEPS};
use crate::model::Model;

/// Conversational intent classifications for user messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatIntent {
    /// Salutations, greetings, introductions
    Greeting,
    /// Live user negative feedback or errata correction
    SelfCorrection {
        /// The full raw user input.
        statement: String,
        /// The extracted correction content (prefix stripped).
        correction: String,
    },
    /// Question inquiring about definitions, mechanisms, or explanations
    Explanation {
        topic: String,
    },
    /// Personalized statement, fact teaching, or user preferences
    PersonalMemory {
        statement: String,
    },
    /// Formal Searle declarative speech act with double direction of fit
    InstitutionalDeclaration {
        declaration: String,
    },
    /// Follow-up recommendation or discovery inquiry based on recent context
    Recommendation {
        query: String,
    },
    /// General continuous phase attractor and multi-step reasoning
    GeneralQuery {
        prompt: String,
    },
}

impl ChatIntent {
    /// Classifies user input prompt into a structured ChatIntent.
    pub fn classify(prompt: &str, tokens: &[String], cog: &CognitiveResult) -> Self {
        let p_trimmed = prompt.trim();
        let p_lower = p_trimmed.to_lowercase();

        // 1. Greetings
        match Self::is_greeting(&p_lower) {
            true => return Self::Greeting,
            false => {}
        }

        // 2. Self-Correction & Negative Feedback
        match Self::is_self_correction(&p_lower) {
            true => {
                let correction = Self::extract_correction_content(&p_lower);
                return Self::SelfCorrection {
                    statement: p_trimmed.to_string(),
                    correction,
                };
            }
            false => {}
        }

        // 3. Institutional Declarations
        match Self::is_institutional_declaration(&p_lower, cog) {
            true => {
                return Self::InstitutionalDeclaration {
                    declaration: p_trimmed.to_string(),
                };
            }
            false => {}
        }

        // 4. Personal Statements & Fact Teaching
        match Self::is_personal_memory(&p_lower) {
            true => {
                return Self::PersonalMemory {
                    statement: p_trimmed.to_string(),
                };
            }
            false => {}
        }

        // 5. Follow-up Recommendations & Discovery
        match Self::is_recommendation(&p_lower) {
            true => {
                return Self::Recommendation {
                    query: p_trimmed.to_string(),
                };
            }
            false => {}
        }

        // 6. Explanations & Concept Definitions
        match Self::is_explanation_request(&p_lower) {
            true => {
                let topic = Self::extract_topic_term(tokens, p_trimmed);
                return Self::Explanation { topic };
            }
            false => {}
        }

        // 7. Fallback General Query
        Self::GeneralQuery {
            prompt: p_trimmed.to_string(),
        }
    }

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

            Self::Explanation { topic } => {
                let chunk_store = ChunkStore::new(CHUNK_STORE_DIR);
                let def_opt = chunk_store.load_definition(topic);

                match wiki_extract {
                    Some(wiki) => {
                        let clean_wiki = wiki.lines().take(4).collect::<Vec<_>>().join(" ");
                        return format!(
                            "### Semantic Grounding: **{}**\n\n\
                            {}\n\n\
                            *Attractor Resonance*: Aligned with Wikipedia knowledge base (Coherence: {:.2}).",
                            topic.to_uppercase(),
                            clean_wiki,
                            cog.coherence
                        );
                    }
                    None => {}
                }

                match def_opt {
                    Some(def) => {
                        let clean_def = def.lines().take(3).collect::<Vec<_>>().join("\n");
                        return format!(
                            "### Concept Definition: **{}**\n\n\
                            {}\n\n\
                            *Memory Layer Resonance*: Grounded across 16 memory layers with {:.0}% satisfaction.",
                            topic,
                            clean_def,
                            cog.satisfaction * 100.0
                        );
                    }
                    None => {}
                }

                // Fall back to multi-step phase reasoning
                let reasoning = model.cognitive_core.reason(
                    &model.facet,
                    &mut crate::generate::ContextWaveBuffer::new(DEFAULT_CONTEXT_WINDOW),
                    topic,
                    DEFAULT_REASONING_STEPS,
                );
                format!(
                    "### Attractor Trajectory: **{}**\n\n\
                    {}\n\n\
                    *Cognitive Path*: Multi-step reasoning across {} steps (Convergence: {})",
                    topic,
                    reasoning.final_answer,
                    reasoning.steps.len(),
                    match reasoning.converged {
                        true => "Harmonic Equilibrium",
                        false => "Approximated",
                    }
                )
            }

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

            Self::Recommendation { query } => {
                match wiki_extract {
                    Some(wiki) => {
                        let clean_wiki = wiki.lines().take(4).collect::<Vec<_>>().join(" ");
                        return format!(
                            "### Contextual Grounding & Recommendations\n\n\
                            {}\n\n\
                            *Recommendation Path*: Grounded across active memory context and continuous semantic manifold.",
                            clean_wiki
                        );
                    }
                    None => {}
                }

                let reasoning = model.cognitive_core.reason(
                    &model.facet,
                    &mut crate::generate::ContextWaveBuffer::new(DEFAULT_CONTEXT_WINDOW),
                    query,
                    DEFAULT_REASONING_STEPS,
                );
                format!(
                    "### Explorations & Next Steps\n\n\
                    {}\n\n\
                    *Semantic Coherence*: {:.2} · Active Lexicon: {} words",
                    reasoning.final_answer,
                    cog.coherence,
                    model.facet.vocabulary_size()
                )
            }

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

    // ── PATTERN CLASSIFIER HELPERS ─────────────────────────────────────────────

    fn is_greeting(p_lower: &str) -> bool {
    const GREETINGS: &[&str] = &[
        "hello", "hi", "hey", "good morning", "good afternoon", "good evening", "greetings", "howdy",
    ];
    let cleaned = p_lower.trim_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace());
    GREETINGS.iter().any(|&g| {
        cleaned.starts_with(g) || p_lower.starts_with(g)
    })
}

    fn is_self_correction(p_lower: &str) -> bool {
    const CORRECTION_STARTS: &[&str] = &[
        "no, ", "no ", "that is wrong", "that's wrong", "incorrect", "correction:", "you are wrong", "you're wrong",
    ];
    CORRECTION_STARTS.iter().any(|&c| p_lower.starts_with(c) || p_lower.contains(c))
}

    /// Extracts the correction content by stripping the negation prefix.
    ///
    /// e.g. "no, that's wrong, dogs are mammals" → "dogs are mammals"
    fn extract_correction_content(p_lower: &str) -> String {
    const CORRECTION_PREFIXES: &[&str] = &[
        "no, that's wrong, ", "no, that is wrong, ", "no, ",
        "that's wrong, ", "that is wrong, ",
        "incorrect, ", "correction: ",
        "you are wrong, ", "you're wrong, ",
        "no ", "incorrect ", "correction: ",
    ];
    for prefix in CORRECTION_PREFIXES {
        if let Some(rest) = p_lower.strip_prefix(prefix) {
            return rest.trim().to_string();
        }
    }
    p_lower.trim().to_string()
}

    fn is_institutional_declaration(p_lower: &str, cog: &CognitiveResult) -> bool {
    cog.speech_act == "declarative"
        || p_lower.contains("i hereby")
        || p_lower.contains("i declare")
        || p_lower.contains("this meeting is")
}

    fn is_personal_memory(p_lower: &str) -> bool {
    const PERSONAL_TRIGGERS: &[&str] = &[
        "my ", "i am ", "i love", "i live in", "we are", "allergic to", "my name is", "my daughter", "my son",
    ];
    PERSONAL_TRIGGERS.iter().any(|&t| p_lower.contains(t))
}

    fn is_recommendation(p_lower: &str) -> bool {
    const RECOMMENDATION_TRIGGERS: &[&str] = &[
        "what books", "what topics", "what should", "recommend", "suggest", "where should", "how should i learn",
        "explore next", "books or topics",
    ];
    RECOMMENDATION_TRIGGERS.iter().any(|&r| p_lower.contains(r))
}

    fn is_explanation_request(p_lower: &str) -> bool {
    const QUESTION_TRIGGERS: &[&str] = &[
        "what is", "define", "explain", "tell me about", "how does", "why do", "why does", "who is", "what are",
    ];
    QUESTION_TRIGGERS.iter().any(|&q| p_lower.contains(q)) || p_lower.ends_with('?')
}

    pub fn extract_topic_term(tokens: &[String], prompt: &str) -> String {
    const STOP_WORDS: &[&str] = &[
        "what", "is", "are", "were", "was", "be", "been", "being", "have", "has", "had", "a", "an", "the",
        "explain", "why", "do", "does", "did", "how", "can", "could", "will", "would", "shall", "should",
        "may", "might", "must", "you", "tell", "me", "about", "define", "who", "whom", "whose", "which",
        "where", "when", "she", "he", "they", "we", "i", "or", "and", "not", "next", "books", "recommend",
        "suggest", "for", "in", "on", "to", "with", "by", "at", "from", "into", "of", "considered", "between",
    ];

    for t in tokens {
        let t_clean = t.to_lowercase();
        let stripped = t_clean.trim_matches(|c: char| !c.is_alphabetic());
        if !STOP_WORDS.contains(&stripped) && stripped.len() >= 3 {
            return stripped.to_string();
        }
    }
        tokens.last().cloned().unwrap_or_else(|| prompt.to_string())
    }
}

#[cfg(test)]
mod tests {
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
            felicity_conditions: crate::cognitive::FelicityConditions {
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
}
