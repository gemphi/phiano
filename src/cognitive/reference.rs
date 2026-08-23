/// Reference agents — how words connect to the world.
///
/// Searle's reference theory: words refer via a Network of beliefs and
/// the Background of pre-intentional capacities. Reference is not just
/// a word→object mapping — it depends on the speaker's intentional states.
///
/// Also implements the propositional content vs illocutionary force separation:
/// "It's raining" (assertive) and "Is it raining?" (directive) share the same
/// propositional content "it is raining" but differ in illocutionary force.

use super::types::*;
use crate::chunker::ChunkStore;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use crate::wave::Wave;

/// 7. ReferenceAgent — resolves definitions from the chunk store.
/// Models Searle's referential act: the speaker uses a word to refer to
/// an object via its definition (the Network of descriptions).
pub struct ReferenceAgent;

impl ReferenceAgent {
    pub fn process(_facet: &Facet, prompt: &str, chunk_store: &ChunkStore) -> AgentContribution {
        let tokens = Tokenizer::tokenize(prompt);
        let entries = chunk_store.load_all();
        let entry_map: std::collections::HashMap<&str, &str> = entries.iter()
            .map(|(w, d)| (w.as_str(), d.as_str()))
            .collect();

        let mut definitions = Vec::new();
        for token in tokens.iter().take(5) {
            if let Some(def) = entry_map.get(token.as_str()) {
                let short: String = def.chars().take(120).collect();
                definitions.push(format!("{}: {}", token, short));
            }
        }

        let output = if definitions.is_empty() {
            "No definitions found — referents ungrounded".to_string()
        } else {
            definitions.join("\n")
        };

        AgentContribution {
            agent_name: "Reference",
            agent_role: "Definition lookup (referential act via Network)",
            confidence: definitions.len() as f64 / tokens.len().max(1) as f64,
            output,
            phase_contribution: 0.0,
        }
    }
}

/// 8. NetworkAgent — traverses the semantic network via bigrams.
/// Searle's Network: the set of beliefs, desires, and intentions that give
/// words their meaning. Changing the Network changes the meaning.
/// Here, the bigram model approximates the Network of associations.
pub struct NetworkAgent;

impl NetworkAgent {
    pub fn process(facet: &Facet, prompt: &str) -> AgentContribution {
        let tokens = Tokenizer::tokenize(prompt);
        let mut paths = Vec::new();

        for token in tokens.iter().take(4) {
            let candidates = facet.next_word_candidates(token);
            if !candidates.is_empty() {
                let top: Vec<String> = candidates.iter()
                    .take(3)
                    .map(|(w, c)| format!("{}({})", w, c))
                    .collect();
                paths.push(format!("{} → {}", token, top.join(" → ")));
            }
        }

        let output = if paths.is_empty() {
            "No Network paths found — word is isolated".to_string()
        } else {
            paths.join("\n")
        };

        AgentContribution {
            agent_name: "Network",
            agent_role: "Semantic network traversal (belief Network)",
            confidence: if paths.is_empty() { 0.0 } else { 0.8 },
            output,
            phase_contribution: 0.0,
        }
    }
}

/// 9. TruthConditionAgent — checks alignment between words and propositional content.
/// For assertives: truth = world matches the proposition.
/// Here we model truth as phase alignment between word and prompt.
pub struct TruthConditionAgent;

impl TruthConditionAgent {
    pub fn process(facet: &Facet, prompt: &str) -> AgentContribution {
        let tokens = Tokenizer::tokenize(prompt);
        let mut truth_score = 0.0;
        let mut checked = 0;

        for token in &tokens {
            if let Some(phasor) = facet.lexicon.get(token) {
                let prompt_wave = Wave::sentence(facet, &tokens);
                let word_wave = phasor.to_complex();
                let alignment = (word_wave * prompt_wave.conj()).arg().abs();
                let normalized = 1.0 - (alignment / std::f64::consts::PI).min(1.0);
                truth_score += normalized;
                checked += 1;
            }
        }

        let avg_truth = if checked > 0 { truth_score / checked as f64 } else { 0.0 };

        AgentContribution {
            agent_name: "TruthCondition",
            agent_role: "Propositional truth alignment",
            confidence: avg_truth,
            output: format!("Truth alignment: {:.3} ({} words checked)", avg_truth, checked),
            phase_contribution: 0.0,
        }
    }
}
