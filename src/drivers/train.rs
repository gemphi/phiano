use crate::chunker::ChunkStore;
use crate::command::Context;

/// Train — trains the facet from all chunked dictionary files in parallel.
///
/// Usage: `train [epochs]`
///
/// Loads all chunks from `data/chunks/` and trains the facet using
/// rayon for parallel file loading.
pub struct Train;

impl Train {
    /// Trains the facet from all chunks with the specified number of epochs.
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: train [epochs]");
            println!("  Loads all chunks from data/chunks/ and trains in parallel.");
            return true;
        }

        let epochs: usize = ctx.arg.trim().parse().unwrap_or(30);
        let store = ChunkStore::new("data/chunks");
        let metrics = store.ingest_parallel(ctx.manifold, ctx.trainer, epochs);
        metrics.report();
        true
    }
}
