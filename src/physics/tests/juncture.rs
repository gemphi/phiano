#[allow(unused_imports)]
use super::super::*;
#[allow(unused_imports)]
use super::*;

/// Phinum's hexagram lift and phi-cal's Q64 lift are the same map: the
/// juncture has one definition, not two.
#[test]
fn hexagram_and_q64_agree() {
    for q in 0..64u8 {
        let h = crate::phinum::Hexagram::from_id(q);
        assert!((h.phase_angle() - phical::q64::lift(q)).abs() < 1e-12);
        let back = crate::phinum::Hexagram::from_phase(phical::q64::lift(q));
        assert_eq!(back.id, q);
        assert_eq!(phical::q64::quantise(h.phase_angle()), q);
    }
}
