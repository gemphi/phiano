mod classify;
mod response;
mod grounded;
mod greeting;
#[cfg(test)]
mod tests;
#[allow(unused_imports)]
use classify::*;
#[allow(unused_imports)]
use response::*;
#[allow(unused_imports)]
use grounded::*;
#[allow(unused_imports)]
use greeting::*;

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
