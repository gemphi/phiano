#[allow(unused_imports)]
use super::*;

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

/// A trained Phinet operator that guides generation when present
/// (`Generator::with_operator`, `OperatorGuide::load`). Absent = unguided.
pub const OPERATOR_FILE: &str = "data/phinet_generate.fno";

/// Learned generator weights (`learn::GenWeight`), fit by `bin/learn` from
/// held-out text. When present the generator scores candidates with them
/// instead of the hand-tuned constants.
pub const LEARNED_FILE: &str = "data/learned.json";

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
