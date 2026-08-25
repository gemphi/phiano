use crate::chunker::ChunkStore;
use crate::command::Context;
use crate::command::Parser;
use crate::config;
use crate::sources::api::ApiSource;
use crate::sources::local::LocalSource;
use crate::sources::DictionarySource;

/// Define - fetches, displays, and learns a word's rich definition.
///
/// Usage: `define <word>`
///
/// Tries the Free Dictionary API first (with pronunciation, parts of speech, and examples),
/// then falls back to the offline 102,217-word chunk store, then local definitions.
/// Trains the facet on each definition found.
pub struct Define;

impl Define {
    /// Fetches definitions for the given word and trains the facet.
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: define <word>");
            return true;
        }

        let word = Parser::strip_quotes(ctx.arg).to_lowercase();
        println!("  Fetching definition for '{}'...\n", word);

        let api = ApiSource::new(config::API_CACHE_FILE);

        // 1. Try Free Dictionary API for rich definition
        if let Some(rich_def) = api.fetch_word_rich(&word) {
            println!("{}", rich_def);
            let defs = api.fetch_word(&word);
            for def in &defs {
                ctx.trainer.train_definition(ctx.manifold, &word, def);
            }
            println!("  [trained] facet: {} words", ctx.manifold.vocabulary_size());
            return true;
        }

        // 2. Fall back to offline ChunkStore (102,217 definitions from Webster's)
        let chunker = ChunkStore::new("data/chunks");
        if let Some(chunk_def) = chunker.load_definition(&word) {
            println!("{}\n(Offline Dictionary)\n{}", word, chunk_def);
            ctx.trainer.train_definition(ctx.manifold, &word, &chunk_def);
            println!("\n  [trained] facet: {} words", ctx.manifold.vocabulary_size());
            return true;
        }

        // 3. Fall back to local definitions file
        self.try_local(ctx, &word)
    }

    /// Falls back to the local definitions file when API and chunks have no results.
    fn try_local(&self, ctx: &mut Context, word: &str) -> bool {
        let local = LocalSource::new(config::DEFINITIONS_FILE);
        let local_defs = local.fetch_definitions(word);

        if local_defs.is_empty() {
            println!(
                "  No definitions found for '{}'. Try: learn \"{} <definition>\"",
                word, word,
            );
            return true;
        }

        for def in &local_defs {
            println!("  (local) {}", def);
            ctx.trainer.train_definition(ctx.manifold, word, def);
        }
        println!("  [trained] facet: {} words", ctx.manifold.vocabulary_size());
        true
    }
}
