/// Central configuration constants for the Phiano chromatic resonance engine.
///
/// All tunable parameters and mathematical constants live here.
/// The system is built on four fundamental constants:
///
///   α  (alpha)  - fine-structure constant (~1/137)
///   φ  (phi)    - golden ratio (1.618...)
///   π  (pi)     - circle constant (3.14159...)
///   2ⁿ          - power-of-2 sector resolution (2⁶ = 64 minimum)

use std::f64::consts::PI;

// ── FUNDAMENTAL CONSTANTS ──────────────────────────────────────────────────

/// Fine-structure constant (α) - spectral interference coupling.
/// α ≈ 1/137 - controls sub-band phase spacing:
///   φ_effective = φ + n·α
pub const ALPHA: f64 = 1.0 / 137.0;

/// Golden ratio (φ) - 1.6180339887498948...
/// Used for deterministic seeding, amplitude growth, color distribution.
pub const PHI: f64 = 1.6180339887498948;

/// Golden ratio conjugate / inverse (1/φ) - 0.6180339887498948...
pub const PHI_CONJUGATE: f64 = 1.0 / PHI;

/// Golden ratio squared (φ²) - 2.6180339887498948...
pub const PHI_SQUARED: f64 = PHI * PHI;

/// Golden angle in radians - 2π/φ² ≈ 2.39996...
/// Produces the most uniform distribution of points on a circle/sphere.
pub const GOLDEN_ANGLE: f64 = 2.0 * PI / PHI_SQUARED;

/// Pi (π) - 3.141592653589793...
pub const PI_CONST: f64 = PI;

/// Full circle (2π) - 6.283185307179586...
pub const TWO_PI: f64 = 2.0 * PI;

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

// ── EVALUATION PARAMETERS ──────────────────────────────────────────────────

/// Novelty distance scaling factor.
/// novelty = 1 - exp(-distance × NOVELTY_SCALE)
pub const NOVELTY_SCALE: f64 = 0.3;

/// Coherence weight in overall eval score.
pub const EVAL_WEIGHT_COHERENCE: f64 = 0.4;

/// Novelty weight in overall eval score.
pub const EVAL_WEIGHT_NOVELTY: f64 = 0.3;

/// Resonance weight in overall eval score.
pub const EVAL_WEIGHT_RESONANCE: f64 = 0.3;

// ── RAY CASTING ────────────────────────────────────────────────────────────

/// Default pool size for ray cast in composition.
pub const RAY_CAST_POOL_SIZE: usize = 512;

/// Default top-K for ray cast word queries.
pub const RAY_CAST_DEFAULT_K: usize = 16;

// ── GENERATIVE & TORUS CONSTANTS ───────────────────────────────────────────

/// Number of discrete harmonics evaluated on the multi-frequency torus (T^D).
pub const TORUS_HARMONICS_COUNT: usize = 32;

/// Default subject→verb syntactic phase lag (β_ij).
#[allow(dead_code)]
pub const SYNTAX_LAG_DEFAULT: f64 = SYNTACTIC_LAG_BETA;

/// EMA rate for learning pairwise phase lags from word order.
pub const SYNTAX_LAG_LEARN_RATE: f64 = 0.08;

/// Anti-phase pulse applied to a misunderstood concept (π radians).
pub const PHASE_REPULSION: f64 = std::f64::consts::PI;

/// Candidate pool size for torus attractor decoding.
pub const TORUS_DECODE_POOL: usize = 48;

/// Default context window ring buffer capacity.
pub const DEFAULT_CONTEXT_WINDOW: usize = 4096;

/// Default reasoning chain iteration limit.
pub const DEFAULT_REASONING_STEPS: usize = 5;

/// Default syntactic forward phase momentum velocity.
pub const SYNTACTIC_MOMENTUM_DEFAULT: f64 = 0.15;

/// Default definition chain search depth.
pub const DEFINITION_CHAIN_DEPTH: usize = 3;

/// Maximum characters extracted from Wikipedia introduction.
pub const WIKI_SNIPPET_MAX_CHARS: usize = 1200;

// ── FILE PATHS ─────────────────────────────────────────────────────────────

/// Path to the serialized facet (lexicon) binary file.
pub const CHROMA_FILE: &str = "data/manifold.chroma";

/// Path to the serialized 16-layer memory log.
pub const MEMORY_FILE: &str = "data/memory.chroma";

/// Path to the dictionary chunk directory.
pub const CHUNK_STORE_DIR: &str = "data/chunks";

/// Path to the local definitions text file.
pub const DEFINITIONS_FILE: &str = "data/definitions.txt";

/// Path to the API response cache file.
pub const API_CACHE_FILE: &str = "data/api_cache.txt";

/// Path to the stop words file (one word per line, space-separated).
pub const STOP_WORDS_FILE: &str = "data/stop_words.txt";

// ── ROTOR MULTI-DIMENSIONAL PHASES (CAPACITY UPGRADE) ──────────────────────

/// Number of auxiliary phase dimensions per word in the rotor manifold.
///
/// The primary phasor stays 1-D for fast paths; rotor dims add independent
/// circular coordinates so lexical capacity scales beyond single-circle
/// collision limits. Empty rotor map = legacy d=1 model (backward compatible).
pub const ROTOR_DIMS: usize = 16;

/// Blend weight of rotor-dim divergence in ray-cast energy deltas.
pub const ROTOR_DIM_WEIGHT: f64 = 0.03;

// ── CONTRASTIVE NEGATIVE SAMPLING ───────────────────────────────────────────

/// Number of negative samples drawn per training sentence.
pub const NEGATIVE_SAMPLES_K: usize = 3;

/// Repulsion magnitude (fraction of π) applied to negative samples.
pub const NEGATIVE_REPULSION: f64 = 0.1;

// ── META-PLASTICITY (HOMEOSTATIC SELF-TUNING) ───────────────────────────────

/// Lower bound for the adaptive learning-rate multiplier.
pub const META_LR_MULT_MIN: f64 = 0.4;

/// Upper bound for the adaptive learning-rate multiplier.
pub const META_LR_MULT_MAX: f64 = 1.6;

/// Lower bound for the semantic/syntax mixing parameter.
pub const META_MIX_MIN: f64 = 0.5;

/// Upper bound for the semantic/syntax mixing parameter.
pub const META_MIX_MAX: f64 = 0.9;

// ── EPISODIC RECALL (MEMORY READ PATH) ─────────────────────────────────────

/// Number of memo episodes recalled into generation context.
pub const RECALL_TOP_K: usize = 4;

/// Score boost multiplier for words present in recalled episodes.
pub const RECALL_BOOST: f64 = 1.3;

/// Phase bias strength toward recalled episode waves.
pub const RECALL_PHASE_BIAS: f64 = 0.15;

// ── CORRECTION JOURNAL (UNDOABLE NEGATIVE FEEDBACK) ─────────────────────────

/// Maximum retained correction records for undo.
pub const CORRECTION_JOURNAL_MAX: usize = 64;

// ── AUTONOMOUS STUDY LOOP ───────────────────────────────────────────────────

/// Default number of gap words self-studied per `study` invocation.
pub const STUDY_BUDGET_DEFAULT: usize = 8;

// ── PASSAGE GENERATION (SPIDER-NET DISCOURSE PLANNER) ───────────────────────

/// Default number of sentences in a planned passage.
pub const PASSAGE_SENTENCES_DEFAULT: usize = 4;

/// Token cap per planned sentence.
pub const PASSAGE_TOKENS_PER_SENTENCE: usize = 14;

/// Max memo episodes used to build the discourse spider-net.
pub const PASSAGE_MEMO_WINDOW: usize = 40;

// ── EXECUTION VERIFIER ──────────────────────────────────────────────────────

/// Bigram probability above which a word pair counts as supported.
pub const VERIFY_SUPPORT_BIGRAM_P: f64 = 0.05;

/// Rotor-dim resonance above which a word pair counts as supported.
pub const VERIFY_SUPPORT_RESONANCE: f64 = 0.8;

/// Tiny phase pull applied to supported pairs (reinforcement).
pub const VERIFY_REINFORCE_PULL: f64 = 0.01;

/// Tiny phase push applied to unsupported pairs (punishment).
pub const VERIFY_REPULSE: f64 = 0.02;


// ── MULTI-CHANNEL PHASE REPRESENTATION (D-dimensional torus) ───────────────

/// Number of independent phase channels per word (D).
///
/// A single angle gives S^1: at 64-sector resolution that is 64 distinguishable
/// states for the whole vocabulary. D independent channels give a representation
/// on the torus T^D, where similarity is mean phase coherence across channels.
/// 64 channels at one byte each costs 64 bytes/word.
pub const PHASE_CHANNELS: usize = 64;

/// Quantisation of each channel: theta_k = phases[k] * 2π / CHANNEL_QUANTA.
pub const CHANNEL_QUANTA: f64 = 256.0;

/// Channels updated per training token. Updating a random subset each step is
/// dropout-like regularisation and keeps the cost of a token update bounded.
pub const CHANNELS_PER_UPDATE: usize = 16;

// ── CONTRASTIVE OBJECTIVE ──────────────────────────────────────────────────

/// Negative samples drawn per token per training step.
///
/// Kuramoto coupling is attraction-only, and attraction-only dynamics have a
/// single stable attractor: total synchronisation. The negative term is what
/// makes the fixed point informative instead of degenerate.
pub const NEG_SAMPLES: usize = 5;

/// Repulsion rate for negative samples, relative to the learning rate.
pub const NEG_RATE: f64 = 0.5;

/// Margin for the hinge loss on next-word retrieval.
pub const HINGE_MARGIN: f64 = 0.05;

/// Weight applied to function words when computing a sentence centroid.
/// Closed-class words appear in nearly every sentence; at full weight they
/// transitively couple the entire vocabulary and accelerate collapse.
pub const FUNCTION_WORD_WEIGHT: f64 = 0.1;

/// Log-frequency amplitude scale: A = 1 + ln(count) / AMPLITUDE_LOG_SCALE.
/// ln(1e6)/14 ≈ 0.99, so a millionfold range maps into [1.0, 2.0].
pub const AMPLITUDE_LOG_SCALE: f64 = 14.0;

/// Highest band_n that contributes to effective phase. 2π/64 ÷ α = 13.45, so
/// capping at 13 keeps the fine-structure correction inside one sector and
/// stops converged words from walking around the circle.
pub const BAND_N_EFFECTIVE_MAX: u32 = 13;

// ── RECURRENT CONTEXT STATE ────────────────────────────────────────────────

/// Base decay per token for the recurrent context state, |λ| < 1.
pub const CONTEXT_LAMBDA: f64 = 0.92;

/// Base rotation per token for the recurrent context state (ω).
/// Channel k uses λ_k and ω_k spread geometrically from these bases, giving
/// each channel its own timescale — a diagonal complex linear recurrence.
pub const CONTEXT_OMEGA: f64 = 0.11;

// ── COMPOSE TOURNAMENT (Huberman stage/measure ordering) ───────────────────

/// Repulsion applied to losing compositions, relative to the learning rate.
pub const LOSER_REPULSION: f64 = 0.25;

/// Sector-score spread below which the variant population is degenerate.
pub const SPREAD_ALARM: f64 = 0.01;

/// Bonus weight for words that appeared in a winning sector last round
/// (Huberman's killer heuristic, 1968).
pub const KILLER_BONUS: f64 = 0.15;

/// Weight for the composition length factor. Previously this term borrowed
/// WEIGHT_NOVELTY, which was therefore counted twice.
pub const WEIGHT_LENGTH: f64 = 0.15;

// ── PERSISTENCE ────────────────────────────────────────────────────────────

/// On-disk format version. Bumped when the serialized layout changes.
pub const FORMAT_VERSION: u32 = 3;

/// Grounding pass version. A facet whose stored version matches is not
/// re-grounded at startup.
pub const GROUNDING_VERSION: u32 = 2;

/// Number of relaxation rounds for definition grounding.
pub const GROUNDING_ROUNDS: usize = 5;

/// Turns between automatic checkpoints in the REPL.
pub const CHECKPOINT_EVERY_TURNS: usize = 20;

// ── MEMORY RECALL ──────────────────────────────────────────────────────────

/// Half-life for recency weighting in memory recall (7 days, in ms).
pub const RECALL_HALF_LIFE_MS: f64 = 7.0 * 24.0 * 3600.0 * 1000.0;

/// Number of past interactions recalled into the generation context.
pub const RECALL_K: usize = 3;

// ── CORRECTION ─────────────────────────────────────────────────────────────

/// Amplitude floor after a correction.
///
/// The floor was AMPLITUDE_INITIAL (1.0), so a repeatedly-corrected word could
/// never become *less* familiar than a word never seen — leaving no way to
/// represent "I have actively learned this is wrong" as distinct from "I have
/// no idea". Those are different epistemic states.
pub const CORRECTION_FLOOR: f64 = 0.3;

/// Default rotation for a graded correction, in radians.
pub const CORRECTION_STRENGTH: f64 = 0.3;

/// Path to the persisted correction journal.
pub const CORRECTION_FILE: &str = "data/corrections.json";

/// Phase dispersion below which generation warns that the manifold is degenerate.
pub const DEGENERACY_WARN: f64 = 0.2;

/// Ground startup phases by multi-channel composition rather than the
/// single-channel centroid.
///
/// The centroid grounder writes one channel of 64 and was measured to move no
/// relation metric. Composition across all 64 moves analogy MRR from
/// 0.0002 ± 0.0001 to 0.0270 ± 0.0031 over five seeds on 296 pairs. Both paths
/// remain reachable for one release so the switch can be reverted without a
/// rebuild; the old one is then retired.
pub const GROUND_BY_COMPOSITION: bool = true;

/// Weight of the pull back toward each word's trained phase during startup
/// composition.
///
/// This is a product decision, not a tuning one, because the sweep has no
/// dominant setting: alpha = 0.25 gives the best analogy MRR and alpha = 1.00
/// the best pair/random and a healthier manifold, monotonically, in opposite
/// directions. 0.5 sits between them, with the dispersion floor in
/// `cognitive::grounding` as the binding constraint rather than this constant.
pub const COMPOSITION_ANCHOR: f64 = 0.5;

/// The first 64 primes, one per phase channel.
///
/// The golden angle solves one problem: a single irrational rotation never
/// repeats, so positions do not collide. It does nothing about a second one.
/// Every channel currently derives its frequency from the same linear ramp
/// (`CONTEXT_OMEGA * (1 + 4·k/K)`), and linearly spaced frequencies are
/// **commensurate** — channel k and channel 2k share periods, alias into each
/// other, and carry overlapping information. Sixty-four channels that alias are
/// fewer than sixty-four channels.
///
/// Coprime moduli fix that, and this is the oldest result in the area: the
/// Chinese remainder theorem says residues modulo pairwise-coprime numbers are
/// independent and jointly determine the value. Give channel *k* a frequency
/// proportional to the *k*-th prime and no two channels share a period until
/// their product, so the composite state's period is the product of all 64
/// primes rather than the smallest common multiple of a ramp.
///
/// It is a different fix from the golden angle rather than a replacement for
/// it, and like everything else here it is a switch with a measurement behind
/// it, not an assertion.
pub const CHANNEL_PRIMES: [u32; 64] = [
    2, 3, 5, 7, 11, 13, 17, 19,
    23, 29, 31, 37, 41, 43, 47, 53,
    59, 61, 67, 71, 73, 79, 83, 89,
    97, 101, 103, 107, 109, 113, 127, 131,
    137, 139, 149, 151, 157, 163, 167, 173,
    179, 181, 191, 193, 197, 199, 211, 223,
    227, 229, 233, 239, 241, 251, 257, 263,
    269, 271, 277, 281, 283, 293, 307, 311,
];

/// Use prime-spaced channel frequencies instead of the linear ramp.
pub const PRIME_CHANNEL_SPACING: bool = true;

/// Modulus the prime frequencies are taken against.
///
/// `omega_k = 2*pi * p_k / PRIME_MODULUS`, and the size of this constant is the
/// whole design, not a detail.
///
/// The first value tried was 313 — the next prime above the largest channel
/// prime — on the reasoning that every channel should stay under one turn per
/// step. That satisfies coprimality and destroys the thing the recurrence
/// actually needs. At 313 the frequencies span 0.04 to 6.24 rad/step, so the
/// fastest channels rotate almost a full turn per step and are
/// indistinguishable from very slow ones running backwards. Measured, it cost
/// 3% of phase-alone perplexity against the linear ramp it replaced
/// (175.10 → 180.92).
///
/// A recurrence needs a *range of timescales*: slow channels that remember far
/// back, fast ones that track the last few tokens, and an ordering between
/// them. A large modulus keeps every frequency slow and well-separated while
/// leaving them pairwise incommensurate, which is what the primes were for.
/// 4099 is prime, so no channel's period divides another's.
pub const PRIME_MODULUS: f64 = 4099.0;
