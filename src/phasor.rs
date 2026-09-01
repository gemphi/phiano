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

/// `cos` of every representable phase difference.
///
/// Index is the wrapping byte difference between two channels, so
/// `COS_BYTE[a.wrapping_sub(b)]` is exactly `cos(theta_a - theta_b)` for
/// quantised phases — cosine is even, so the wrap direction does not matter.
///
/// Generated once because `const fn` cannot call `cos`;
/// `test_cos_table_matches_cos` recomputes every entry and fails if this
/// array ever drifts from the real cosine.
static COS_BYTE: [f64; 256] = [
    1.00000000000000000e+00, 9.99698818696204250e-01, 9.98795456205172405e-01, 9.97290456678690207e-01,
    9.95184726672196929e-01, 9.92479534598709967e-01, 9.89176509964781014e-01, 9.85277642388941222e-01,
    9.80785280403230431e-01, 9.75702130038528570e-01, 9.70031253194543974e-01, 9.63776065795439840e-01,
    9.56940335732208824e-01, 9.49528180593036675e-01, 9.41544065183020806e-01, 9.32992798834738957e-01,
    9.23879532511286738e-01, 9.14209755703530691e-01, 9.03989293123443338e-01, 8.93224301195515324e-01,
    8.81921264348355050e-01, 8.70086991108711461e-01, 8.57728610000272118e-01, 8.44853565249707117e-01,
    8.31469612302545236e-01, 8.17584813151583711e-01, 8.03207531480644943e-01, 7.88346427626606339e-01,
    7.73010453362736993e-01, 7.57208846506484567e-01, 7.40951125354959106e-01, 7.24247082951467003e-01,
    7.07106781186547573e-01, 6.89540544737066941e-01, 6.71558954847018330e-01, 6.53172842953776756e-01,
    6.34393284163645488e-01, 6.15231590580626819e-01, 5.95699304492433468e-01, 5.75808191417845339e-01,
    5.55570233019602289e-01, 5.34997619887097264e-01, 5.14102744193221661e-01, 4.92898192229784093e-01,
    4.71396736825997809e-01, 4.49611329654606595e-01, 4.27555093430282196e-01, 4.05241314004989861e-01,
    3.82683432365089837e-01, 3.59895036534988277e-01, 3.36889853392220051e-01, 3.13681740398891573e-01,
    2.90284677254462331e-01, 2.66712757474898421e-01, 2.42980179903263982e-01, 2.19101240156869770e-01,
    1.95090322016128331e-01, 1.70961888760301356e-01, 1.46730474455361748e-01, 1.22410675199216279e-01,
    9.80171403295607702e-02, 7.35645635996674541e-02, 4.90676743274181260e-02, 2.45412285229122638e-02,
    6.12323399573676604e-17, -2.45412285229121424e-02, -4.90676743274180080e-02, -7.35645635996673292e-02,
    -9.80171403295606453e-02, -1.22410675199216154e-01, -1.46730474455361637e-01, -1.70961888760301245e-01,
    -1.95090322016128193e-01, -2.19101240156869659e-01, -2.42980179903263871e-01, -2.66712757474898310e-01,
    -2.90284677254462165e-01, -3.13681740398891407e-01, -3.36889853392219940e-01, -3.59895036534988166e-01,
    -3.82683432365089726e-01, -4.05241314004989750e-01, -4.27555093430281863e-01, -4.49611329654606706e-01,
    -4.71396736825997698e-01, -4.92898192229783982e-01, -5.14102744193221661e-01, -5.34997619887097042e-01,
    -5.55570233019601956e-01, -5.75808191417845339e-01, -5.95699304492433357e-01, -6.15231590580626708e-01,
    -6.34393284163645377e-01, -6.53172842953776533e-01, -6.71558954847018441e-01, -6.89540544737066941e-01,
    -7.07106781186547462e-01, -7.24247082951466781e-01, -7.40951125354958884e-01, -7.57208846506484567e-01,
    -7.73010453362736993e-01, -7.88346427626606228e-01, -8.03207531480644832e-01, -8.17584813151583600e-01,
    -8.31469612302545347e-01, -8.44853565249707117e-01, -8.57728610000272007e-01, -8.70086991108711350e-01,
    -8.81921264348354939e-01, -8.93224301195515213e-01, -9.03989293123443338e-01, -9.14209755703530691e-01,
    -9.23879532511286738e-01, -9.32992798834738846e-01, -9.41544065183020695e-01, -9.49528180593036675e-01,
    -9.56940335732208824e-01, -9.63776065795439840e-01, -9.70031253194543974e-01, -9.75702130038528459e-01,
    -9.80785280403230431e-01, -9.85277642388941222e-01, -9.89176509964781014e-01, -9.92479534598709967e-01,
    -9.95184726672196818e-01, -9.97290456678690207e-01, -9.98795456205172405e-01, -9.99698818696204250e-01,
    -1.00000000000000000e+00, -9.99698818696204250e-01, -9.98795456205172405e-01, -9.97290456678690207e-01,
    -9.95184726672196929e-01, -9.92479534598709967e-01, -9.89176509964781014e-01, -9.85277642388941333e-01,
    -9.80785280403230431e-01, -9.75702130038528570e-01, -9.70031253194543974e-01, -9.63776065795439951e-01,
    -9.56940335732208935e-01, -9.49528180593036786e-01, -9.41544065183020806e-01, -9.32992798834738957e-01,
    -9.23879532511286850e-01, -9.14209755703530691e-01, -9.03989293123443449e-01, -8.93224301195515324e-01,
    -8.81921264348355050e-01, -8.70086991108711461e-01, -8.57728610000272118e-01, -8.44853565249707228e-01,
    -8.31469612302545458e-01, -8.17584813151583711e-01, -8.03207531480644943e-01, -7.88346427626606339e-01,
    -7.73010453362737104e-01, -7.57208846506484790e-01, -7.40951125354959106e-01, -7.24247082951467003e-01,
    -7.07106781186547684e-01, -6.89540544737067052e-01, -6.71558954847018663e-01, -6.53172842953777089e-01,
    -6.34393284163645932e-01, -6.15231590580627263e-01, -5.95699304492433135e-01, -5.75808191417845228e-01,
    -5.55570233019602178e-01, -5.34997619887097264e-01, -5.14102744193221772e-01, -4.92898192229784204e-01,
    -4.71396736825997864e-01, -4.49611329654606928e-01, -4.27555093430282473e-01, -4.05241314004990361e-01,
    -3.82683432365090337e-01, -3.59895036534987944e-01, -3.36889853392219940e-01, -3.13681740398891462e-01,
    -2.90284677254462442e-01, -2.66712757474898532e-01, -2.42980179903264121e-01, -2.19101240156870103e-01,
    -1.95090322016128664e-01, -1.70961888760301689e-01, -1.46730474455362303e-01, -1.22410675199215960e-01,
    -9.80171403295604510e-02, -7.35645635996673569e-02, -4.90676743274180288e-02, -2.45412285229123887e-02,
    -1.83697019872102969e-16, 2.45412285229120210e-02, 4.90676743274176611e-02, 7.35645635996669822e-02,
    9.80171403295600902e-02, 1.22410675199215599e-01, 1.46730474455361942e-01, 1.70961888760301328e-01,
    1.95090322016128304e-01, 2.19101240156869742e-01, 2.42980179903263760e-01, 2.66712757474898199e-01,
    2.90284677254462053e-01, 3.13681740398891129e-01, 3.36889853392219607e-01, 3.59895036534987611e-01,
    3.82683432365090004e-01, 4.05241314004990028e-01, 4.27555093430282140e-01, 4.49611329654606595e-01,
    4.71396736825997587e-01, 4.92898192229783871e-01, 5.14102744193221550e-01, 5.34997619887096931e-01,
    5.55570233019601845e-01, 5.75808191417844895e-01, 5.95699304492432913e-01, 6.15231590580627041e-01,
    6.34393284163645599e-01, 6.53172842953776756e-01, 6.71558954847018330e-01, 6.89540544737066829e-01,
    7.07106781186547351e-01, 7.24247082951466670e-01, 7.40951125354958884e-01, 7.57208846506484234e-01,
    7.73010453362736660e-01, 7.88346427626605895e-01, 8.03207531480645054e-01, 8.17584813151583711e-01,
    8.31469612302545236e-01, 8.44853565249707006e-01, 8.57728610000272007e-01, 8.70086991108711350e-01,
    8.81921264348354828e-01, 8.93224301195515102e-01, 9.03989293123443116e-01, 9.14209755703530469e-01,
    9.23879532511286516e-01, 9.32992798834738957e-01, 9.41544065183020806e-01, 9.49528180593036675e-01,
    9.56940335732208824e-01, 9.63776065795439840e-01, 9.70031253194543974e-01, 9.75702130038528459e-01,
    9.80785280403230320e-01, 9.85277642388941111e-01, 9.89176509964780903e-01, 9.92479534598709967e-01,
    9.95184726672196929e-01, 9.97290456678690207e-01, 9.98795456205172405e-01, 9.99698818696204250e-01,
];


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
        // Exact, not approximate. Channels are quantised to one byte
        // (`CHANNEL_QUANTA` = 256), so the angular difference between any two
        // channels is one of 256 values and its cosine is a table lookup on the
        // byte difference. Nothing is lost — the quantisation already happened
        // when the phase was stored.
        //
        // The previous version called `cos` 64 times per comparison. Resonance
        // is the innermost operation in every retrieval, every relation probe
        // and every analogy: one pass of the expanded benchmark makes roughly
        // 190 million of these calls, and 64 transcendentals each is what made
        // that pass take minutes per condition.
        let mut sum = 0.0f64;
        for w in 0..PACKED_WORDS {
            let (a, b) = (self.packed[w], other.packed[w]);
            for shift in 0..8 {
                let da = ((a >> (8 * shift)) & 0xFF) as u8;
                let db = ((b >> (8 * shift)) & 0xFF) as u8;
                sum += COS_BYTE[da.wrapping_sub(db) as usize];
            }
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

    /// The lookup table must be the real cosine, or every resonance in the
    /// engine is quietly wrong. Generated tables rot; this recomputes.
    #[test]
    fn test_cos_table_matches_cos() {
        for i in 0..256usize {
            let expected = (i as f64 * TWO_PI / CHANNEL_QUANTA).cos();
            assert!(
                (COS_BYTE[i] - expected).abs() < 1e-15,
                "COS_BYTE[{}] = {} but cos = {}",
                i,
                COS_BYTE[i],
                expected
            );
        }
    }

    /// The fast path must agree with the definition it replaced.
    #[test]
    fn test_resonance_matches_the_transcendental_form() {
        let words = ["cat", "dog", "grandmother", "oscillate", "phi", "a"];
        for a in &words {
            for b in &words {
                let (pa, pb) = (
                    SpectralPhasor::seeded(a, 1.0, 1),
                    SpectralPhasor::seeded(b, 1.0, 1),
                );
                let reference = {
                    let mut sum = 0.0;
                    for k in 0..PHASE_CHANNELS {
                        sum += (pa.theta(k) - pb.theta(k)).cos();
                    }
                    (sum / PHASE_CHANNELS as f64).clamp(-1.0, 1.0)
                };
                assert!(
                    (pa.resonance(&pb) - reference).abs() < 1e-12,
                    "{}/{}: table {} vs cos {}",
                    a,
                    b,
                    pa.resonance(&pb),
                    reference
                );
            }
        }
    }
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
