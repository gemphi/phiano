use crate::attention;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use crate::wave::{Wave, c64};
use std::collections::VecDeque;
use std::f64::consts::PI;

pub const CONTEXT_WINDOW_SIZE: usize = 4096; // 2^12
#[allow(dead_code)]
pub const CONTEXT_LAYERS_COUNT: usize = 16;  // 2^4
pub const CONTEXT_DECAY_BASE: f64 = 0.5;    // 2^(-1)

/// ContextWaveBuffer — maintains a running superposition wave of multi-turn conversation context.
pub struct ContextWaveBuffer {
    /// The running complex superposition wave (represented as x + iy -> (r, theta))
    pub sum_x: f64,
    pub sum_y: f64,
    /// Ring buffer of recent tokens
    tokens: VecDeque<String>,
    max_capacity: usize,
}

impl Default for ContextWaveBuffer {
    fn default() -> Self {
        Self::new(CONTEXT_WINDOW_SIZE)
    }
}

impl ContextWaveBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            sum_x: 0.0,
            sum_y: 0.0,
            tokens: VecDeque::with_capacity(capacity),
            max_capacity: capacity,
        }
    }

    /// Appends new text to the context wave, applying exponential decay to past context.
    pub fn push_turn(&mut self, facet: &Facet, text: &str) {
        // Decay past context wave
        self.sum_x *= CONTEXT_DECAY_BASE;
        self.sum_y *= CONTEXT_DECAY_BASE;

        let turn_tokens = Tokenizer::tokenize(text);
        for token in turn_tokens {
            if self.tokens.len() >= self.max_capacity {
                self.tokens.pop_front();
            }
            if let Some(phasor) = facet.lexicon.get(&token) {
                self.sum_x += phasor.amplitude * phasor.phase.cos();
                self.sum_y += phasor.amplitude * phasor.phase.sin();
            }
            self.tokens.push_back(token);
        }
    }

    /// Computes the current context phase angle in [0, 2pi).
    pub fn context_phase(&self) -> f64 {
        let angle = self.sum_y.atan2(self.sum_x);
        if angle < 0.0 {
            angle + 2.0 * PI
        } else {
            angle
        }
    }

    /// Returns the context wave magnitude (amplitude).
    pub fn context_amplitude(&self) -> f64 {
        (self.sum_x * self.sum_x + self.sum_y * self.sum_y).sqrt()
    }
}

/// Generator — phase-guided sequence sampler (Phase 2).
pub struct Generator {
    pub max_tokens: usize,
    pub temperature: f64,
}

impl Generator {
    pub fn new(max_tokens: usize, temperature: f64) -> Self {
        Self {
            max_tokens,
            temperature,
        }
    }

    /// Generates a response sequence using phase-guided ray-cast sampling
    /// with bigram transition probabilities for word ordering.
    pub fn generate(
        &self,
        facet: &Facet,
        context_buffer: &mut ContextWaveBuffer,
        prompt: &str,
    ) -> String {
        // Push prompt into context wave buffer
        context_buffer.push_turn(facet, prompt);

        let mut generated_tokens: Vec<String> = Vec::new();
        let mut current_phase = context_buffer.context_phase();

        // Get prompt tokens to seed bigram-based generation
        let prompt_tokens = Tokenizer::tokenize(prompt);
        let mut last_word: Option<String> = prompt_tokens.last().cloned();
        let mut prev_word: Option<String> = if prompt_tokens.len() >= 2 {
            Some(prompt_tokens[prompt_tokens.len() - 2].clone())
        } else {
            None
        };
        let mut recent_words: std::collections::HashSet<String> = std::collections::HashSet::new();

        for step in 0..self.max_tokens {
            // Compute artificial jitter based on temperature
            let jitter = if self.temperature > 0.0 {
                (step as f64 * 0.618033988749895).sin() * self.temperature * 0.1
            } else {
                0.0
            };
            let target_phase = (current_phase + jitter).rem_euclid(2.0 * PI);

            // Try trigram-guided selection first, then bigram, then attention
            let next_word = {
                let tri_candidates = if let (Some(a), Some(b)) = (&prev_word, &last_word) {
                    let mut tc = facet.trigram_candidates(a, b);
                    tc.sort_by(|x, y| y.1.cmp(&x.1));
                    tc.truncate(15);
                    tc
                } else {
                    Vec::new()
                };

                if !tri_candidates.is_empty() {
                    // Use attention to re-rank trigram candidates
                    let ctx_tokens: Vec<String> = generated_tokens.iter().take(8).cloned()
                        .chain(prompt_tokens.iter().take(4).cloned()).collect();
                    let attn_scored = attention::attention_next_words(
                        facet, &ctx_tokens, &tri_candidates, target_phase, 5,
                    );
                    attn_scored.iter()
                        .filter(|(w, _)| !recent_words.contains(w))
                        .map(|(w, _)| w.clone())
                        .next()
                } else if let Some(prev) = &last_word {
                    let mut bigram_candidates = facet.next_word_candidates(prev);
                    bigram_candidates.sort_by(|a, b| b.1.cmp(&a.1));
                    bigram_candidates.truncate(20);
                    if !bigram_candidates.is_empty() {
                        // Use attention to re-rank bigram candidates
                        let ctx_tokens: Vec<String> = generated_tokens.iter().take(8).cloned()
                            .chain(prompt_tokens.iter().take(4).cloned()).collect();
                        let attn_scored = attention::attention_next_words(
                            facet, &ctx_tokens, &bigram_candidates, target_phase, 5,
                        );
                        attn_scored.iter()
                            .filter(|(w, _)| !recent_words.contains(w))
                            .map(|(w, _)| w.clone())
                            .next()
                    } else { None }
                } else { None }
            };

            // Fall back to phase-only ray cast if no bigram match
            let next_word = match next_word {
                Some(w) => Some(w),
                None => {
                    let target_wave = c64::new(target_phase.cos(), target_phase.sin());
                    let candidates = Wave::ray_cast(facet, target_wave, 16);
                    candidates
                        .iter()
                        .map(|(w, _)| w)
                        .find(|w| !recent_words.contains(*w))
                        .cloned()
                }
            };

            match next_word {
                Some(word) => {
                    // Track recent words (keep last 6)
                    recent_words.insert(word.clone());
                    if generated_tokens.len() > 6 {
                        if let Some(old) = generated_tokens.get(generated_tokens.len() - 6) {
                            recent_words.remove(old);
                        }
                    }
                    // Update current phase toward selected word's phase
                    if let Some(phasor) = facet.lexicon.get(&word) {
                        current_phase = (current_phase + 0.3 * (phasor.phase - current_phase).sin())
                            .rem_euclid(2.0 * PI);
                    }
                    generated_tokens.push(word.clone());
                    prev_word = last_word.clone();
                    last_word = Some(word);

                    // End if sequence completes sentence phase cycle
                    if generated_tokens.len() >= 24 && step % 8 == 0 {
                        break;
                    }
                }
                None => break,
            }
        }

        let result = generated_tokens.join(" ");
        // Also push generated tokens into context buffer
        context_buffer.push_turn(facet, &result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_wave_buffer() {
        let mut facet = Facet::new();
        facet.get_or_init("rust");
        facet.get_or_init("code");

        let mut buffer = ContextWaveBuffer::new(CONTEXT_WINDOW_SIZE);
        buffer.push_turn(&facet, "rust code");

        assert!(buffer.context_amplitude() > 0.0);
        assert_eq!(CONTEXT_LAYERS_COUNT, 16);
    }
}
