#[allow(unused_imports)]
use super::*;

impl ChatIntent {
    /// Classifies user input prompt into a structured ChatIntent.
    pub fn classify(prompt: &str, tokens: &[String], cog: &CognitiveResult) -> Self {
        let p_trimmed = prompt.trim();
        let p_lower = p_trimmed.to_lowercase();

        // 1. Greetings
        if Self::is_greeting(&p_lower) {
            return Self::Greeting;
        }

        // 2. Self-Correction & Negative Feedback
        if Self::is_self_correction(&p_lower) {
            let correction = Self::extract_correction_content(&p_lower);
            return Self::SelfCorrection {
                statement: p_trimmed.to_string(),
                correction,
            };
        }

        // 3. Institutional Declarations
        if Self::is_institutional_declaration(&p_lower, cog) {
            return Self::InstitutionalDeclaration {
                declaration: p_trimmed.to_string(),
            };
        }

        // 4. Personal Statements & Fact Teaching
        if Self::is_personal_memory(&p_lower) {
            return Self::PersonalMemory {
                statement: p_trimmed.to_string(),
            };
        }

        // 5. Follow-up Recommendations & Discovery
        if Self::is_recommendation(&p_lower) {
            return Self::Recommendation {
                query: p_trimmed.to_string(),
            };
        }

        // 6. Explanations & Concept Definitions
        if Self::is_explanation_request(&p_lower) {
            let topic = Self::extract_topic_term(tokens, p_trimmed);
            return Self::Explanation { topic };
        }

        // 7. Fallback General Query
        Self::GeneralQuery {
            prompt: p_trimmed.to_string(),
        }
    }
}
