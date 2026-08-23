/// Intentionality agents — model *aboutness* and pre-intentional Background.
///
/// Searle's Intentionality thesis: mental states are *about* objects/states of affairs.
/// The content of an intentional state is its representational content.
/// The Background is the set of pre-intentional capacities that enable intentionality —
/// knowing *how* to do things, not knowing *that* things are the case.

use super::types::*;
use crate::facet::Facet;
use crate::generate::ContextWaveBuffer;
use crate::tokenizer::Tokenizer;
use crate::wave::Wave;
use std::f64::consts::PI;

/// 1. IntentionalityAgent — determines what the utterance is *about*.
/// Models the intentional state's content: the prompt's phase centroid
/// represents the "aboutness vector" — the direction the mental state points.
pub struct IntentionalityAgent;

impl IntentionalityAgent {
    pub fn process(facet: &Facet, prompt: &str) -> AgentContribution {
        let tokens = Tokenizer::tokenize(prompt);
        let known: Vec<String> = tokens.iter()
            .filter(|t| facet.lexicon.contains_key(*t))
            .cloned()
            .collect();

        let (phase, about, confidence) = if known.is_empty() {
            (0.0, "unknown topic".to_string(), 0.0)
        } else {
            let wave = Wave::sentence(facet, &known);
            let phase = wave.arg().rem_euclid(2.0 * PI);
            let about = known.iter().take(5).cloned().collect::<Vec<_>>().join(", ");
            let conf = known.len() as f64 / tokens.len().max(1) as f64;
            (phase, about, conf)
        };

        AgentContribution {
            agent_name: "Intentionality",
            agent_role: "What is this about? (intentional content)",
            confidence,
            output: format!("Intentional content: about '{}' (phase={:.3})", about, phase),
            phase_contribution: phase,
        }
    }
}

/// 2. AboutnessAgent — maps words to their referents via phase proximity.
/// In Searle's framework, aboutness is the directedness of the mental state.
/// Here we model it as: each word *points to* its nearest semantic neighbors.
pub struct AboutnessAgent;

impl AboutnessAgent {
    pub fn process(facet: &Facet, prompt: &str) -> AgentContribution {
        let tokens = Tokenizer::tokenize(prompt);
        let mut grounded = Vec::new();

        for token in &tokens {
            if let Some(phasor) = facet.lexicon.get(token) {
                let mut neighbors: Vec<(String, f64)> = facet.lexicon.iter()
                    .filter(|(w, _)| *w != token)
                    .map(|(w, p)| {
                        let mut diff = (p.phase - phasor.phase).abs();
                        if diff > PI { diff = 2.0 * PI - diff; }
                        (w.clone(), diff)
                    })
                    .collect();
                neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
                let refs: Vec<String> = neighbors.iter().take(3).map(|(w, _)| w.clone()).collect();
                grounded.push(format!("{} → [{}]", token, refs.join(", ")));
            }
        }

        let output = if grounded.is_empty() {
            "No referents found — all words unknown".to_string()
        } else {
            grounded.join("; ")
        };

        AgentContribution {
            agent_name: "Aboutness",
            agent_role: "Word-to-referent directedness",
            confidence: if grounded.is_empty() { 0.0 } else { 1.0 },
            output,
            phase_contribution: 0.0,
        }
    }
}

/// 3. BackgroundAgent — Searle's pre-intentional Background.
/// The Background is NOT a set of beliefs or representations — it's the
/// capacity to *use* representations. It's "knowing how" vs "knowing that".
/// Here we model it as the accumulated context wave — the pre-reflective
/// stance that shapes how intentional states are interpreted.
pub struct BackgroundAgent;

impl BackgroundAgent {
    pub fn process(context_buffer: &ContextWaveBuffer) -> AgentContribution {
        let phase = context_buffer.context_phase();
        let amplitude = context_buffer.context_amplitude();

        // The Background "capacity" is modeled as the accumulated amplitude —
        // more context = more Background capacity to interpret meaning.
        let capacity = (amplitude / 50.0).min(1.0);

        AgentContribution {
            agent_name: "Background",
            agent_role: "Pre-intentional capacities (knowing-how)",
            confidence: capacity,
            output: format!(
                "Background capacity: {:.0}% (phase={:.3}, amplitude={:.2}) — pre-reflective stance",
                capacity * 100.0, phase, amplitude
            ),
            phase_contribution: phase,
        }
    }
}
