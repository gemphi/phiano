pub mod metrics;
pub use metrics::{TrainingMetrics, MultiEpochResult};

use crate::config::{self, PHI};
use crate::facet::Facet;
use crate::phasor::SpectralPhasor;
use crate::tokenizer::Tokenizer;
use std::f64::consts::PI;

/// Trainer — unsupervised language learning via Kuramoto phase attraction.
///
/// The trainer learns language by reading definitions and example sentences.
/// Words that co-occur in a sentence get their phase angles pulled toward
/// the sentence's centroid phase. Over many epochs, words that appear in
/// similar contexts converge to similar phases, creating a self-organizing
/// semantic space.
#[derive(Clone)]
pub struct Trainer {
    /// Kuramoto learning rate — controls how fast phases converge.
    pub learning_rate: f64,
}

impl Trainer {
    /// Creates a new trainer with the given learning rate.
    pub fn new(learning_rate: f64) -> Self {
        Self { learning_rate }
    }

    /// Trains on a single sentence using Kuramoto phase attraction.
    ///
    /// Steps:
    /// 1. Tokenize text; initialize unseen tokens at deterministic pseudo-random phases
    /// 2. Compute context centroid phase from all token phasors
    /// 3. Shift each token's phase toward centroid by `lr * sin(target - current)`
    /// 4. Bump `band_n` for tokens already close (prevents phase collapse)
    ///
    /// Returns the number of tokens that were updated.
    pub fn train_sentence(&self, facet: &mut Facet, text: &str) -> usize {
        let tokens = Tokenizer::tokenize(text);
        if tokens.is_empty() {
            return 0;
        }

        self.initialize_tokens(facet, &tokens);

        // Record bigram and trigram co-occurrences
        for window in tokens.windows(2) {
            facet.record_bigram(&window[0], &window[1]);
        }
        for window in tokens.windows(3) {
            facet.record_trigram(&window[0], &window[1], &window[2]);
        }

        let target_phase = self.compute_centroid_phase(facet, &tokens);

        let mut updated = 0;
        for token in &tokens {
            let phasor = facet.lexicon.get_mut(token).unwrap();
            let phase_error = (target_phase - phasor.phase).sin();

            phasor.phase = (phasor.phase + self.learning_rate * phase_error)
                .rem_euclid(2.0 * PI);

            if phase_error.abs() < config::CONVERGENCE_THRESHOLD {
                phasor.band_n += 1;
            }

            phasor.amplitude = (phasor.amplitude + config::AMPLITUDE_INCREMENT).min(config::AMPLITUDE_MAX);
            updated += 1;
        }

        updated
    }

    /// Initializes unseen tokens at deterministic pseudo-random phases.
    ///
    /// The seed phase is derived from the token length multiplied by the
    /// golden ratio, modulo 2*pi. This gives each new word a unique but
    /// deterministic starting position on the phase circle.
    fn initialize_tokens(&self, facet: &mut Facet, tokens: &[String]) {
        for token in tokens {
            facet.lexicon.entry(token.clone()).or_insert_with(|| {
                let seed_phase = (token.len() as f64 * PHI) % (2.0 * PI);
                SpectralPhasor::new(seed_phase, config::AMPLITUDE_INITIAL, config::BAND_N_INITIAL)
            });
        }
    }

    /// Computes the amplitude-weighted centroid phase across all tokens.
    ///
    /// Each token's phasor contributes its cosine (x) and sine (y) components
    /// scaled by amplitude. The centroid phase is the atan2 of the summed
    /// y and x components.
    fn compute_centroid_phase(&self, facet: &Facet, tokens: &[String]) -> f64 {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;

        for token in tokens {
            let phasor = facet.lexicon.get(token).unwrap();
            sum_x += phasor.phase.cos() * phasor.amplitude;
            sum_y += phasor.phase.sin() * phasor.amplitude;
        }

        sum_y.atan2(sum_x)
    }

    /// Trains on a single sentence in online mode (single pass).
    pub fn train_online(&self, facet: &mut Facet, text: &str) -> usize {
        self.train_sentence(facet, text)
    }

    /// Trains on a word-definition pair — the core learning unit.
    pub fn train_definition(&self, facet: &mut Facet, word: &str, definition: &str) {
        let combined = format!("{} {}", word, definition);
        self.train_sentence(facet, &combined);
    }

    /// Recursively learns a word and its definition chain.
    /// For each unknown word: look up its definition, train on it,
    /// then recursively learn any unknown words in that definition.
    /// Stops at max_depth or when all words are known.
    /// Returns the list of words learned (including recursively).
    pub fn learn_definition_chain(
        &self,
        facet: &mut Facet,
        chunk_store: &crate::chunker::ChunkStore,
        word: &str,
        max_depth: usize,
    ) -> Vec<String> {
        let mut learned: Vec<String> = Vec::new();
        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
        self.learn_chain_recursive(facet, chunk_store, word, max_depth, &mut learned, &mut visited);
        learned
    }

    fn learn_chain_recursive(
        &self,
        facet: &mut Facet,
        chunk_store: &crate::chunker::ChunkStore,
        word: &str,
        depth_left: usize,
        learned: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
    ) {
        if depth_left == 0 || visited.contains(word) {
            return;
        }
        visited.insert(word.to_string());

        // If word is already known with high amplitude, skip
        if let Some(phasor) = facet.lexicon.get(word) {
            if phasor.amplitude > 5.0 {
                return;
            }
        }

        // Look up definition
        let definition = match chunk_store.load_definition(word) {
            Some(d) => d,
            None => return,
        };

        // Train on the word-definition pair
        self.train_definition(facet, word, &definition);
        learned.push(word.to_string());

        // Find unknown words in the definition and recurse
        let def_tokens = crate::tokenizer::Tokenizer::tokenize(&definition);
        for token in &def_tokens {
            if !facet.lexicon.contains_key(token) && !visited.contains(token) {
                self.learn_chain_recursive(facet, chunk_store, token, depth_left - 1, learned, visited);
            }
        }
    }

    /// Multi-epoch training with warmup and convergence detection.
    ///
    /// Inspired by Phi-4 finetuning:
    /// - Warmup: gradually increase LR for first `warmup` epochs
    /// - Convergence: stop early if phase shifts become negligible
    /// - Returns metrics including epochs completed and convergence status
    pub fn train_multi_epoch(
        &self,
        facet: &mut Facet,
        text: &str,
        max_epochs: usize,
        warmup: usize,
    ) -> MultiEpochResult {
        let tokens = Tokenizer::tokenize(text);
        if tokens.is_empty() {
            return MultiEpochResult {
                epochs: 0,
                tokens_learned: 0,
                converged: false,
            };
        }

        self.initialize_tokens(facet, &tokens);

        // Record bigram and trigram co-occurrences
        for window in tokens.windows(2) {
            facet.record_bigram(&window[0], &window[1]);
        }
        for window in tokens.windows(3) {
            facet.record_trigram(&window[0], &window[1], &window[2]);
        }

        let mut converged = false;
        let mut epochs_done = 0;

        for epoch in 0..max_epochs {
            let effective_lr = if epoch < warmup {
                self.learning_rate * (epoch as f64 + 1.0) / warmup as f64
            } else {
                self.learning_rate
            };

            let target_phase = self.compute_centroid_phase(facet, &tokens);
            let mut max_shift = 0.0f64;

            for token in &tokens {
                let phasor = facet.lexicon.get_mut(token).unwrap();
                let phase_error = (target_phase - phasor.phase).sin();
                let shift = effective_lr * phase_error;
                phasor.phase = (phasor.phase + shift).rem_euclid(2.0 * PI);
                max_shift = max_shift.max(shift.abs());

                if phase_error.abs() < config::CONVERGENCE_THRESHOLD {
                    phasor.band_n += 1;
                }
                phasor.amplitude = (phasor.amplitude + config::AMPLITUDE_INCREMENT)
                    .min(config::AMPLITUDE_MAX);
            }

            epochs_done = epoch + 1;
            if max_shift < config::CONVERGENCE_THRESHOLD {
                converged = true;
                break;
            }
        }

        MultiEpochResult {
            epochs: epochs_done,
            tokens_learned: tokens.len(),
            converged,
        }
    }
}
