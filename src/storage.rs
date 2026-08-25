use crate::config::ALPHA;
use crate::facet::Facet;
use crate::phasor::SpectralPhasor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Result};

/// Header metadata prepended to binary .chroma files.
///
/// Contains version info, vocabulary size, and the fine-structure alpha
/// used when the file was written, for compatibility checking.
#[derive(Serialize, Deserialize, Debug)]
pub struct ChromaHeader {
    /// File format version number.
    pub version: u32,
    /// Number of words in the lexicon when saved.
    pub vocabulary_size: usize,
    /// Fine-structure constant used during training.
    pub fine_structure_alpha: f64,
}

/// Serialized payload container for the entire facet.
///
/// This is the on-disk representation of a `Facet`, consisting of
/// a header and a clone of the lexicon.
#[derive(Serialize, Deserialize, Debug)]
pub struct SerializedFacet {
    /// File header with metadata.
    pub header: ChromaHeader,
    /// The word-to-phasor lexicon.
    pub lexicon: HashMap<String, SpectralPhasor>,
    /// Bigram transition counts: word_a -> {word_b -> count}.
    #[serde(default)]
    pub bigrams: HashMap<String, HashMap<String, u32>>,
    /// Trigram transition counts: "word_a word_b" -> {word_c -> count}.
    #[serde(default)]
    pub trigrams: HashMap<String, HashMap<String, u32>>,
    /// Learned Kuramoto-Sakaguchi phase lags β_ij.
    #[serde(default)]
    pub phase_lags: HashMap<String, HashMap<String, f64>>,
}

impl SerializedFacet {
    /// Creates a serialized facet from a `Facet` reference.
    pub fn from_facet(facet: &Facet) -> Self {
        Self {
            header: ChromaHeader {
                version: 1,
                vocabulary_size: facet.vocabulary_size(),
                fine_structure_alpha: ALPHA,
            },
            lexicon: facet.lexicon.clone(),
            bigrams: facet.bigrams.clone(),
            trigrams: facet.trigrams.clone(),
            phase_lags: facet.phase_lags.clone(),
        }
    }

    /// Deserializes back into a `Facet`.
    pub fn into_facet(self) -> Facet {
        Facet {
            lexicon: self.lexicon,
            bigrams: self.bigrams,
            trigrams: self.trigrams,
            phase_lags: self.phase_lags,
        }
    }

    /// Saves the serialized facet to a binary file using bincode.
    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let file = File::create(file_path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, self)
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }

    /// Loads a serialized facet from a binary file using bincode.
    pub fn load_from_file(file_path: &str) -> Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        bincode::deserialize_from(reader)
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }
}

/// Storage - facade for persisting and loading facets to/from disk.
pub struct Storage;

/// Legacy serialized facet (v1 format, no bigrams).
/// Used for backward-compatible loading of old .chroma files.
#[derive(Serialize, Deserialize, Debug)]
struct LegacySerializedFacet {
    pub header: ChromaHeader,
    pub lexicon: HashMap<String, SpectralPhasor>,
}

impl Storage {
    /// Saves a facet to a binary .chroma file.
    pub fn save(facet: &Facet, path: &str) -> Result<()> {
        SerializedFacet::from_facet(facet).save_to_file(path)
    }

    /// Loads a facet from a binary .chroma file.
    /// Tries the new format (with bigrams) first, falls back to legacy format.
    pub fn load(path: &str) -> Result<Facet> {
        // Try new format first
        match SerializedFacet::load_from_file(path) {
            Ok(sf) => Ok(sf.into_facet()),
            Err(_) => {
                // Fall back to legacy format (no bigrams field)
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                let legacy: LegacySerializedFacet = bincode::deserialize_from(reader)
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
                Ok(Facet {
                    lexicon: legacy.lexicon,
                    bigrams: HashMap::new(),
                    trigrams: HashMap::new(),
                    phase_lags: HashMap::new(),
                })
            }
        }
    }
}
