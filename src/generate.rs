use crate::attention;
use crate::config::{
    DEFAULT_CONTEXT_WINDOW, PHI_CONJUGATE, TWO_PI,
    SYNTACTIC_MOMENTUM_DEFAULT, TORUS_DECODE_POOL,
    CONTEXT_LAMBDA, CONTEXT_OMEGA, PHASE_CHANNELS,
};
use crate::facet::Facet;
use crate::phase_flow::PhaseFlow;
use crate::phasor::{SpectralPhasor, TorusPhasor};
use crate::tokenizer::Tokenizer;
use crate::wave::{Wave, c64};
use std::collections::VecDeque;

pub const CONTEXT_WINDOW_SIZE: usize = DEFAULT_CONTEXT_WINDOW;
#[allow(dead_code)]
pub const CONTEXT_LAYERS_COUNT: usize = 16;
pub const CONTEXT_DECAY_BASE: f64 = 0.5;

/// ContextWaveBuffer — a diagonal complex linear recurrence over the context.
///
/// State evolves as `h_t = λ_k · e^{i ω_k} · h_{t-1} + z_t`, independently per
/// channel, with `λ_k` and `ω_k` spread geometrically so that each channel has
/// its own timescale and rotation frequency.
///
/// The previous buffer was a decayed *sum*, which is commutative: it could not
/// distinguish "dog bites man" from "man bites dog", and every token in a turn
/// carried the same weight regardless of position. A rotation-and-decay
/// recurrence is order-sensitive by construction, gives recent tokens more
/// influence than old ones, and is the same mechanism modern state-space models
/// (S4/S5, LRU, Mamba's linear component) use to carry long-range structure at
/// constant memory.
pub struct ContextWaveBuffer {
    /// Channel-0 state, exposed as (x, y) for compatibility.
    pub sum_x: f64,
    pub sum_y: f64,
    /// Per-channel recurrent state.
    h: Vec<c64>,
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
            h: vec![c64::new(0.0, 0.0); PHASE_CHANNELS],
            tokens: VecDeque::with_capacity(capacity.min(4096)),
            max_capacity: capacity,
        }
    }

    /// Decay and rotation for channel `k`.
    ///
    /// Channel 0 keeps the slowest decay and no rotation, so it behaves like a
    /// smoothed running topic; higher channels decay faster and rotate quicker,
    /// giving the state a spectrum of timescales rather than a single one.
    #[inline]
    fn kernel(k: usize) -> c64 {
        let frac = k as f64 / PHASE_CHANNELS as f64;
        let lambda = CONTEXT_LAMBDA.powf(1.0 + 3.0 * frac);
        let omega = CONTEXT_OMEGA * (1.0 + 4.0 * frac);
        c64::from_polar(lambda, omega)
    }

    /// Advances the state by one token.
    pub fn push_token(&mut self, facet: &Facet, token: &str) {
        for k in 0..PHASE_CHANNELS {
            self.h[k] *= Self::kernel(k);
        }
        if let Some(phasor) = facet.lexicon.get(token) {
            for k in 0..PHASE_CHANNELS {
                self.h[k] += c64::from_polar(phasor.amplitude, phasor.theta(k));
            }
        }
        if self.tokens.len() >= self.max_capacity {
            self.tokens.pop_front();
        }
        self.tokens.push_back(token.to_string());
        self.sum_x = self.h[0].re;
        self.sum_y = self.h[0].im;
    }

    /// Appends a whole turn, token by token, so order is preserved.
    pub fn push_turn(&mut self, facet: &Facet, text: &str) {
        for token in Tokenizer::tokenize(text) {
            self.push_token(facet, &token);
        }
    }

    /// Computes the current context phase angle in [0, 2pi).
    pub fn context_phase(&self) -> f64 {
        self.h[0].arg().rem_euclid(TWO_PI)
    }

    /// Returns the context wave magnitude (amplitude).
    pub fn context_amplitude(&self) -> f64 {
        self.h[0].norm()
    }

    /// The full multi-channel context state, for channel-aware retrieval.
    pub fn channels(&self) -> &[c64] {
        &self.h
    }

    /// Clears the recurrent state without dropping the token ring.
    pub fn reset_state(&mut self) {
        for z in self.h.iter_mut() {
            *z = c64::new(0.0, 0.0);
        }
        self.sum_x = 0.0;
        self.sum_y = 0.0;
    }
}

/// Generator - phase-guided sequence sampler (Phase 2).
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

    /// Generates a response sequence using torus attractor decoding.
    pub fn generate(
        &self,
        facet: &Facet,
        context_buffer: &mut ContextWaveBuffer,
        prompt: &str,
    ) -> String {
        self.generate_with_flow(facet, context_buffer, prompt).0
    }

    /// Generates text and returns the live PhaseFlow trajectory alongside it.
    pub fn generate_with_flow(
        &self,
        facet: &Facet,
        context_buffer: &mut ContextWaveBuffer,
        prompt: &str,
    ) -> (String, PhaseFlow) {
        let (tokens, flow) = self.decode(facet, context_buffer, prompt);
        let formatted = Self::format_output(&tokens);
        context_buffer.push_turn(facet, &formatted);
        (formatted, flow)
    }

    /// Token-level decode used by generate and SSE streaming.
    pub fn decode(
        &self,
        facet: &Facet,
        context_buffer: &mut ContextWaveBuffer,
        prompt: &str,
    ) -> (Vec<String>, PhaseFlow) {
        context_buffer.push_turn(facet, prompt);

        let mut generated_tokens: Vec<String> = Vec::new();
        let mut current_phase = context_buffer.context_phase();
        let mut phase_momentum: f64 = SYNTACTIC_MOMENTUM_DEFAULT;
        let mut flow = PhaseFlow::build(facet, prompt);
        flow.propagate(2);

        let prompt_tokens = Tokenizer::tokenize(prompt);
        let mut last_word: Option<String> = prompt_tokens.last().cloned();
        let mut prev_word: Option<String> = match prompt_tokens.len() >= 2 {
            true => Some(prompt_tokens[prompt_tokens.len() - 2].clone()),
            false => None,
        };
        let mut recent_words: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut function_streak = 0usize;

        for step in 0..self.max_tokens {
            let jitter = match self.temperature > 0.0 {
                true => (step as f64 * PHI_CONJUGATE).sin() * self.temperature * 0.08,
                false => 0.0,
            };
            let flow_bias = 0.45 * (flow.collective_phase - current_phase).sin();
            let target_phase = (current_phase + phase_momentum + jitter + flow_bias).rem_euclid(TWO_PI);

            let next_word = self.attractor_select(
                facet, &flow, &prev_word, &last_word, &recent_words, target_phase,
            );

            match next_word {
                Some(word) => {
                    if Tokenizer::is_function_word(&word) {
                        function_streak += 1;
                        if function_streak >= 4 {
                            break;
                        }
                    } else {
                        function_streak = 0;
                    }
                    recent_words.insert(word.clone());
                    self.evict_old(&mut recent_words, &generated_tokens, 12);
                    self.apply_phase_kick(
                        facet, &last_word, &word, &mut current_phase, &mut phase_momentum,
                    );
                    let resonance = flow.resonance_with(facet, &word);
                    flow.record_step(step, Some(&word), resonance, flow.novelty());
                    flow.update_momentum(phase_momentum);
                    generated_tokens.push(word.clone());
                    prev_word = last_word.clone();
                    last_word = Some(word);
                    if generated_tokens.len() >= 20 {
                        break;
                    }
                }
                None => break,
            }
        }

        (generated_tokens, flow)
    }

    /// n-gram first. Ray-cast only among speakable, high-amplitude words.
    fn attractor_select(
        &self,
        facet: &Facet,
        flow: &PhaseFlow,
        prev_word: &Option<String>,
        last_word: &Option<String>,
        recent: &std::collections::HashSet<String>,
        target_phase: f64,
    ) -> Option<String> {
        if let (Some(a), Some(b)) = (prev_word, last_word) {
            let mut tri = facet.trigram_candidates(a, b);
            tri.sort_by(|x, y| y.1.cmp(&x.1));
            tri.truncate(12);
            if let Some(word) = self.pick_ngram(facet, flow, &tri, recent, target_phase) {
                return Some(word);
            }
        }
        if let Some(prev) = last_word {
            let mut bigram = facet.next_word_candidates(prev);
            bigram.sort_by(|a, b| b.1.cmp(&a.1));
            bigram.truncate(16);
            if let Some(word) = self.pick_ngram(facet, flow, &bigram, recent, target_phase) {
                return Some(word);
            }
        }
        self.torus_ray_cast(facet, flow, target_phase, recent)
    }

    /// Whether a token may be emitted.
    ///
    /// Numerals are allowed: the previous rule required every character to be
    /// alphabetic, so the model could not state a quantity, a version or a date.
    fn speakable(word: &str) -> bool {
        let n = word.chars().count();
        if n == 0 || n > 16 || Self::boilerplate(word) {
            return false;
        }
        let all_digits = word.chars().all(|c| c.is_ascii_digit());
        if all_digits {
            return n <= 6;
        }
        n >= 2 && word.chars().all(|c| c.is_ascii_alphanumeric())
    }

    fn boilerplate(word: &str) -> bool {
        matches!(
            word,
            "pertaining" | "genus" | "species" | "extant" | "parvorder"
                | "tokenizer" | "vocab" | "obsolete" | "archaic" | "namely"
                | "viz" | "ie" | "unabridged" | "webster" | "etymology"
                | "plural" | "singular" | "participle" | "adjective" | "adverb"
                | "noun" | "verb" | "hence" | "thereof" | "therein" | "whereby"
                | "whereof" | "aforesaid" | "called" | "written" | "also"
                | "syn" | "obs" | "shak" | "milton" | "cf" | "opp"
                | "idiom" | "dialect" | "phraseology" | "diction"
                | "edifieth" | "puffeth" | "bloodguiltiness"
        )
    }

    fn pick_ngram(
        &self,
        facet: &Facet,
        flow: &PhaseFlow,
        candidates: &[(String, u32)],
        recent: &std::collections::HashSet<String>,
        target_phase: f64,
    ) -> Option<String> {
        let mut best: Option<(String, f64)> = None;
        for (word, count) in candidates {
            if !Self::speakable(word) {
                continue;
            }
            // Soft penalty rather than a hard block: an outright ban on any word
            // seen in the last 12 tokens makes "the cat sat on the mat"
            // unwriteable.
            let repeat_penalty = if recent.contains(word) { 0.15 } else { 1.0 };
            let phase_align = facet
                .lexicon
                .get(word)
                .map(|p| (p.phase - target_phase).cos().max(0.0))
                .unwrap_or(0.0);
            let resonance = flow.resonance_with(facet, word);
            let capped = (*count as f64).min(24.0).ln_1p();
            let content = if Tokenizer::is_function_word(word) { 0.55 } else { 1.35 };
            let score =
                capped * (0.35 + 0.25 * phase_align + 0.40 * resonance) * content * repeat_penalty;
            match &best {
                Some((_, best_score)) if score <= *best_score => {}
                _ => best = Some((word.clone(), score)),
            }
        }
        best.map(|(w, _)| w)
    }

    /// Attention-reranked candidate selection.
    #[allow(dead_code)]
    fn attention_pick(
        &self,
        facet: &Facet,
        prompt_tokens: &[String],
        generated: &[String],
        candidates: &[(String, u32)],
        recent: &std::collections::HashSet<String>,
        target_phase: f64,
    ) -> Option<String> {
        let ctx: Vec<String> = generated.iter().take(8).cloned()
            .chain(prompt_tokens.iter().take(4).cloned()).collect();
        let scored = attention::Attention::next_words(facet, &ctx, candidates, target_phase, 5);
        scored.iter()
            .filter(|(w, _)| !recent.contains(w))
            .map(|(w, _)| w.clone())
            .next()
    }

    /// Torus ray-cast among speakable, high-amplitude words only.
    fn torus_ray_cast(
        &self,
        facet: &Facet,
        flow: &PhaseFlow,
        target_phase: f64,
        recent: &std::collections::HashSet<String>,
    ) -> Option<String> {
        let target_phasor = SpectralPhasor::new(target_phase, 1.0, 0);
        let target_torus = TorusPhasor::from_spectral(&target_phasor);
        let target_wave = c64::new(target_phase.cos(), target_phase.sin());
        let pool = Wave::ray_cast(facet, target_wave, TORUS_DECODE_POOL * 4);

        let mut scored_pool: Vec<(String, f64)> = pool
            .into_iter()
            .filter(|(w, _)| Self::speakable(w) && !recent.contains(w.as_str()))
            .filter_map(|(w, _)| {
                facet.lexicon.get(&w).and_then(|p| {
                    if p.amplitude < 1.05 {
                        return None;
                    }
                    let word_torus = TorusPhasor::from_spectral(p);
                    let res = target_torus.resonance(&word_torus);
                    let flow_res = flow.resonance_with(facet, &w);
                    Some((w, 0.6 * res + 0.4 * flow_res + 0.05 * p.amplitude))
                })
            })
            .collect();

        scored_pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored_pool.first().map(|(w, _)| w.clone())
    }

    /// Applies a phase-kick to the context wave after emitting a token.
    /// Uses learned β_ij phase lag between prev_word and word for syntax-aware steering.
    fn apply_phase_kick(
        &self,
        facet: &Facet,
        prev_word: &Option<String>,
        word: &str,
        current_phase: &mut f64,
        phase_momentum: &mut f64,
    ) {
        match facet.lexicon.get(word) {
            Some(phasor) => {
                let beta = match prev_word {
                    Some(prev) => facet.phase_lag(prev, word),
                    None => 0.0,
                };
                let phase_diff = (phasor.phase - *current_phase + beta).sin();
                *current_phase = (*current_phase + 0.35 * phase_diff).rem_euclid(TWO_PI);
                *phase_momentum = (0.85 * *phase_momentum + 0.15 * phase_diff.abs().max(0.05)).min(0.5);
            }
            None => {}
        }
    }

    /// Evicts the oldest token from the recent-words set.
    fn evict_old(
        &self,
        recent: &mut std::collections::HashSet<String>,
        generated: &[String],
        window: usize,
    ) {
        match generated.len() > window {
            true => {
                if let Some(old) = generated.get(generated.len() - window) {
                    recent.remove(old);
                }
            }
            false => {}
        }
    }

    /// Formats tokens with capitalization and terminal punctuation.
    fn format_output(tokens: &[String]) -> String {
        let mut formatted = String::new();
        for (i, t) in tokens.iter().enumerate() {
            match i {
                0 => {
                    let mut chars = t.chars();
                    match chars.next() {
                        Some(first) => {
                            formatted.push_str(&first.to_uppercase().collect::<String>());
                            formatted.push_str(chars.as_str());
                        }
                        None => {}
                    }
                }
                _ => {
                    formatted.push(' ');
                    formatted.push_str(t);
                }
            }
        }
        let needs_punct = !formatted.is_empty()
            && !formatted.ends_with('.')
            && !formatted.ends_with('!')
            && !formatted.ends_with('?');
        match needs_punct {
            true => formatted.push('.'),
            false => {}
        }
        formatted
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

    #[test]
    fn test_generate_with_flow_records_trajectory() {
        let mut facet = Facet::new();
        let trainer = crate::trainer::Trainer::new(0.05);
        trainer.train_sentence(&mut facet, "the cat sat on the mat");
        trainer.train_sentence(&mut facet, "the dog sat on the rug");

        let gen = Generator::new(8, 0.1);
        let mut buffer = ContextWaveBuffer::new(64);
        let (text, flow) = gen.generate_with_flow(&facet, &mut buffer, "the cat");

        assert!(flow.order_parameter >= 0.0);
        assert!(flow.collective_phase >= 0.0);
        let _ = text;
    }
}
