use crate::compose::better::{Evaluator as ComposerEvaluator, SectorScore};
use crate::compose::flow::RiverFlow;
use crate::compose::worse::Discarder;
use crate::compose::{SectorPalette, Composition};
use crate::facet::Facet;
use crate::trainer::Trainer;

/// CompositionTuner - the "Monitor" in the Flower-Hayes model.
///
/// The monitor orchestrates the recursive writing process:
///
///   1. PROPOSE: Generate 64 sector variations (flow.rs)
///   2. EVALUATE: Score each variation (better.rs)
///   3. DISCARD + TRAIN: Keep the better, discard the worse,
///      train on the better (worse.rs + Kuramoto)
///   4. RECURSE: Generate again with the re-tuned facet
///   5. CONVERGE: Stop when the top score stops improving
///
/// This is how a child learns to write:
/// - The teacher gives a prompt and examples
/// - The child tries many approaches (64 sectors)
/// - The teacher says "this is better, this is worse"
/// - The child internalizes the better patterns
/// - Next attempt is improved because the internal model shifted
/// - Repeat until the composition is good
///
/// In Phiano, "internalizing" = Kuramoto phase relaxation on the
/// winning texts. The facet literally re-tunes itself.
pub struct CompositionTuner {
    /// Maximum recursive refinement rounds.
    pub max_rounds: usize,
    /// How many sectors to visit in each river flow (context depth).
    pub depth: usize,
    /// The evaluator (better.rs).
    pub evaluator: ComposerEvaluator,
    /// The discarder (worse.rs).
    pub discarder: Discarder,
}

impl CompositionTuner {
    /// Creates a new tuner with the given max rounds.
    ///
    /// Defaults: depth=8 sectors per flow, keep top 16, discard bottom 16.
    pub fn new(max_rounds: usize) -> Self {
        Self {
            max_rounds,
            depth: 8,
            evaluator: ComposerEvaluator::new(),
            discarder: Discarder::new(),
        }
    }

    /// Runs the full recursive refinement loop.
    ///
    /// This is the heart of the compose system. It cycles through
    /// propose → evaluate → discard → train → recurse until convergence.
    pub fn refine(
        &mut self,
        facet: &mut Facet,
        trainer: &Trainer,
        prompt: &str,
        examples: &[String],
    ) -> Composition {
        let mut prev_best: f64 = 0.0;
        let mut best_score: SectorScore;
        let mut all_scores: Vec<SectorScore>;
        let mut rounds_completed = 0;

        for round in 0..self.max_rounds {
            println!("  [round {}/{}] generating 64 sector variations...", round + 1, self.max_rounds);

            // Phase 1: PROPOSE - generate 64 variations
            let flows = RiverFlow::generate_variations(facet, prompt, self.depth);

            // Phase 2: EVALUATE - score all 64
            all_scores = self.evaluator.evaluate_variations(facet, &flows);

            best_score = all_scores[0].clone();
            let avg = self.evaluator.average_score(&all_scores);
            let spread = self.evaluator.score_spread(&all_scores);

            println!(
                "  [round {}/{}] best: {:.4} (sector {} {}) avg: {:.4} spread: {:.4}",
                round + 1,
                self.max_rounds,
                best_score.score,
                best_score.sector,
                best_score.color,
                avg,
                spread,
            );

            // Phase 3: DISCARD + TRAIN - keep better, discard worse, train on better
            let result = self.discarder.discard_and_train(facet, trainer, &all_scores);
            self.discarder.print_summary(&result);

            // Phase 4: CONVERGE - check if we're done
            if self.evaluator.has_converged(&all_scores, prev_best) && round > 0 {
                println!("  [converged] improvement below threshold");
                rounds_completed = round + 1;
                break;
            }

            prev_best = best_score.score;
            rounds_completed = round + 1;
        }

        // Final evaluation with the re-tuned facet
        let final_flows = RiverFlow::generate_variations(facet, prompt, self.depth);
        let final_scores = self.evaluator.evaluate_variations(facet, &final_flows);

        let winner = &final_scores[0];
        let sector_scores: Vec<(u16, f64)> = final_scores
            .iter()
            .map(|s| (s.sector, s.score))
            .collect();

        let eval = crate::eval::Evaluator::new().eval(facet, &winner.text);

        Composition {
            prompt: prompt.to_string(),
            text: winner.text.clone(),
            winning_sector: winner.sector,
            winning_color: SectorPalette::color(winner.sector),
            rounds: rounds_completed,
            eval,
            sector_scores,
            examples: examples.to_vec(),
        }
    }
}
