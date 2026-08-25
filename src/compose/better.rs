use crate::config;
use crate::facet::Facet;
use crate::compose::flow::RiverFlow;
use crate::compose::sector_color;

/// Evaluator - scores each of the 64 sector variations.
///
/// Based on the Flower-Hayes "Reviewing" process:
///   evaluating + revising
///
/// This is the "better" half - it identifies which compositions
/// are strong and worth keeping. It works with `Discarder` (worse.rs)
/// which removes the weak ones.
///
/// Scoring uses three dimensions from the phase geometry:
/// - **Coherence**: how well the gathered words resonate together
/// - **Novelty**: distance from the facet centroid (originality)
/// - **Resonance**: fraction of words the facet actually knows
///
/// The overall score determines which sectors survive the tournament.
pub struct Evaluator {
    /// The inner semantic evaluator.
    inner: crate::eval::Evaluator,
}

/// SectorScore - the evaluation of a single sector variation.
#[derive(Debug, Clone)]
pub struct SectorScore {
    pub sector: u16,
    pub color: String,
    pub text: String,
    pub score: f64,
}

impl Evaluator {
    /// Creates a new composer evaluator.
    pub fn new() -> Self {
        Self {
            inner: crate::eval::Evaluator::new(),
        }
    }

    /// Evaluates all 64 sector variations and returns ranked scores.
    ///
    /// Each variation is scored on coherence, novelty, resonance,
    /// plus composition-specific metrics: word diversity, sector
    /// coverage, and prompt alignment. These extra metrics
    /// differentiate variations even when the base eval scores
    /// are all ~1.0 (large vocabularies).
    pub fn evaluate_variations(
        &self,
        facet: &Facet,
        flows: &[RiverFlow],
    ) -> Vec<SectorScore> {
        let mut scores = Vec::with_capacity(flows.len());

        // Compute prompt wave once for alignment scoring
        let prompt_tokens = crate::tokenizer::Tokenizer::tokenize(
            flows.first().map(|f| f.prompt.as_str()).unwrap_or(""),
        );
        let prompt_wave = crate::wave::Wave::sentence(facet, &prompt_tokens);

        for flow in flows {
            let eval = self.inner.eval(facet, &flow.text);

            // Composition-specific discrimination
            let tokens = crate::tokenizer::Tokenizer::tokenize(&flow.text);
            let unique: std::collections::HashSet<&String> = tokens.iter().collect();
            let diversity = if tokens.is_empty() {
                0.0
            } else {
                unique.len() as f64 / tokens.len() as f64
            };

            // Sector coverage: how many distinct sectors do the words span?
            let mut sectors_used = std::collections::HashSet::new();
            for token in &tokens {
                if let Some(s) = crate::wave::Wave::word_sector(facet, token) {
                    sectors_used.insert(s);
                }
            }
            let n = crate::wave::Wave::sector_count() as f64;
            let coverage = sectors_used.len() as f64 / n;

            // Length factor: penalize very short compositions
            let length_factor = if tokens.len() < 10 {
                tokens.len() as f64 / 10.0
            } else {
                1.0
            };

            // Prompt alignment: how well does the composition's wave
            // align with the prompt's wave? This is the key semantic
            // differentiator - each sector produces different words,
            // so the wave alignment varies even with large vocabularies.
            let comp_wave = crate::wave::Wave::sentence(facet, &tokens);
            let prompt_norm = prompt_wave.norm();
            let comp_norm = comp_wave.norm();
            let alignment = if prompt_norm > 0.0 && comp_norm > 0.0 {
                // Cosine similarity between prompt and composition waves
                let dot = (prompt_wave.re * comp_wave.re + prompt_wave.im * comp_wave.im)
                    / (prompt_norm * comp_norm);
                (dot + 1.0) / 2.0 // map [-1, 1] to [0, 1]
            } else {
                0.0
            };

            // Combined score using config weights:
            // eval.overall bundles coherence+novelty+resonance (0.55 total),
            // then diversity, coverage, and alignment are composition-specific.
            let base_weight = config::WEIGHT_COHERENCE
                + config::WEIGHT_NOVELTY
                + config::WEIGHT_RESONANCE;
            let comp_score = eval.overall * base_weight
                + diversity * config::WEIGHT_DIVERSITY
                + coverage * config::WEIGHT_COVERAGE
                + length_factor * config::WEIGHT_NOVELTY
                + alignment * config::WEIGHT_ALIGNMENT;

            scores.push(SectorScore {
                sector: flow.source_sector,
                color: sector_color(flow.source_sector),
                text: flow.text.clone(),
                score: comp_score,
            });
        }

        // Sort best first
        scores.sort_by(|a, b| {
            b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal)
        });

        scores
    }

    /// Computes the average score across all 64 sectors.
    pub fn average_score(&self, scores: &[SectorScore]) -> f64 {
        if scores.is_empty() {
            return 0.0;
        }
        let total: f64 = scores.iter().map(|s| s.score).sum();
        total / scores.len() as f64
    }

    /// Computes the spread (standard deviation) of sector scores.
    ///
    /// A low spread means all sectors produce similar quality -
    /// the facet is uniform. A high spread means some sectors
    /// are much better than others - the facet has structure.
    pub fn score_spread(&self, scores: &[SectorScore]) -> f64 {
        if scores.len() < 2 {
            return 0.0;
        }
        let avg = self.average_score(scores);
        let variance: f64 = scores
            .iter()
            .map(|s| (s.score - avg).powi(2))
            .sum::<f64>()
            / scores.len() as f64;
        variance.sqrt()
    }

    /// Checks if the tournament has converged.
    ///
    /// Convergence happens when the top score stops improving
    /// between rounds, or the spread becomes very small.
    pub fn has_converged(&self, scores: &[SectorScore], prev_best: f64) -> bool {
        if scores.is_empty() {
            return true;
        }
        let current_best = scores[0].score;
        let improvement = (current_best - prev_best).abs();
        improvement < config::COMPOSE_CONVERGENCE_DELTA
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
