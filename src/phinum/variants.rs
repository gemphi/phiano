/// Phinum variation engine — 16/32/64 level primitives.
///
/// Each level doubles the classification granularity:
/// - **Phinum16**: 16 core categories — fast, coarse
/// - **Phinum32**: 32 core categories — balanced
/// - **Phinum64**: 64 core categories — fine, complete
///
/// The "64 ways to look at anything" principle: every linguistic unit
/// (word, phrase, sentence, paragraph) can be classified at any of
/// these three levels. The variations and their links form the spider
/// net that captures language without storing examples.

use serde::{Deserialize, Serialize};

/// The three Phinum granularity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhinumLevel {
    /// 16 core classifications — coarse, fast.
    N16,
    /// 32 core classifications — balanced.
    N32,
    /// 64 core classifications — fine, complete.
    N64,
}

impl PhinumLevel {
    /// Returns the number of categories at this level.
    pub fn count(self) -> usize {
        match self {
            Self::N16 => 16,
            Self::N32 => 32,
            Self::N64 => 64,
        }
    }

    /// Returns the bit width for indexing at this level.
    pub fn bits(self) -> u32 {
        match self {
            Self::N16 => 4,
            Self::N32 => 5,
            Self::N64 => 6,
        }
    }

    /// Returns the next finer level, or None if already at N64.
    pub fn finer(self) -> Option<Self> {
        match self {
            Self::N16 => Some(Self::N32),
            Self::N32 => Some(Self::N64),
            Self::N64 => None,
        }
    }

    /// Returns the next coarser level, or None if already at N16.
    pub fn coarser(self) -> Option<Self> {
        match self {
            Self::N16 => None,
            Self::N32 => Some(Self::N16),
            Self::N64 => Some(Self::N32),
        }
    }
}

/// A single variation slot within a Phinum level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Variation {
    /// The Phinum level this variation belongs to.
    pub level: PhinumLevel,
    /// Index [0, count) within the level.
    pub index: u8,
}

impl Variation {
    pub fn new(level: PhinumLevel, index: u8) -> Self {
        let max = level.count() as u8;
        Self { level, index: index % max }
    }

    /// Returns a human-readable label for this variation.
    pub fn label(self) -> String {
        let prefix = match self.level {
            PhinumLevel::N16 => "n16",
            PhinumLevel::N32 => "n32",
            PhinumLevel::N64 => "n64",
        };
        format!("{}#{}", prefix, self.index)
    }
}

/// Trait for Phinum-level engines.
pub trait PhinumEngine: Sized {
    const LEVEL: PhinumLevel;

    /// Total number of classification slots.
    const SLOTS: usize;

    /// Classifies a hash value into a variation slot.
    fn classify_hash(hash: u64) -> Variation {
        let mask = (Self::SLOTS as u64) - 1;
        Variation::new(Self::LEVEL, ((hash & mask) % Self::SLOTS as u64) as u8)
    }

    /// Classifies a string into a variation slot using the global config.
    fn classify_str(s: &str) -> Variation {
        let hash = super::config::PhinumConfig::global().hash_str(s);
        Self::classify_hash(hash)
    }
}

/// 16-core classification engine — coarse, fast.
#[derive(Debug, Clone, Copy)]
pub struct Phinum16;

impl PhinumEngine for Phinum16 {
    const LEVEL: PhinumLevel = PhinumLevel::N16;
    const SLOTS: usize = 16;
}

/// 32-core classification engine — balanced.
#[derive(Debug, Clone, Copy)]
pub struct Phinum32;

impl PhinumEngine for Phinum32 {
    const LEVEL: PhinumLevel = PhinumLevel::N32;
    const SLOTS: usize = 32;
}

/// 64-core classification engine — fine, complete.
#[derive(Debug, Clone, Copy)]
pub struct Phinum64;

impl PhinumEngine for Phinum64 {
    const LEVEL: PhinumLevel = PhinumLevel::N64;
    const SLOTS: usize = 64;
}
