#[allow(unused_imports)]
use super::*;

// ── FUNDAMENTAL CONSTANTS ──────────────────────────────────────────────────

/// Fine-structure constant (α) and the golden ratio (φ): one source,
/// phiton's units (docs/159). The physics tests used to assert these were
/// equal to phiton's; now they are phiton's.
/// The golden ratio's conjugate (1/φ = 0.618...) is phiton's too.
pub use phiton::units::{ALPHA, PHI, PHI_CONJUGATE};


/// Golden angle in radians - 2π/φ² ≈ 2.39996 — phinet's, the one the seed uses.
pub use phistd::GOLDEN_ANGLE;


/// Full circle (2π) — phiton's.
pub use phiton::units::TWO_PI;








// ── SECTOR RESOLUTION (2ⁿ where n ≥ 6) ────────────────────────────────────

/// Sector resolution - number of sectors dividing the 2π phase circle.
/// Must be a power of 2 with exponent ≥ 6 (64, 128, 256, 512, 1024).
pub use phiecs::phasor::facet::SECTOR_RESOLUTION;

/// Minimum sector resolution exponent (2⁶ = 64).
pub use phiecs::phasor::facet::SECTOR_EXPONENT_MIN;


// ── LEARNING PARAMETERS ────────────────────────────────────────────────────

/// Kuramoto learning rate for phase relaxation.
/// Recommended range: [0.01, 0.15]. Default: 0.05 (balanced).
pub use phient::train::constants::LEARNING_RATE;

/// Kuramoto-Sakaguchi asymmetric syntax phase lag (β = π/16).
/// Enforces time-ordered syntactic flow in sequences.
pub use phiecs::phasor::facet::SYNTACTIC_LAG_BETA;

/// Instantaneous anti-phase pulse (π rad = 180°) for live negative feedback and self-correction.
#[allow(dead_code)]
pub const ANTI_PHASE_PULSE: f64 = PI;

/// Phase convergence threshold - when |sin(φ_target - φ_old)| < this,
/// band_n is incremented (prevents phase collapse).
pub use phient::train::constants::CONVERGENCE_THRESHOLD;

/// Amplitude increment per learning epoch (familiarity growth).
pub const AMPLITUDE_INCREMENT: f64 = 0.001;

/// Maximum amplitude - the familiarity ceiling (the spectral phasor's, in phiecs).
pub use phiecs::phasor::spectral::AMPLITUDE_MAX;

/// Initial amplitude for new words.
pub use phiecs::phasor::facet::AMPLITUDE_INITIAL;

/// Initial band_n for new words.
pub use phiecs::phasor::facet::BAND_N_INITIAL;

/// Default number of epochs for bulk ingestion.
pub const INGEST_EPOCHS: usize = 64;

// ── COMPOSITION PARAMETERS ─────────────────────────────────────────────────

/// Default composition depth - how many sectors the river flow traverses.
pub const COMPOSE_DEPTH_DEFAULT: usize = 4;

/// Maximum composition depth - the deepest river flow allowed.
pub const COMPOSE_DEPTH_MAX: usize = 16;

/// Default number of recursive refinement rounds for composition.
pub const COMPOSE_ROUNDS_DEFAULT: usize = 8;

pub use phient::generate::constants::WEIGHT_COHERENCE;

pub use phient::generate::constants::WEIGHT_NOVELTY;

pub use phient::generate::constants::WEIGHT_RESONANCE;

pub use phient::generate::constants::WEIGHT_DIVERSITY;

pub use phient::generate::constants::WEIGHT_COVERAGE;

pub use phient::generate::constants::WEIGHT_ALIGNMENT;

pub use phient::generate::constants::COMPOSE_CONVERGENCE_DELTA;

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
pub use phiton::physics::oscillator::OSCILLATOR_FREQ_SCALE;

/// Frequency tolerance for synchronization.
pub use phiton::physics::oscillator::OSCILLATOR_FREQ_TOLERANCE;

/// Latitude scaling - maps amplitude to latitude on the sphere.
pub use phiton::physics::oscillator::OSCILLATOR_LAT_SCALE;

/// Number of latitude bands shown in sphere projection.
pub use phiton::physics::oscillator::OSCILLATOR_LATITUDE_BANDS;

/// Warmup steps for oscillator training (gradual LR ramp-up).
pub use phiton::physics::oscillator::OSCILLATOR_WARMUP_STEPS;

/// Weight decay for oscillator training (amplitude regularization).
pub use phiton::physics::oscillator::OSCILLATOR_WEIGHT_DECAY;

/// Convergence delta for oscillator training (stop if improvement < this).
pub use phiton::physics::oscillator::OSCILLATOR_CONVERGENCE_DELTA;
