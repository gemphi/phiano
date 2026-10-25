#[allow(unused_imports)]
use super::*;

// ── FUNDAMENTAL CONSTANTS ──────────────────────────────────────────────────

/// Fine-structure constant (α) and the golden ratio (φ): one source,
/// phiton's units (docs/159). The physics tests used to assert these were
/// equal to phiton's; now they are phiton's.
pub use phiton::units::{ALPHA, PHI};

/// Golden ratio conjugate / inverse (1/φ) - 0.6180339887498948...
pub const PHI_CONJUGATE: f64 = 1.0 / PHI;

/// Golden ratio squared (φ²) - 2.6180339887498948...
pub const PHI_SQUARED: f64 = PHI * PHI;

/// Golden angle in radians - 2π/φ² ≈ 2.39996 — phinet's, the one the seed uses.
pub use phistd::GOLDEN_ANGLE;

/// Pi (π) - 3.141592653589793...
pub const PI_CONST: f64 = PI;

/// Full circle (2π) — phiton's.
pub use phiton::units::TWO_PI;

/// Euler's number (e) - 2.718281828459045...
/// Used in exponential decay/growth for novelty, sync, saturation.
pub const E: f64 = std::f64::consts::E;

// ── PRIME NUMBERS ──────────────────────────────────────────────────────────

/// The first prime ≥ 64 - mixing constant for sector hashing.
pub const PRIME_64: u64 = 67;

/// The first prime ≥ 128 - mixing constant at 128-sector resolution.
pub const PRIME_128: u64 = 131;

/// The first prime ≥ 256 - mixing constant at 256-sector resolution.
pub const PRIME_256: u64 = 257;

/// The first prime ≥ 512 - mixing constant at 512-sector resolution.
pub const PRIME_512: u64 = 521;

/// The first prime ≥ 1024 - mixing constant at 1024-sector resolution.
pub const PRIME_1024: u64 = 1031;

// ── SECTOR RESOLUTION (2ⁿ where n ≥ 6) ────────────────────────────────────

/// Sector resolution - number of sectors dividing the 2π phase circle.
/// Must be a power of 2 with exponent ≥ 6 (64, 128, 256, 512, 1024).
pub const SECTOR_RESOLUTION: u16 = 64;

/// Minimum sector resolution exponent (2⁶ = 64).
pub const SECTOR_EXPONENT_MIN: u32 = 6;

/// Number of color bands in the sector color wheel (always 16).
pub const COLOR_BANDS: u16 = 16;

// ── LEARNING PARAMETERS ────────────────────────────────────────────────────

/// Kuramoto learning rate for phase relaxation.
/// Recommended range: [0.01, 0.15]. Default: 0.05 (balanced).
pub const LEARNING_RATE: f64 = 0.05;

/// Kuramoto-Sakaguchi asymmetric syntax phase lag (β = π/16).
/// Enforces time-ordered syntactic flow in sequences.
pub const SYNTACTIC_LAG_BETA: f64 = PI_CONST / 16.0;

/// Instantaneous anti-phase pulse (π rad = 180°) for live negative feedback and self-correction.
#[allow(dead_code)]
pub const ANTI_PHASE_PULSE: f64 = PI_CONST;

/// Phase convergence threshold - when |sin(φ_target - φ_old)| < this,
/// band_n is incremented (prevents phase collapse).
pub const CONVERGENCE_THRESHOLD: f64 = 0.0005;

/// Amplitude increment per learning epoch (familiarity growth).
pub const AMPLITUDE_INCREMENT: f64 = 0.001;

/// Maximum amplitude - the familiarity ceiling.
pub const AMPLITUDE_MAX: f64 = 2.0;

/// Initial amplitude for new words.
pub const AMPLITUDE_INITIAL: f64 = 1.0;

/// Initial band_n for new words.
pub const BAND_N_INITIAL: u32 = 1;

/// Default number of epochs for bulk ingestion.
pub const INGEST_EPOCHS: usize = 64;

// ── COMPOSITION PARAMETERS ─────────────────────────────────────────────────

/// Default composition depth - how many sectors the river flow traverses.
pub const COMPOSE_DEPTH_DEFAULT: usize = 4;

/// Maximum composition depth - the deepest river flow allowed.
pub const COMPOSE_DEPTH_MAX: usize = 16;

/// Default number of recursive refinement rounds for composition.
pub const COMPOSE_ROUNDS_DEFAULT: usize = 8;

/// Weight for coherence in composition scoring.
pub const WEIGHT_COHERENCE: f64 = 0.25;

/// Weight for novelty in composition scoring.
pub const WEIGHT_NOVELTY: f64 = 0.15;

/// Weight for resonance in composition scoring.
pub const WEIGHT_RESONANCE: f64 = 0.15;

/// Weight for word diversity in composition scoring.
pub const WEIGHT_DIVERSITY: f64 = 0.10;

/// Weight for sector coverage in composition scoring.
pub const WEIGHT_COVERAGE: f64 = 0.05;

/// Weight for prompt alignment in composition scoring.
pub const WEIGHT_ALIGNMENT: f64 = 0.30;

/// Convergence delta for composition - stop refining if improvement is below this.
pub const COMPOSE_CONVERGENCE_DELTA: f64 = 0.001;

// ── PERSONA PARAMETERS ─────────────────────────────────────────────────────

/// Weight for word-level contributions to the fingerprint histogram.
pub const FINGERPRINT_WORD_WEIGHT: f64 = 0.3;

/// Number of dominant sectors shown in persona displays.
pub const PERSONA_DOMINANT_SECTORS: usize = 8;

/// Impersonation rounds - how many recursive refinement rounds.
pub const IMPERSONATE_ROUNDS_DEFAULT: usize = 4;

/// Impersonation quality weight vs persona fit weight.
pub const IMPERSONATE_QUALITY_WEIGHT: f64 = 0.4;

/// Impersonation fit weight.
pub const IMPERSONATE_FIT_WEIGHT: f64 = 0.6;

/// Convergence delta - stop refining if improvement is below this.
pub const IMPERSONATE_CONVERGENCE_DELTA: f64 = 0.001;

/// Quality sub-weights for impersonation scoring.
pub const IMPERSONATE_QUALITY_OVERALL: f64 = 0.5;
pub const IMPERSONATE_QUALITY_DIVERSITY: f64 = 0.2;
pub const IMPERSONATE_QUALITY_COVERAGE: f64 = 0.15;
pub const IMPERSONATE_QUALITY_LENGTH: f64 = 0.15;

/// Partial fit factor for adjacent sectors in sector_fit.
pub const IMPERSONATE_ADJACENT_FIT_FACTOR: f64 = 0.5;

// ── OSCILLATOR PARAMETERS ──────────────────────────────────────────────────

/// Frequency scaling factor for oscillator rotation speed.
pub const OSCILLATOR_FREQ_SCALE: f64 = 100.0;

/// Frequency tolerance for synchronization.
pub const OSCILLATOR_FREQ_TOLERANCE: f64 = 5.0;

/// Latitude scaling - maps amplitude to latitude on the sphere.
pub const OSCILLATOR_LAT_SCALE: f64 = 4.0;

/// Number of latitude bands shown in sphere projection.
pub const OSCILLATOR_LATITUDE_BANDS: usize = 8;

/// Warmup steps for oscillator training (gradual LR ramp-up).
pub const OSCILLATOR_WARMUP_STEPS: usize = 4;

/// Weight decay for oscillator training (amplitude regularization).
pub const OSCILLATOR_WEIGHT_DECAY: f64 = 0.001;

/// Convergence delta for oscillator training (stop if improvement < this).
pub const OSCILLATOR_CONVERGENCE_DELTA: f64 = 0.0005;
