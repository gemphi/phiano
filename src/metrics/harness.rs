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
        let seed = Trainer { learning_rate: 0.0, neg_samples: 0 };
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

/// Channels used by the language model's phase back-off.
///
/// The full torus is 64 channels; scoring every position against all of them is
/// affordable but wasteful, and 16 already captures far more than the single
/// angle a 1-D representation could offer.
const LM_CHANNELS: usize = 16;

/// Inverse temperature of the phase back-off distribution.
const PHASE_BETA: f64 = 1.0;

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
}

impl<'a> PhianoLM<'a> {
    pub fn new(facet: &'a Facet, use_phase: bool) -> Self {
        Self::with_gamma(facet, if use_phase { 1.0 } else { 0.0 })
    }

    pub fn with_gamma(facet: &'a Facet, gamma: f64) -> Self {
        let n = facet.lexicon.len();
        let mut index = std::collections::HashMap::with_capacity(n);
        let mut vecs = Vec::with_capacity(n * 2 * LM_CHANNELS);
        let mut counts = Vec::with_capacity(n);

        for (w, p) in &facet.lexicon {
            index.insert(w.clone(), counts.len());
            for k in 0..LM_CHANNELS {
                let t = p.theta(k);
                vecs.push(t.cos());
                vecs.push(t.sin());
            }
            counts.push(p.count.max(1) as f64);
        }

        let total: f64 = counts.iter().sum();
        let unigram: Vec<f64> = counts
            .iter()
            .map(|c| if total > 0.0 { c / total } else { 0.0 })
            .collect();

        Self { facet, index, vecs, unigram, gamma }
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

    /// Softmax over the vocabulary of mean phase coherence with the context.
    /// Returns the probability assigned to `target_idx`.
    fn p_phase(&self, ctx: &[f64], target_idx: usize) -> f64 {
        let v = self.unigram.len();
        let mut scores = Vec::with_capacity(v);
        let mut max = f64::NEG_INFINITY;

        for i in 0..v {
            let base = i * 2 * LM_CHANNELS;
            let mut dot = 0.0;
            for k in 0..LM_CHANNELS {
                dot += self.vecs[base + 2 * k] * ctx[2 * k]
                    + self.vecs[base + 2 * k + 1] * ctx[2 * k + 1];
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

    /// Perplexity over held-out sentences.
    pub fn perplexity(&self, sentences: &[String]) -> f64 {
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

    /// Recurrent context state, per channel, over a sentence prefix.
    ///
    /// `h_k = λ_k · e^{i ω_k} · h_{k-1} + z_k`, the same diagonal complex
    /// recurrence `ContextWaveBuffer` uses. Unlike a two-word centroid this
    /// carries the whole prefix, with a per-channel timescale — information a
    /// trigram table structurally cannot have.
    #[inline]
    fn kernel(k: usize) -> crate::wave::c64 {
        let frac = k as f64 / LM_CHANNELS as f64;
        let lambda = crate::config::CONTEXT_LAMBDA.powf(1.0 + 3.0 * frac);
        let omega = crate::config::CONTEXT_OMEGA * (1.0 + 4.0 * frac);
        crate::wave::c64::from_polar(lambda, omega)
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
            buf.push(dot);
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

                    let ctx = match recurrent {
                        true => Self::state_to_ctx(&h),
                        false => match self.context_vec(a, b) {
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

        let label = match (recurrent, against_uniform) {
            (true, false) => "recurrent",
            (false, false) => "2-word",
            (true, true) => "recurrent/unif",
            (false, true) => "2-word/unif",
        };
        let mut rows = Vec::with_capacity(nb * ng);
        for (bi, beta) in betas.iter().enumerate() {
            for (gi, g) in GAMMA_GRID.iter().enumerate() {
                rows.push(SweepRow {
                    context: label.to_string(),
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
