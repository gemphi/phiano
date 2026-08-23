pub mod ingest;
pub mod chunk;
pub mod train;

use crate::command::Context;

/// Driver — represents a source/device command, separate from the core.
///
/// Like Unix device drivers, these are not part of the kernel core.
/// They interface with external data sources (files, APIs, dumps, models)
/// and feed data into the facet. The core Command enum stays clean.
///
/// Drivers are dispatched before core commands — if a line matches
/// a driver command, it's handled here. Otherwise it falls through
/// to the core Dispatcher.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Driver {
    Ingest,
    IngestJson,
    IngestWiktionary,
    IngestPhi4,
    Chunk,
    Train,
}

impl Driver {
    /// Parses a command name string into a Driver variant.
    ///
    /// Returns None if the string is not a driver command.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ingest" => Some(Self::Ingest),
            "ingest-json" => Some(Self::IngestJson),
            "ingest-wiktionary" => Some(Self::IngestWiktionary),
            "ingest-phi4" | "learn-phi4" => Some(Self::IngestPhi4),
            "chunk" => Some(Self::Chunk),
            "train" => Some(Self::Train),
            _ => None,
        }
    }

    /// Dispatches a driver command.
    ///
    /// Returns true if handled (REPL continues), false if it should stop.
    /// Drivers always return true — they never exit the REPL.
    pub fn dispatch<'a>(driver: Self, ctx: &mut Context<'a>) -> bool {
        match driver {
            Self::Ingest => ingest::Ingest.local(ctx),
            Self::IngestJson => ingest::Ingest.json(ctx),
            Self::IngestWiktionary => ingest::Ingest.wiktionary(ctx),
            Self::IngestPhi4 => ingest::Ingest.phi4(ctx),
            Self::Chunk => chunk::Chunk.apply(ctx),
            Self::Train => train::Train.apply(ctx),
        }
    }

    /// Tries to handle a line as a driver command.
    ///
    /// Returns Some(true/false) if handled, None if not a driver command.
    pub fn try_dispatch<'a>(line: &'a str, ctx: &mut Context<'a>) -> Option<bool> {
        let parts: Vec<&str> = line.splitn(2, char::is_whitespace).collect();
        let driver = Self::from_str(parts[0])?;
        ctx.arg = parts.get(1).map(|s| s.trim()).unwrap_or("");
        Some(Self::dispatch(driver, ctx))
    }
}
