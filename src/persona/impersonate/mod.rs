/// Impersonator - composes text in a persona's style.
///
/// Given a persona's fingerprint, the impersonator biases the river flow
/// to start from and prefer the persona's dominant sectors.

mod result;

pub use result::ImpersonationResult;

use crate::compose::flow::RiverFlow;
use crate::compose::sector_color;
use crate::config;
use crate::facet::Facet;
use crate::persona::Fingerprint;
use crate::trainer::Trainer;
use crate::wave::Wave;

pub struct Impersonator {
    /// Number of recursive refinement rounds.
    pub max_rounds: usize,
}

impl Impersonator {
    pub fn new() -> Self {
        Self {
            max_rounds: config::IMPERSONATE_ROUNDS_DEFAULT,
        }
    }

    /// Impersonates a persona by composing text in their style.
    pub fn impersonate(
        &self,
        facet: &mut Facet,
        trainer: &Trainer,
        fingerprint: &Fingerprint,
        persona_name: &str,
        prompt: &str,
    ) -> ImpersonationResult {
        let mut best_text = String::new();
        let mut best_sector: u16 = 0;
        let mut best_combined = 0.0f64;
        let mut best_quality = 0.0f64;
        let mut best_fit = 0.0f64;
        let mut rounds_done = 0;
        let mut prev_combined = 0.0f64;

        let dominant = fingerprint.dominant_sectors(16);

        for round in 0..self.max_rounds {
            let depth = 4 + round * 2;

            println!(
                "  [impersonate] round {}/{} (depth {}) - biasing toward {}'s sectors",
                round + 1, self.max_rounds, depth, persona_name,
            );

            let mut flows = Vec::new();
            for &(sector, _weight) in &dominant {
                let flow = RiverFlow::trace(facet, prompt, Some(sector), depth);
                flows.push(flow);
            }
            let all_flows = RiverFlow::generate_variations(facet, prompt, depth);
            flows.extend(all_flows);

            let base_eval = crate::eval::Evaluator::new();
            let mut scored: Vec<(usize, f64, f64, f64)> = Vec::new();

            for (i, flow) in flows.iter().enumerate() {
                let eval = base_eval.eval(facet, &flow.text);

                let tokens = crate::tokenizer::Tokenizer::tokenize(&flow.text);
                let unique: std::collections::HashSet<&String> = tokens.iter().collect();
                let diversity = if tokens.is_empty() {
                    0.0
                } else {
                    unique.len() as f64 / tokens.len() as f64
                };

                let mut sectors_used = std::collections::HashSet::new();
                for token in &tokens {
                    if let Some(s) = Wave::word_sector(facet, token) {
                        sectors_used.insert(s);
                    }
                }
                let n = crate::wave::Wave::sector_count() as f64;
                let coverage = sectors_used.len() as f64 / n;

                let length_factor = if tokens.len() < 10 {
                    tokens.len() as f64 / 10.0
                } else {
                    1.0
                };

                let quality = eval.overall * config::IMPERSONATE_QUALITY_OVERALL
                    + diversity * config::IMPERSONATE_QUALITY_DIVERSITY
                    + coverage * config::IMPERSONATE_QUALITY_COVERAGE
                    + length_factor * config::IMPERSONATE_QUALITY_LENGTH;

                let text_wave = Wave::text(facet, &flow.text);
                let text_sector = Wave::wave_sector(text_wave);
                let fit = Self::sector_fit(&text_sector, &dominant);

                let combined = quality * config::IMPERSONATE_QUALITY_WEIGHT
                    + fit * config::IMPERSONATE_FIT_WEIGHT;

                scored.push((i, combined, quality, fit));
            }

            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

            let (best_idx, combined, quality, fit) = scored[0];
            let best_flow = &flows[best_idx];

            let improvement = combined - prev_combined;
            println!(
                "  [impersonate] best: quality {:.4} fit {:.4} combined {:.4} (Δ {:+.4}) sector {} ({})",
                quality, fit, combined, improvement,
                best_flow.source_sector, sector_color(best_flow.source_sector),
            );

            for (idx, _, _, _) in scored.iter().take(8) {
                trainer.train_sentence(facet, &flows[*idx].text);
            }

            if combined >= best_combined {
                best_combined = combined;
                best_quality = quality;
                best_text = best_flow.text.clone();
                best_sector = best_flow.source_sector;
                best_fit = fit;
            }

            rounds_done = round + 1;
            prev_combined = combined;

            if round > 0 && improvement.abs() < config::IMPERSONATE_CONVERGENCE_DELTA {
                println!("  [impersonate] converged - improvement below threshold");
                break;
            }
        }

        ImpersonationResult {
            persona_name: persona_name.to_string(),
            prompt: prompt.to_string(),
            text: best_text,
            winning_sector: best_sector,
            winning_color: sector_color(best_sector),
            quality_score: best_quality,
            persona_fit: best_fit,
            rounds: rounds_done,
        }
    }

    /// Computes how well a sector fits a persona's dominant sectors.
    fn sector_fit(sector: &u16, dominant: &[(u16, f64)]) -> f64 {
        for &(s, w) in dominant {
            if s == *sector {
                return w * dominant.len() as f64;
            }
        }
        let n = crate::wave::Wave::sector_count();
        for &(s, w) in dominant {
            let dist = ((*sector as i16 - s as i16).rem_euclid(n as i16)).unsigned_abs();
            if dist <= 2 {
                return w * dominant.len() as f64 * config::IMPERSONATE_ADJACENT_FIT_FACTOR;
            }
        }
        0.0
    }
}

impl Default for Impersonator {
    fn default() -> Self {
        Self::new()
    }
}
