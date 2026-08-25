#![allow(dead_code)]

use crate::cognitive::CognitiveCore;
use crate::facet::Facet;
use crate::generate::{ContextWaveBuffer, Generator};
use crate::layers::HierarchicalPhaseField;
use crate::tokenizer::Tokenizer;
use crate::trainer::Trainer;
use crate::wave::Wave;
use std::fmt;

/// InstructionKind - type of instruction issued by user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionKind {
    Code,
    Explain,
    Creative,
    Analyze,
    Command,
}

impl InstructionKind {
    pub fn parse(prompt: &str) -> Self {
        let p = prompt.to_lowercase();
        match () {
            _ if p.contains("explain") || p.contains("what is") || p.contains("how does") || p.contains("how do") || p.contains("why") => InstructionKind::Explain,
            _ if p.contains("code") || p.contains("function") || p.contains("implement") || p.contains("fix") || p.contains("debug") => InstructionKind::Code,
            _ if p.contains("compare") || p.contains("benchmark") || p.contains("analyze") => InstructionKind::Analyze,
            _ if p.contains("write") || p.contains("story") || p.contains("haiku") || p.contains("poem") => InstructionKind::Creative,
            _ => InstructionKind::Command,
        }
    }

    pub fn to_persona_name(&self) -> &'static str {
        match self {
            InstructionKind::Code => "Coder",
            InstructionKind::Explain => "Teacher",
            InstructionKind::Creative => "Poet",
            InstructionKind::Analyze => "Analyst",
            InstructionKind::Command => "Assistant",
        }
    }
}

/// Extract content words from a prompt, then fill remaining slots from the manifold.
pub fn extract_topic_words(facet: &Facet, prompt: &str, n: usize) -> Vec<String> {
    let mut result = Tokenizer::content_words(prompt);
    result.retain(|w| facet.lexicon.contains_key(w));
    result.truncate(n);

    match result.len() < n {
        true => {
            for seed in result.clone().iter().take(3) {
                for (w, _) in facet.next_word_candidates(seed) {
                    match !Tokenizer::is_function_word(&w) && !result.contains(&w) && result.len() < n {
                        true => result.push(w),
                        false => {}
                    }
                }
            }
        }
        false => {}
    }

    match result.len() < n {
        true => {
            let tokens = Tokenizer::tokenize(prompt);
            let wave = Wave::sentence(facet, &tokens);
            for (w, _) in Wave::ray_cast(facet, wave, n * 4) {
                match !Tokenizer::is_function_word(&w) && !result.contains(&w) && result.len() < n {
                    true => result.push(w),
                    false => {}
                }
            }
        }
        false => {}
    }

    result
}

/// Looks up grounded definitions for content words in the prompt.
fn lookup_definitions(cognitive_core: &CognitiveCore, prompt: &str) -> Vec<(String, String)> {
    Tokenizer::content_words(prompt)
        .into_iter()
        .filter_map(|word| {
            cognitive_core
                .chunk_store
                .load_definition(&word)
                .map(|def| (word, def))
        })
        .collect()
}

/// Builds a conversational reply from grounded definitions, then torus attractor generation.
fn compose_reply(
    facet: &Facet,
    context_buffer: &mut ContextWaveBuffer,
    generator: &Generator,
    prompt: &str,
    definitions: &[(String, String)],
) -> String {
    let def_reply: Option<String> = match definitions.is_empty() {
        false => {
            let parts: Vec<String> = definitions.iter().take(3).filter_map(|(word, def)| {
                let short: String = def.chars().take(220).collect();
                let trimmed = short.trim();
                match trimmed.is_empty() {
                    true => None,
                    false => Some(format!("{}: {}", word, trimmed)),
                }
            }).collect();
            match parts.is_empty() {
                false => Some(parts.join("\n")),
                true => None,
            }
        }
        true => None,
    };

    match def_reply {
        Some(reply) => reply,
        None => {
            let generated = generator.generate(facet, context_buffer, prompt);
            match generated.split_whitespace().count() >= 3 {
                true => generated,
                false => format!("I heard you, but I do not yet have a grounded meaning for: {}", prompt),
            }
        }
    }
}

/// Generate readable sentences using templates filled with resonant words.
pub fn templated_output(facet: &Facet, prompt: &str, kind: InstructionKind) -> String {
    let words = extract_topic_words(facet, prompt, 8);
    match words.len() < 3 {
        true => return format!("I need more context to respond to: {}", prompt),
        false => {}
    }

    let w = |i: usize| -> &str { words.get(i).map(|s| s.as_str()).unwrap_or("concepts") };

    match kind {
        InstructionKind::Explain => {
            let mut lines = Vec::new();
            lines.push(format!("{} is a concept that relates to {} and {}.", w(0), w(1), w(2)));
            lines.push(format!("It involves understanding how {} connects to {}.", w(1), w(3)));
            lines.push(format!("Key aspects include {}, {}, and {}.", w(2), w(4), w(5)));
            lines.push(format!("In practice, {} requires considering {} alongside {}.", w(0), w(6), w(7)));
            lines.join(" ")
        }
        InstructionKind::Analyze => {
            let mut lines = Vec::new();
            lines.push(format!("Analyzing: {} in relation to {}.", w(0), w(1)));
            lines.push(format!("{} and {} share connections through {}.", w(0), w(1), w(2)));
            lines.push(format!("However, {} differs from {} in terms of {}.", w(3), w(4), w(5)));
            lines.push(format!("Overall, {} and {} are complementary aspects of {}.", w(0), w(1), w(6)));
            lines.join(" ")
        }
        InstructionKind::Code => {
            let mut lines = Vec::new();
            lines.push(format!("To implement {}, consider the role of {} and {}.", w(0), w(1), w(2)));
            lines.push(format!("The pattern involves {} for structure and {} for safety.", w(3), w(4)));
            lines.push(format!("Use {} to manage {} effectively.", w(5), w(6)));
            lines.join(" ")
        }
        InstructionKind::Creative => {
            let mut lines = Vec::new();
            lines.push(format!("{} dances with {} in the light of {}.", w(0), w(1), w(2)));
            lines.push(format!("Through {} and {}, the story unfolds.", w(3), w(4)));
            lines.push(format!("{} whispers of {} and the promise of {}.", w(5), w(6), w(7)));
            lines.join(" ")
        }
        InstructionKind::Command => {
            let mut lines = Vec::new();
            lines.push(format!("Processing: {} with context from {}.", w(0), w(1)));
            lines.push(format!("Related factors: {}, {}, and {}.", w(2), w(3), w(4)));
            lines.push(format!("Result: {} informed by {}.", w(0), w(5)));
            lines.join(" ")
        }
    }
}

/// ChatResponse - clean response from the chat/instruct pipeline.
/// Separates the conversational text from cognitive metadata.
pub struct ChatResponse {
    /// The main response text shown to the user.
    pub text: String,
    /// The cognitive synthesis output (intentional state-driven).
    pub cognitive_synthesis: String,
    /// Detected intent type.
    pub intent: InstructionKind,
    /// Speech act classification.
    pub speech_act: String,
    /// Direction of fit (mind→world, world→mind, etc.).
    pub direction_of_fit: String,
    /// Satisfaction score [0, 1].
    pub satisfaction: f64,
    /// Phase-guided generation trace.
    pub phase_trace: String,
}

impl fmt::Display for ChatResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  {}
", self.text)?;
        writeln!(f, "  ── cognitive synthesis ──")?;
        writeln!(f, "  {}", self.cognitive_synthesis)?;
        writeln!(f)?;
        writeln!(f, "  [intent: {:?} | speech act: {} | satisfaction: {:.0}%]", self.intent, self.speech_act, self.satisfaction * 100.0)?;
        writeln!(f, "  [phase trace: {}]", self.phase_trace)
    }
}

/// Generates a chat response using shared cognitive core + context buffer.
/// Trains the manifold online, then answers from grounded definitions or ray-cast decoding.
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

/// InstructionEngine - handles instruction parsing, formatting, and phase-guided execution (Phase 4).
pub struct InstructionEngine {
    pub context_buffer: ContextWaveBuffer,
    pub phase_field: HierarchicalPhaseField,
    pub generator: Generator,
}

impl Default for InstructionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl InstructionEngine {
    pub fn new() -> Self {
        Self {
            context_buffer: ContextWaveBuffer::new(4096),
            phase_field: HierarchicalPhaseField::new(),
            generator: Generator::new(128, 0.15),
        }
    }

    /// Formats prompt into a clean instruction template.
    pub fn format_template(prompt: &str) -> String {
        format!("user\n{}\n<|end|>\nassistant\n", prompt.trim())
    }

    /// Executes an incoming instruction using shared cognitive core + context buffer.
    pub fn execute_instruction(
        &mut self,
        facet: &mut Facet,
        _trainer: &Trainer,
        cognitive_core: &CognitiveCore,
        context_buffer: &mut ContextWaveBuffer,
        prompt: &str,
    ) -> ChatResponse {
        self.phase_field.build_hierarchy(facet);
        generate_response(facet, _trainer, cognitive_core, context_buffer, &self.generator, prompt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_parsing() {
        assert_eq!(InstructionKind::parse("write code for rust function"), InstructionKind::Code);
        assert_eq!(InstructionKind::parse("explain ownership in rust"), InstructionKind::Explain);
        assert_eq!(InstructionKind::parse("write a story about golf"), InstructionKind::Creative);
        assert_eq!(InstructionKind::Code.to_persona_name(), "Coder");
    }
}
