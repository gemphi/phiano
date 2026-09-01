use crate::config::{
    TWO_PI, GOLDEN_ANGLE, TORUS_HARMONICS_COUNT,
    PHASE_CHANNELS, CHANNEL_QUANTA, BAND_N_EFFECTIVE_MAX, AMPLITUDE_LOG_SCALE,
    AMPLITUDE_MAX,
};
use crate::phical::Phical;
use crate::phiton::{LightQuantum, PhitonColor, PhitonSpectrum};
use crate::wave::c64;
use serde::{Deserialize, Serialize};

/// Number of 64-bit words needed to pack `PHASE_CHANNELS` single-byte channels.
const PACKED_WORDS: usize = PHASE_CHANNELS / 8;

/// FNV-1a 64-bit hash. Deterministic, fast, and dependency-free.
///
/// Used to seed a word's phase channels from its *identity* rather than its
/// length, which is what gives distinct words distinct starting positions.
pub fn fnv1a(text: &str) -> u64 {
    let mut hash: u64 = 14695981039346656037;
    for byte in text.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    hash
}

/// SpectralPhasor — a word's position on the phase torus T^D.
///
/// The complex wave representation of channel 0 is `Z = A * e^(i*(phi + n*alpha))`,
/// where alpha is the fine-structure constant. Beyond channel 0 the word carries
/// `PHASE_CHANNELS` independent phase angles, quantised to one byte each and
/// packed into `[u64; PACKED_WORDS]`.
///
/// The channels are what give the representation capacity. A single angle lives
/// on S^1, where at 64-sector resolution there are 64 distinguishable states for
/// the entire vocabulary; D independent channels live on T^D, where similarity
/// is mean phase coherence across channels and the binding constraint moves from
/// representation to data.
///
/// `phase` is retained as the canonical channel-0 view so that every existing
/// caller keeps working; `sync_phase` keeps the two consistent.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpectralPhasor {
    /// Primary phase angle on the unit circle, in radians [0, 2*pi).
    /// Mirrors channel 0.
    pub phase: f64,
    /// Amplitude / intensity / familiarity weight.
    pub amplitude: f64,
    /// Quantized fine-structure energy sub-band level (n = 1, 2, 3...).
    pub band_n: u32,
    /// Number of times this word has been observed in training.
    /// Drives log-frequency amplitude, which keeps its dynamic range useful
    /// across the six orders of magnitude a Zipfian vocabulary spans.
    #[serde(default)]
    pub count: u32,
    /// `PHASE_CHANNELS` independent phase channels, one byte each.
    #[serde(default)]
    packed: [u64; PACKED_WORDS],
}

impl SpectralPhasor {
    /// Creates a phasor with the given phase, amplitude, and band level.
    ///
    /// Channels are spread from the base phase by the golden angle, which is the
    /// most uniform deterministic spread available on a circle. Prefer
    /// [`SpectralPhasor::seeded`] for lexicon entries: it gives channels that
    /// carry independent information rather than a fixed function of one number.
    pub fn new(phase: f64, amplitude: f64, band_n: u32) -> Self {
        let mut p = Self {
            phase: phase.rem_euclid(TWO_PI),
            amplitude,
            band_n,
            count: 1,
            packed: [0; PACKED_WORDS],
        };
        for k in 0..PHASE_CHANNELS {
            p.set_theta(k, phase + (k as f64) * GOLDEN_ANGLE);
        }
        p.set_theta(0, p.phase);
        p
    }

    /// Creates a phasor seeded from a word's identity.
    ///
    /// Every channel is derived from an independent mix of the word's hash, so
    /// two distinct words occupy distinct positions in every channel. This
    /// replaces seeding by `word.len() * PHI`, under which all words of the same
    /// character length shared one exact starting angle — roughly twenty
    /// distinct positions for an entire vocabulary.
    pub fn seeded(word: &str, amplitude: f64, band_n: u32) -> Self {
        let h = fnv1a(word);
        let base = ((h >> 11) as f64 / (1u64 << 53) as f64) * TWO_PI;

        let mut p = Self {
            phase: base.rem_euclid(TWO_PI),
            amplitude,
            band_n,
            count: 1,
            packed: [0; PACKED_WORDS],
        };
        // Each channel gets its own hash stream: re-mix the word hash with the
        // channel index, then spread by the golden angle so that even correlated
        // hash bits land far apart on the circle.
        for k in 0..PHASE_CHANNELS {
            let mut hk = h ^ (k as u64).wrapping_mul(0x9E3779B97F4A7C15);
            hk = hk.wrapping_mul(1099511628211);
            hk ^= hk >> 29;
            let frac = (hk >> 11) as f64 / (1u64 << 53) as f64;
            p.set_theta(k, frac * TWO_PI + (k as f64) * GOLDEN_ANGLE);
        }
        p.set_theta(0, p.phase);
        p
    }

    /// The raw packed channel words, for compact serialization.
    #[inline]
    pub fn packed(&self) -> [u64; PACKED_WORDS] {
        self.packed
    }

    /// Rebuilds a phasor from its packed channels.
    ///
    /// `phase` is not stored on disk: it is by construction the angle of
    /// channel 0, so it is recovered rather than duplicated.
    pub fn from_packed(packed: [u64; PACKED_WORDS], amplitude: f64, band_n: u32, count: u32) -> Self {
        let mut p = Self { phase: 0.0, amplitude, band_n, count, packed };
        p.sync_phase();
        p
    }

    /// Number of independent phase channels.
    #[inline]
    pub const fn channels() -> usize {
        PHASE_CHANNELS
    }

    /// Returns the phase angle of channel `k`, in radians [0, 2*pi).
    #[inline]
    pub fn theta(&self, k: usize) -> f64 {
        let k = k % PHASE_CHANNELS;
        let byte = (self.packed[k / 8] >> (8 * (k % 8))) & 0xFF;
        byte as f64 * TWO_PI / CHANNEL_QUANTA
    }

    /// Sets the phase angle of channel `k`, wrapping into [0, 2*pi).
    #[inline]
    pub fn set_theta(&mut self, k: usize, theta: f64) {
        let k = k % PHASE_CHANNELS;
        let q = ((theta.rem_euclid(TWO_PI) / TWO_PI) * CHANNEL_QUANTA)
            .round()
            .rem_euclid(CHANNEL_QUANTA) as u64;
        let shift = 8 * (k % 8);
        let w = k / 8;
        self.packed[w] = (self.packed[w] & !(0xFFu64 << shift)) | (q << shift);
        if k == 0 {
            self.phase = self.theta(0);
        }
    }

    /// Rotates channel `k` by `delta` radians.
    #[inline]
    pub fn nudge(&mut self, k: usize, delta: f64) {
        let t = self.theta(k);
        self.set_theta(k, t + delta);
    }

    /// True if no channel has ever been written (a phasor loaded from a
    /// pre-multi-channel model file).
    #[inline]
    pub fn channels_unset(&self) -> bool {
        self.packed.iter().all(|w| *w == 0)
    }

    /// Fills the channels for a legacy phasor that has none, using the word's
    /// identity, while preserving its learned base `phase` on channel 0.
    pub fn ensure_channels(&mut self, word: &str) {
        if !self.channels_unset() {
            return;
        }
        let learned_phase = self.phase;
        let seeded = Self::seeded(word, self.amplitude, self.band_n);
        self.packed = seeded.packed;
        self.set_theta(0, learned_phase);
    }

    /// Re-derives `phase` from channel 0. Call after writing channel 0 directly.
    #[inline]
    pub fn sync_phase(&mut self) {
        self.phase = self.theta(0);
    }

    /// Writes `phase` into channel 0. Call after mutating `phase` directly, so
    /// that legacy single-angle updates stay visible to channel-aware code.
    #[inline]
    pub fn sync_channel0(&mut self) {
        let p = self.phase;
        self.set_theta(0, p);
    }

    /// Records one more observation and recomputes amplitude as a log-frequency.
    ///
    /// A = 1 + ln(count) / AMPLITUDE_LOG_SCALE, capped at AMPLITUDE_MAX. Linear
    /// increments saturated after a thousand touches, which is well below the
    /// frequency of the words the model sees most.
    #[inline]
    pub fn observe(&mut self) {
        self.count = self.count.saturating_add(1);
        self.amplitude =
            (1.0 + (self.count as f64).ln() / AMPLITUDE_LOG_SCALE).min(AMPLITUDE_MAX);
    }

    /// Mean phase coherence across all channels: (1/D) * sum_k cos(θ_k^a − θ_k^b).
    ///
    /// Ranges from 1.0 (identical in every channel) to −1.0 (antiphase in every
    /// channel). This is the multi-channel replacement for a single angular
    /// difference, and it is the similarity the torus representation exists for.
    pub fn resonance(&self, other: &Self) -> f64 {
        let mut sum = 0.0;
        for k in 0..PHASE_CHANNELS {
            sum += (self.theta(k) - other.theta(k)).cos();
        }
        (sum / PHASE_CHANNELS as f64).clamp(-1.0, 1.0)
    }

    /// Binds a filler to a role by channel-wise phase addition.
    ///
    /// This is circular convolution in the phase domain: binding is addition,
    /// unbinding is subtraction, and both are one operation per channel. It is
    /// what lets a representation carry structure — `dog bites man` distinct
    /// from `man bites dog` — rather than an unordered sum.
    pub fn bind(&self, role: &Self) -> Self {
        let mut out = *self;
        for k in 0..PHASE_CHANNELS {
            out.set_theta(k, self.theta(k) + role.theta(k));
        }
        out.sync_phase();
        out
    }

    /// Inverse of [`SpectralPhasor::bind`]: recovers the filler given the role.
    pub fn unbind(&self, role: &Self) -> Self {
        let mut out = *self;
        for k in 0..PHASE_CHANNELS {
            out.set_theta(k, self.theta(k) - role.theta(k));
        }
        out.sync_phase();
        out
    }

    /// Rotates every channel by `delta` — used for positional binding.
    pub fn rotate_all(&self, delta: f64) -> Self {
        let mut out = *self;
        for k in 0..PHASE_CHANNELS {
            out.set_theta(k, self.theta(k) + delta);
        }
        out.sync_phase();
        out
    }

    /// Converts the phasor into its complex wave representation.
    ///
    /// Computes `Z = A * e^(i*(phi + n*alpha))` where alpha is the
    /// fine-structure constant from config. `band_n` is capped at
    /// `BAND_N_EFFECTIVE_MAX` so the sub-band correction stays inside one sector
    /// instead of accumulating a full rotation.
    pub fn to_complex(&self) -> c64 {
        c64::from_polar(self.amplitude, self.effective_phase())
    }

    /// Complex representation of a single channel.
    #[inline]
    pub fn to_complex_ch(&self, k: usize) -> c64 {
        c64::from_polar(self.amplitude, self.theta(k))
    }

    /// Computes the multi-frequency harmonic spectrum Z(k) across D frequencies on the torus T^D.
    #[allow(dead_code)]
    pub fn harmonic_spectrum(&self, d: usize) -> Vec<c64> {
        (0..d)
            .map(|k| {
                let harmonic_amp = self.amplitude / (1.0 + 0.1 * k as f64);
                c64::from_polar(harmonic_amp, self.theta(k))
            })
            .collect()
    }

    /// Returns the effective phase including the fine-structure sub-band correction.
    ///
    /// φ_eff = φ + min(n, BAND_N_EFFECTIVE_MAX)·α
    pub fn effective_phase(&self) -> f64 {
        Phical::effective_phase(self.phase, self.band_n.min(BAND_N_EFFECTIVE_MAX))
    }

    /// Resolves this phasor to a physics-derived [`PhitonColor`].
    #[allow(dead_code)]
    pub fn to_color(&self) -> PhitonColor {
        PhitonSpectrum::phase_to_color(self.effective_phase(), 0)
    }

    /// Converts this phasor into a [`LightQuantum`] (phiton particle).
    #[allow(dead_code)]
    pub fn to_light_quantum(&self) -> LightQuantum {
        LightQuantum::from_phase(self.phase, self.amplitude, self.band_n)
    }
}

/// Multi-frequency Torus Phasor (T^D).
///
/// Reads the phasor's actual independent channels. Previously every harmonic was
/// a deterministic function of the single `phase` field, so the torus carried
/// exactly one number's worth of information — a 1-D curve embedded in T^D.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TorusPhasor {
    pub base_phase: f64,
    pub amplitude: f64,
    pub harmonics: [f64; TORUS_HARMONICS_COUNT],
}

impl TorusPhasor {
    pub fn from_spectral(phasor: &SpectralPhasor) -> Self {
        let mut harmonics = [0.0; TORUS_HARMONICS_COUNT];
        for k in 0..TORUS_HARMONICS_COUNT {
            harmonics[k] = phasor.theta(k);
        }
        Self {
            base_phase: phasor.phase,
            amplitude: phasor.amplitude,
            harmonics,
        }
    }

    /// Resonance overlap between two torus phasors across all discrete frequencies.
    pub fn resonance(&self, other: &Self) -> f64 {
        let mut sum = (self.base_phase - other.base_phase).cos();
        for k in 0..TORUS_HARMONICS_COUNT {
            sum += (self.harmonics[k] - other.harmonics[k]).cos();
        }
        (sum / ((TORUS_HARMONICS_COUNT + 1) as f64)).clamp(-1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ALPHA, PI_CONST, TWO_PI};

    #[test]
    fn test_phase_wrapping() {
        let p = SpectralPhasor::new(3.0 * PI_CONST, 1.0, 0);
        assert!((p.phase - PI_CONST).abs() < 0.03);
    }

    #[test]
    fn test_phase_zero() {
        let p = SpectralPhasor::new(0.0, 1.0, 0);
        assert!(p.phase.abs() < 0.03);
    }

    #[test]
    fn test_phase_2pi_wraps_to_zero() {
        let p = SpectralPhasor::new(TWO_PI, 1.0, 0);
        assert!(p.phase.abs() < 0.03);
    }

    #[test]
    fn test_to_complex_amplitude() {
        let p = SpectralPhasor::new(0.0, 2.5, 0);
        assert!((p.to_complex().norm() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_to_complex_band_shift() {
        let p0 = SpectralPhasor::new(0.0, 1.0, 0);
        let p1 = SpectralPhasor::new(0.0, 1.0, 1);
        assert!((p1.to_complex().arg() - p0.to_complex().arg() - ALPHA).abs() < 1e-10);
    }

    /// Seeding must depend on word identity, not word length.
    #[test]
    fn test_seed_distinguishes_same_length_words() {
        let a = SpectralPhasor::seeded("cat", 1.0, 1);
        let b = SpectralPhasor::seeded("dog", 1.0, 1);
        let c = SpectralPhasor::seeded("war", 1.0, 1);
        assert!((a.phase - b.phase).abs() > 0.05);
        assert!((b.phase - c.phase).abs() > 0.05);
        assert!((a.phase - c.phase).abs() > 0.05);
    }

    #[test]
    fn test_seed_is_deterministic() {
        let a = SpectralPhasor::seeded("borrow", 1.0, 1);
        let b = SpectralPhasor::seeded("borrow", 1.0, 1);
        assert!((a.phase - b.phase).abs() < 1e-12);
        assert_eq!(a.resonance(&b), 1.0_f64.min(a.resonance(&b)));
        assert!(a.resonance(&b) > 0.999);
    }

    /// Two unrelated words should have near-zero mean coherence across channels.
    #[test]
    fn test_resonance_of_unrelated_words_is_low() {
        let a = SpectralPhasor::seeded("photosynthesis", 1.0, 1);
        let b = SpectralPhasor::seeded("mortgage", 1.0, 1);
        assert!(a.resonance(&b).abs() < 0.35, "got {}", a.resonance(&b));
    }

    #[test]
    fn test_channel_roundtrip() {
        let mut p = SpectralPhasor::seeded("rust", 1.0, 1);
        p.set_theta(37, 2.5);
        assert!((p.theta(37) - 2.5).abs() < 0.03);
        p.set_theta(0, 1.25);
        assert!((p.phase - 1.25).abs() < 0.03);
    }

    #[test]
    fn test_bind_unbind_roundtrip() {
        let filler = SpectralPhasor::seeded("dog", 1.0, 1);
        let role = SpectralPhasor::seeded("__SUBJ", 1.0, 1);
        let bound = filler.bind(&role);
        let recovered = bound.unbind(&role);
        assert!(
            recovered.resonance(&filler) > 0.99,
            "resonance {}",
            recovered.resonance(&filler)
        );
        // and the bound form must NOT look like the filler
        assert!(bound.resonance(&filler) < 0.9);
    }

    #[test]
    fn test_observe_is_log_frequency() {
        let mut p = SpectralPhasor::seeded("the", 1.0, 1);
        for _ in 0..40 { p.observe(); }
        let a40 = p.amplitude;
        for _ in 0..100_000 { p.observe(); }
        assert!(p.amplitude > a40, "amplitude must keep rising past 1000 touches");
        assert!(p.amplitude <= AMPLITUDE_MAX);
    }

    #[test]
    fn test_band_n_capped() {
        let low = SpectralPhasor::new(1.0, 1.0, 13);
        let high = SpectralPhasor::new(1.0, 1.0, 100_000);
        assert!((low.effective_phase() - high.effective_phase()).abs() < 1e-12);
    }

    #[test]
    fn test_legacy_phasor_gets_channels() {
        let mut p = SpectralPhasor { phase: 1.75, amplitude: 1.3, band_n: 4, count: 9, packed: [0; PACKED_WORDS] };
        assert!(p.channels_unset());
        p.ensure_channels("ownership");
        assert!(!p.channels_unset());
        assert!((p.phase - 1.75).abs() < 0.03, "learned phase must survive");
    }

    #[test]
    fn test_torus_reads_independent_channels() {
        let a = SpectralPhasor::seeded("alpha", 1.0, 1);
        let t = TorusPhasor::from_spectral(&a);
        // harmonics must not all be a fixed function of base_phase
        let distinct: std::collections::HashSet<u64> =
            t.harmonics.iter().map(|h| (h * 1000.0) as u64).collect();
        assert!(distinct.len() > TORUS_HARMONICS_COUNT / 2);
    }
}
