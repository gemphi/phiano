use crate::command::Context;
use crate::memory::{MemoryBand, MEMORY_LAYERS};

/// Stats — displays facet and memory statistics.
///
/// Shows vocabulary size, average amplitude, dominant band, centroid wave,
/// and memory layer counts organized by band.
pub struct Stats;

impl Stats {
    /// Prints facet and memory statistics to stdout.
    pub fn apply(&self, ctx: &mut Context) -> bool {
        crate::config::print_summary();
        println!();
        println!("  ── facet ──");
        println!("  Vocabulary:     {} words", ctx.manifold.vocabulary_size());
        println!("  Avg amplitude:  {:.4}", ctx.manifold.average_amplitude());
        println!("  Dominant band:  n={}", ctx.manifold.dominant_band());

        let c = ctx.manifold.centroid();
        println!("  Centroid wave:  ({:.4}, {:.4})", c.re, c.im);

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
