//! Configurable Phinum classification parameters.
//!
//! Replaces hardcoded hash multipliers with a resource-driven,
//! configurable structure. The config holds per-level hash
//! multipliers and a string hash seed, all overridable at runtime.

use super::variants::PhinumLevel;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Configuration for Phinum classification engines.
///
/// Default values use small primes — the same values that were
/// previously hardcoded — but any configuration can be supplied
/// via [`PhinumConfig::new`] or deserialized from a resource file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhinumConfig {
    /// Multiplier for string hash folding (FNV-style).
    pub str_hash_multiplier: u64,
    /// Initial seed for string hash folding.
    pub str_hash_seed: u64,
    /// Multiplier for Phinum16 sub-classification hashing.
    pub n16_multiplier: u64,
    /// Multiplier for Phinum32 sub-classification hashing.
    pub n32_multiplier: u64,
    /// Multiplier for Phinum64 sub-classification hashing.
    pub n64_multiplier: u64,
}

impl Default for PhinumConfig {
    fn default() -> Self {
        Self {
            str_hash_multiplier: 31,
            str_hash_seed: 0,
            n16_multiplier: 17,
            n32_multiplier: 31,
            n64_multiplier: 67,
        }
    }
}

impl PhinumConfig {
    /// Accesses the global static config singleton.
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<PhinumConfig> = OnceLock::new();
        INSTANCE.get_or_init(Self::default)
    }

    /// Creates a custom configuration.
    pub fn new(
        str_hash_multiplier: u64,
        str_hash_seed: u64,
        n16_multiplier: u64,
        n32_multiplier: u64,
        n64_multiplier: u64,
    ) -> Self {
        Self {
            str_hash_multiplier,
            str_hash_seed,
            n16_multiplier,
            n32_multiplier,
            n64_multiplier,
        }
    }

    /// Returns the hash multiplier for a given Phinum level.
    pub fn multiplier(&self, level: PhinumLevel) -> u64 {
        match level {
            PhinumLevel::N16 => self.n16_multiplier,
            PhinumLevel::N32 => self.n32_multiplier,
            PhinumLevel::N64 => self.n64_multiplier,
        }
    }

    /// Computes a string hash using the configured multiplier and seed.
    pub fn hash_str(&self, s: &str) -> u64 {
        s.bytes().fold(self.str_hash_seed, |acc, b| {
            acc.wrapping_mul(self.str_hash_multiplier).wrapping_add(b as u64)
        })
    }

    /// Computes a hash from a base value using the level-specific multiplier.
    pub fn hash_base(&self, base: u64, level: PhinumLevel) -> u64 {
        base.wrapping_mul(self.multiplier(level))
    }
}
