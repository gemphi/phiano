use crate::config::ALPHA;
use crate::wave::c64;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// SpectralPhasor — a 16-byte fixed-width complex phasor for a single word.
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
            phase: phase.rem_euclid(2.0 * PI),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_wrapping() {
        let p = SpectralPhasor::new(3.0 * PI, 1.0, 0);
        assert!((p.phase - PI).abs() < 1e-10);
    }

    #[test]
    fn test_phase_zero() {
        let p = SpectralPhasor::new(0.0, 1.0, 0);
        assert!(p.phase.abs() < 1e-10);
    }

    #[test]
    fn test_phase_2pi_wraps_to_zero() {
        let p = SpectralPhasor::new(2.0 * PI, 1.0, 0);
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
