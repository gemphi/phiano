use crate::chunker::ChunkStore;
use crate::command::Context;
use crate::command::Parser;

/// Chunk - splits a large JSON dictionary into letter-based chunk files.
///
/// Usage: `chunk <dictionary.json>`
///
/// Creates `data/chunks/<letter>/<letter>.json` files for parallel ingestion.
pub struct Chunk;

impl Chunk {
    /// Splits the specified JSON dictionary into chunk files.
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: chunk <dictionary.json>");
            println!("  Splits a large JSON dictionary into letter-based subfolders.");
            return true;
        }

        let path = Parser::strip_quotes(ctx.arg);
        let store = ChunkStore::new("data/chunks");
        let count = store.split(&path);
        println!("  [chunk] {} words split into data/chunks/", count);
        true
    }
}
