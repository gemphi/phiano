//! Phase-space configuration functions.
//!
//! Provides derived values and utilities computed from the raw constants
//! defined in [`constants`]. Every function is an associated method on
//! [`PhiConfig`], eliminating bare standalone functions in accordance
//! with the Diem convention that all public symbols belong to a named type.
//!
//! # Thread Safety
//!
//! All methods are stateless and safe to call from any thread. The stop-word
//! set is lazily initialised via [`OnceLock`] and cached for the process lifetime.
//!
//! # Invariants
//!
//! - `SECTOR_RESOLUTION` is always a power of two with exponent ≥ `SECTOR_EXPONENT_MIN`.
//! - `sector_prime()` returns a value coprime to `SECTOR_RESOLUTION`, preventing
//!   aliasing in phase-to-sector mapping.
//! - Weight sums are normalised: `total_compose_weight()` equals the sum of all
//!   `WEIGHT_*` constants.

use super::constants::*;

/// Central configuration façade for the Phiano phase-resonance engine.
///
/// All methods are stateless associated functions (`PhiConfig::method_name()`).
/// No instances are ever created—this struct exists solely to namespace
/// configuration logic, following the Diem convention.
///
/// # Architecture
///
/// ```text
/// constants.rs         functions.rs (this file)
/// ┌────────────┐       ┌───────────────────────┐
/// │ ALPHA, PHI │──────▶│ PhiConfig::sector_*()  │
/// │ SECTOR_RES │       │ PhiConfig::eval_*()    │
/// │ WEIGHT_*   │       │ PhiConfig::stop_word*()│
/// │ PRIME_*    │       │ PhiConfig::print_*()   │
/// └────────────┘       └───────────────────────┘
/// ```
pub struct PhiConfig;

impl PhiConfig {
    /// Returns the appropriate prime mixing constant for the current sector
    /// resolution.
    ///
    /// Primes are coprime to power-of-2 sector counts, preventing aliasing
    /// artifacts in phase-to-sector mapping.
    ///
    /// # Invariant
    ///
    /// The returned value `p` satisfies `gcd(p, SECTOR_RESOLUTION) == 1`.
    pub fn sector_prime() -> u64 {
        match SECTOR_RESOLUTION {
            64 => PRIME_64,
            128 => PRIME_128,
            256 => PRIME_256,
            512 => PRIME_512,
            1024 => PRIME_1024,
            _ => PRIME_64,
        }
    }

    /// Returns the configured sector resolution as a `u16`.
    ///
    /// # Panics
    ///
    /// Panics if `SECTOR_RESOLUTION` is not a power of two with exponent
    /// ≥ `SECTOR_EXPONENT_MIN`.
    pub fn sector_resolution() -> u16 {
        let s = SECTOR_RESOLUTION;
        if s < (1 << SECTOR_EXPONENT_MIN) || !s.is_power_of_two() {
            panic!(
                "SECTOR_RESOLUTION must be a power of 2 with exponent >= {} (min {}), got {}",
                SECTOR_EXPONENT_MIN,
                1u16 << SECTOR_EXPONENT_MIN,
                s,
            );
        }
        s
    }

    /// Returns the exponent `n` such that `SECTOR_RESOLUTION = 2ⁿ`.
    pub fn sector_exponent() -> u32 {
        Self::sector_resolution().trailing_zeros()
    }

    /// Returns the number of sectors per colour band.
    ///
    /// Each colour band spans `SECTOR_RESOLUTION / COLOR_BANDS` contiguous
    /// sectors in the phase ring.
    pub fn sectors_per_color() -> u16 {
        Self::sector_resolution() / COLOR_BANDS
    }

    /// Number of sector variations generated per composition round.
    pub fn compose_variations() -> usize {
        Self::sector_resolution() as usize
    }

    /// Total composition weight (for normalisation).
    ///
    /// Returns the sum of all `WEIGHT_*` constants:
    /// `WEIGHT_COHERENCE + WEIGHT_NOVELTY + WEIGHT_RESONANCE
    ///  + WEIGHT_DIVERSITY + WEIGHT_COVERAGE + WEIGHT_ALIGNMENT`.
    pub fn total_compose_weight() -> f64 {
        WEIGHT_COHERENCE + WEIGHT_NOVELTY + WEIGHT_RESONANCE
            + WEIGHT_DIVERSITY + WEIGHT_COVERAGE + WEIGHT_ALIGNMENT
    }

    /// Computes the overall evaluation score from component scores.
    ///
    /// `score = coherence × EVAL_WEIGHT_COHERENCE
    ///        + novelty   × EVAL_WEIGHT_NOVELTY
    ///        + resonance × EVAL_WEIGHT_RESONANCE`
    pub fn eval_overall(coherence: f64, novelty: f64, resonance: f64) -> f64 {
        coherence * EVAL_WEIGHT_COHERENCE
            + novelty * EVAL_WEIGHT_NOVELTY
            + resonance * EVAL_WEIGHT_RESONANCE
    }

    /// Returns `true` if `word` is a stop word.
    ///
    /// The stop-word set is lazily loaded from `STOP_WORDS_FILE` on first call
    /// and cached for the process lifetime.
    pub fn is_stop_word(word: &str) -> bool {
        Self::stop_words().contains(&word.to_lowercase())
    }

    /// Returns the number of stop words loaded from file.
    pub fn stop_word_count() -> usize {
        Self::stop_words().len()
    }

    /// Prints a human-readable summary of all configuration values to stdout.
    pub fn print_summary() {
        println!("  ── configuration ──");
        println!();
        println!("  fundamental constants:");
        println!("    α (alpha)     = {:.12}", ALPHA);
        println!("    φ (phi)       = {:.12}", PHI);
        println!("    π (pi)        = {:.12}", PI_CONST);
        println!("    e             = {:.12}", E);
        println!("    golden angle  = {:.12}", GOLDEN_ANGLE);
        println!();
        println!("  sector resolution:");
        println!("    sectors       = {} (2^{})", Self::sector_resolution(), Self::sector_exponent());
        println!("    color bands   = {}", COLOR_BANDS);
        println!("    sectors/color = {}", Self::sectors_per_color());
        println!("    sector prime  = {}", Self::sector_prime());
        println!();
        println!("  learning:");
        println!("    learning rate = {}", LEARNING_RATE);
        println!("    convergence   = {}", CONVERGENCE_THRESHOLD);
        println!("    amp increment = {}", AMPLITUDE_INCREMENT);
        println!("    amp max       = {}", AMPLITUDE_MAX);
        println!("    ingest epochs = {}", INGEST_EPOCHS);
        println!();
        println!("  composition:");
        println!("    variations    = {}", Self::compose_variations());
        println!("    depth default = {}", COMPOSE_DEPTH_DEFAULT);
        println!("    depth max     = {}", COMPOSE_DEPTH_MAX);
        println!("    rounds        = {}", COMPOSE_ROUNDS_DEFAULT);
        println!("    weights       = coh {:.2} + nov {:.2} + res {:.2} + div {:.2} + cov {:.2} + align {:.2} = {:.2}",
            WEIGHT_COHERENCE, WEIGHT_NOVELTY, WEIGHT_RESONANCE,
            WEIGHT_DIVERSITY, WEIGHT_COVERAGE, WEIGHT_ALIGNMENT,
            Self::total_compose_weight());
        println!();
        println!("  persona:");
        println!("    word weight   = {}", FINGERPRINT_WORD_WEIGHT);
        println!("    dominant shown = {}", PERSONA_DOMINANT_SECTORS);
        println!("    imp. rounds   = {}", IMPERSONATE_ROUNDS_DEFAULT);
        println!("    imp. quality  = {:.2}", IMPERSONATE_QUALITY_WEIGHT);
        println!("    imp. fit      = {:.2}", IMPERSONATE_FIT_WEIGHT);
        println!("    stop words    = {}", Self::stop_word_count());
        println!();
        println!("  oscillator:");
        println!("    freq scale    = {}", OSCILLATOR_FREQ_SCALE);
        println!("    freq tolerance= {}", OSCILLATOR_FREQ_TOLERANCE);
        println!("    lat scale     = {}", OSCILLATOR_LAT_SCALE);
        println!("    lat bands     = {}", OSCILLATOR_LATITUDE_BANDS);
        println!();
        println!("  evaluation:");
        println!("    novelty scale = {}", NOVELTY_SCALE);
        println!("    eval weights  = coh {:.2} + nov {:.2} + res {:.2}",
            EVAL_WEIGHT_COHERENCE, EVAL_WEIGHT_NOVELTY, EVAL_WEIGHT_RESONANCE);
        println!();
        println!("  ray casting:");
        println!("    pool size     = {}", RAY_CAST_POOL_SIZE);
        println!("    default k     = {}", RAY_CAST_DEFAULT_K);
    }

    // ── private helpers ─────────────────────────────────────────────

    /// Returns the cached set of stop words loaded from `STOP_WORDS_FILE`.
    fn stop_words() -> &'static std::collections::HashSet<String> {
        use std::sync::OnceLock;
        static WORDS: OnceLock<std::collections::HashSet<String>> = OnceLock::new();
        WORDS.get_or_init(|| {
            let text = std::fs::read_to_string(STOP_WORDS_FILE)
                .unwrap_or_default();
            text.split_whitespace()
                .map(|w| w.to_lowercase())
                .collect()
        })
    }
}
