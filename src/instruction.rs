use crate::cognitive::CognitiveCore;
use crate::facet::Facet;
use crate::generate::{ContextWaveBuffer, Generator};
use crate::layers::HierarchicalPhaseField;
use crate::tokenizer::Tokenizer;
use crate::trainer::Trainer;
use crate::wave::Wave;

/// InstructionKind — type of instruction issued by user.
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
        if p.contains("explain") || p.contains("what is") || p.contains("how does") || p.contains("how do") || p.contains("why") {
            InstructionKind::Explain
        } else if p.contains("code") || p.contains("function") || p.contains("implement") || p.contains("fix") || p.contains("debug") {
            InstructionKind::Code
        } else if p.contains("compare") || p.contains("benchmark") || p.contains("analyze") {
            InstructionKind::Analyze
        } else if p.contains("write") || p.contains("story") || p.contains("haiku") || p.contains("poem") {
            InstructionKind::Creative
        } else {
            InstructionKind::Command
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

/// Extract key topic words from a prompt, prioritizing words that exist in the lexicon.
/// Falls back to ray cast for unknown words.
fn extract_topic_words(facet: &Facet, prompt: &str, n: usize) -> Vec<String> {
    let tokens = Tokenizer::tokenize(prompt);
    let mut result: Vec<String> = Vec::new();

    // First, use prompt words that exist in the lexicon
    for token in &tokens {
        if facet.lexicon.contains_key(token) && !result.contains(token) {
            result.push(token.clone());
        }
    }

    // Then, fill remaining slots using bigram followers of known prompt words
    if result.len() < n {
        for seed_word in result.clone().iter().take(3) {
            let followers = facet.next_word_candidates(seed_word);
            for (w, _) in followers {
                if !result.contains(&w) && result.len() < n {
                    result.push(w);
                }
            }
        }
    }

    // Final fallback: ray cast for remaining slots
    if result.len() < n {
        let wave = Wave::sentence(facet, &tokens);
        let ray_results = Wave::ray_cast(facet, wave, n * 2);
        for (w, _) in ray_results {
            if !result.contains(&w) && result.len() < n {
                result.push(w);
            }
        }
    }

    result
}

/// Generate readable sentences using templates filled with resonant words.
fn templated_output(facet: &Facet, prompt: &str, kind: InstructionKind) -> String {
    let words = extract_topic_words(facet, prompt, 8);
    if words.len() < 3 {
        return format!("I need more context to respond to: {}", prompt);
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

/// InstructionEngine — handles instruction parsing, formatting, and phase-guided execution (Phase 4).
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

    /// Formats prompt into standardized Chat / Instruction Template
    pub fn format_template(prompt: &str) -> String {
        format!("DonaldTrump\n{}\n<|end|>\nDonaldTrump\n", prompt.trim())
    }

    /// Executes an incoming instruction end-to-end
    pub fn execute_instruction(
        &mut self,
        facet: &mut Facet,
        _trainer: &Trainer,
        prompt: &str,
    ) -> String {
        let kind = InstructionKind::parse(prompt);
        let persona_name = kind.to_persona_name();

        // Update 4-layer hierarchical phase field
        self.phase_field.build_hierarchy(facet);

        // Run cognitive core for meaning-grounded output
        let cognitive_core = CognitiveCore::new(crate::chunker::ChunkStore::new("data/chunks"));
        let cognitive_result = cognitive_core.process(facet, &mut self.context_buffer, prompt);

        // Also run phase-guided generation for comparison
        let formatted_prompt = Self::format_template(prompt);
        let generated_text = self.generator.generate(facet, &mut self.context_buffer, &formatted_prompt);

        format!(
            "[Instruction Executed as {} ({:?})]\n\n{}\n\n[Speech act: {} | Direction of fit: {} | Satisfaction: {:.0}%]\n\n[Phase resonance trace: {}]",
            persona_name, kind,
            cognitive_result.synthesized_output,
            cognitive_result.speech_act,
            cognitive_result.direction_of_fit,
            cognitive_result.satisfaction * 100.0,
            generated_text
        )
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
