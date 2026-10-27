#[allow(unused_imports)]
use super::*;

/// Channels updated per training token. Updating a random subset each step is
/// dropout-like regularisation and keeps the cost of a token update bounded.
pub const CHANNELS_PER_UPDATE: usize = 16;

// ── RELATION CHANNELS ──────────────────────────────────────────────────────

/// Kuramoto rate for coupling a pair on the channel of the relation it was
/// observed under. Matches `relational::RATE`, where it was measured.
pub const RELATION_RATE: f64 = 0.06;

/// How far apart two words may stand and still be coupled as a pair.
pub use phiecs::phasor::facet::RELATION_WINDOW;

/// Marker channels the relation inventory may claim, above the 8 offsets.
///
/// Channel ownership: the ranking objective moves only channels no relation
/// claims (`RelationVocab::unclaimed`), so this decides the split between
/// relation channels and prediction channels. 56 hands every channel to a
/// relation and the ranking objective falls back to all 64, which is the
/// arrangement `bin/promoted` first measured — both objectives fighting over
/// the same angles. 24 leaves 32 channels to prediction.
pub use phiecs::relation::MARKER_CHANNELS_MAX;

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

/// Log-frequency amplitude scale (the spectral phasor's, in phiecs).
pub use phiecs::phasor::spectral::AMPLITUDE_LOG_SCALE;

/// Highest band_n that contributes to effective phase. 2π/64 ÷ α = 13.45, so
/// capping at 13 keeps the fine-structure correction inside one sector and
/// stops converged words from walking around the circle.
pub use phiton::units::BAND_N_EFFECTIVE_MAX;

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
pub const FORMAT_VERSION: u32 = 6;

/// Below this many observations on a channel, a word's target-side step is
/// multiplied by `RARE_TARGET_BOOST`. A target side moves only when the
/// word is the later word of a pair, so rare words' target sides are
/// under-trained and full-vocabulary perplexity pays (docs/71 §1).
pub const RARE_TARGET_FLOOR: u32 = 16;
/// See `RARE_TARGET_FLOOR`. 1.0 disables.
///
/// Measured at 3.0 on the Rust Book harness, directed reading (docs/72):
/// identity 158.5 → 158.5, hinge 153.4 → 157.2, mixture 142.8 → 145.8 —
/// the operators alone get worse — while the best mix with the n-gram
/// tables improves 125.52 → 125.25, the largest phase gain over the tables
/// yet, and still a third of a point. Not a win; disabled, kept reproducible.
pub const RARE_TARGET_BOOST: f64 = 1.0;

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
