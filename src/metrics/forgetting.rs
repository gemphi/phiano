//! Catastrophic-forgetting benchmark.
//!
//! `docs/45_native_learning_vs_bloated_llms.md` places Phiano in a comparison
//! matrix against Phi-4, GLM and GPT-4 and asserts **"Catastrophic Forgetting:
//! Zero"**. Nothing in the repository tested that. It is also the one claim
//! where this architecture genuinely should win — updating one word's phase
//! does not overwrite a shared weight matrix — which makes leaving it
//! unmeasured the largest missed opportunity in the project.
//!
//! # Design
//!
//! Two disjoint domains, A and B, and three models:
//!
//! | model | trained on | meaning |
//! |:---|:---|:---|
//! | **ceiling** | A and B together | no forgetting is possible |
//! | **sequential** | A, then B | the thing being measured |
//! | **floor** | B only | total forgetting |
//!
//! All three are scored on held-out **A**. Retention normalises the sequential
//! model between the two bounds, in log space because perplexity is
//! multiplicative:
//!
//! ```text
//! retention = (ln floor − ln sequential) / (ln floor − ln ceiling)
//! ```
//!
//! 1.0 means the second domain cost nothing. 0.0 means A was forgotten as
//! completely as if it had never been trained. Both bounds are measured rather
//! than assumed, so the number cannot be flattered by a weak baseline.

use crate::facet::Facet;
use crate::metrics::harness::PhianoLM;
use crate::trainer::Trainer;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ForgettingReport {
    pub domain_a_name: String,
    pub domain_b_name: String,
    pub n_train_a: usize,
    pub n_train_b: usize,
    pub n_eval_a: usize,
    /// Trained on A and B together — the no-forgetting bound.
    pub ceiling_ppl: f64,
    /// Trained on A, then B — the measurement.
    pub sequential_ppl: f64,
    /// Trained on B only — the total-forgetting bound.
    pub floor_ppl: f64,
    /// Trained on A only, before B was ever seen.
    pub a_only_ppl: f64,
    /// The sequential model scored from counts alone, for reference. The n-gram
    /// tables are tallies nothing overwrites, so this cannot degrade.
    pub counts_only_ppl: f64,
    /// 1.0 = nothing forgotten, 0.0 = forgotten completely.
    pub retention: f64,
    /// How much worse A got, as a percentage, after learning B.
    pub degradation_pct: f64,
    pub verdict: String,
}

pub struct ForgettingBenchmark;

impl ForgettingBenchmark {
    fn feed(trainer: &Trainer, f: &mut Facet, s: &str, ranking: bool) {
        match ranking {
            true => { trainer.train(f, s); }
            false => { trainer.train_sentence(f, s); }
        }
    }

    /// Sequential exposure: all of the first set, then all of the second.
    fn train_sequential(trainer: &Trainer, sets: &[&[String]], ranking: bool) -> Facet {
        let mut f = Facet::new();
        for set in sets {
            for s in set.iter() {
                Self::feed(trainer, &mut f, s, ranking);
            }
        }
        f
    }

    /// Joint exposure: the two domains **interleaved**.
    ///
    /// This is the distinction the whole benchmark turns on. Training all of A
    /// and then all of B *is* the sequential condition, so using it as the
    /// ceiling makes the two models identical and retention comes out at 100%
    /// by construction rather than by measurement. Round-robin interleaving is
    /// the standard joint-training upper bound: the same data, without the
    /// ordering that causes forgetting.
    fn train_joint(trainer: &Trainer, a: &[String], b: &[String], ranking: bool) -> Facet {
        let mut f = Facet::new();
        let n = a.len().max(b.len());
        for i in 0..n {
            if let Some(s) = a.get(i) {
                Self::feed(trainer, &mut f, s, ranking);
            }
            if let Some(s) = b.get(i) {
                Self::feed(trainer, &mut f, s, ranking);
            }
        }
        f
    }

    /// Perplexity with the phase manifold as the back-off distribution.
    ///
    /// Measured at γ = 1 deliberately. At γ = 0 the score depends only on the
    /// n-gram counts, which are pure tallies that nothing ever overwrites — so
    /// they cannot forget, and a benchmark run there measures a property that
    /// is true by construction rather than anything about the manifold.
    fn ppl(facet: &Facet, held_out: &[String]) -> f64 {
        PhianoLM::with_gamma(facet, 1.0).perplexity(held_out)
    }

    /// Perplexity from the counts alone, reported for reference.
    fn ppl_counts(facet: &Facet, held_out: &[String]) -> f64 {
        PhianoLM::with_gamma(facet, 0.0).perplexity(held_out)
    }

    /// Runs the three-model comparison.
    pub fn run(
        trainer: &Trainer,
        a_name: &str,
        a_train: &[String],
        a_eval: &[String],
        b_name: &str,
        b_train: &[String],
        ranking: bool,
    ) -> ForgettingReport {
        let ceiling = Self::train_joint(trainer, a_train, b_train, ranking);
        let floor = Self::train_sequential(trainer, &[b_train], ranking);

        // Sequential: A first, then B, on the same model.
        let mut sequential = Self::train_sequential(trainer, &[a_train], ranking);
        let a_only_ppl = Self::ppl(&sequential, a_eval);
        for s in b_train {
            Self::feed(trainer, &mut sequential, s, ranking);
        }

        let ceiling_ppl = Self::ppl(&ceiling, a_eval);
        let floor_ppl = Self::ppl(&floor, a_eval);
        let sequential_ppl = Self::ppl(&sequential, a_eval);
        let counts_ppl = Self::ppl_counts(&sequential, a_eval);

        let span = floor_ppl.ln() - ceiling_ppl.ln();
        let retention = match span.abs() < 1e-9 {
            true => f64::NAN,
            false => ((floor_ppl.ln() - sequential_ppl.ln()) / span).clamp(0.0, 1.5),
        };
        let degradation_pct = (sequential_ppl / a_only_ppl - 1.0) * 100.0;

        let verdict = match retention {
            r if r.is_nan() => "bounds collapsed — the domains are not distinct enough".to_string(),
            r if r >= 0.95 => format!(
                "retention {:.1}% — learning B cost domain A essentially nothing",
                r * 100.0
            ),
            r if r >= 0.7 => format!(
                "retention {:.1}% — mild forgetting, well short of catastrophic",
                r * 100.0
            ),
            r => format!(
                "retention {:.1}% — substantial forgetting; the 'zero catastrophic \
                 forgetting' claim is not supported",
                r * 100.0
            ),
        };

        ForgettingReport {
            domain_a_name: a_name.to_string(),
            domain_b_name: b_name.to_string(),
            n_train_a: a_train.len(),
            n_train_b: b_train.len(),
            n_eval_a: a_eval.len(),
            ceiling_ppl,
            sequential_ppl,
            floor_ppl,
            a_only_ppl,
            counts_only_ppl: counts_ppl,
            retention,
            degradation_pct,
            verdict,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn domain(prefix: &str, n: usize) -> Vec<String> {
        (0..n)
            .map(|i| format!("{} alpha{} beta{} gamma{}", prefix, i % 7, i % 5, i % 3))
            .collect()
    }

    /// The bug this benchmark shipped with: training all of A then all of B is
    /// the *sequential* condition, so using it as the joint ceiling made the two
    /// models identical and retention came out at exactly 100% by construction.
    #[test]
    fn test_joint_training_differs_from_sequential() {
        let t = Trainer::new(0.05);
        let a = domain("harbour vessel", 40);
        let b = domain("mineral basalt", 40);
        let joint = ForgettingBenchmark::train_joint(&t, &a, &b, false);
        let seq = ForgettingBenchmark::train_sequential(&t, &[&a, &b], false);

        let differs = joint.lexicon.iter().any(|(w, p)| {
            seq.lexicon.get(w).map(|q| (p.phase - q.phase).abs() > 1e-9).unwrap_or(true)
        });
        assert!(differs, "the ceiling must not be the sequential model in disguise");
    }

    #[test]
    fn test_bounds_are_ordered() {
        let t = Trainer::new(0.05);
        let a = domain("harbour vessel", 60);
        let b = domain("mineral basalt", 60);
        let r = ForgettingBenchmark::run(&t, "A", &a[..50], &a[50..], "B", &b, false);

        assert!(r.ceiling_ppl.is_finite() && r.floor_ppl.is_finite());
        // A model that never saw A cannot beat one trained on it.
        assert!(
            r.floor_ppl >= r.ceiling_ppl,
            "floor {} should not beat ceiling {}",
            r.floor_ppl, r.ceiling_ppl
        );
    }

    #[test]
    fn test_retention_is_bounded() {
        let t = Trainer::new(0.05);
        let a = domain("harbour vessel", 40);
        let b = domain("mineral basalt", 40);
        let r = ForgettingBenchmark::run(&t, "A", &a[..30], &a[30..], "B", &b, false);
        assert!(r.retention.is_nan() || (r.retention >= 0.0 && r.retention <= 1.5));
    }
}
