use crate::command::Context;
use crate::command::Parser;
use crate::sources::json::JsonDictionarySource;
use crate::sources::local::LocalSource;
use crate::sources::phi4::Phi4Source;
use crate::sources::wiktionary::WiktionarySource;
use crate::sources::DictionarySource;

/// Ingest — bulk ingestion of definitions and models from various sources.
///
/// Subcommands:
/// - `ingest <file.txt>`          — local text file
/// - `ingest-json <file.json>`    — JSON dictionary
/// - `ingest-wiktionary <file>`   — Wiktionary JSON/JSONL dump
/// - `ingest-phi4 [dir]`          — Learn from Phi-4 model in refs
pub struct Ingest;

impl Ingest {
    /// Ingests definitions from a local text file.
    pub fn local(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: ingest <file.txt>");
            return true;
        }
        let path = Parser::strip_quotes(ctx.arg);
        self.ingest_and_report(ctx, &LocalSource::new(&path), 50)
    }

    /// Ingests definitions from a JSON dictionary file.
    pub fn json(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: ingest-json <file.json> [epochs]");
            return true;
        }
        let parts: Vec<&str> = ctx.arg.split_whitespace().collect();
        let path = Parser::strip_quotes(parts[0]);
        let epochs: usize = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(30);
        self.ingest_and_report(ctx, &JsonDictionarySource::new(&path), epochs)
    }

    /// Ingests definitions from a Wiktionary dump file.
    pub fn wiktionary(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: ingest-wiktionary <file>");
            return true;
        }
        let path = Parser::strip_quotes(ctx.arg);
        self.ingest_and_report(ctx, &WiktionarySource::new(&path), 30)
    }

    /// Learns from the Phi-4 model in the refs folder.
    pub fn phi4(&self, ctx: &mut Context) -> bool {
        let path = if ctx.arg.is_empty() {
            "refs/Phi-4-multimodal-instruct"
        } else {
            ctx.arg
        };

        println!("  ── LEARNING FROM PHI-4 MODEL REFERENCES ──");
        println!("  Source path: {}", path);

        let source = Phi4Source::discover();
        let summary = source.learn_into_facet(ctx.manifold, ctx.trainer);

        println!("  [Phi-4 Ingestion Summary]");
        println!("    • Tiktoken tokens loaded: {}", summary.vocab_tokens_loaded);
        println!("    • BPE token merges trained: {}", summary.merges_trained);
        println!("    • Doc & prompt sentences trained: {}", summary.doc_sentences_trained);
        println!("    • Total Facet Vocabulary: {} words\n", summary.final_vocabulary_size);

        true
    }

    /// Ingests all definitions from a source and reports the result.
    fn ingest_and_report(
        &self,
        ctx: &mut Context,
        source: &dyn DictionarySource,
        epochs: usize,
    ) -> bool {
        let entries = source.fetch_all();
        if entries.is_empty() {
            println!("  [error] No definitions found.");
            return true;
        }

        println!(
            "  [ingesting] {} definitions, {} epochs...",
            entries.len(),
            epochs,
        );

        for _ in 0..epochs {
            for (word, def) in &entries {
                ctx.trainer.train_definition(ctx.manifold, word, def);
            }
        }

        println!("  [done] facet: {} words", ctx.manifold.vocabulary_size());
        true
    }
}
