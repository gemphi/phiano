use crate::compose::better::SectorScore;
use crate::compose::sector_color;
use crate::facet::Facet;
use crate::trainer::Trainer;

/// Discarder — the "worse" half of the reviewing process.
///
/// Based on the Flower-Hayes "Revising" subprocess:
///   discarding weak material and reinforcing strong material.
///
/// After the evaluator (better.rs) scores all 64 sector variations,
/// the discarder:
/// 1. Identifies the bottom N sectors (the "worse" ones to discard)
/// 2. Identifies the top N sectors (the "better" ones to keep)
/// 3. Trains the facet on the better compositions (Kuramoto re-tuning)
/// 4. This reshapes the phase geometry for the next recursive round
///
/// The discarder does NOT delete words from the facet. Instead, it
/// trains on the winning texts, which pulls related words closer
/// together via Kuramoto coupling. The "worse" sectors naturally
/// drift apart as their words get pulled toward the winning clusters.
pub struct Discarder {
    /// Number of top sectors to keep and train on.
    pub keep_count: usize,
    /// Number of bottom sectors to discard.
    pub discard_count: usize,
}

/// DiscardResult — the outcome of a discard round.
#[derive(Debug, Clone)]
pub struct DiscardResult {
    /// Sectors that were kept (trained on).
    pub kept: Vec<u16>,
    /// Sectors that were discarded.
    pub discarded: Vec<u16>,
    /// The texts that were trained on.
    pub trained_texts: Vec<String>,
    /// Number of tokens updated during training.
    pub tokens_updated: usize,
}

impl Discarder {
    /// Creates a new discarder that keeps the top N and discards the bottom N.
    ///
    /// Defaults: keep top 16, discard bottom 16.
    pub fn new() -> Self {
        Self {
            keep_count: 16,
            discard_count: 16,
        }
    }

    /// Performs the discard + train cycle.
    ///
    /// 1. Takes the ranked sector scores (best first)
    /// 2. Keeps the top `keep_count` sectors
    /// 3. Discards the bottom `discard_count` sectors
    /// 4. Trains the facet on each kept sector's text
    /// 5. Returns the result for the monitor (tune.rs)
    pub fn discard_and_train(
        &self,
        facet: &mut Facet,
        trainer: &Trainer,
        scores: &[SectorScore],
    ) -> DiscardResult {
        let keep_end = self.keep_count.min(scores.len());
        let discard_start = scores.len().saturating_sub(self.discard_count);

        let kept: Vec<u16> = scores[..keep_end]
            .iter()
            .map(|s| s.sector)
            .collect();

        let discarded: Vec<u16> = scores[discard_start..]
            .iter()
            .map(|s| s.sector)
            .collect();

        let mut trained_texts = Vec::new();
        let mut tokens_updated = 0;

        for score in scores.iter().take(keep_end) {
            tokens_updated += trainer.train_sentence(facet, &score.text);
            trained_texts.push(score.text.clone());
        }

        DiscardResult {
            kept,
            discarded,
            trained_texts,
            tokens_updated,
        }
    }

    /// Prints a summary of the discard round.
    pub fn print_summary(&self, result: &DiscardResult) {
        print!("  [keep]   sectors: ");
        for (i, &sector) in result.kept.iter().enumerate().take(8) {
            if i > 0 {
                print!(", ");
            }
            print!("{} ({})", sector, sector_color(sector));
        }
        if result.kept.len() > 8 {
            print!(", ... ({} total)", result.kept.len());
        }
        println!();

        print!("  [discard] sectors: ");
        for (i, &sector) in result.discarded.iter().enumerate().take(8) {
            if i > 0 {
                print!(", ");
            }
            print!("{} ({})", sector, sector_color(sector));
        }
        if result.discarded.len() > 8 {
            print!(", ... ({} total)", result.discarded.len());
        }
        println!();

        println!(
            "  [train]  {} tokens updated across {} texts",
            result.tokens_updated,
            result.trained_texts.len(),
        );
    }
}

impl Default for Discarder {
    fn default() -> Self {
        Self::new()
    }
}
