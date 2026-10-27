//! Ingested personas on disk.
//!
//! A persona is the conditions a writer puts words under, aggregated — and
//! those conditions live in two places: the shared lexicon (saved as a
//! `.chroma`) and the writer's own sentences, which is what the fingerprint
//! and the pair evidence are read from. This stores the second half, so a
//! dialogue can load a persona in milliseconds instead of re-ingesting a
//! corpus. `bin/persona_ingest` writes it, `bin/persona_chat` reads it.

use phiecs::phasor::facet::Facet;
use crate::persona::Styles;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct StoredPersona {
    name: String,
    /// Every sentence this persona was ingested from.
    examples: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Stored {
    /// The lexicon these personas were ingested against.
    lexicon: String,
    personas: Vec<StoredPersona>,
}

pub struct PersonaStore;

impl PersonaStore {
    /// Writes every persona's name and sentences, and which lexicon they
    /// were ingested against.
    pub fn save(world: &Styles, lexicon: &str, path: &str) -> std::io::Result<()> {
        let mut personas: Vec<StoredPersona> = world.personas.values().map(|p| StoredPersona { name: p.name.clone(), examples: p.examples.clone() }).collect();
        personas.sort_by(|a, b| a.name.cmp(&b.name));
        let stored = Stored { lexicon: lexicon.to_string(), personas };
        let json = serde_json::to_string(&stored).map_err(std::io::Error::other)?;
        std::fs::write(path, json)
    }

    /// The lexicon path a store was ingested against.
    pub fn lexicon_of(path: &str) -> std::io::Result<String> {
        let text = std::fs::read_to_string(path)?;
        let stored: Stored = serde_json::from_str(&text).map_err(std::io::Error::other)?;
        Ok(stored.lexicon)
    }

    /// Rebuilds the world by *reading* `facet` — the lexicon saved beside the
    /// store already carries what ingestion put there, so loading must not
    /// train it again. The fingerprint is re-extracted rather than stored:
    /// it is a reading of the lexicon, and a lexicon that has moved since
    /// ingestion must give a different one.
    pub fn load(path: &str, facet: &Facet) -> std::io::Result<Styles> {
        let text = std::fs::read_to_string(path)?;
        let stored: Stored = serde_json::from_str(&text).map_err(std::io::Error::other)?;
        let mut world = Styles::new();
        for p in stored.personas {
            world.add_persona_read(&p.name, &p.examples, facet);
        }
        Ok(world)
    }
}
