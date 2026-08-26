/// color-space-time manifold — the geometric space where words live.
///
/// Words are mapped onto a continuous manifold that unifies color
/// (wavelength) and time (frequency). The manifold is a torus T²
/// parameterized by (phase, band_n×α), where the fine-structure
/// constant α provides the quantum sub-band spacing.
///
/// The manifold supports:
/// - **Geodesic distance**: shortest path between two words in color-space-time
/// - **Gradient flow**: direction of steepest semantic change
/// - **Standing wave nodes**: stable equilibrium positions (word meanings)

use super::ColorSpaceTimePoint;
use crate::config::{ALPHA, PHI, TWO_PI};
use crate::phiton::chiton::Chiton;
use crate::phiton::types::LightQuantum;
use std::f64::consts::PI;

/// The color-space-time manifold T².
///
/// A 2-torus where:
/// - Axis 1: phase angle θ ∈ [0, 2π) — the color wheel
/// - Axis 2: sub-band energy n·α — the quantum spectrum
///
/// Words occupy positions on this torus. Their semantic similarity
/// is measured by geodesic distance on the manifold surface.
pub struct ColorSpaceTimeManifold;

impl ColorSpaceTimeManifold {
    /// Computes the geodesic distance between two points on the manifold.
    ///
    /// The geodesic accounts for the toroidal topology (phase wraps
    /// around at 2π) and the golden-ratio-weighted sub-band spacing.
    pub fn geodesic(a: &ColorSpaceTimePoint, b: &ColorSpaceTimePoint) -> f64 {
        let mut phase_delta = (a.effective_phase - b.effective_phase).abs();
        if phase_delta > PI {
            phase_delta = TWO_PI - phase_delta;
        }

        let band_delta = (a.band_n as f64 - b.band_n as f64) * ALPHA;

        (phase_delta * phase_delta + PHI * band_delta * band_delta).sqrt()
    }

    /// Computes the gradient direction from point a toward point b.
    ///
    /// Returns a unit vector (dθ, dn) in the manifold's tangent space,
    /// pointing in the direction of steepest descent of the geodesic
    /// distance. Used for Kuramoto-style phase relaxation.
    pub fn gradient(a: &ColorSpaceTimePoint, b: &ColorSpaceTimePoint) -> (f64, f64) {
        let mut d_phase = b.effective_phase - a.effective_phase;
        if d_phase > PI { d_phase -= TWO_PI; }
        if d_phase < -PI { d_phase += TWO_PI; }

        let d_band = (b.band_n as f64 - a.band_n as f64) * ALPHA;

        let norm = (d_phase * d_phase + d_band * d_band).sqrt();
        if norm < 1e-12 {
            return (0.0, 0.0);
        }
        (d_phase / norm, d_band / norm)
    }

    /// Finds the standing wave nodes (stable equilibrium positions).
    ///
    /// Nodes occur where the phiton (roll) and chiton (wave) interfere
    /// constructively. These are the positions where words settle into
    /// stable meanings — the "attractors" of the linguistic model.
    pub fn is_standing_wave_node(point: &ColorSpaceTimePoint) -> bool {
        let quantum = LightQuantum::from_phase(point.effective_phase, 1.0, point.band_n);
        let chiton = Chiton::from_quantum(&quantum);

        // A node exists where the standing wave amplitude is maximal
        // at position 0 and time 0: cos(0)·cos(0) = 1
        let amplitude = chiton.standing_wave_amplitude(0.0, 0.0, 1.0);
        amplitude > 0.99
    }

    /// Computes the manifold curvature at a point.
    ///
    /// The curvature is determined by the fine-structure constant
    /// and the band level. Higher band levels create tighter curvature,
    /// corresponding to more specific/nuanced word meanings.
    pub fn curvature(point: &ColorSpaceTimePoint) -> f64 {
        ALPHA * (point.band_n as f64 + 1.0) / PHI
    }

    /// Maps a sector index to a color-space-time point.
    ///
    /// This is the primary entry point for connecting the phase-space
    /// sector model to the color-space-time manifold.
    pub fn sector_to_point(sector: u16, sector_count: u16, band_n: u32) -> ColorSpaceTimePoint {
        let phase = (sector as f64 / sector_count as f64) * TWO_PI;
        ColorSpaceTimePoint::from_phase(phase, band_n)
    }
}
