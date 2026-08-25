pub mod metrics;
pub use metrics::{TrainingMetrics, MultiEpochResult};

use crate::config::{
    PHI, TWO_PI, PHASE_REPULSION,
    CONVERGENCE_THRESHOLD, AMPLITUDE_INCREMENT, AMPLITUDE_MAX,
    AMPLITUDE_INITIAL, BAND_N_INITIAL,
};
use crate::facet::Facet;
use crate::phasor::SpectralPhasor;
use crate::tokenizer::Tokenizer;

/// Trainer - unsupervised language learning via Kuramoto-Sakaguchi phase attraction.
///
/// Words that co-occur in a sentence get their phase angles pulled toward
/// the sentence's centroid phase with an asymmetric phase lag (beta) to encode
/// forward syntactic temporal direction.
#[derive(Clone)]
pub struct Trainer {
    /// Kuramoto learning rate - controls how fast phases converge.
    pub learning_rate: f64,
}

impl Trainer {
    /// Creates a new trainer with the given learning rate.
    pub fn new(learning_rate: f64) -> Self {
        Self { learning_rate }
    }

    /// Trains on a single sentence using Kuramoto-Sakaguchi asymmetric phase coupling.
    ///
    /// Steps:
    /// 1. Tokenize text; initialize unseen tokens at deterministic pseudo-random phases
    /// 2. Record n-gram co-occurrences for sequence modeling
    /// 3. Compute semantic centroid phase from all token phasors
    /// 4. Shift each token's phase toward centroid + directional syntactic neighbor lag
    /// 5. Bump `band_n` for tokens already close (prevents phase collapse)
    ///
    /// Returns the number of tokens that were updated.
    pub fn train_sentence(&self, facet: &mut Facet, text: &str) -> usize {
        let tokens = Tokenizer::tokenize(text);
        match tokens.is_empty() {
            true => return 0,
            false => {}
        }

        self.initialize_tokens(facet, &tokens);

        // Record bigram, trigram, and learned syntactic phase lags
        for window in tokens.windows(2) {
            facet.record_bigram(&window[0], &window[1]);
            if let (Some(p0), Some(p1)) = (facet.lexicon.get(&window[0]), facet.lexicon.get(&window[1])) {
                let observed_lag = (p1.phase - p0.phase).rem_euclid(TWO_PI);
                facet.record_phase_lag(&window[0], &window[1], observed_lag);
            }
        }
        for window in tokens.windows(3) {
            facet.record_trigram(&window[0], &window[1], &window[2]);
        }

        let target_phase = self.compute_centroid_phase(facet, &tokens);
        let n_tokens = tokens.len();

        // Capture snapshot of current token phases and learned β_ij for asymmetric neighbor coupling
        let token_phases: Vec<f64> = tokens.iter()
            .map(|t| facet.lexicon.get(t).map(|p| p.phase).unwrap_or(0.0))
            .collect();
        let beta_prev: Vec<f64> = tokens.iter().enumerate().map(|(i, t)| {
            match i > 0 {
                true => facet.phase_lag(&tokens[i - 1], t),
                false => 0.0,
            }
        }).collect();
        let beta_next: Vec<f64> = tokens.iter().enumerate().map(|(i, t)| {
            match i + 1 < n_tokens {
                true => facet.phase_lag(t, &tokens[i + 1]),
                false => 0.0,
            }
        }).collect();

        let mut updated = 0;
        for (i, token) in tokens.iter().enumerate() {
            let phasor = facet.lexicon.get_mut(token).unwrap();
            let semantic_force = (target_phase - phasor.phase).sin();

            // Compute directional syntactic lag from preceding and subsequent words
            let mut syntax_force = 0.0;
            let mut syntax_neighbors = 0;

            match i > 0 {
                true => {
                    let prev_phase = token_phases[i - 1];
                    syntax_force += (prev_phase - phasor.phase + beta_prev[i]).sin();
                    syntax_neighbors += 1;
                }
                false => {}
            }
            match i + 1 < n_tokens {
                true => {
                    let next_phase = token_phases[i + 1];
                    syntax_force += (next_phase - phasor.phase - beta_next[i]).sin();
                    syntax_neighbors += 1;
                }
                false => {}
            }

            let combined_error = match syntax_neighbors > 0 {
                true => 0.7 * semantic_force + 0.3 * (syntax_force / syntax_neighbors as f64),
                false => semantic_force,
            };

            phasor.phase = (phasor.phase + self.learning_rate * combined_error)
                .rem_euclid(TWO_PI);

            match semantic_force.abs() < CONVERGENCE_THRESHOLD {
                true => phasor.band_n += 1,
                false => {}
            }

            phasor.amplitude = (phasor.amplitude + AMPLITUDE_INCREMENT).min(AMPLITUDE_MAX);
            updated += 1;
        }

        updated
    }

    /// In-chat real-time self-correction: applies an instantaneous anti-phase pulse (π radians)
    /// to suppress erroneous associations and aligns the corrected target.
    pub fn correct_mistake(&self, facet: &mut Facet, wrong_phrase: &str, correct_phrase: &str) {
        let wrong_tokens = Tokenizer::tokenize(wrong_phrase);

        for token in &wrong_tokens {
            match facet.lexicon.get_mut(token) {
                Some(phasor) => {
                    phasor.phase = (phasor.phase + PHASE_REPULSION).rem_euclid(TWO_PI);
                    phasor.amplitude = (phasor.amplitude * 0.8).max(AMPLITUDE_INITIAL);
                }
                None => {}
            }
        }
        
        // 2. Train and reinforce the correct phrase
        self.train_sentence(facet, correct_phrase);
    }

    /// Initializes unseen tokens at deterministic pseudo-random phases.
    fn initialize_tokens(&self, facet: &mut Facet, tokens: &[String]) {
        for token in tokens {
            facet.lexicon.entry(token.clone()).or_insert_with(|| {
                let seed_phase = (token.len() as f64 * PHI).rem_euclid(TWO_PI);
                SpectralPhasor::new(seed_phase, AMPLITUDE_INITIAL, BAND_N_INITIAL)
            });
        }
    }

    /// Computes the amplitude-weighted centroid phase across all tokens.
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

    /// Batch-trains a corpus of sentences. Returns total token updates.
    pub fn train_corpus(&self, facet: &mut Facet, sentences: &[String]) -> usize {
        sentences.iter().map(|s| self.train_sentence(facet, s)).sum()
    }

    /// Trains on a word-definition pair - the core learning unit.
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
        match depth_left == 0 || visited.contains(word) {
            true => return,
            false => {}
        }
        visited.insert(word.to_string());

        match facet.lexicon.get(word) {
            Some(phasor) if phasor.amplitude > 5.0 => return,
            _ => {}
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
            match facet.lexicon.contains_key(token) || visited.contains(token) {
                true => {}
                false => self.learn_chain_recursive(facet, chunk_store, token, depth_left - 1, learned, visited),
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
        match tokens.is_empty() {
            true => return MultiEpochResult {
                epochs: 0,
                tokens_learned: 0,
                converged: false,
            },
            false => {}
        }

        self.initialize_tokens(facet, &tokens);

        // Record bigram and trigram co-occurrences
        for window in tokens.windows(2) {
            facet.record_bigram(&window[0], &window[1]);
            let prev_phase = facet.lexicon.get(&window[0]).map(|p| p.phase).unwrap_or(0.0);
            let curr_phase = facet.lexicon.get(&window[1]).map(|p| p.phase).unwrap_or(0.0);
            let observed_lag = (curr_phase - prev_phase).rem_euclid(TWO_PI);
            facet.record_phase_lag(&window[0], &window[1], observed_lag);
        }
        for window in tokens.windows(3) {
            facet.record_trigram(&window[0], &window[1], &window[2]);
        }

        let mut converged = false;
        let mut epochs_done = 0;

        for epoch in 0..max_epochs {
            let effective_lr = match epoch < warmup {
                true => self.learning_rate * (epoch as f64 + 1.0) / warmup as f64,
                false => self.learning_rate,
            };

            let target_phase = self.compute_centroid_phase(facet, &tokens);
            let mut max_shift = 0.0f64;

            for token in &tokens {
                let phasor = facet.lexicon.get_mut(token).unwrap();
                let phase_error = (target_phase - phasor.phase).sin();
                let shift = effective_lr * phase_error;
                phasor.phase = (phasor.phase + shift).rem_euclid(TWO_PI);
                max_shift = max_shift.max(shift.abs());

                match phase_error.abs() < CONVERGENCE_THRESHOLD {
                    true => phasor.band_n += 1,
                    false => {}
                }
                phasor.amplitude = (phasor.amplitude + AMPLITUDE_INCREMENT)
                    .min(AMPLITUDE_MAX);
            }

            epochs_done = epoch + 1;
            match max_shift < CONVERGENCE_THRESHOLD {
                true => {
                    converged = true;
                    break;
                }
                false => {}
            }
        }

        MultiEpochResult {
            epochs: epochs_done,
            tokens_learned: tokens.len(),
            converged,
        }
    }
}
