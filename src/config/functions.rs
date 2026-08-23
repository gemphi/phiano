/// Config functions — derived values and utilities computed from constants.
///
/// This module separates functions from the raw constant values in
/// `constants.rs`. Functions here validate, compute, or look up values
/// based on the configured constants.

use super::constants::*;

/// Returns the appropriate prime mixing constant for the current sector
/// resolution. Primes are coprime to power-of-2 sector counts, preventing
/// aliasing artifacts in phase-to-sector mapping.
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

/// Returns the configured sector resolution as a u16.
///
/// Validates that the value is a power of 2 with exponent ≥ 6.
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

/// Returns the exponent n such that SECTOR_RESOLUTION = 2ⁿ.
pub fn sector_exponent() -> u32 {
    sector_resolution().trailing_zeros()
}

/// Returns the number of sectors per color band.
pub fn sectors_per_color() -> u16 {
    sector_resolution() / COLOR_BANDS
}

/// Number of sector variations generated per composition round.
pub fn compose_variations() -> usize {
    sector_resolution() as usize
}

/// Total composition weight (for normalization). Sum of all WEIGHT_* constants.
pub fn total_compose_weight() -> f64 {
    WEIGHT_COHERENCE + WEIGHT_NOVELTY + WEIGHT_RESONANCE
        + WEIGHT_DIVERSITY + WEIGHT_COVERAGE + WEIGHT_ALIGNMENT
}

/// Computes the overall eval score from component scores.
pub fn eval_overall(coherence: f64, novelty: f64, resonance: f64) -> f64 {
    coherence * EVAL_WEIGHT_COHERENCE
        + novelty * EVAL_WEIGHT_NOVELTY
        + resonance * EVAL_WEIGHT_RESONANCE
}

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

/// Returns true if a word is a stop word.
pub fn is_stop_word(word: &str) -> bool {
    stop_words().contains(&word.to_lowercase())
}

/// Returns the number of stop words loaded from file.
pub fn stop_word_count() -> usize {
    stop_words().len()
}

/// Prints a summary of all configuration values.
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
    println!("    sectors       = {} (2^{})", sector_resolution(), sector_exponent());
    println!("    color bands   = {}", COLOR_BANDS);
    println!("    sectors/color = {}", sectors_per_color());
    println!("    sector prime  = {}", sector_prime());
    println!();
    println!("  learning:");
    println!("    learning rate = {}", LEARNING_RATE);
    println!("    convergence   = {}", CONVERGENCE_THRESHOLD);
    println!("    amp increment = {}", AMPLITUDE_INCREMENT);
    println!("    amp max       = {}", AMPLITUDE_MAX);
    println!("    ingest epochs = {}", INGEST_EPOCHS);
    println!();
    println!("  composition:");
    println!("    variations    = {}", compose_variations());
    println!("    depth default = {}", COMPOSE_DEPTH_DEFAULT);
    println!("    depth max     = {}", COMPOSE_DEPTH_MAX);
    println!("    rounds        = {}", COMPOSE_ROUNDS_DEFAULT);
    println!("    weights       = coh {:.2} + nov {:.2} + res {:.2} + div {:.2} + cov {:.2} + align {:.2} = {:.2}",
        WEIGHT_COHERENCE, WEIGHT_NOVELTY, WEIGHT_RESONANCE,
        WEIGHT_DIVERSITY, WEIGHT_COVERAGE, WEIGHT_ALIGNMENT,
        total_compose_weight());
    println!();
    println!("  persona:");
    println!("    word weight   = {}", FINGERPRINT_WORD_WEIGHT);
    println!("    dominant shown = {}", PERSONA_DOMINANT_SECTORS);
    println!("    imp. rounds   = {}", IMPERSONATE_ROUNDS_DEFAULT);
    println!("    imp. quality  = {:.2}", IMPERSONATE_QUALITY_WEIGHT);
    println!("    imp. fit      = {:.2}", IMPERSONATE_FIT_WEIGHT);
    println!("    stop words    = {}", stop_word_count());
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
