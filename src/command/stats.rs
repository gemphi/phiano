use crate::command::Context;
use crate::memory::{MemoryBand, MEMORY_LAYERS};

/// Stats - displays facet and memory statistics.
///
/// Shows vocabulary size, average amplitude, dominant band, centroid wave,
/// and memory layer counts organized by band.
pub struct Stats;

impl Stats {
    /// Prints facet and memory statistics to stdout.
    pub fn apply(&self, ctx: &mut Context) -> bool {
        crate::config::PhiConfig::print_summary();
        println!();
        println!("  ── facet ──");
        println!("  Vocabulary:     {} words", ctx.manifold.vocabulary_size());
        println!("  Avg amplitude:  {:.4}", ctx.manifold.average_amplitude());
        println!("  Dominant band:  n={}", ctx.manifold.dominant_band());

        let c = ctx.manifold.centroid();
        println!("  Centroid wave:  ({:.4}, {:.4})", c.re, c.im);

        // Manifold health. Coherence-style scores rise as the lexicon
        // synchronises, so they must never be read without these beside them.
        let dispersion = ctx.manifold.phase_dispersion();
        let gini = ctx.manifold.sector_gini();
        println!();
        println!("  ── manifold health ──");
        println!("  Phase dispersion: {:.4}  (1.0 spread, 0.0 collapsed)", dispersion);
        println!("  Sector Gini:      {:.4}  (0.0 even, 1.0 one sector holds all)", gini);

        let hist = crate::wave::Wave::sector_histogram(ctx.manifold);
        let max = hist.iter().copied().max().unwrap_or(0);
        let occupied = hist.iter().filter(|c| **c > 0).count();
        println!("  Sectors occupied: {}/{}  (busiest holds {})", occupied, hist.len(), max);
        if max > 0 {
            print!("  Occupancy:        ");
            for count in hist.iter().step_by(hist.len() / 32) {
                let level = (*count as f64 / max as f64 * 8.0).round() as usize;
                print!("{}", [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"][level.min(8)]);
            }
            println!();
        }
        if dispersion < crate::config::DEGENERACY_WARN {
            println!("  [WARN] dispersion below {:.2} — the lexicon is synchronising;",
                crate::config::DEGENERACY_WARN);
            println!("         see docs/how/02_the_kuramoto_step.md");
        }

        if !ctx.corrections.is_empty() {
            println!();
            println!("  ── corrections ──");
            println!("  Taught and journalled: {}", ctx.corrections.len());
        }

        println!();
        println!("  ── memory ──");
        if ctx.memory.is_empty() {
            println!("  (no interactions recorded yet)");
        } else {
            println!("  Memory entries: {}", ctx.memory.len());
            println!("  Memory layers:");

            for layer in 0..MEMORY_LAYERS {
                let count = ctx.memory.layer_count(layer);
                if count > 0 {
                    let band = MemoryBand::from_layer(layer);
                    println!("    L{:>2} ({:>7}): {} entries", layer, band, count);
                }
            }
        }
        true
    }
}
