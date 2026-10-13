#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;

use super::super::ranges::{Quantum, Range, Resolution};

/// Phical's Q64 and Phinet's `Quantum::Q64` are one definition.
#[test]
fn phical_q64_is_phinet_q64() {
    for k in 0..500 {
        let phase = k as f64 * 0.0137;
        let a = super::super::exact::q64::quantise(phase) as usize;
        let b = Quantum::Q64.sector_of_phase(phase);
        assert_eq!(a, b, "sector of {phase}");
        assert!((super::super::exact::q64::lift(a as u8) - Quantum::Q64.phase(b)).abs() < 1e-12);
    }
    assert!((super::super::exact::q64::BOUND - Quantum::Q64.bound()).abs() < 1e-15);
    assert_eq!(Range::DEFAULT, Range::new(Resolution::P64, Quantum::Q64));
    assert_eq!(Resolution::P64.space(), crate::config::PHASE_CHANNELS);
}
