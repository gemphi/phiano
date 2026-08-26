/// Phical — physics-calculus abstraction of color space and time.
///
/// Unifies the phiton (light particle) and chiton (light wave) into a
/// single mathematical framework that describes both color space and
/// temporal dynamics. This is where the fine-structure constant α
/// "shines" — it governs the quantum oscillations that connect
/// wavelength to phase to time.
///
/// # Key Insight
///
/// Light is governed by quantum oscillations. The fine-structure
/// constant α ≈ 1/137 controls the coupling between:
/// - **Color space**: phase → wavelength → spectral band → color
/// - **Time**: frequency → period → angular frequency → oscillation
///
/// Phical provides the operators that let the linguistic model treat
/// words as standing waves in a coupled color-space-time manifold, making
/// intelligence plausible through natural linguistic functions.
///
/// # Architecture
///
/// ```text
///   phiton (roll)  ──┐
///                    ├── phical ─── word mapping (gemgum)
///   chiton (wave)  ──┘
/// ```

pub mod manifold;
pub mod operators;
pub mod topology;

pub use manifold::ColorSpaceTimeManifold;
pub use operators::PhicalOps;
#[allow(unused_imports)]
pub use topology::{Edge, Path, Region, Surface, Topology, Vertex};

use crate::config::{ALPHA, PHI, TWO_PI};
use serde::{Deserialize, Serialize};

/// Phical — the physics-calculus engine for color-space-time abstraction.
///
/// Provides the mathematical bridge between the spectral domain
/// (wavelengths, colors, bands) and the temporal domain (frequencies,
/// periods, oscillations). Both are governed by the fine-structure
/// constant α and the golden ratio φ.
pub struct Phical;

impl Phical {
    /// The coupling constant between color and time.
    ///
    /// α governs how much the sub-band energy level shifts both the
    /// color (wavelength) and the temporal frequency of a word's
    /// oscillation. This is where the fine-structure constant "shines".
    pub const COUPLING: f64 = ALPHA;

    /// The natural scaling ratio for color-space-time mapping.
    ///
    /// φ (golden ratio) provides the most uniform distribution of
    /// semantic points across the color-space-time manifold, just as it
    /// does for sunflower seeds on a disk.
    pub const NATURAL_RATIO: f64 = PHI;

    /// Computes the color-space-time phase of a word.
    ///
    /// The effective phase combines the word's base phase with its
    /// sub-band energy correction: φ_eff = φ + n·α.
    /// This single value determines both the word's color (via wavelength)
    /// and its temporal oscillation (via frequency).
    pub fn effective_phase(phase: f64, band_n: u32) -> f64 {
        (phase + (band_n as f64 * ALPHA)).rem_euclid(TWO_PI)
    }

    /// Computes the temporal frequency (THz) from an effective phase.
    ///
    /// Maps the phase to a wavelength, then converts to frequency
    /// using ν = c/λ. This is the "time" aspect of phical.
    pub fn phase_to_frequency_thz(effective_phase: f64) -> f64 {
        let wavelength = crate::phiton::Phiton::phase_to_wavelength(effective_phase, 0);
        crate::phiton::Phiton::frequency_thz(wavelength)
    }

    /// Computes the color-space-time distance between two words.
    ///
    /// Combines spectral distance (color) and frequency distance (time)
    /// into a single metric, weighted by the golden ratio:
    ///   d = √(Δλ² + φ·Δν²)
    pub fn color_space_time_distance(
        phase_a: f64,
        band_a: u32,
        phase_b: f64,
        band_b: u32,
    ) -> f64 {
        let wa = crate::phiton::Phiton::phase_to_wavelength(phase_a, band_a);
        let wb = crate::phiton::Phiton::phase_to_wavelength(phase_b, band_b);
        let fa = crate::phiton::Phiton::frequency_thz(wa);
        let fb = crate::phiton::Phiton::frequency_thz(wb);

        let delta_lambda = wa - wb;
        let delta_freq = fa - fb;
        (delta_lambda * delta_lambda + PHI * delta_freq * delta_freq).sqrt()
    }

    /// Computes the quantum oscillation energy of a word.
    ///
    /// E = n·α·h·ν, where n is the band level, α is the fine-structure
    /// constant, h is Planck's constant, and ν is the frequency.
    /// This energy determines how strongly a word participates in
    /// the standing wave patterns of the color-space-time manifold.
    pub fn oscillation_energy(phase: f64, band_n: u32) -> f64 {
        let wavelength = crate::phiton::Phiton::phase_to_wavelength(phase, band_n);
        let freq = crate::phiton::Phiton::frequency_thz(wavelength) * 1e12;
        (band_n as f64 + 1.0) * ALPHA * crate::phiton::Phiton::H * freq
    }
}

/// A point in the color-space-time manifold.
///
/// Each word maps to a point in this 4D space: (wavelength, frequency,
/// phase, band_n). The manifold is continuous and differentiable,
/// enabling gradient-based optimization for the linguistic model.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorSpaceTimePoint {
    /// Wavelength (nm) — the color dimension.
    pub wavelength_nm: f64,
    /// Frequency (THz) — the time dimension.
    pub frequency_thz: f64,
    /// Effective phase [0, 2π) — the phase dimension.
    pub effective_phase: f64,
    /// Sub-band energy level — the quantum dimension.
    pub band_n: u32,
}

impl ColorSpaceTimePoint {
    /// Creates a color-space-time point from a phase and band level.
    pub fn from_phase(phase: f64, band_n: u32) -> Self {
        let eff = Phical::effective_phase(phase, band_n);
        let wavelength = crate::phiton::Phiton::phase_to_wavelength(eff, 0);
        let frequency = crate::phiton::Phiton::frequency_thz(wavelength);
        Self {
            wavelength_nm: wavelength,
            frequency_thz: frequency,
            effective_phase: eff,
            band_n,
        }
    }

    /// Returns the spectral band of this point.
    pub fn band(&self) -> crate::phiton::SpectralBand {
        crate::phiton::SpectralBand::from_wavelength(self.wavelength_nm)
    }

    /// Returns the color of this point.
    pub fn color(&self) -> crate::phiton::PhitonColor {
        crate::phiton::PhitonColor::new(self.wavelength_nm, self.band())
    }
}
