#[allow(unused_imports)]
use super::*;

mod response;

impl InstructionEngine {
    /// Creates a new [`InstructionEngine`] with default capacity.
    ///
    /// - Context buffer: 4096 tokens
    /// - Generator: 128-step decode, 0.15 temperature
    pub fn new() -> Self {
        Self {
            context_buffer: ContextWaveBuffer::new(4096),
            phase_field: HierarchicalPhaseField::new(),
            generator: Generator::new(128, 0.15),
        }
    }

    /// Formats a prompt into a clean instruction template.
    ///
    /// ```text
    /// user
    /// {prompt}
    /// <|end|>
    /// assistant
    /// ```
    pub fn format_template(prompt: &str) -> String {
        format!("user\n{}\n<|end|>\nassistant\n", prompt.trim())
    }

    /// Extracts content words from a prompt, filling remaining slots from the
    /// phase manifold via bigram candidates and ray-cast decoding.
    ///
    /// # Arguments
    ///
    /// - `facet` — the trained phase-space manifold
    /// - `prompt` — raw user input
    /// - `n` — maximum number of topic words to return
    ///
    /// # Returns
    ///
    /// Up to `n` content words, ordered by prompt occurrence then manifold
    /// proximity.
    pub fn extract_topic_words(facet: &Facet, prompt: &str, n: usize) -> Vec<String> {
        let mut result = Tokenizer::content_words(prompt);
        result.retain(|w| facet.lexicon.contains_key(w));
        result.truncate(n);

        if result.len() < n {
            for seed in result.clone().iter().take(3) {
                for (w, _) in facet.next_word_candidates(seed) {
                    if !Tokenizer::is_function_word(&w) && !result.contains(&w) && result.len() < n {
                        result.push(w);
                    }
                }
            }
        }

        if result.len() < n {
            let tokens = Tokenizer::tokenize(prompt);
            let wave = Wave::sentence(facet, &tokens);
            for (w, _) in Wave::ray_cast(facet, wave, n * 4) {
                if !Tokenizer::is_function_word(&w) && !result.contains(&w) && result.len() < n {
                    result.push(w);
                }
            }
        }

        result
    }

    /// Generates readable sentences using templates filled with resonant words.
    ///
    /// Each [`InstructionKind`] produces a different rhetorical structure
    /// (explanatory, analytical, creative, code-oriented, or command).
    pub fn templated_output(facet: &Facet, prompt: &str, kind: InstructionKind) -> String {
        let words = Self::extract_topic_words(facet, prompt, 8);
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
}
