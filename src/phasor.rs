use crate::config::{ALPHA, PHI, TWO_PI, TORUS_HARMONICS_COUNT};
use crate::phical::Phical;
use crate::phiton::{LightQuantum, PhitonColor, PhitonSpectrum};
use crate::wave::c64;
use serde::{Deserialize, Serialize};

/// SpectralPhasor - a 16-byte fixed-width complex phasor for a single word.
///
/// Words are mapped onto a continuous 2*pi phase manifold. The complex wave
/// representation is `Z = A * e^(i*(phi + n*alpha))`, where alpha is the
/// fine-structure constant. This creates a spectral interference space where
/// semantic similarity is measured by destructive interference (energy delta).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpectralPhasor {
    /// Primary phase angle on the unit circle, in radians [0, 2*pi).
    pub phase: f64,
    /// Amplitude / intensity / familiarity weight.
    pub amplitude: f64,
    /// Quantized fine-structure energy sub-band level (n = 1, 2, 3...).
    pub band_n: u32,
}

impl SpectralPhasor {
    /// Creates a new phasor with the given phase, amplitude, and band level.
    ///
    /// The phase is automatically wrapped to [0, 2*pi) using modular arithmetic.
    pub fn new(phase: f64, amplitude: f64, band_n: u32) -> Self {
        Self {
            phase: phase.rem_euclid(TWO_PI),
            amplitude,
            band_n,
        }
    }

    /// Converts the phasor into its complex wave representation.
    ///
    /// Computes `Z = A * e^(i*(phi + n*alpha))` where alpha is the
    /// fine-structure constant from config.
    pub fn to_complex(&self) -> c64 {
        let effective_phase = self.phase + (self.band_n as f64 * ALPHA);
        c64::from_polar(self.amplitude, effective_phase)
    }

    /// Computes the multi-frequency harmonic spectrum Z(k) across D frequencies on the torus T^D.
    #[allow(dead_code)]
    pub fn harmonic_spectrum(&self, d: usize) -> Vec<c64> {
        (0..d).map(|k| {
            let freq_multiplier = PHI.powi(k as i32 % (TORUS_HARMONICS_COUNT as i32));
            let harmonic_phase = (self.phase * freq_multiplier + (self.band_n as f64 * ALPHA * (k + 1) as f64)).rem_euclid(TWO_PI);
            let harmonic_amp = self.amplitude / (1.0 + 0.1 * k as f64);
            c64::from_polar(harmonic_amp, harmonic_phase)
        }).collect()
    }

    /// Returns the effective phase including the fine-structure sub-band correction.
    ///
    /// φ_eff = φ + n·α
    #[allow(dead_code)]
    pub fn effective_phase(&self) -> f64 {
        Phical::effective_phase(self.phase, self.band_n)
    }

    /// Resolves this phasor to a physics-derived [`PhitonColor`].
    ///
    /// Maps the effective phase through the phiton spectral domain to
    /// produce a color from the visible spectrum.
    #[allow(dead_code)]
    pub fn to_color(&self) -> PhitonColor {
        PhitonSpectrum::phase_to_color(self.effective_phase(), 0)
    }

    /// Converts this phasor into a [`LightQuantum`] (phiton particle).
    ///
    /// The light quantum carries wavelength, frequency, and energy
    /// information derived from the phasor's phase and band level.
    #[allow(dead_code)]
    pub fn to_light_quantum(&self) -> LightQuantum {
        LightQuantum::from_phase(self.phase, self.amplitude, self.band_n)
    }
}

/// Multi-frequency Torus Phasor (T^D) for multi-dimensional semantic and syntactic representation.
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
            harmonics[k] = (phasor.phase * PHI.powi(k as i32 % 4) + (k as f64 * ALPHA)).rem_euclid(TWO_PI);
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
        (sum / ((TORUS_HARMONICS_COUNT + 1) as f64)).max(-1.0).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{PI_CONST, TWO_PI};

    #[test]
    fn test_phase_wrapping() {
        let p = SpectralPhasor::new(3.0 * PI_CONST, 1.0, 0);
        assert!((p.phase - PI_CONST).abs() < 1e-10);
    }

    #[test]
    fn test_phase_zero() {
        let p = SpectralPhasor::new(0.0, 1.0, 0);
        assert!(p.phase.abs() < 1e-10);
    }

    #[test]
    fn test_phase_2pi_wraps_to_zero() {
        let p = SpectralPhasor::new(TWO_PI, 1.0, 0);
        assert!(p.phase.abs() < 1e-10);
    }

    #[test]
    fn test_to_complex_amplitude() {
        let p = SpectralPhasor::new(0.0, 2.5, 0);
        let z = p.to_complex();
        assert!((z.norm() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_to_complex_band_shift() {
        let p0 = SpectralPhasor::new(0.0, 1.0, 0);
        let p1 = SpectralPhasor::new(0.0, 1.0, 1);
        let z0 = p0.to_complex();
        let z1 = p1.to_complex();
        assert!((z1.arg() - z0.arg() - ALPHA).abs() < 1e-10);
    }
}
