/// Semantics agents — the syntax/semantics distinction (Chinese Room argument).
///
/// Searle's Chinese Room: a system can manipulate symbols according to rules
/// (syntax) without understanding what they mean (semantics).
/// This is the core argument against strong AI — mere symbol manipulation
/// is not sufficient for understanding.
///
/// These agents model the gap between syntactic processing (bigram ordering,
/// phase matching) and semantic understanding (definition grounding, coherence).

use super::types::*;
use crate::eval::Evaluator;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use std::f64::consts::PI;

/// 10. SemanticsAgent — maps phase relationships to meaning relations.
/// Models the semantic level: synonymy, antonymy, relatedness.
/// This is distinct from syntax (word ordering) — it's about *meaning*.
pub struct SemanticsAgent;

impl SemanticsAgent {
    pub fn process(facet: &Facet, prompt: &str) -> AgentContribution {
        let tokens = Tokenizer::tokenize(prompt);
        let known: Vec<&String> = tokens.iter()
            .filter(|t| facet.lexicon.contains_key(*t))
            .collect();

        if known.len() < 2 {
            return AgentContribution {
                agent_name: "Semantics",
                agent_role: "Phase-to-meaning mapping (semantic relations)",
                confidence: 0.0,
                output: "Insufficient known words for semantic analysis".to_string(),
                phase_contribution: 0.0,
            };
        }

        let mut relationships = Vec::new();
        for i in 0..known.len().min(4) {
            for j in (i+1)..known.len().min(5) {
                let p1 = &facet.lexicon[known[i]];
                let p2 = &facet.lexicon[known[j]];
                let mut diff = (p1.phase - p2.phase).abs();
                if diff > PI { diff = 2.0 * PI - diff; }
                let relation = if diff < 0.3 {
                    "synonym (same meaning)"
                } else if diff < 1.0 {
                    "related (overlapping meaning)"
                } else if diff > 2.5 {
                    "antonym (opposite meaning)"
                } else {
                    "tangential (distant meaning)"
                };
                relationships.push(format!("{}-{}: {} ({:.3})", known[i], known[j], relation, diff));
            }
        }

        AgentContribution {
            agent_name: "Semantics",
            agent_role: "Phase-to-meaning mapping (semantic relations)",
            confidence: 0.75,
            output: relationships.join(", "),
            phase_contribution: 0.0,
        }
    }
}

/// 11. SyntaxAgent — handles word ordering via bigram transitions.
/// This is the *syntactic* level — Searle's Chinese Room shows that syntax
/// alone is insufficient for understanding. But syntax is necessary for
/// producing well-formed output.
pub struct SyntaxAgent;

impl SyntaxAgent {
    pub fn process(facet: &Facet, prompt: &str) -> AgentContribution {
        let tokens = Tokenizer::tokenize(prompt);
        let mut ordered: Vec<String> = Vec::new();
        let mut current: Option<&str> = None;

        for token in &tokens {
            if facet.lexicon.contains_key(token) {
                if let Some(prev) = current {
                    let prob = facet.bigram_probability(prev, token);
                    if prob > 0.0 {
                        ordered.push(format!("{}→{} ({:.2})", prev, token, prob));
                    }
                }
                current = Some(token);
            }
        }

        // Generate a bigram-ordered sequence from prompt words
        let mut seq = Vec::new();
        if let Some(first) = tokens.first() {
            if facet.lexicon.contains_key(first) {
                seq.push(first.clone());
                let mut last = first.clone();
                for _ in 0..8 {
                    let candidates = facet.next_word_candidates(&last);
                    if candidates.is_empty() { break; }
                    let best = candidates.iter()
                        .max_by_key(|(_, c)| *c)
                        .map(|(w, _)| w.clone());
                    if let Some(w) = best {
                        if !seq.contains(&w) {
                            seq.push(w.clone());
                            last = w;
                        } else { break; }
                    } else { break; }
                }
            }
        }

        let output = if seq.is_empty() {
            "No syntactic ordering possible — syntax insufficient".to_string()
        } else {
            format!("Bigram-ordered: {}\nTransitions: {}", seq.join(" "), ordered.join(", "))
        };

        AgentContribution {
            agent_name: "Syntax",
            agent_role: "Word ordering (syntactic — necessary but not sufficient)",
            confidence: if seq.len() > 2 { 0.7 } else { 0.3 },
            output,
            phase_contribution: 0.0,
        }
    }
}

/// 12. ConsciousnessAgent — evaluates qualitative coherence.
/// Searle argues consciousness is a biological phenomenon with qualitative,
/// subjective character. Here we model the *qualitative* aspect as the
/// coherence/novelty/resonance of the phase state — a proxy for the
/// "what it is like" aspect of processing.
pub struct ConsciousnessAgent;

impl ConsciousnessAgent {
    pub fn process(facet: &Facet, prompt: &str) -> AgentContribution {
        let evaluator = Evaluator::new();
        let eval = evaluator.eval(facet, prompt);

        AgentContribution {
            agent_name: "Consciousness",
            agent_role: "Qualitative coherence (what-it-is-like)",
            confidence: eval.coherence,
            output: format!(
                "Qualitative state: coherence={:.3}, novelty={:.3}, resonance={:.3} — {}",
                eval.coherence, eval.novelty, eval.resonance, eval.verdict
            ),
            phase_contribution: 0.0,
        }
    }
}
