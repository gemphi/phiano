#[allow(unused_imports)]
use super::*;

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
pub use phient::train::constants::CHANNEL_PRIMES;

/// Use prime-spaced channel frequencies instead of the linear ramp.
pub use phient::metrics::kernel::PRIME_CHANNEL_SPACING;

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
pub use phient::train::constants::PRIME_MODULUS;
