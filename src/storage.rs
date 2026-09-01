use crate::config::{ALPHA, FORMAT_VERSION};
use crate::facet::Facet;
use crate::phasor::SpectralPhasor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Result};

/// Header metadata prepended to binary .chroma files.
#[derive(Serialize, Deserialize, Debug)]
pub struct ChromaHeader {
    /// File format version number.
    pub version: u32,
    /// Number of words in the lexicon when saved.
    pub vocabulary_size: usize,
    /// Fine-structure constant used during training.
    pub fine_structure_alpha: f64,
}

/// Borrowed view of a facet for serialization.
///
/// Serializing by reference avoids cloning the entire model — lexicon, bigrams,
/// trigrams and phase lags — into a second copy before writing, which on a large
/// model doubled peak memory for the duration of every save.
#[derive(Serialize)]
struct FacetRef<'a> {
    header: ChromaHeader,
    lexicon: &'a HashMap<String, SpectralPhasor>,
    bigrams: &'a HashMap<String, HashMap<String, u32>>,
    trigrams: &'a HashMap<String, HashMap<String, u32>>,
    phase_lags: &'a HashMap<String, HashMap<String, f64>>,
    grounded_version: u32,
}

/// Owned on-disk representation of a `Facet`.
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
    /// Grounding pass already applied to this facet.
    #[serde(default)]
    pub grounded_version: u32,
}

impl SerializedFacet {
    /// Deserializes back into a `Facet`.
    pub fn into_facet(self) -> Facet {
        Facet {
            lexicon: self.lexicon,
            bigrams: self.bigrams,
            trigrams: self.trigrams,
            phase_lags: self.phase_lags,
            grounded_version: self.grounded_version,
            sample_pool: Vec::new(),
        }
    }

    /// Loads a serialized facet from a binary file using bincode.
    pub fn load_from_file(file_path: &str) -> Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        bincode::deserialize_from(reader).map_err(|e| Error::new(ErrorKind::Other, e))
    }
}

/// Storage - facade for persisting and loading facets to/from disk.
pub struct Storage;

/// Legacy serialized facet (v1 format, no bigrams).
#[derive(Serialize, Deserialize, Debug)]
struct LegacySerializedFacet {
    pub header: ChromaHeader,
    pub lexicon: HashMap<String, SpectralPhasor>,
}

impl Storage {
    /// Saves a facet to a binary .chroma file, atomically.
    ///
    /// The model is written to a sibling temporary file and then renamed over
    /// the target. `rename` is atomic on every mainstream filesystem, so an
    /// interrupted save can no longer leave a truncated model that fails to load
    /// and silently starts the next session from an empty lexicon.
    pub fn save(facet: &Facet, path: &str) -> Result<()> {
        let tmp = format!("{}.tmp", path);

        {
            let file = File::create(&tmp)?;
            let writer = BufWriter::new(file);
            let view = FacetRef {
                header: ChromaHeader {
                    version: FORMAT_VERSION,
                    vocabulary_size: facet.vocabulary_size(),
                    fine_structure_alpha: ALPHA,
                },
                lexicon: &facet.lexicon,
                bigrams: &facet.bigrams,
                trigrams: &facet.trigrams,
                phase_lags: &facet.phase_lags,
                grounded_version: facet.grounded_version,
            };
            bincode::serialize_into(writer, &view).map_err(|e| Error::new(ErrorKind::Other, e))?;
        }

        std::fs::rename(&tmp, path)
    }

    /// Loads a facet from a binary .chroma file.
    ///
    /// Tries the current format, then the legacy v1 format. A file written by a
    /// *newer* build is rejected with a message that names both versions, rather
    /// than falling through to "starting empty".
    pub fn load(path: &str) -> Result<Facet> {
        match SerializedFacet::load_from_file(path) {
            Ok(sf) => {
                if sf.header.version > FORMAT_VERSION {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "{} is format v{}, this build reads v{}",
                            path, sf.header.version, FORMAT_VERSION
                        ),
                    ));
                }
                if (sf.header.fine_structure_alpha - ALPHA).abs() > 1e-12 {
                    eprintln!(
                        "  [warn] {} was trained with alpha={:.9}, this build uses {:.9}",
                        path, sf.header.fine_structure_alpha, ALPHA
                    );
                }
                Ok(sf.into_facet())
            }
            Err(_) => {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                let legacy: LegacySerializedFacet = bincode::deserialize_from(reader)
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;
                Ok(Facet {
                    lexicon: legacy.lexicon,
                    bigrams: HashMap::new(),
                    trigrams: HashMap::new(),
                    phase_lags: HashMap::new(),
                    grounded_version: 0,
                    sample_pool: Vec::new(),
                })
            }
        }
    }
}
