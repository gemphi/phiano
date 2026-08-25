/// Hybrid reasoner: combines geometric pathfinding with structural analogy.
/// Implements Ch 14.4's key lesson: combine geometric pattern recognition
/// with discrete, program-like reasoning.

use crate::facet::Facet;
use crate::generate::ContextWaveBuffer;
use crate::reasoning::analogy::{value_centric_analogy, find_analogies};
use crate::reasoning::pathfinding::ReasoningEngine;
use crate::reasoning::program_analogy::{extract_structure, shares_structure};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct HybridResult {
    pub pathfinding_chain: super::pathfinding::ReasoningChain,
    pub analogies: Vec<(String, f64)>,
    pub structural_matches: Vec<String>,
    pub final_answer: String,
    pub confidence: f64,
}

pub struct HybridReasoner {
    pub engine: ReasoningEngine,
}

impl Default for HybridReasoner {
    fn default() -> Self {
        Self::new()
    }
}

impl HybridReasoner {
    pub fn new() -> Self {
        Self { engine: ReasoningEngine }
    }

    /// Solves a problem using both geometric pathfinding and structural analogy.
    pub fn solve_hybrid(&self, facet: &Facet, problem: &str) -> HybridResult {
        let chain = self.engine.solve(facet, problem);

        let tokens = crate::tokenizer::Tokenizer::tokenize(problem);
        let mut analogies = Vec::new();
        let mut structural_matches = Vec::new();

        for token in tokens.iter().take(3) {
            let analogs = find_analogies(facet, token, 5);
            for (word, score) in analogs {
                analogies.push((word, score));
                if score > 0.7 {
                    structural_matches.push(word.clone());
                }
            }
        }

        analogies.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        analogies.truncate(10);

        let confidence = if chain.converged { 0.8 } else { 0.4 }
            * (1.0 + analogies.len() as f64 * 0.05).min(1.5);

        let final_answer = if !structural_matches.is_empty() {
            format!("{} (via structural analogy with: {})",
                chain.final_answer,
                structural_matches.iter().take(3).cloned().collect::<Vec<_>>().join(", "))
        } else {
            chain.final_answer.clone()
        };

        HybridResult {
            pathfinding_chain: chain,
            analogies,
            structural_matches,
            final_answer,
            confidence: confidence.min(1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hybrid_reasoner() {
        let mut facet = Facet::new();
        facet.get_or_init("ownership");
        facet.get_or_init("borrowing");
        facet.get_or_init("lifetime");

        let reasoner = HybridReasoner::new();
        let result = reasoner.solve_hybrid(&facet, "ownership borrowing lifetime");

        assert!(!result.final_answer.is_empty());
        assert!(result.confidence > 0.0);
    }
}
