//! Impersonator - composes text as a persona.
//!
//! In Phient's world a persona is which conditions a writer puts words
//! under. Composing as one means generating under those conditions: the
//! generator (`generate::Generator`) conditioned on the persona's sector
//! histogram and on its own pair evidence (`Generator::with_persona_examples`,
//! measured in `bin/persona_eval`: 53% of compositions read back as the
//! persona asked for, chance 33%). Several candidates are composed at
//! different temperatures and seeds; the one whose own fingerprint the
//! persona's likelihood favours most, weighed with the evaluator's quality,
//! is returned. Until this existed impersonation filled four fixed sentence
//! frames with words from the persona's sectors (`bin/impersonate_eval`
//! before/after in docs/80).

mod result;

pub use result::ImpersonationResult;

use crate::compose::SectorPalette;
use crate::config;
use phiecs::phasor::facet::Facet;
use crate::generate::{ContextWaveBuffer, Decode, Generator};
use crate::persona::Fingerprint;
use crate::trainer::Trainer;

pub struct Impersonator {
    /// How many candidate compositions to draw (was: refinement rounds).
    pub max_rounds: usize,
    /// Tokens per composition.
    pub tokens: usize,
    /// Weight of the persona's sector prior in the generator.
    pub sector_weight: f64,
    /// Weight of the persona's own bigram evidence in the generator.
    pub ngram_weight: f64,
}

/// Candidates drawn per impersonation.
pub const IMPERSONATE_CANDIDATES: usize = 6;
/// Tokens per candidate.
pub const IMPERSONATE_TOKENS: usize = 40;

impl Impersonator {
    pub fn new() -> Self {
        Self { max_rounds: IMPERSONATE_CANDIDATES, tokens: IMPERSONATE_TOKENS, sector_weight: 0.0, ngram_weight: 1.0 }
    }

    /// Impersonates from the fingerprint alone (sector prior only; the
    /// pair evidence needs the persona's examples — prefer
    /// [`Self::impersonate_with`]).
    pub fn impersonate(&self, facet: &mut Facet, trainer: &Trainer, fingerprint: &Fingerprint, persona_name: &str, prompt: &str) -> ImpersonationResult {
        let _ = trainer;
        let me = Self { sector_weight: 1.0, ngram_weight: 0.0, ..*self };
        me.impersonate_with(facet, trainer, fingerprint, &[], persona_name, prompt)
    }

    /// Composes as the persona: `max_rounds` candidates, the best by
    /// persona likelihood and quality. The lexicon is read, not trained.
    pub fn impersonate_with(&self, facet: &mut Facet, _trainer: &Trainer, fingerprint: &Fingerprint, examples: &[String], persona_name: &str, prompt: &str) -> ImpersonationResult {
        let eval = crate::eval::Evaluator::new();
        let mut best: Option<(f64, f64, f64, String)> = None;
        let seed0 = phistd::mix64(prompt.len() as u64 ^ 0x5EED);
        for round in 0..self.max_rounds.max(1) {
            let temperature = 0.3 + 0.1 * (round % 3) as f64;
            let g = Generator::deterministic(self.tokens, temperature, seed0.wrapping_add(round as u64 * 7919)).with_decode(Decode::Trigram).with_persona_examples(facet, fingerprint, examples, self.sector_weight, self.ngram_weight);
            let mut buf = ContextWaveBuffer::new(config::DEFAULT_CONTEXT_WINDOW);
            let text = g.generate(facet, &mut buf, prompt);
            if crate::tokenizer::Tokenizer::tokenize(&text).len() < 3 {
                continue;
            }
            let fit = fingerprint.likelihood(&Fingerprint::extract(facet, std::slice::from_ref(&text)));
            let quality = eval.eval(facet, &text).overall;
            let combined = quality * config::IMPERSONATE_QUALITY_WEIGHT + fit * config::IMPERSONATE_FIT_WEIGHT;
            if best.as_ref().is_none_or(|b| combined > b.0) {
                best = Some((combined, quality, fit, text));
            }
        }
        let (_, quality, fit, text) = best.unwrap_or((0.0, 0.0, 0.0, String::new()));
        let sector = Fingerprint::extract(facet, std::slice::from_ref(&text)).dominant_sectors(1).first().map_or(0, |(s, _)| *s);
        ImpersonationResult {
            persona_name: persona_name.to_string(),
            prompt: prompt.to_string(),
            text,
            winning_sector: sector,
            winning_color: SectorPalette::color(sector),
            quality_score: quality,
            persona_fit: fit,
            rounds: self.max_rounds,
        }
    }
}

impl Default for Impersonator {
    fn default() -> Self {
        Self::new()
    }
}
