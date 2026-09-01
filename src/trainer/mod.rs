pub mod metrics;
pub use metrics::{TrainingMetrics, MultiEpochResult};

use crate::config::{
    TWO_PI, PHASE_REPULSION,
    CONVERGENCE_THRESHOLD, AMPLITUDE_MAX, AMPLITUDE_INITIAL, BAND_N_INITIAL,
    CORRECTION_FLOOR,
    PHASE_CHANNELS, CHANNELS_PER_UPDATE, NEG_SAMPLES, NEG_RATE,
    HINGE_MARGIN, FUNCTION_WORD_WEIGHT,
};
use crate::facet::Facet;
use crate::phasor::{fnv1a, SpectralPhasor};
use crate::tokenizer::Tokenizer;

/// SplitMix64 — a deterministic mixer used to derive sampling indices.
///
/// Training draws its randomness from the input itself rather than from a
/// mutable generator, so a run is reproducible and `Trainer` stays `Sync`.
#[inline]
fn splitmix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E3779B97F4A7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

/// Shortest signed angular difference, wrapped into (−π, π].
#[inline]
pub fn wrap_signed(delta: f64) -> f64 {
    let d = delta.rem_euclid(TWO_PI);
    if d > std::f64::consts::PI { d - TWO_PI } else { d }
}

/// Trainer — contrastive Kuramoto-Sakaguchi learning on the phase torus.
///
/// Each training step has two halves:
///
/// * **Attraction** — words co-occurring in a sentence are pulled toward the
///   sentence's per-channel centroid, with an asymmetric lag encoding word order.
/// * **Repulsion** — sampled non-co-occurring words are pushed away.
///
/// The second half is not an optimisation. Kuramoto coupling with all-positive
/// coupling has one globally stable attractor — total synchronisation — so an
/// attraction-only rule drives the lexicon to a single point, at which the
/// Kuramoto order parameter (reported as `coherence`) reads 1.0 for every input
/// including noise. The negative term is what makes the fixed point track
/// pointwise mutual information instead.
#[derive(Clone)]
pub struct Trainer {
    /// Kuramoto learning rate - controls how fast phases converge.
    pub learning_rate: f64,
    /// Negative samples drawn per token. Zero disables repulsion.
    pub neg_samples: usize,
    /// When present, negative samples are filtered against it so a word from
    /// the anchor's own definition is never used as a negative. `None` keeps
    /// uniform sampling, which is the control the filter has to beat.
    pub definitions: Option<std::sync::Arc<crate::conception::DefinitionGraph>>,
    /// Mixed into every stochastic decision so a run can be repeated exactly
    /// *and* varied deliberately.
    ///
    /// Training became reproducible when the sample pool stopped depending on
    /// HashMap order, but reproducible is only half of what a measurement needs:
    /// one number from one deterministic run has no error bar, and the effects
    /// now being reported are small enough that the interval decides them.
    /// Varying this and holding everything else fixed is what turns a single
    /// figure into a distribution.
    pub seed: u64,
}

impl Trainer {
    /// Creates a new trainer with the given learning rate.
    pub fn new(learning_rate: f64) -> Self {
        Self { learning_rate, neg_samples: NEG_SAMPLES, definitions: None, seed: 0 }
    }

    /// Attaches a definition graph, enabling controlled negative sampling.
    ///
    /// Without this the filter in [`Trainer::apply_negatives`] is unreachable —
    /// the field stays `None` at every construction site — so calling it is
    /// what makes the mechanism live rather than merely present.
    /// A trainer at an explicit seed.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_definitions(
        mut self,
        graph: std::sync::Arc<crate::conception::DefinitionGraph>,
    ) -> Self {
        self.definitions = Some(graph);
        self
    }

    /// Creates a trainer with repulsion disabled — attraction only.
    /// Retained for ablation studies; not a good default.
    pub fn attraction_only(learning_rate: f64) -> Self {
        Self { learning_rate, neg_samples: 0, definitions: None, seed: 0 }
    }

    /// Trains on a single sentence.
    ///
    /// Steps:
    /// 1. Tokenize; initialize unseen tokens at identity-seeded phases
    /// 2. Record n-gram co-occurrences and directional phase lags
    /// 3. Pull each token toward the per-channel sentence centroid, plus a
    ///    directional syntactic neighbour lag on channel 0
    /// 4. Push sampled negatives away on the same channels
    /// 5. Update log-frequency amplitude; bump `band_n` for converged tokens
    ///
    /// Returns the number of tokens that were updated.
    pub fn train_sentence(&self, facet: &mut Facet, text: &str) -> usize {
        let tokens = Tokenizer::tokenize(text);
        if tokens.is_empty() {
            return 0;
        }

        self.initialize_tokens(facet, &tokens);
        self.record_ngrams(facet, &tokens);

        let n_tokens = tokens.len();
        let step_seed = fnv1a(text) ^ splitmix(self.seed);

        // Which channels this step touches. Updating a rotating subset is
        // dropout-like regularisation and bounds the cost of a token update.
        let ch_offset = (splitmix(step_seed) as usize) % PHASE_CHANNELS;
        let channels: Vec<usize> = (0..CHANNELS_PER_UPDATE.min(PHASE_CHANNELS))
            .map(|i| (ch_offset + i * 7) % PHASE_CHANNELS)
            .collect();

        // Per-channel centroid of the sentence, weighted by amplitude and
        // down-weighting closed-class words. Function words appear in nearly
        // every sentence; at full weight they transitively couple the whole
        // vocabulary into one cluster.
        let mut targets = vec![0.0f64; channels.len()];
        for (ci, &k) in channels.iter().enumerate() {
            let (mut sx, mut sy) = (0.0f64, 0.0f64);
            for t in &tokens {
                if let Some(p) = facet.lexicon.get(t) {
                    let w = p.amplitude * Self::token_weight(t);
                    let th = p.theta(k);
                    sx += w * th.cos();
                    sy += w * th.sin();
                }
            }
            targets[ci] = sy.atan2(sx);
        }

        // Channel-0 snapshot for the asymmetric syntactic term.
        let token_phases: Vec<f64> = tokens
            .iter()
            .map(|t| facet.lexicon.get(t).map(|p| p.theta(0)).unwrap_or(0.0))
            .collect();
        let beta_prev: Vec<f64> = tokens
            .iter()
            .enumerate()
            .map(|(i, t)| if i > 0 { facet.phase_lag(&tokens[i - 1], t) } else { 0.0 })
            .collect();
        let beta_next: Vec<f64> = tokens
            .iter()
            .enumerate()
            .map(|(i, t)| if i + 1 < n_tokens { facet.phase_lag(t, &tokens[i + 1]) } else { 0.0 })
            .collect();

        let mut updated = 0;
        for (i, token) in tokens.iter().enumerate() {
            let phasor = match facet.lexicon.get_mut(token) {
                Some(p) => p,
                None => continue,
            };

            // --- attraction: every touched channel toward its centroid ---
            let mut ch0_semantic = 0.0;
            for (ci, &k) in channels.iter().enumerate() {
                let err = (targets[ci] - phasor.theta(k)).sin();
                if k == 0 {
                    ch0_semantic = err;
                }
                phasor.nudge(k, self.learning_rate * err);
            }

            // --- channel 0 also carries word order (Sakaguchi lag) ---
            let mut syntax_force = 0.0;
            let mut syntax_neighbors = 0;
            if i > 0 {
                syntax_force += (token_phases[i - 1] - phasor.theta(0) + beta_prev[i]).sin();
                syntax_neighbors += 1;
            }
            if i + 1 < n_tokens {
                syntax_force += (token_phases[i + 1] - phasor.theta(0) - beta_next[i]).sin();
                syntax_neighbors += 1;
            }
            if syntax_neighbors > 0 {
                phasor.nudge(0, self.learning_rate * 0.3 * (syntax_force / syntax_neighbors as f64));
            }
            phasor.sync_phase();

            if ch0_semantic.abs() < CONVERGENCE_THRESHOLD {
                phasor.band_n += 1;
            }
            phasor.observe();
            updated += 1;
        }

        self.apply_negatives(facet, &tokens, &channels, step_seed);
        updated
    }

    /// Pushes sampled non-co-occurring words away from the sentence's centroid.
    fn apply_negatives(
        &self,
        facet: &mut Facet,
        tokens: &[String],
        channels: &[usize],
        step_seed: u64,
    ) {
        if self.neg_samples == 0 {
            return;
        }
        facet.rebuild_sample_pool();

        // Collect the (word, channel, delta) triples first: sampling borrows the
        // facet immutably, applying borrows it mutably.
        let mut pushes: Vec<(String, usize, f64)> = Vec::new();
        for (i, token) in tokens.iter().enumerate() {
            let anchor = match facet.lexicon.get(token) {
                Some(p) => *p,
                None => continue,
            };
            for j in 0..self.neg_samples {
                let r = splitmix(step_seed ^ ((i as u64) << 32) ^ (j as u64).wrapping_mul(0x2545F491));
                // Controlled negative sampling: when a definition graph is
                // attached, never draw a word that is definitionally related to
                // the anchor. A uniform draw will occasionally pick a word from
                // the anchor's own definition, and this update then pushes apart
                // exactly the pair the dictionary says belongs together — the
                // training signal working against itself. Dict2vec (Tissier et
                // al., EMNLP 2017) measures the filter firing on ~2% of draws.
                let sampled = match &self.definitions {
                    Some(g) => facet.sample_negative_controlled(r, token, g, 8),
                    None => facet.sample_negative(r),
                };
                let neg = match sampled {
                    Some(w) if !tokens.iter().any(|t| t == w) => w.clone(),
                    _ => continue,
                };
                let negp = match facet.lexicon.get(&neg) {
                    Some(p) => p,
                    None => continue,
                };
                for &k in channels {
                    // gradient of −cos(Δ) — push the negative away from the anchor
                    let away = -(anchor.theta(k) - negp.theta(k)).sin();
                    pushes.push((neg.clone(), k, self.learning_rate * NEG_RATE * away));
                }
            }
        }

        for (word, k, delta) in pushes {
            if let Some(p) = facet.lexicon.get_mut(&word) {
                p.nudge(k, delta);
                if k == 0 {
                    p.sync_phase();
                }
            }
        }
    }

    /// Trains the model to *predict*, not merely to cluster.
    ///
    /// For each position the context wave is compared against the true next word
    /// and against a sampled wrong one. When the wrong word ranks at least as
    /// close, the true word is rotated toward the context and the wrong word
    /// away — a hinge loss on next-word retrieval, applied online with no
    /// backpropagation.
    ///
    /// Centroid attraction is a *descriptive* target ("be like your neighbours")
    /// whose optimum is collapse. Next-word prediction is the objective that
    /// forces a representation to encode syntax, semantics and facts, because
    /// predicting well requires them.
    ///
    /// Returns the number of hinge violations corrected.
    pub fn train_predictive(&self, facet: &mut Facet, text: &str) -> usize {
        let tokens = Tokenizer::tokenize(text);
        if tokens.len() < 2 {
            return 0;
        }
        self.initialize_tokens(facet, &tokens);
        facet.rebuild_sample_pool();

        let step_seed = fnv1a(text) ^ 0xA5A5_5A5A_A5A5_5A5A ^ splitmix(self.seed);
        let ch_offset = (splitmix(step_seed) as usize) % PHASE_CHANNELS;
        let channels: Vec<usize> = (0..CHANNELS_PER_UPDATE.min(PHASE_CHANNELS))
            .map(|i| (ch_offset + i * 7) % PHASE_CHANNELS)
            .collect();

        let mut violations = 0;

        // Running per-channel context accumulators.
        //
        // Recomputing the prefix centroid at every position made this O(L²·D)
        // per sentence, which is why the ranking objective cost 15x what
        // co-occurrence training did. Carrying the sums forward makes it O(L·D)
        // and produces the same context, one token at a time.
        let mut sx = vec![0.0f64; channels.len()];
        let mut sy = vec![0.0f64; channels.len()];

        let mut add_token = |facet: &Facet, sx: &mut Vec<f64>, sy: &mut Vec<f64>, t: &str| {
            if let Some(p) = facet.lexicon.get(t) {
                let w = p.amplitude * Self::token_weight(t);
                for (ci, &k) in channels.iter().enumerate() {
                    let th = p.theta(k);
                    sx[ci] += w * th.cos();
                    sy[ci] += w * th.sin();
                }
            }
        };

        add_token(facet, &mut sx, &mut sy, &tokens[0]);

        for i in 1..tokens.len() {
            let ctx: Vec<f64> = (0..channels.len())
                .map(|ci| sy[ci].atan2(sx[ci]))
                .collect();

            let pos_word = tokens[i].clone();
            let r = splitmix(step_seed ^ (i as u64).wrapping_mul(0x9E3779B9));
            let neg_word = match facet.sample_negative(r) {
                Some(w) if *w != pos_word && !tokens[..i].contains(w) => w.clone(),
                _ => {
                    add_token(facet, &mut sx, &mut sy, &pos_word);
                    continue;
                }
            };

            let (pos_score, neg_score) = {
                let pos = match facet.lexicon.get(&pos_word) { Some(p) => p, None => continue };
                let neg = match facet.lexicon.get(&neg_word) { Some(p) => p, None => continue };
                let mut ps = 0.0;
                let mut ns = 0.0;
                for (ci, &k) in channels.iter().enumerate() {
                    ps += (ctx[ci] - pos.theta(k)).cos();
                    ns += (ctx[ci] - neg.theta(k)).cos();
                }
                (ps / channels.len() as f64, ns / channels.len() as f64)
            };

            // Perceptron-style: only update when the ranking is wrong.
            if neg_score >= pos_score - HINGE_MARGIN {
                violations += 1;

                if let Some(p) = facet.lexicon.get_mut(&pos_word) {
                    for (ci, &k) in channels.iter().enumerate() {
                        p.nudge(k, self.learning_rate * (ctx[ci] - p.theta(k)).sin());
                    }
                    p.sync_phase();
                }
                if let Some(p) = facet.lexicon.get_mut(&neg_word) {
                    for (ci, &k) in channels.iter().enumerate() {
                        p.nudge(k, -self.learning_rate * 0.5 * (ctx[ci] - p.theta(k)).sin());
                    }
                    p.sync_phase();
                }
            }

            add_token(facet, &mut sx, &mut sy, &pos_word);
        }

        violations
    }

    /// One full pass: co-occurrence structure, then predictive ranking.
    ///
    /// Measurement does not favour this. Running both objectives is worse than
    /// running the ranking objective alone on every relational metric — the two
    /// pull the phases in different directions. Prefer [`Trainer::train`].
    pub fn train_full(&self, facet: &mut Facet, text: &str) -> usize {
        let n = self.train_sentence(facet, text);
        self.train_predictive(facet, text);
        n
    }

    /// The recommended training path: record structure, then rank.
    ///
    /// N-gram statistics, phase lags and amplitudes are recorded exactly as
    /// usual, but the manifold is shaped by the **ranking objective only** —
    /// no centroid attraction.
    ///
    /// Three independent measurements support this over
    /// [`Trainer::train_sentence`]:
    ///
    /// * predictive signal recovered rises from 0.9% to 24.3% of what unigram
    ///   frequency provides (`docs/how/RESULTS.md` §3),
    /// * analogy accuracy rises from exactly 0.00% to 0.62%, against a 0.0024%
    ///   chance rate, and mean reciprocal rank from 0.0005 to 0.0120 (§3e),
    /// * phase dispersion holds higher, 0.997 against 0.954, so it collapses
    ///   less.
    ///
    /// Since the context accumulators became incremental it is also the cheaper
    /// path: 5.4s against 10.1s over 12,000 dictionary definitions.
    pub fn train(&self, facet: &mut Facet, text: &str) -> usize {
        let tokens = Tokenizer::tokenize(text);
        if tokens.is_empty() {
            return 0;
        }
        self.initialize_tokens(facet, &tokens);
        self.record_ngrams(facet, &tokens);
        for t in &tokens {
            if let Some(p) = facet.lexicon.get_mut(t) {
                p.observe();
            }
        }
        self.train_predictive(facet, text);
        tokens.len()
    }

    /// Rotates one token toward a target phase, scaled by `weight`.
    ///
    /// The primitive credit assignment needs: reinforce a word in proportion to
    /// how much it contributed, rather than reinforcing every word in a winning
    /// output equally.
    pub fn nudge_token(&self, facet: &mut Facet, word: &str, target: f64, weight: f64) -> bool {
        match facet.lexicon.get_mut(word) {
            Some(p) => {
                let d = (target - p.theta(0)).sin();
                p.nudge(0, self.learning_rate * weight * d);
                p.sync_phase();
                true
            }
            None => false,
        }
    }

    /// Weight of a token when computing a sentence centroid.
    #[inline]
    fn token_weight(token: &str) -> f64 {
        if Tokenizer::is_function_word(token) { FUNCTION_WORD_WEIGHT } else { 1.0 }
    }

    /// Records bigram, trigram and directional phase-lag statistics.
    fn record_ngrams(&self, facet: &mut Facet, tokens: &[String]) {
        for window in tokens.windows(2) {
            facet.record_bigram(&window[0], &window[1]);
            // Anchor the lag to corpus order statistics rather than to the
            // phases the lag itself is moving, so the loop has a fixed point
            // outside its own geometry.
            let target = facet.target_phase_lag(&window[0], &window[1]);
            facet.record_phase_lag(&window[0], &window[1], target);
        }
        for window in tokens.windows(3) {
            facet.record_trigram(&window[0], &window[1], &window[2]);
        }
    }

    /// In-chat self-correction: suppresses a wrong association and reinforces
    /// the corrected one.
    ///
    /// Only tokens that appear in the wrong phrase and *not* in the correction
    /// are pushed: a word present in both (typically a function word) is not the
    /// thing that was wrong, and inverting it degrades the model globally to fix
    /// one specific fact.
    pub fn correct_mistake(&self, facet: &mut Facet, wrong_phrase: &str, correct_phrase: &str) {
        let wrong_tokens = Tokenizer::tokenize(wrong_phrase);
        let correct_tokens = Tokenizer::tokenize(correct_phrase);

        for token in &wrong_tokens {
            if correct_tokens.contains(token) {
                continue;
            }
            if let Some(phasor) = facet.lexicon.get_mut(token) {
                phasor.phase = (phasor.phase + PHASE_REPULSION).rem_euclid(TWO_PI);
                phasor.sync_channel0();
                phasor.amplitude = (phasor.amplitude * 0.8).max(CORRECTION_FLOOR);
            }
        }

        self.train_sentence(facet, correct_phrase);
    }

    /// Graded correction: rotates the offending tokens away from the corrected
    /// meaning by `strength` radians rather than by a full π.
    ///
    /// A π pulse is the maximum possible rotation, so it destroys every *other*
    /// association the word had in order to fix one. Most corrections want a
    /// nudge.
    pub fn correct_graded(
        &self,
        facet: &mut Facet,
        wrong_phrase: &str,
        correct_phrase: &str,
        strength: f64,
    ) {
        let wrong_tokens = Tokenizer::tokenize(wrong_phrase);
        let correct_tokens = Tokenizer::tokenize(correct_phrase);
        let offenders: Vec<String> = wrong_tokens
            .iter()
            .filter(|t| !correct_tokens.contains(t))
            .cloned()
            .collect();

        let target = self.compute_centroid_phase(facet, &correct_tokens);
        for token in &offenders {
            if let Some(p) = facet.lexicon.get_mut(token) {
                let away = -(target - p.theta(0)).sin();
                p.nudge(0, strength * away);
                p.sync_phase();
            }
        }
        self.train_sentence(facet, correct_phrase);
    }

    /// Initializes unseen tokens at identity-seeded phases.
    fn initialize_tokens(&self, facet: &mut Facet, tokens: &[String]) {
        for token in tokens {
            let entry = facet
                .lexicon
                .entry(token.clone())
                .or_insert_with(|| SpectralPhasor::seeded(token, AMPLITUDE_INITIAL, BAND_N_INITIAL));
            entry.ensure_channels(token);
        }
    }

    /// Computes the amplitude-weighted centroid phase across all tokens
    /// (channel 0), down-weighting closed-class words.
    fn compute_centroid_phase(&self, facet: &Facet, tokens: &[String]) -> f64 {
        let (mut sum_x, mut sum_y) = (0.0f64, 0.0f64);
        for token in tokens {
            if let Some(phasor) = facet.lexicon.get(token) {
                let w = phasor.amplitude * Self::token_weight(token);
                sum_x += phasor.theta(0).cos() * w;
                sum_y += phasor.theta(0).sin() * w;
            }
        }
        sum_y.atan2(sum_x)
    }

    /// Trains on a single sentence in online mode (single pass).
    pub fn train_online(&self, facet: &mut Facet, text: &str) -> usize {
        self.train_sentence(facet, text)
    }

    /// Batch-trains a corpus of sentences. Returns total token updates.
    #[allow(dead_code)]
    pub fn train_corpus(&self, facet: &mut Facet, sentences: &[String]) -> usize {
        sentences.iter().map(|s| self.train_sentence(facet, s)).sum()
    }

    /// Trains on a word-definition pair - the core learning unit.
    pub fn train_definition(&self, facet: &mut Facet, word: &str, definition: &str) {
        let combined = format!("{} {}", word, definition);
        self.train_sentence(facet, &combined);
    }

    /// Recursively learns a word and its definition chain.
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

        // Skip words already well known. The previous threshold was 5.0, which
        // AMPLITUDE_MAX (2.0) makes unreachable, so this branch never fired and
        // known words were re-trained on every call.
        if let Some(phasor) = facet.lexicon.get(word) {
            if phasor.amplitude >= AMPLITUDE_MAX * 0.9 {
                return;
            }
        }

        let definition = match chunk_store.load_definition(word) {
            Some(d) => d,
            None => return,
        };

        self.train_definition(facet, word, &definition);
        learned.push(word.to_string());

        let def_tokens = Tokenizer::tokenize(&definition);
        for token in &def_tokens {
            if !facet.lexicon.contains_key(token) && !visited.contains(token) {
                self.learn_chain_recursive(facet, chunk_store, token, depth_left - 1, learned, visited);
            }
        }
    }

    /// Multi-epoch training with warmup and convergence detection.
    pub fn train_multi_epoch(
        &self,
        facet: &mut Facet,
        text: &str,
        max_epochs: usize,
        warmup: usize,
    ) -> MultiEpochResult {
        let tokens = Tokenizer::tokenize(text);
        if tokens.is_empty() {
            return MultiEpochResult { epochs: 0, tokens_learned: 0, converged: false };
        }

        self.initialize_tokens(facet, &tokens);
        self.record_ngrams(facet, &tokens);

        let mut converged = false;
        let mut epochs_done = 0;

        for epoch in 0..max_epochs {
            let effective_lr = if epoch < warmup && warmup > 0 {
                self.learning_rate * (epoch as f64 + 1.0) / warmup as f64
            } else {
                self.learning_rate
            };

            let target_phase = self.compute_centroid_phase(facet, &tokens);
            let mut max_shift = 0.0f64;

            for token in &tokens {
                if let Some(phasor) = facet.lexicon.get_mut(token) {
                    let phase_error = (target_phase - phasor.theta(0)).sin();
                    let shift = effective_lr * phase_error;
                    phasor.nudge(0, shift);
                    phasor.sync_phase();
                    max_shift = max_shift.max(shift.abs());

                    if phase_error.abs() < CONVERGENCE_THRESHOLD {
                        phasor.band_n += 1;
                    }
                    phasor.observe();
                }
            }

            let epoch_trainer = Self { learning_rate: effective_lr, neg_samples: self.neg_samples, definitions: self.definitions.clone(), seed: self.seed };
            let ch: Vec<usize> = (0..CHANNELS_PER_UPDATE.min(PHASE_CHANNELS)).collect();
            epoch_trainer.apply_negatives(facet, &tokens, &ch, fnv1a(text) ^ epoch as u64);

            epochs_done = epoch + 1;
            if max_shift < CONVERGENCE_THRESHOLD {
                converged = true;
                break;
            }
        }

        MultiEpochResult { epochs: epochs_done, tokens_learned: tokens.len(), converged }
    }
}

#[cfg(test)]
mod trainer_tests {
    use super::*;

    /// The defining property of the contrastive rule: repeated training must not
    /// drive the lexicon to a single point.
    #[test]
    fn test_repulsion_prevents_collapse() {
        let sentences = [
            "the cat sat on the mat",
            "the dog ran in the park",
            "the sun set on the sea",
            "rust guarantees memory safety",
            "the borrow checker prevents data races",
        ];

        let mut with_neg = Facet::new();
        let t1 = Trainer::new(0.05);
        for _ in 0..200 {
            for s in &sentences { t1.train_sentence(&mut with_neg, s); }
        }

        let mut without_neg = Facet::new();
        let t2 = Trainer::attraction_only(0.05);
        for _ in 0..200 {
            for s in &sentences { t2.train_sentence(&mut without_neg, s); }
        }

        let d_with = with_neg.phase_dispersion();
        let d_without = without_neg.phase_dispersion();
        assert!(
            d_with > d_without,
            "contrastive training must preserve more dispersion: {} vs {}",
            d_with, d_without
        );
    }

    #[test]
    fn test_correction_spares_shared_function_words() {
        let mut f = Facet::new();
        let t = Trainer::new(0.05);
        t.train_sentence(&mut f, "rust is slow");
        let before = f.lexicon.get("is").map(|p| p.phase).unwrap();
        t.correct_mistake(&mut f, "rust is slow", "rust is fast");
        let after = f.lexicon.get("is").map(|p| p.phase).unwrap();
        // "is" appears in both phrases, so it must not have taken a π pulse
        let moved = wrap_signed(after - before).abs();
        assert!(moved < 1.0, "shared word moved {} rad", moved);
    }

    #[test]
    fn test_predictive_training_runs_and_is_bounded() {
        let mut f = Facet::new();
        let t = Trainer::new(0.05);
        for _ in 0..10 {
            t.train_full(&mut f, "the borrow checker prevents data races");
        }
        assert!(f.vocabulary_size() >= 6);
        for p in f.lexicon.values() {
            assert!(p.phase.is_finite() && p.phase >= 0.0 && p.phase < TWO_PI);
            assert!(p.amplitude >= 1.0 && p.amplitude <= AMPLITUDE_MAX);
        }
    }

    #[test]
    fn test_same_length_words_diverge() {
        let mut f = Facet::new();
        let t = Trainer::new(0.05);
        t.train_sentence(&mut f, "cat dog war");
        let c = f.lexicon.get("cat").unwrap().phase;
        let d = f.lexicon.get("dog").unwrap().phase;
        assert!(wrap_signed(c - d).abs() > 0.01, "identity seeding must separate same-length words");
    }
}
