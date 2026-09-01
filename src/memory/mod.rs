#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, ErrorKind, Result};
use std::time::{SystemTime, UNIX_EPOCH};

/// Total number of memory layers in the system.
pub const MEMORY_LAYERS: usize = 16;

/// Memory band - a group of four layers representing a depth of understanding.
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
    #[serde(default)]
    pub text: String,
    pub layer: usize,
}

/// 16-layer memory log - records every interaction and organizes it by depth.
///
/// `layers` holds *indices* into `entries` rather than clones of them. The
/// previous layout stored every interaction twice, including its full text.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Memo {
    pub entries: Vec<ContextWaveEntry>,
    #[serde(default)]
    pub layers: [Vec<usize>; MEMORY_LAYERS],
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
            text: text.to_string(),
            layer,
        };

        self.layers[layer].push(self.entries.len());
        self.entries.push(entry);
    }

    /// The `k` past interactions whose waves are closest to `query`.
    ///
    /// The memory log has always recorded every interaction with its wave, its
    /// timestamp and its text — and nothing ever read it back during inference.
    /// This is the retrieval half.
    pub fn recall(&self, query: (f64, f64), k: usize) -> Vec<&ContextWaveEntry> {
        let mut scored: Vec<(&ContextWaveEntry, f64)> = self
            .entries
            .iter()
            .map(|e| {
                let dx = query.0 - e.superposition_wave.0;
                let dy = query.1 - e.superposition_wave.1;
                (e, dx.hypot(dy))
            })
            .collect();
        scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(k).map(|(e, _)| e).collect()
    }

    /// Recency-weighted recall: distance is divided by an exponential decay in
    /// age, so an older memory must be substantially closer to outrank a recent
    /// one.
    pub fn recall_weighted(
        &self,
        query: (f64, f64),
        k: usize,
        half_life_ms: f64,
    ) -> Vec<&ContextWaveEntry> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as f64)
            .unwrap_or(0.0);

        let mut scored: Vec<(&ContextWaveEntry, f64)> = self
            .entries
            .iter()
            .map(|e| {
                let dx = query.0 - e.superposition_wave.0;
                let dy = query.1 - e.superposition_wave.1;
                let age = (now - e.timestamp_ms as f64).max(0.0);
                let recency = 0.5f64.powf(age / half_life_ms.max(1.0));
                (e, dx.hypot(dy) / recency.max(1e-6))
            })
            .collect();
        scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(k).map(|(e, _)| e).collect()
    }

    /// Novelty of a wave measured against *experience* rather than geometry.
    ///
    /// Distance to the nearest thing ever processed, squashed to [0, 1]. This
    /// does not degrade as the lexicon grows, and it cannot be driven to a
    /// constant by phase collapse the way centroid-distance novelty can.
    pub fn novelty(&self, query: (f64, f64)) -> f64 {
        if self.entries.is_empty() {
            return 1.0;
        }
        let nearest = self
            .entries
            .iter()
            .map(|e| {
                let dx = query.0 - e.superposition_wave.0;
                let dy = query.1 - e.superposition_wave.1;
                dx.hypot(dy)
            })
            .fold(f64::MAX, f64::min);
        1.0 - (-nearest).exp()
    }

    /// Rebuilds the layer index from `entries`. Used after loading a file
    /// written before layers held indices.
    pub fn reindex(&mut self) {
        for l in self.layers.iter_mut() {
            l.clear();
        }
        for (i, e) in self.entries.iter().enumerate() {
            let layer = e.layer.min(MEMORY_LAYERS - 1);
            self.layers[layer].push(i);
        }
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

    /// Saves the memory log to a binary file, atomically (write then rename).
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let tmp = format!("{}.tmp", path);
        {
            let file = File::create(&tmp)?;
            let writer = BufWriter::new(file);
            bincode::serialize_into(writer, self)
                .map_err(|e| Error::new(ErrorKind::Other, e))?;
        }
        std::fs::rename(&tmp, path)
    }

    /// Loads a memory log from a binary file using bincode deserialization.
    pub fn load_from_file(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut memo: Self =
            bincode::deserialize_from(reader).map_err(|e| Error::new(ErrorKind::Other, e))?;
        if memo.layers.iter().all(|l| l.is_empty()) && !memo.entries.is_empty() {
            memo.reindex();
        }
        Ok(memo)
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
