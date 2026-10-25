#[allow(unused_imports)]
use super::*;

/// The recurrence kernel for channel `k`: decay and rotation per step.
///
/// One definition, used by the language model, the sentence benchmark and the
/// generation buffer, so the three cannot silently drift apart — they had three
/// copies of the same formula before this.
///
/// The rotation is where [`CHANNEL_PRIMES`] enters. A linear frequency ramp
/// makes channel *k* and channel *2k* commensurate: they share periods and
/// carry overlapping information, so the effective channel count is smaller
/// than 64. Prime-proportional frequencies are pairwise coprime, so no two
/// channels share a period until their product.
///
/// Decay stays a smooth ramp: `lambda` sets how far back a channel remembers,
/// and there is no aliasing argument for spacing memory horizons by primes.
pub fn channel_kernel(k: usize, channels: usize) -> num_complex::Complex64 {
    let frac = k as f64 / channels.max(1) as f64;
    let lambda = CONTEXT_LAMBDA.powf(1.0 + 3.0 * frac);
    let omega = match PRIME_CHANNEL_SPACING {
        true => {
            let p = CHANNEL_PRIMES[k % CHANNEL_PRIMES.len()] as f64;
            TWO_PI * p / PRIME_MODULUS
        }
        false => CONTEXT_OMEGA * (1.0 + 4.0 * frac),
    };
    num_complex::Complex64::from_polar(lambda, omega)
}
