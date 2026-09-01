//! The evaluation harness.
//!
//! Every claim about this architecture is a hypothesis until it is measured
//! against held-out data. This module supplies the measurement:
//!
//! * a deterministic 80/10/10 split with no leakage,
//! * a Kneser-Ney trigram baseline to beat,
//! * held-out perplexity for the model itself,
//! * and, logged beside them, the two diagnostics that distinguish learning
//!   from collapse — phase dispersion and sector Gini.
//!
//! Coherence is deliberately *not* the headline number. It is the Kuramoto
//! order parameter, which the training rule maximises and which reads 1.0 on a
//! fully synchronised lexicon for any input, including noise. It can rise while
//! the model gets worse. Perplexity cannot.

use crate::eval::Evaluator;
use crate::facet::Facet;
use crate::metrics::kn_baseline::KneserNey;
use crate::phasor::fnv1a;
use crate::tokenizer::Tokenizer;
use crate::trainer::Trainer;
use rayon::prelude::*;
use serde::Serialize;

/// A deterministic corpus split. `test` is read once, at the end.
#[derive(Debug, Clone, Default)]
pub struct Split {
    pub train: Vec<String>,
    pub valid: Vec<String>,
    pub test: Vec<String>,
}

/// Per-epoch record. Written to `data/evaluation.json`.
#[derive(Debug, Clone, Serialize)]
pub struct EpochMetrics {
    pub epoch: usize,
    pub train_ppl: f64,
    /// The number that matters: perplexity on text never trained on.
    pub valid_ppl: f64,
    /// Same model, same counts, unigram back-off instead of the phase manifold.
    /// The gap to `valid_ppl` is what the phase layer contributes, per epoch.
    pub valid_ppl_no_phase: f64,
    /// Kept for continuity, never used alone.
    pub coherence: f64,
    /// 1.0 = phases spread uniformly, 0.0 = fully collapsed.
    pub phase_dispersion: f64,
    /// The same over the frequent band. Collapse shows up here epochs before the
    /// global figure moves, because the global figure is diluted by the tail.
    pub band_dispersion: f64,
    /// Sector-occupancy inequality; rises as the manifold concentrates.
    pub sector_gini: f64,
    pub vocab_size: usize,
    pub mean_amplitude: f64,
}

/// Final comparison across the four configurations that isolate each component.
#[derive(Debug, Clone, Serialize)]
pub struct HarnessReport {
    pub kn_trigram_ppl: f64,
    pub phiano_best_valid_ppl: f64,
    pub phiano_final_valid_ppl: f64,
    pub best_epoch: usize,
    pub attraction_only_best_ppl: f64,
    /// Same counts, same interpolation, unigram back-off instead of phase.
    pub no_phase_backoff_ppl: f64,
    /// (gamma, perplexity) over the mixing grid, at the best epoch.
    pub gamma_sweep: Vec<(f64, f64)>,
    /// The mixing weight that minimised held-out perplexity.
    pub best_gamma: f64,
    /// Same sweep, on a facet trained with extra predictive passes. Answers
    /// whether the predictive objective is what makes the manifold contribute.
    pub gamma_sweep_predictive: Vec<(f64, f64)>,
    pub best_gamma_predictive: f64,
    pub verdict: String,
    pub epochs: Vec<EpochMetrics>,
}

pub struct Harness;

impl Harness {
    /// Splits a corpus 80/10/10 deterministically.
    ///
    /// The shuffle is a hash-order sort, so the split is identical on every
    /// machine and every run — a split that moves between runs cannot support a
    /// comparison.
    pub fn split(sentences: Vec<String>, seed: u64) -> Split {
        let mut keyed: Vec<(u64, String)> = sentences
            .into_iter()
            .map(|s| (fnv1a(&s) ^ seed, s))
            .collect();
        keyed.sort_by_key(|(k, _)| *k);
        let s: Vec<String> = keyed.into_iter().map(|(_, v)| v).collect();

        let n = s.len();
        Split {
            train: s[..n * 80 / 100].to_vec(),
            valid: s[n * 80 / 100..n * 90 / 100].to_vec(),
            test: s[n * 90 / 100..].to_vec(),
        }
    }

    /// Builds a scoring model over a facet.
    pub fn language_model(facet: &Facet, use_phase: bool) -> PhianoLM<'_> {
        PhianoLM::new(facet, use_phase)
    }

    /// Held-out perplexity of the Phiano model, using the phase manifold as the
    /// back-off distribution.
    pub fn perplexity(facet: &Facet, sentences: &[String]) -> f64 {
        PhianoLM::new(facet, true).perplexity(sentences)
    }

    /// Held-out perplexity with the manifold replaced by a plain unigram
    /// back-off — identical counts, identical interpolation, phase removed.
    /// The difference between the two is the phase layer's contribution.
    pub fn perplexity_no_phase(facet: &Facet, sentences: &[String]) -> f64 {
        PhianoLM::new(facet, false).perplexity(sentences)
    }

    /// Mean coherence over a set of sentences.
    fn mean_coherence(facet: &Facet, sentences: &[String]) -> f64 {
        if sentences.is_empty() {
            return 0.0;
        }
        let e = Evaluator::new();
        sentences.iter().map(|s| e.eval(facet, s).coherence).sum::<f64>() / sentences.len() as f64
    }

    /// Trains a facet with a clustering pass followed by several predictive
    /// passes per epoch.
    ///
    /// Centroid attraction teaches the manifold *what co-occurs*. Next-word
    /// ranking teaches it *what follows*. Only the second is the thing a
    /// language model is scored on, so this isolates its effect.
    pub fn train_predictive_heavy(
        split: &Split,
        trainer: &Trainer,
        epochs: usize,
        predictive_passes: usize,
    ) -> Facet {
        let mut facet = Facet::new();
        for _ in 0..epochs {
            for s in &split.train {
                trainer.train_sentence(&mut facet, s);
                for _ in 0..predictive_passes {
                    trainer.train_predictive(&mut facet, s);
                }
            }
        }
        facet
    }

    /// Trains the phase layer on the ranking objective alone.
    ///
    /// Centroid attraction teaches the manifold what co-occurs; next-word
    /// ranking teaches it what follows. Only the second is what a language model
    /// is scored on, and running both pulls the phases in two directions at
    /// once. This keeps the n-gram statistics (recorded either way) but trains
    /// the manifold on the ranking objective only.
    pub fn train_ranking_only(split: &Split, trainer: &Trainer, passes: usize) -> Facet {
        let mut facet = Facet::new();
        // One structural pass to populate n-grams and initialise the lexicon.
        let seed = Trainer { learning_rate: 0.0, neg_samples: 0, definitions: None, seed: trainer.seed };
        for s in &split.train {
            seed.train_sentence(&mut facet, s);
        }
        for _ in 0..passes {
            for s in &split.train {
                trainer.train_predictive(&mut facet, s);
            }
        }
        facet
    }

    /// Trains for `epochs` and records the full metric set each epoch.
    pub fn train_and_measure(
        split: &Split,
        trainer: &Trainer,
        epochs: usize,
        predictive: bool,
    ) -> (Facet, Vec<EpochMetrics>) {
        let mut facet = Facet::new();
        let mut log = Vec::with_capacity(epochs);

        for epoch in 0..epochs {
            for s in &split.train {
                match predictive {
                    true => { trainer.train_full(&mut facet, s); }
                    false => { trainer.train_sentence(&mut facet, s); }
                }
            }

            log.push(EpochMetrics {
                epoch,
                train_ppl: Self::perplexity(&facet, &split.train),
                valid_ppl: Self::perplexity(&facet, &split.valid),
                valid_ppl_no_phase: Self::perplexity_no_phase(&facet, &split.valid),
                coherence: Self::mean_coherence(&facet, &split.valid),
                phase_dispersion: facet.phase_dispersion(),
                band_dispersion: facet
                    .dispersion_top(crate::cognitive::grounding::GUARD_BAND_TOP),
                sector_gini: facet.sector_gini(),
                vocab_size: facet.vocabulary_size(),
                mean_amplitude: facet.average_amplitude(),
            });
        }

        (facet, log)
    }

    /// Runs the full comparison and returns a report.
    pub fn run(corpus: Vec<String>, epochs: usize) -> HarnessReport {
        let split = Self::split(corpus, 42);

        let kn = KneserNey::train(&split.train);
        let kn_ppl = kn.perplexity(&split.valid);

        let contrastive = Trainer::new(crate::config::LEARNING_RATE);
        let (trained, log) = Self::train_and_measure(&split, &contrastive, epochs, true);
        let no_phase = Self::perplexity_no_phase(&trained, &split.valid);

        let attraction = Trainer::attraction_only(crate::config::LEARNING_RATE);
        let (_, log_attr) = Self::train_and_measure(&split, &attraction, epochs, false);

        let (best_epoch, best) = log
            .iter()
            .enumerate()
            .min_by(|a, b| {
                a.1.valid_ppl
                    .partial_cmp(&b.1.valid_ppl)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, m)| (i, m.valid_ppl))
            .unwrap_or((0, f64::INFINITY));

        let attr_best = log_attr
            .iter()
            .map(|m| m.valid_ppl)
            .fold(f64::INFINITY, f64::min);

        let final_ppl = log.last().map(|m| m.valid_ppl).unwrap_or(f64::INFINITY);

        // Sweep the mixing weight on a facet trained for the best number of
        // epochs, so the contribution is measured where the model is strongest.
        let (best_facet, _) =
            Self::train_and_measure(&split, &contrastive, best_epoch + 1, true);
        let sweep = PhianoLM::with_gamma(&best_facet, 1.0).gamma_sweep(&split.valid);
        let best_gamma = sweep
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(g, _)| *g)
            .unwrap_or(0.0);

        let pred_facet = Self::train_predictive_heavy(&split, &contrastive, best_epoch + 1, 4);
        let sweep_pred = PhianoLM::with_gamma(&pred_facet, 1.0).gamma_sweep(&split.valid);
        let best_gamma_pred = sweep_pred
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(g, _)| *g)
            .unwrap_or(0.0);

        let best_m = &log[best_epoch];
        let phase_helps = best_m.valid_ppl < best_m.valid_ppl_no_phase;

        let verdict = match (best < kn_ppl, phase_helps) {
            (true, true) => format!(
                "Phiano beats Kneser-Ney ({:.2} vs {:.2}), and the phase manifold \
                 beats a unigram back-off on the same counts ({:.2} vs {:.2}).",
                best, kn_ppl, best_m.valid_ppl, best_m.valid_ppl_no_phase
            ),
            (false, true) => format!(
                "Phiano does not yet beat Kneser-Ney ({:.2} vs {:.2}), but the phase \
                 manifold DOES beat a unigram back-off on identical counts \
                 ({:.2} vs {:.2}) — the manifold is contributing; the n-gram \
                 smoothing is what lags.",
                best, kn_ppl, best_m.valid_ppl, best_m.valid_ppl_no_phase
            ),
            (_, false) => format!(
                "Phiano does not beat Kneser-Ney ({:.2} vs {:.2}), and the phase \
                 manifold does not beat a unigram back-off ({:.2} vs {:.2}). \
                 The phase layer is not paying for itself at this epoch.",
                best, kn_ppl, best_m.valid_ppl, best_m.valid_ppl_no_phase
            ),
        };

        HarnessReport {
            kn_trigram_ppl: kn_ppl,
            phiano_best_valid_ppl: best,
            phiano_final_valid_ppl: final_ppl,
            best_epoch,
            attraction_only_best_ppl: attr_best,
            no_phase_backoff_ppl: no_phase,
            gamma_sweep: sweep,
            best_gamma,
            gamma_sweep_predictive: sweep_pred,
            best_gamma_predictive: best_gamma_pred,
            verdict,
            epochs: log,
        }
    }
}


/// One cell of the experiment grid.
#[derive(Debug, Clone, Serialize)]
pub struct SweepRow {
    /// How the context vector was built: "2-word" or "recurrent".
    pub context: String,
    /// Softmax inverse temperature of the phase distribution.
    pub beta: f64,
    /// Mixing weight between the phase back-off and a unigram back-off.
    pub gamma: f64,
    pub ppl: f64,
}

/// How the context vector is built from the prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextKind {
    /// Recency-weighted sum of the two preceding words. Order enters only as a
    /// magnitude weight, so a swap barely moves the vector.
    TwoWord,
    /// The same two words, each rotated by its offset from the prediction point.
    /// Position is carried in the phase, so a swap is a different context.
    Bound,
    /// Diagonal complex recurrence over the whole prefix, with a per-channel
    /// timescale. Order enters through the rotation kernel.
    Recurrent,
}

impl ContextKind {
    pub fn label(self) -> &'static str {
        match self {
            ContextKind::TwoWord => "2-word",
            ContextKind::Bound => "bound",
            ContextKind::Recurrent => "recurrent",
        }
    }

    pub fn is_recurrent(self) -> bool {
        matches!(self, ContextKind::Recurrent)
    }

    pub const ALL: [ContextKind; 3] =
        [ContextKind::TwoWord, ContextKind::Bound, ContextKind::Recurrent];
}

/// Channels used by the language model's phase back-off.
///
/// The full torus is 64 channels; scoring every position against all of them is
/// affordable but wasteful, and 16 already captures far more than the single
/// angle a 1-D representation could offer.
const LM_CHANNELS: usize = 16;

/// Inverse temperature of the phase back-off distribution.
///
/// Scores are the *mean* cosine across channels, so they live in [-1, 1] and
/// beta is a real temperature. With a per-channel sum the scale was ~16× wider,
/// the softmax saturated, and every non-top candidate underflowed the
/// probability floor — which made the phase back-off a constant and guaranteed
/// it lost to a unigram at every mixing weight.
const PHASE_BETA: f64 = 8.0;

/// Absolute discount for Phiano's own n-gram tables.
const DISCOUNT: f64 = 0.75;

/// Mixing weights swept when measuring the phase layer's contribution.
pub const GAMMA_GRID: [f64; 11] =
    [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

/// A proper language model over a trained facet.
///
/// Phiano's bigram and trigram tables are maximum-likelihood estimates: an
/// unseen n-gram has probability exactly zero, which makes held-out perplexity
/// infinite and the model brittle off-distribution. This wraps them in absolute
/// discounting with a hierarchical back-off:
///
/// ```text
/// P(c | a,b) = max(C(abc)−D,0)/C(ab) + λ(ab)·P(c | b)
/// P(c | b)   = max(C(bc)−D,0)/C(b)   + λ(b)·P_base(c | ctx)
/// P_base     = γ·P_phase(c | ctx) + (1−γ)·P_unigram(c)
/// ```
///
/// `P_phase` is a softmax over **mean phase coherence across `LM_CHANNELS`
/// channels** — the similarity the torus representation exists for, rather than
/// a single angle.
///
/// Because γ = 0 removes the manifold entirely while changing nothing else,
/// sweeping γ measures exactly what the phase layer contributes. If the best γ
/// is 0, it contributes nothing; if the best γ is positive, the gap is the
/// contribution, in perplexity, on held-out text.
pub struct PhianoLM<'a> {
    facet: &'a Facet,
    index: std::collections::HashMap<String, usize>,
    /// Flattened V × (2·LM_CHANNELS): cos and sin of every channel angle.
    vecs: Vec<f64>,
    unigram: Vec<f64>,
    gamma: f64,
    /// Target sector of every vocabulary item, on channel 0. Precomputed so the
    /// readout lookup inside the scoring loop is one indexed read.
    sectors: Vec<usize>,
    /// The conditional non-linear correction, once one has been fitted.
    readout: Option<crate::nonlinear::SectorReadout>,
}

/// Weight of the readout's bias against the raw coherence score.
///
/// Coherence dots live in [-1, 1] and the bias is clamped to [-2, 2]. Weight 1
/// would let the table dominate the manifold outright, which would measure the
/// table rather than the combination.
const READOUT_WEIGHT: f64 = 0.25;

impl<'a> PhianoLM<'a> {
    pub fn new(facet: &'a Facet, use_phase: bool) -> Self {
        Self::with_gamma(facet, if use_phase { 1.0 } else { 0.0 })
    }

    pub fn with_gamma(facet: &'a Facet, gamma: f64) -> Self {
        let n = facet.lexicon.len();
        let mut index = std::collections::HashMap::with_capacity(n);
        let mut vecs = Vec::with_capacity(n * 2 * LM_CHANNELS);
        let mut counts = Vec::with_capacity(n);
        let mut sectors = Vec::with_capacity(n);

        // Sorted, so the index a word receives does not depend on HashMap
        // iteration order. The readout's negative sampling draws by index, so a
        // shifting index made readout fitting differ between runs on identical
        // data.
        let mut words: Vec<(&String, &crate::phasor::SpectralPhasor)> =
            facet.lexicon.iter().collect();
        words.sort_unstable_by(|a, b| a.0.cmp(b.0));

        for (w, p) in words {
            index.insert(w.clone(), counts.len());
            for k in 0..LM_CHANNELS {
                let t = p.theta(k);
                vecs.push(t.cos());
                vecs.push(t.sin());
            }
            sectors.push(crate::nonlinear::SectorReadout::target_sector(p.theta(0)));
            counts.push(p.count.max(1) as f64);
        }

        let total: f64 = counts.iter().sum();
        let unigram: Vec<f64> = counts
            .iter()
            .map(|c| if total > 0.0 { c / total } else { 0.0 })
            .collect();

        Self { facet, index, vecs, unigram, gamma, sectors, readout: None }
    }

    /// Angles of a context vector, for keying the readout table.
    fn ctx_angles(ctx: &[f64]) -> Vec<f64> {
        (0..LM_CHANNELS)
            .map(|k| ctx[2 * k + 1].atan2(ctx[2 * k]))
            .collect()
    }

    /// Fits the conditional readout on training text.
    ///
    /// For every trigram position the context cell is rewarded for the sector
    /// the true next word occupies and penalised for the sector of a frequency
    /// sampled negative — the same contrastive shape the trainer uses, applied
    /// to the discretised readout instead of to the phases.
    ///
    /// Deliberately fitted on `train` only. Fitting a lookup table on the same
    /// text it is scored against would measure memorisation.
    pub fn fit_readout(&mut self, sentences: &[String], lr: f64, recurrent: bool) {
        self.fit_readout_shaped(
            sentences,
            lr,
            recurrent,
            crate::nonlinear::KEY_CHANNELS,
            crate::nonlinear::KEY_SECTORS,
        );
    }

    /// As [`PhianoLM::fit_readout`], at an explicit key resolution.
    ///
    /// The resolution is the whole trade-off. A fine key distinguishes more
    /// contexts but misses on almost every held-out one, and a table that never
    /// hits cannot change a held-out score however well it fits the training
    /// split. Sweeping it is how that gets measured rather than assumed.
    pub fn fit_readout_shaped(
        &mut self,
        sentences: &[String],
        lr: f64,
        recurrent: bool,
        key_channels: usize,
        key_sectors: usize,
    ) {
        let mut table = crate::nonlinear::SectorReadout::with_shape(key_channels, key_sectors);
        let mut rng: u64 = 0x9E3779B97F4A7C15;
        let v = self.unigram.len();
        if v == 0 {
            return;
        }

        for sentence in sentences {
            let toks = Tokenizer::tokenize(sentence);
            if toks.len() < 3 {
                continue;
            }
            let mut h = vec![crate::wave::c64::new(0.0, 0.0); LM_CHANNELS];
            if recurrent {
                self.advance(&mut h, &toks[0]);
                self.advance(&mut h, &toks[1]);
            }
            for i in 2..toks.len() {
                let (a, b, c) = (&toks[i - 2], &toks[i - 1], &toks[i]);
                let idx = match self.index.get(c) {
                    Some(x) => *x,
                    None => {
                        if recurrent {
                            self.advance(&mut h, c);
                        }
                        continue;
                    }
                };
                let ctx = match recurrent {
                    true => Self::state_to_ctx(&h),
                    false => match self.context_vec(a, b) {
                        Some(x) => x,
                        None => {
                            if recurrent {
                                self.advance(&mut h, c);
                            }
                            continue;
                        }
                    },
                };
                // xorshift64*, so the negative sample is deterministic per run.
                rng ^= rng << 13;
                rng ^= rng >> 7;
                rng ^= rng << 17;
                let neg = (rng % v as u64) as usize;

                let key = table.key(&Self::ctx_angles(&ctx));
                table.learn(key, self.sectors[idx], self.sectors[neg], lr);

                if recurrent {
                    self.advance(&mut h, c);
                }
            }
        }
        self.readout = Some(table);
    }

    /// Number of context cells the fitted readout holds; 0 when there is none.
    pub fn readout_cells(&self) -> usize {
        self.readout.as_ref().map(|r| r.cells()).unwrap_or(0)
    }

    /// Fraction of readout lookups since the last reset that hit a fitted cell.
    pub fn readout_coverage(&self) -> f64 {
        self.readout.as_ref().map(|r| r.coverage()).unwrap_or(0.0)
    }

    pub fn reset_readout_coverage(&self) {
        if let Some(r) = &self.readout {
            r.reset_coverage();
        }
    }

    /// Discards the fitted readout, so the same model can be scored both ways.
    pub fn clear_readout(&mut self) {
        self.readout = None;
    }

    /// Per-channel context direction from the two preceding words.
    fn context_vec(&self, a: &str, b: &str) -> Option<Vec<f64>> {
        let mut acc = vec![0.0f64; 2 * LM_CHANNELS];
        let mut any = false;
        for (w, weight) in [(a, 0.4f64), (b, 1.0f64)] {
            if let Some(p) = self.facet.lexicon.get(w) {
                any = true;
                let m = p.amplitude * weight;
                for k in 0..LM_CHANNELS {
                    let t = p.theta(k);
                    acc[2 * k] += m * t.cos();
                    acc[2 * k + 1] += m * t.sin();
                }
            }
        }
        if !any {
            return None;
        }
        // Normalise each channel back to the unit circle.
        for k in 0..LM_CHANNELS {
            let (x, y) = (acc[2 * k], acc[2 * k + 1]);
            let n = x.hypot(y);
            if n > 1e-12 {
                acc[2 * k] = x / n;
                acc[2 * k + 1] = y / n;
            } else {
                acc[2 * k] = 1.0;
                acc[2 * k + 1] = 0.0;
            }
        }
        Some(acc)
    }

    /// Order-bound context vector.
    ///
    /// [`PhianoLM::context_vec`] weights the two context words 0.4 and 1.0 and
    /// sums them. That is a recency weighting, not an encoding of order: the
    /// two words land in the same superposition, and swapping them changes only
    /// their magnitudes. "dog bites man" and "man bites dog" are near-identical
    /// contexts under it.
    ///
    /// Here each word is *rotated* by its offset from the prediction point
    /// before it is summed — position −2 by −2·φ, position −1 by −1·φ, where φ
    /// is the golden angle. Rotation by an irrational multiple never collides
    /// across positions, so the sum is a binding rather than a blur, and it is
    /// relative to the target, so the encoding does not drift with sentence
    /// length.
    ///
    /// This is the same construction [`crate::wave::Wave::sentence_channels`]
    /// uses with `bound = true`. It was in the codebase and not in the scoring
    /// path.
    fn context_vec_bound(&self, a: &str, b: &str) -> Option<Vec<f64>> {
        let mut acc = vec![0.0f64; 2 * LM_CHANNELS];
        let mut any = false;
        for (w, offset) in [(a, -2.0f64), (b, -1.0f64)] {
            if let Some(p) = self.facet.lexicon.get(w) {
                any = true;
                let roll = offset * crate::config::GOLDEN_ANGLE;
                for k in 0..LM_CHANNELS {
                    let t = p.theta(k) + roll;
                    acc[2 * k] += p.amplitude * t.cos();
                    acc[2 * k + 1] += p.amplitude * t.sin();
                }
            }
        }
        if !any {
            return None;
        }
        for k in 0..LM_CHANNELS {
            let (x, y) = (acc[2 * k], acc[2 * k + 1]);
            let n = x.hypot(y);
            if n > 1e-12 {
                acc[2 * k] = x / n;
                acc[2 * k + 1] = y / n;
            } else {
                acc[2 * k] = 1.0;
                acc[2 * k + 1] = 0.0;
            }
        }
        Some(acc)
    }

    /// How much a context construction changes when the two words are swapped.
    ///
    /// Returns mean cosine similarity between `ctx(a, b)` and `ctx(b, a)` over
    /// the given pairs, averaged across channels. 1.0 means order is not
    /// represented at all; lower means a swap produces a different context.
    ///
    /// This is the diagnostic that separates "order is encoded" from "order is
    /// weighted": a recency weight leaves the two words in the same
    /// superposition and scores near 1.0 however the weights are set.
    pub fn order_sensitivity(&self, pairs: &[(String, String)], kind: ContextKind) -> f64 {
        let build = |a: &str, b: &str| match kind {
            ContextKind::Bound => self.context_vec_bound(a, b),
            _ => self.context_vec(a, b),
        };
        let mut sum = 0.0;
        let mut n = 0usize;
        for (a, b) in pairs {
            if let (Some(x), Some(y)) = (build(a, b), build(b, a)) {
                let mut c = 0.0;
                for k in 0..LM_CHANNELS {
                    c += x[2 * k] * y[2 * k] + x[2 * k + 1] * y[2 * k + 1];
                }
                sum += c / LM_CHANNELS as f64;
                n += 1;
            }
        }
        match n {
            0 => 1.0,
            _ => sum / n as f64,
        }
    }

    /// Softmax over the vocabulary of mean phase coherence with the context.
    /// Returns the probability assigned to `target_idx`.
    fn p_phase(&self, ctx: &[f64], target_idx: usize) -> f64 {
        let v = self.unigram.len();
        let mut scores = Vec::with_capacity(v);
        let mut max = f64::NEG_INFINITY;

        let rk = self
            .readout
            .as_ref()
            .map(|r| r.key(&Self::ctx_angles(ctx)));

        for i in 0..v {
            let base = i * 2 * LM_CHANNELS;
            let mut dot = 0.0;
            for k in 0..LM_CHANNELS {
                dot += self.vecs[base + 2 * k] * ctx[2 * k]
                    + self.vecs[base + 2 * k + 1] * ctx[2 * k + 1];
            }
            let mut dot = dot / LM_CHANNELS as f64;
            if let (Some(r), Some(key)) = (&self.readout, rk) {
                dot += READOUT_WEIGHT * r.bias_for(key, self.sectors[i]);
            }
            let s = PHASE_BETA * dot;
            if s > max {
                max = s;
            }
            scores.push(s);
        }

        let mut z = 0.0;
        for s in &scores {
            z += (s - max).exp();
        }
        if z <= 0.0 {
            return 1.0 / v as f64;
        }
        (scores[target_idx] - max).exp() / z
    }

    /// The affine decomposition of the trigram probability in `P_base`.
    ///
    /// `P(c|a,b) = const + coef · P_base(c|ctx)` — being affine means a whole
    /// grid of mixing weights can be evaluated from one pass over the data.
    fn affine(&self, a: &str, b: &str, c: &str) -> (f64, f64) {
        // Absolute discounting from the facet's own interned tables. The
        // context key is a (u32, u32) tuple, so scoring a position no longer
        // allocates a joined String per lookup.
        let (bi_const, bi_coef) = match self.facet.bigram_stats(b, c) {
            Some(st) if st.total > 0 => {
                let lambda = DISCOUNT * st.types as f64 / st.total as f64;
                (
                    (st.count as f64 - DISCOUNT).max(0.0) / st.total as f64,
                    lambda,
                )
            }
            _ => (0.0, 1.0),
        };

        match self.facet.trigram_stats(a, b, c) {
            Some(st) if st.total > 0 => {
                let lambda = DISCOUNT * st.types as f64 / st.total as f64;
                (
                    (st.count as f64 - DISCOUNT).max(0.0) / st.total as f64
                        + lambda * bi_const,
                    lambda * bi_coef,
                )
            }
            _ => (bi_const, bi_coef),
        }
    }

    /// Probability of `c` following `a b`, at this model's γ.
    /// The context construction used by [`PhianoLM::probability`] and the
    /// γ sweep.
    ///
    /// B3: measured best of the three in both training regimes — phase-alone
    /// perplexity 173.08 against 182.69 for the two-word centroid under the
    /// ranking objective, and 188.63 against 192.57 under co-occurrence. The
    /// two-word centroid barely encodes order at all (swap-cosine 0.62: the
    /// swapped context is still more similar to the original than not), and it
    /// was the default anyway.
    pub const DEFAULT_CONTEXT: ContextKind = ContextKind::Recurrent;

    pub fn probability(&self, a: &str, b: &str, c: &str) -> f64 {
        let floor = 1.0 / (self.unigram.len().max(1) as f64 * 100.0);
        let idx = match self.index.get(c) {
            Some(i) => *i,
            None => return floor,
        };
        let (k, coef) = self.affine(a, b, c);
        let p_uni = self.unigram[idx].max(floor);
        let p_ph = match (self.gamma > 0.0, self.context_vec(a, b)) {
            (true, Some(ctx)) => self.p_phase(&ctx, idx),
            _ => p_uni,
        };
        let base = self.gamma * p_ph + (1.0 - self.gamma) * p_uni;
        (k + coef * base).max(1e-12)
    }

    /// Perplexity over held-out sentences, at [`PhianoLM::DEFAULT_CONTEXT`].
    ///
    /// B3: the recurrent state is carried across each sentence rather than
    /// rebuilt from the last two words at every position. `probability` sees
    /// only a trigram and structurally cannot do this, which is why the
    /// two-word centroid stayed the default long after it was measured worse.
    pub fn perplexity(&self, sentences: &[String]) -> f64 {
        self.perplexity_kind(sentences, Self::DEFAULT_CONTEXT)
    }

    /// Perplexity through the trigram `probability` path only.
    ///
    /// Kept as the comparison point B3 is measured against, and used wherever a
    /// caller genuinely has nothing but three words.
    pub fn perplexity_two_word(&self, sentences: &[String]) -> f64 {
        let mut log_sum = 0.0f64;
        let mut n = 0usize;
        for sentence in sentences {
            let toks = Tokenizer::tokenize(sentence);
            if toks.len() < 3 {
                continue;
            }
            for w in toks.windows(3) {
                log_sum += self.probability(&w[0], &w[1], &w[2]).ln();
                n += 1;
            }
        }
        match n {
            0 => f64::INFINITY,
            _ => (-log_sum / n as f64).exp(),
        }
    }

    /// Perplexity at an explicit context construction.
    pub fn perplexity_kind(&self, sentences: &[String], kind: ContextKind) -> f64 {
        if self.gamma <= 0.0 {
            // With the phase term switched off the context is never consulted,
            // so the cheap path is also the exact one.
            return self.perplexity_two_word(sentences);
        }

        let floor = 1.0 / (self.unigram.len().max(1) as f64 * 100.0);
        let (log_sum, n) = sentences
            .par_iter()
            .map(|sentence| {
                let toks = Tokenizer::tokenize(sentence);
                if toks.len() < 3 {
                    return (0.0f64, 0usize);
                }
                let mut h = vec![crate::wave::c64::new(0.0, 0.0); LM_CHANNELS];
                if kind.is_recurrent() {
                    self.advance(&mut h, &toks[0]);
                    self.advance(&mut h, &toks[1]);
                }

                let (mut sum, mut count) = (0.0f64, 0usize);
                for i in 2..toks.len() {
                    let (a, b, c) = (&toks[i - 2], &toks[i - 1], &toks[i]);
                    let idx = self.index.get(c).copied();

                    let ctx = match kind {
                        ContextKind::Recurrent => Some(Self::state_to_ctx(&h)),
                        ContextKind::Bound => self.context_vec_bound(a, b),
                        ContextKind::TwoWord => self.context_vec(a, b),
                    };

                    let p = match idx {
                        None => floor,
                        Some(idx) => {
                            let (k, coef) = self.affine(a, b, c);
                            let p_uni = self.unigram[idx].max(floor);
                            let p_ph = match ctx {
                                Some(ref v) => self.p_phase(v, idx),
                                None => p_uni,
                            };
                            let base = self.gamma * p_ph + (1.0 - self.gamma) * p_uni;
                            (k + coef * base).max(1e-12)
                        }
                    };
                    sum += p.ln();
                    count += 1;

                    if kind.is_recurrent() {
                        self.advance(&mut h, c);
                    }
                }
                (sum, count)
            })
            .reduce(|| (0.0f64, 0usize), |a, b| (a.0 + b.0, a.1 + b.1));

        match n {
            0 => f64::INFINITY,
            _ => (-log_sum / n as f64).exp(),
        }
    }

    /// Recurrent context state, per channel, over a sentence prefix.
    ///
    /// `h_k = λ_k · e^{i ω_k} · h_{k-1} + z_k`, the same diagonal complex
    /// recurrence `ContextWaveBuffer` uses. Unlike a two-word centroid this
    /// carries the whole prefix, with a per-channel timescale — information a
    /// trigram table structurally cannot have.
    #[inline]
    fn kernel(k: usize) -> crate::wave::c64 {
        crate::config::channel_kernel(k, LM_CHANNELS)
    }

    fn advance(&self, h: &mut [crate::wave::c64], token: &str) {
        for (k, z) in h.iter_mut().enumerate() {
            *z *= Self::kernel(k);
        }
        if let Some(p) = self.facet.lexicon.get(token) {
            for k in 0..LM_CHANNELS {
                h[k] += crate::wave::c64::from_polar(p.amplitude, p.theta(k));
            }
        }
    }

    /// Unit-normalises a recurrent state into a context vector.
    fn state_to_ctx(h: &[crate::wave::c64]) -> Vec<f64> {
        let mut out = vec![0.0f64; 2 * LM_CHANNELS];
        for k in 0..LM_CHANNELS {
            let n = h[k].norm();
            if n > 1e-12 {
                out[2 * k] = h[k].re / n;
                out[2 * k + 1] = h[k].im / n;
            } else {
                out[2 * k] = 1.0;
            }
        }
        out
    }

    /// Raw coherence score of every vocabulary item against a context vector.
    fn scores(&self, ctx: &[f64], buf: &mut Vec<f64>) {
        buf.clear();
        let v = self.unigram.len();
        for i in 0..v {
            let base = i * 2 * LM_CHANNELS;
            let mut dot = 0.0;
            for k in 0..LM_CHANNELS {
                dot += self.vecs[base + 2 * k] * ctx[2 * k]
                    + self.vecs[base + 2 * k + 1] * ctx[2 * k + 1];
            }
            // Mean cosine across channels, not the sum. A sum over 16 channels
            // spans [-16, 16]; exponentiating a spread that wide saturates the
            // softmax, every non-top candidate underflows the probability
            // floor, and the phase distribution degenerates into a constant.
            // A constant back-off can only ever lose to a unigram, which is how
            // a saturated scale masquerades as "the manifold contributes
            // nothing". Dividing by the channel count puts the score in
            // [-1, 1] and hands the temperature back to beta, where it belongs.
            buf.push(dot / LM_CHANNELS as f64);
        }

        // The non-linear correction, when fitted. It is added *after* the linear
        // dot, and it depends on both the context cell and the candidate's
        // sector — a bias depending on only one of the two could not reorder
        // anything.
        if let Some(r) = &self.readout {
            let key = r.key(&Self::ctx_angles(ctx));
            for (i, s) in buf.iter_mut().enumerate() {
                *s += READOUT_WEIGHT * r.bias_for(key, self.sectors[i]);
            }
        }
    }

    /// Full experiment sweep: context construction × temperature × mixing weight.
    ///
    /// One pass per context kind. The coherence scores are computed once per
    /// position and reused across every temperature, and the trigram probability
    /// is affine in the base distribution, so every mixing weight comes out of
    /// the same pass.
    pub fn sweep(&self, sentences: &[String], betas: &[f64], recurrent: bool) -> Vec<SweepRow> {
        self.sweep_against(sentences, betas, recurrent, false)
    }

    /// [`PhianoLM::sweep`] over an explicit context construction.
    pub fn sweep_kind(
        &self,
        sentences: &[String],
        betas: &[f64],
        kind: ContextKind,
        against_uniform: bool,
    ) -> Vec<SweepRow> {
        self.sweep_kinded(sentences, betas, kind, against_uniform)
    }

    /// As [`PhianoLM::sweep`], but the thing the phase distribution is mixed
    /// against can be a **uniform** distribution instead of a unigram.
    ///
    /// This is the control that says how much information the manifold carries
    /// at all. Against a unigram, phase has to beat word frequency — a strong
    /// opponent. Against uniform it only has to beat knowing nothing. If γ = 1
    /// loses to γ = 0 even here, the phase distribution is no better than chance.
    pub fn sweep_against(
        &self,
        sentences: &[String],
        betas: &[f64],
        recurrent: bool,
        against_uniform: bool,
    ) -> Vec<SweepRow> {
        let kind = match recurrent {
            true => ContextKind::Recurrent,
            false => ContextKind::TwoWord,
        };
        self.sweep_kinded(sentences, betas, kind, against_uniform)
    }

    fn sweep_kinded(
        &self,
        sentences: &[String],
        betas: &[f64],
        kind: ContextKind,
        against_uniform: bool,
    ) -> Vec<SweepRow> {
        let recurrent = kind.is_recurrent();
        let floor = 1.0 / (self.unigram.len().max(1) as f64 * 100.0);
        let nb = betas.len();
        let ng = GAMMA_GRID.len();

        let (sums, n) = sentences
            .par_iter()
            .map(|sentence| {
                let mut local = vec![0.0f64; nb * ng];
                let mut count = 0usize;
                let toks = Tokenizer::tokenize(sentence);
                if toks.len() < 3 {
                    return (local, count);
                }

                let mut h = vec![crate::wave::c64::new(0.0, 0.0); LM_CHANNELS];
                if recurrent {
                    self.advance(&mut h, &toks[0]);
                    self.advance(&mut h, &toks[1]);
                }

                let mut score_buf: Vec<f64> = Vec::with_capacity(self.unigram.len());

                for i in 2..toks.len() {
                    let (a, b, c) = (&toks[i - 2], &toks[i - 1], &toks[i]);
                    let idx = match self.index.get(c) {
                        Some(v) => *v,
                        None => {
                            if recurrent {
                                self.advance(&mut h, c);
                            }
                            continue;
                        }
                    };

                    let ctx = match kind {
                        ContextKind::Recurrent => Self::state_to_ctx(&h),
                        ContextKind::Bound => match self.context_vec_bound(a, b) {
                            Some(v) => v,
                            None => vec![1.0, 0.0].repeat(LM_CHANNELS),
                        },
                        ContextKind::TwoWord => match self.context_vec(a, b) {
                            Some(v) => v,
                            None => vec![1.0, 0.0].repeat(LM_CHANNELS),
                        },
                    };

                    self.scores(&ctx, &mut score_buf);
                    let max = score_buf.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

                    let (k_const, coef) = self.affine(a, b, c);
                    let p_uni = match against_uniform {
                        true => 1.0 / self.unigram.len().max(1) as f64,
                        false => self.unigram[idx].max(floor),
                    };

                    for (bi, beta) in betas.iter().enumerate() {
                        let mut z = 0.0;
                        for s in score_buf.iter() {
                            z += (beta * (s - max)).exp();
                        }
                        let p_ph = match z > 0.0 {
                            true => ((beta * (score_buf[idx] - max)).exp() / z).max(floor),
                            false => p_uni,
                        };
                        for (gi, g) in GAMMA_GRID.iter().enumerate() {
                            let base = g * p_ph + (1.0 - g) * p_uni;
                            local[bi * ng + gi] += (k_const + coef * base).max(1e-12).ln();
                        }
                    }

                    count += 1;
                    if recurrent {
                        self.advance(&mut h, c);
                    }
                }

                (local, count)
            })
            .reduce(
                || (vec![0.0f64; nb * ng], 0usize),
                |mut a, b| {
                    for (x, y) in a.0.iter_mut().zip(b.0.iter()) {
                        *x += y;
                    }
                    a.1 += b.1;
                    a
                },
            );

        let label = match against_uniform {
            false => kind.label().to_string(),
            true => format!("{}/unif", kind.label()),
        };
        let mut rows = Vec::with_capacity(nb * ng);
        for (bi, beta) in betas.iter().enumerate() {
            for (gi, g) in GAMMA_GRID.iter().enumerate() {
                rows.push(SweepRow {
                    context: label.clone(),
                    beta: *beta,
                    gamma: *g,
                    ppl: match n {
                        0 => f64::INFINITY,
                        _ => (-sums[bi * ng + gi] / n as f64).exp(),
                    },
                });
            }
        }
        rows
    }

    /// Evaluates the whole γ grid in a single pass over the data.
    ///
    /// Returns `(gamma, perplexity)` pairs. γ = 0 is the model with the phase
    /// manifold removed; γ = 1 is the manifold alone as the back-off.
    pub fn gamma_sweep(&self, sentences: &[String]) -> Vec<(f64, f64)> {
        let floor = 1.0 / (self.unigram.len().max(1) as f64 * 100.0);
        let mut sums = [0.0f64; GAMMA_GRID.len()];
        let mut n = 0usize;

        for sentence in sentences {
            let toks = Tokenizer::tokenize(sentence);
            if toks.len() < 3 {
                continue;
            }
            for w in toks.windows(3) {
                let idx = match self.index.get(&w[2]) {
                    Some(i) => *i,
                    None => continue,
                };
                let (k, coef) = self.affine(&w[0], &w[1], &w[2]);
                let p_uni = self.unigram[idx].max(floor);
                let p_ph = match self.context_vec(&w[0], &w[1]) {
                    Some(ctx) => self.p_phase(&ctx, idx),
                    None => p_uni,
                };
                for (gi, g) in GAMMA_GRID.iter().enumerate() {
                    let base = g * p_ph + (1.0 - g) * p_uni;
                    sums[gi] += (k + coef * base).max(1e-12).ln();
                }
                n += 1;
            }
        }

        GAMMA_GRID
            .iter()
            .zip(sums.iter())
            .map(|(g, s)| {
                (*g, match n { 0 => f64::INFINITY, _ => (-s / n as f64).exp() })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_corpus() -> Vec<String> {
        let mut c = Vec::new();
        let subjects = ["the cat", "the dog", "a bird", "my friend"];
        let verbs = ["sat on", "ran to", "looked at", "jumped over"];
        let objects = ["the mat", "the park", "a fence", "the table"];
        for s in &subjects {
            for v in &verbs {
                for o in &objects {
                    c.push(format!("{} {} {}", s, v, o));
                }
            }
        }
        c
    }

    #[test]
    fn test_split_is_deterministic_and_disjoint() {
        let a = Harness::split(toy_corpus(), 42);
        let b = Harness::split(toy_corpus(), 42);
        assert_eq!(a.train, b.train);
        assert_eq!(a.valid, b.valid);
        for v in &a.valid {
            assert!(!a.train.contains(v), "validation leaked into train");
        }
        for t in &a.test {
            assert!(!a.train.contains(t), "test leaked into train");
        }
    }

    #[test]
    fn test_perplexity_is_finite() {
        let split = Harness::split(toy_corpus(), 42);
        let t = Trainer::new(0.05);
        let (facet, log) = Harness::train_and_measure(&split, &t, 2, true);
        assert!(facet.vocabulary_size() > 5);
        for m in &log {
            assert!(m.valid_ppl.is_finite(), "perplexity must be finite: {:?}", m);
            assert!(m.phase_dispersion >= 0.0 && m.phase_dispersion <= 1.0);
        }
    }

    #[test]
    fn test_kn_baseline_beats_nothing_is_reported_honestly() {
        let split = Harness::split(toy_corpus(), 42);
        let kn = KneserNey::train(&split.train);
        let ppl = kn.perplexity(&split.valid);
        assert!(ppl.is_finite() && ppl > 1.0);
    }

    /// Training the same data twice must give the same model.
    ///
    /// It did not. `rebuild_sample_pool` built the negative-sample pool by
    /// iterating `lexicon`, a HashMap that Rust seeds randomly per process, so
    /// every run drew a different negative sequence. Two runs of one
    /// composition experiment returned analogy MRR 0.1066 and 0.0521 — same
    /// ranking, magnitudes a factor of two apart. Every measured effect smaller
    /// than that gap was unfalsifiable.
    #[test]
    fn test_training_is_reproducible() {
        let split = Harness::split(toy_corpus(), 42);
        let a = Harness::train_ranking_only(&split, &Trainer::new(0.05), 3);
        let b = Harness::train_ranking_only(&split, &Trainer::new(0.05), 3);

        assert_eq!(a.lexicon.len(), b.lexicon.len());
        for (w, pa) in &a.lexicon {
            let pb = b.lexicon.get(w).expect("same vocabulary");
            for k in 0..crate::config::PHASE_CHANNELS {
                assert_eq!(
                    pa.theta(k),
                    pb.theta(k),
                    "word {} channel {} differs between two identical runs",
                    w,
                    k
                );
            }
        }
    }

    /// The default context must be the one that was measured best, and the
    /// no-phase path must be unaffected by the choice.
    ///
    /// γ = 0 removes the phase term, so context construction cannot legally
    /// touch it. If it did, every no-phase baseline in the results would have
    /// silently moved when B3 landed.
    #[test]
    fn test_default_context_is_recurrent_and_gamma_zero_is_untouched() {
        assert_eq!(
            PhianoLM::DEFAULT_CONTEXT,
            ContextKind::Recurrent,
            "B3 selected the recurrent construction on measurement"
        );

        let split = Harness::split(toy_corpus(), 42);
        let (facet, _) = Harness::train_and_measure(&split, &Trainer::new(0.05), 3, true);

        let off = PhianoLM::with_gamma(&facet, 0.0);
        assert_eq!(
            off.perplexity(&split.valid),
            off.perplexity_two_word(&split.valid),
            "at γ=0 the context is never consulted, so both paths must agree exactly"
        );

        // At γ=1 the construction is consulted, so it must make a difference —
        // otherwise the default is a label with no effect behind it.
        let on = PhianoLM::with_gamma(&facet, 1.0);
        let rec = on.perplexity_kind(&split.valid, ContextKind::Recurrent);
        let two = on.perplexity_kind(&split.valid, ContextKind::TwoWord);
        assert!(rec.is_finite() && two.is_finite());
        assert!(
            (rec - two).abs() > 1e-9,
            "the context construction must change scoring at γ=1: {} vs {}",
            rec,
            two
        );
    }

    /// A seed must do both jobs: the same seed reproduces, a different seed
    /// varies.
    ///
    /// Half of this is easy to get wrong in each direction. A seed that is
    /// accepted but never mixed in gives identical runs and a fake error bar of
    /// zero; a seed that leaks into the corpus split rather than the training
    /// stochasticity varies the *data*, and the spread then measures the split
    /// rather than the model.
    #[test]
    fn test_seed_varies_training_without_varying_the_data() {
        let split = Harness::split(toy_corpus(), 42);
        let train = |seed: u64| {
            Harness::train_ranking_only(&split, &Trainer::new(0.05).with_seed(seed), 3)
        };

        let a = train(7);
        let b = train(7);
        let c = train(8);

        let differs = |x: &Facet, y: &Facet| {
            x.lexicon.iter().any(|(w, px)| match y.lexicon.get(w) {
                Some(py) => (0..crate::config::PHASE_CHANNELS).any(|k| px.theta(k) != py.theta(k)),
                None => true,
            })
        };

        assert!(!differs(&a, &b), "the same seed must reproduce exactly");
        assert!(differs(&a, &c), "a different seed must change the model");

        // The vocabulary is a property of the corpus, not of the seed. If this
        // moves, the seed is varying the data and the spread it produces would
        // not be a measure of training variance.
        assert_eq!(
            a.vocabulary_size(),
            c.vocabulary_size(),
            "the seed must not change what data was seen"
        );
    }

    /// The phase distribution must be a distribution, not a constant.
    ///
    /// This guards a bug that invalidated every γ sweep before it was found:
    /// the per-candidate score was a *sum* over 16 channels, spanning ~32 units,
    /// so exponentiating it underflowed the probability floor for every
    /// candidate but the top one. `p_phase` then returned the same floor value
    /// at nearly every position — a constant back-off, which loses to a unigram
    /// by construction at every γ. The measured conclusion "γ* = 0, the manifold
    /// contributes nothing" was partly a statement about the score scale.
    #[test]
    fn test_phase_distribution_is_not_saturated() {
        let split = Harness::split(toy_corpus(), 42);
        let (facet, _) = Harness::train_and_measure(&split, &Trainer::new(0.05), 3, true);
        let lm = PhianoLM::with_gamma(&facet, 1.0);

        let mut ctx_buf = Vec::new();
        let mut spread: f64 = 0.0;
        for sentence in &split.valid {
            let toks = Tokenizer::tokenize(sentence);
            for w in toks.windows(3) {
                if let Some(ctx) = lm.context_vec(&w[0], &w[1]) {
                    lm.scores(&ctx, &mut ctx_buf);
                    let hi = ctx_buf.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    let lo = ctx_buf.iter().cloned().fold(f64::INFINITY, f64::min);
                    spread = spread.max(hi - lo);
                }
            }
        }
        assert!(spread > 0.0, "scores are identical for every candidate");
        assert!(
            spread <= 2.0 + 1e-9,
            "scores must be a mean cosine in [-1, 1]; spread {} means the \
             softmax will saturate and the phase back-off will degenerate \
             into a constant",
            spread
        );
    }

    /// The readout must change scoring, and must change it only where the
    /// phase term is actually used.
    ///
    /// γ = 0 removes the phase back-off entirely, so a readout that moved the
    /// γ = 0 perplexity would be leaking into the n-gram path and every
    /// on-versus-off comparison built on it would be void.
    #[test]
    fn test_readout_changes_phase_scoring_only() {
        let split = Harness::split(toy_corpus(), 42);
        let (facet, _) = Harness::train_and_measure(&split, &Trainer::new(0.05), 3, true);

        let mut lm = PhianoLM::with_gamma(&facet, 1.0);
        let before = lm.sweep(&split.valid, &[1.0], false);
        lm.fit_readout(&split.train, 0.5, false);
        assert!(lm.readout_cells() > 0, "readout fitted no cells");
        let after = lm.sweep(&split.valid, &[1.0], false);

        let at = |rows: &[SweepRow], g: f64| {
            rows.iter().find(|r| (r.gamma - g).abs() < 1e-9).map(|r| r.ppl).unwrap()
        };

        assert!(
            (at(&before, 0.0) - at(&after, 0.0)).abs() < 1e-9,
            "readout leaked into the γ=0 path: {} vs {}",
            at(&before, 0.0),
            at(&after, 0.0)
        );
        assert!(
            (at(&before, 1.0) - at(&after, 1.0)).abs() > 1e-9,
            "readout was fitted but changed nothing at γ=1 — a bias that cannot \
             reorder candidates is the bug this table was rewritten to fix"
        );
    }

    /// Contrastive training must preserve more phase dispersion than
    /// attraction-only training over the same data.
    #[test]
    fn test_contrastive_preserves_dispersion() {
        let split = Harness::split(toy_corpus(), 42);
        let (f1, _) = Harness::train_and_measure(&split, &Trainer::new(0.05), 6, false);
        let (f2, _) =
            Harness::train_and_measure(&split, &Trainer::attraction_only(0.05), 6, false);
        assert!(
            f1.phase_dispersion() > f2.phase_dispersion(),
            "contrastive {} vs attraction-only {}",
            f1.phase_dispersion(),
            f2.phase_dispersion()
        );
    }
}
