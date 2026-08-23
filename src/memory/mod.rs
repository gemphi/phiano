#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Result};
use std::time::{SystemTime, UNIX_EPOCH};

/// Total number of memory layers in the system.
pub const MEMORY_LAYERS: usize = 16;

/// Memory band — a group of four layers representing a depth of understanding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryBand {
    Surface,
    Pattern,
    Semantic,
    Deep,
}

impl MemoryBand {
    /// Returns the starting layer index for this band (0, 4, 8, or 12).
    pub fn base_layer(self) -> usize {
        match self {
            Self::Surface => 0,
            Self::Pattern => 4,
            Self::Semantic => 8,
            Self::Deep => 12,
        }
    }

    /// Returns the band that contains the given layer index.
    pub fn from_layer(layer: usize) -> Self {
        match layer / 4 {
            0 => Self::Surface,
            1 => Self::Pattern,
            2 => Self::Semantic,
            _ => Self::Deep,
        }
    }
}

impl fmt::Display for MemoryBand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => write!(f, "surface"),
            Self::Pattern => write!(f, "pattern"),
            Self::Semantic => write!(f, "semantic"),
            Self::Deep => write!(f, "deep"),
        }
    }
}

/// A single recorded interaction in the memory log.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContextWaveEntry {
    pub timestamp_ms: u64,
    pub superposition_wave: (f64, f64),
    pub text_hash: u64,
    pub layer: usize,
}

/// 16-layer memory log — records every interaction and organizes it by depth.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Memo {
    pub entries: Vec<ContextWaveEntry>,
    pub layers: [Vec<ContextWaveEntry>; MEMORY_LAYERS],
}

impl Memo {
    /// Creates an empty memory log.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a new interaction in the memory log.
    pub fn record(&mut self, wave: (f64, f64), text: &str) {
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let text_hash = Self::fnv1a_hash(text);
        let layer = Self::classify_layer(text);

        let entry = ContextWaveEntry {
            timestamp_ms,
            superposition_wave: wave,
            text_hash,
            layer,
        };

        self.entries.push(entry.clone());
        self.layers[layer].push(entry);
    }

    /// Classifies input text into one of 16 memory layers.
    fn classify_layer(text: &str) -> usize {
        let words: Vec<&str> = text.split_whitespace().collect();
        let word_count = words.len();

        let avg_len = if word_count > 0 {
            words.iter().map(|w| w.len()).sum::<usize>() / word_count
        } else {
            0
        };

        let band = match word_count {
            0..=3 => MemoryBand::Surface,
            4..=8 => MemoryBand::Pattern,
            9..=16 => MemoryBand::Semantic,
            _ => MemoryBand::Deep,
        };

        let sub_layer = match avg_len {
            0..=4 => 0,
            5..=6 => 1,
            7..=8 => 2,
            _ => 3,
        };

        (band.base_layer() + sub_layer).min(15)
    }

    /// Returns the total number of recorded interactions.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the memory log contains no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the number of entries in a specific memory layer.
    pub fn layer_count(&self, layer: usize) -> usize {
        self.layers
            .get(layer)
            .map(|l| l.len())
            .unwrap_or(0)
    }

    /// Saves the memory log to a binary file using bincode serialization.
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, self)
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }

    /// Loads a memory log from a binary file using bincode deserialization.
    pub fn load_from_file(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        bincode::deserialize_from(reader)
            .map_err(|e| Error::new(ErrorKind::Other, e))
    }

    /// Computes an FNV-1a 64-bit hash of the input text.
    fn fnv1a_hash(text: &str) -> u64 {
        let mut hash: u64 = 14695981039346656037;
        for byte in text.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(1099511628211);
        }
        hash
    }
}
