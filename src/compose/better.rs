use crate::config;
use crate::facet::Facet;
use crate::compose::flow::RiverFlow;
use crate::compose::SectorPalette;
use crate::tokenizer::Tokenizer;
use crate::wave::Wave;
use std::collections::{HashMap, HashSet};

/// Evaluator — scores each of the 64 sector variations.
///
/// Based on the Flower-Hayes "Reviewing" process (evaluating + revising), and on
/// Huberman's 1968 chess-endgame design, in which progress is judged by a pair of
/// predicates over a **(stage, measure)** ordering rather than by a single number.
///
/// A flat weighted sum lets every quality be traded against every other: a
/// composition that is semantically wrong but long, diverse and sector-spanning
/// can outscore one that is short and right. A lexicographic order forbids that —
/// no amount of fine-grained score buys a stage.
pub struct Evaluator {
    /// The inner semantic evaluator.
    inner: crate::eval::Evaluator,
}

/// SectorScore — the evaluation of a single sector variation.
#[derive(Debug, Clone)]
pub struct SectorScore {
    pub sector: u16,
    pub color: String,
    pub text: String,
    /// Coarse subgoal index. Compared **before** `score`; higher is further along.
    pub stage: u8,
    /// Fine-grained measure within the stage, in [0, 1].
    pub score: f64,
}

impl SectorScore {
    /// Huberman's `better(p, q)`: advance a stage, or improve the measure within
    /// the current one.
    ///
    /// `better(p,q) = st(q) > st(p) ∨ [st(q) = st(p) ∧ m(q) < m(p)]`  (thesis 3.6)
    pub fn better_than(&self, other: &Self) -> bool {
        self.stage > other.stage || (self.stage == other.stage && self.score > other.score)
    }

    /// Huberman's `worse(p, q)` — deliberately *not* the negation of `better`.
    ///
    /// Stage zero is unconditionally worse (thesis 3.8: `st(q) = 0 ∨ …`). In the
    /// original this is a search guard: the tree is never expanded through a
    /// worse position. Here it is a survival guard: a stage-0 variant is never a
    /// survivor, whatever its fine score.
    pub fn is_worse(&self) -> bool {
        self.stage == 0
    }
}

/// Cross-round memory of words that appeared in winning sectors.
///
/// Huberman's *killer heuristic* (thesis §2, p. 22): try first the moves that
/// produced a better position elsewhere in the search. Still standard in
/// alpha-beta engines nearly sixty years later.
#[derive(Debug, Default, Clone)]
pub struct KillerWords {
    hits: HashMap<String, u32>,
}

impl KillerWords {
    pub fn new() -> Self {
        Self::default()
    }

    /// Records the content words of a winning composition.
    pub fn record_winner(&mut self, text: &str) {
        for w in Tokenizer::content_words(text) {
            *self.hits.entry(w).or_insert(0) += 1;
        }
    }

    /// Multiplicative bonus for a composition containing previously-winning words.
    pub fn bonus(&self, text: &str) -> f64 {
        if self.hits.is_empty() {
            return 1.0;
        }
        let words = Tokenizer::content_words(text);
        if words.is_empty() {
            return 1.0;
        }
        let hit: u32 = words.iter().filter_map(|w| self.hits.get(w)).sum();
        1.0 + config::KILLER_BONUS * (hit as f64).ln_1p() / (words.len() as f64).sqrt()
    }
}

impl Evaluator {
    /// Creates a new composer evaluator.
    pub fn new() -> Self {
        Self { inner: crate::eval::Evaluator::new() }
    }

    /// Assigns a composition its coarse subgoal stage.
    ///
    /// Stages are disjoint and ordered. Only within a stage does the fine
    /// measure decide, so progress on the coarse goal can never be sold for
    /// diversity or length.
    ///
    /// * 0 — degenerate: too short, or the wave cancels to nothing
    /// * 1 — off-prompt: shares no content word with the prompt
    /// * 2 — weakly on-prompt: shares vocabulary, but the waves barely align
    /// * 3 — on-prompt
    pub fn stage(facet: &Facet, prompt: &str, text: &str) -> u8 {
        let toks = Tokenizer::tokenize(text);
        let content = Tokenizer::content_words(text);
        if content.len() < 3 {
            return 0;
        }
        let w = Wave::sentence(facet, &toks);
        if w.norm() < 1e-6 {
            return 0;
        }

        let prompt_content: HashSet<String> = Tokenizer::content_words(prompt).into_iter().collect();
        let shares = content.iter().any(|c| prompt_content.contains(c));
        let align = Self::alignment(facet, prompt, &toks);

        match (shares, align) {
            (false, a) if a < 0.55 => 1,
            (_, a) if a < 0.70 => 2,
            _ => 3,
        }
    }

    /// Cosine alignment between the prompt wave and a composition's wave,
    /// mapped from [-1, 1] to [0, 1].
    fn alignment(facet: &Facet, prompt: &str, tokens: &[String]) -> f64 {
        let pw = Wave::text(facet, prompt);
        let cw = Wave::sentence(facet, tokens);
        let (pn, cn) = (pw.norm(), cw.norm());
        if pn <= 0.0 || cn <= 0.0 {
            return 0.0;
        }
        (((pw.re * cw.re + pw.im * cw.im) / (pn * cn)) + 1.0) / 2.0
    }

    /// Evaluates all sector variations and returns them ranked, best first.
    ///
    /// Ranking is lexicographic on `(stage, score)`. `score` itself is a
    /// normalised weighted sum — normalised because the raw coefficients sum to
    /// 1.15, and because `length_factor` previously borrowed `WEIGHT_NOVELTY`,
    /// counting novelty twice and leaving length without a weight of its own.
    pub fn evaluate_variations(
        &self,
        facet: &Facet,
        flows: &[RiverFlow],
        killers: &KillerWords,
    ) -> Vec<SectorScore> {
        let mut scores = Vec::with_capacity(flows.len());
        let prompt = flows.first().map(|f| f.prompt.as_str()).unwrap_or("");

        let base_weight =
            config::WEIGHT_COHERENCE + config::WEIGHT_NOVELTY + config::WEIGHT_RESONANCE;
        let total_weight = base_weight
            + config::WEIGHT_DIVERSITY
            + config::WEIGHT_COVERAGE
            + config::WEIGHT_LENGTH
            + config::WEIGHT_ALIGNMENT;

        for flow in flows {
            let eval = self.inner.eval(facet, &flow.text);
            let tokens = Tokenizer::tokenize(&flow.text);

            let unique: HashSet<&String> = tokens.iter().collect();
            let diversity = if tokens.is_empty() {
                0.0
            } else {
                unique.len() as f64 / tokens.len() as f64
            };

            let mut sectors_used = HashSet::new();
            for token in &tokens {
                if let Some(s) = Wave::word_sector(facet, token) {
                    sectors_used.insert(s);
                }
            }
            let coverage = sectors_used.len() as f64 / Wave::sector_count() as f64;

            let length_factor = if tokens.len() < 10 {
                tokens.len() as f64 / 10.0
            } else {
                1.0
            };

            let alignment = Self::alignment(facet, prompt, &tokens);

            let raw = eval.overall * base_weight
                + diversity * config::WEIGHT_DIVERSITY
                + coverage * config::WEIGHT_COVERAGE
                + length_factor * config::WEIGHT_LENGTH
                + alignment * config::WEIGHT_ALIGNMENT;

            let measure = ((raw / total_weight) * killers.bonus(&flow.text)).clamp(0.0, 1.0);

            scores.push(SectorScore {
                sector: flow.source_sector,
                color: SectorPalette::color(flow.source_sector),
                stage: Self::stage(facet, prompt, &flow.text),
                text: flow.text.clone(),
                score: measure,
            });
        }

        // Lexicographic: stage first, measure only as a tie-break.
        scores.sort_by(|a, b| {
            b.stage
                .cmp(&a.stage)
                .then(b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal))
        });

        scores
    }

    /// Computes the average measure across all sectors.
    pub fn average_score(&self, scores: &[SectorScore]) -> f64 {
        if scores.is_empty() {
            return 0.0;
        }
        scores.iter().map(|s| s.score).sum::<f64>() / scores.len() as f64
    }

    /// Standard deviation of sector measures.
    ///
    /// This is the manifold-health readout of the tournament. A spread trending
    /// to zero means the variants have become indistinguishable — there is
    /// nothing left to select between — which looks identical to convergence
    /// from the top score alone.
    pub fn score_spread(&self, scores: &[SectorScore]) -> f64 {
        if scores.len() < 2 {
            return 0.0;
        }
        let avg = self.average_score(scores);
        (scores.iter().map(|s| (s.score - avg).powi(2)).sum::<f64>() / scores.len() as f64).sqrt()
    }

    /// True when the variant population has collapsed rather than converged.
    pub fn is_degenerate(&self, scores: &[SectorScore]) -> bool {
        self.score_spread(scores) < config::SPREAD_ALARM
    }

    /// Checks whether the tournament has converged on the top score.
    pub fn has_converged(&self, scores: &[SectorScore], prev_best: f64) -> bool {
        match scores.first() {
            None => true,
            Some(top) => (top.score - prev_best).abs() < config::COMPOSE_CONVERGENCE_DELTA,
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
