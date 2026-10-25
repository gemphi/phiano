#[allow(unused_imports)]
use super::*;

/// A counter of 1.8 million and a quantum number of 13 are the same point.
#[test]
fn counters_do_not_walk_the_wheel() {
    let a = ColorSpaceTimePoint::from_phase(1.0, 13);
    let b = ColorSpaceTimePoint::from_phase(1.0, 1_800_000);
    assert_eq!(a.band_n, b.band_n);
    assert!((a.effective_phase - b.effective_phase).abs() < 1e-12);
    assert!((Phical::oscillation_energy(1.0, 13) - Phical::oscillation_energy(1.0, 1_800_000)).abs() < 1e-40);
    assert!(Phical::color_space_time_distance(1.0, 13, 1.0, 1_800_000).abs() < 1e-9);
}

/// PhiTon's constants were phiano's until it became its own crate; they must
/// stay the same numbers.
#[test]
fn phiton_units_match_phiano_config() {
    assert_eq!(::phiton::ALPHA, crate::config::ALPHA);
    assert_eq!(::phiton::PHI, crate::config::PHI);
    assert_eq!(::phiton::TWO_PI, crate::config::TWO_PI);
    assert_eq!(::phiton::BAND_N_EFFECTIVE_MAX, crate::config::BAND_N_EFFECTIVE_MAX);
}

#[cfg(test)]
#[cfg(feature = "phinum")]
mod juncture;

#[cfg(test)]
mod range;
