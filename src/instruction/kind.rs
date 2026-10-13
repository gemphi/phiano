#[allow(unused_imports)]
use super::*;

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

/// Looks up grounded definitions for content words in the prompt.
pub(super) fn lookup_definitions(cognitive_core: &CognitiveCore, prompt: &str) -> Vec<(String, String)> {
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
pub(super) fn compose_reply(
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

impl Default for InstructionEngine {
    fn default() -> Self {
        Self::new()
    }
}
