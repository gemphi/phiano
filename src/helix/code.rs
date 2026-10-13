#[allow(unused_imports)]
use super::*;

pub use phistd::codon::{codon, codon_index, hexagram_glyph, BASES, CODONS};
/// B-DNA: codons per turn of the helix.
pub const PITCH_PER_TURN: f64 = 10.5;
/// B-DNA: rise per codon along the axis, in nanometres.
pub const RISE: f64 = 0.34;
/// Radius of the helix, in nanometres.
const RADIUS: f64 = 1.0;

/// One channel's full code: everything the 64-fold correspondence says about it.
#[derive(Clone, Debug, PartialEq)]
pub struct Codon {
    pub k: usize,
    pub codon: String,
    pub hexagram: String,
    pub degree: usize,
    pub order: isize,
    pub position: [f64; 3],
}

/// Where codon `k` sits on the helix: angle `2πk / 10.5`, rise `0.34 k`.
pub fn helix_position(k: usize) -> [f64; 3] {
    let theta = 2.0 * std::f64::consts::PI * k as f64 / PITCH_PER_TURN;
    [RADIUS * theta.cos(), RADIUS * theta.sin(), RISE * k as f64]
}

impl Codon {
    pub fn of(k: usize) -> Self {
        Self {
            k,
            codon: codon(k),
            hexagram: hexagram_glyph(k),
            degree: phifno::sphere::degree_of(k),
            order: phifno::sphere::order_of(k),
            position: helix_position(k),
        }
    }

    /// The whole table.
    pub fn table() -> Vec<Codon> {
        (0..CODONS).map(Codon::of).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_helix_turns_every_ten_and_a_half() {
        let a = helix_position(0);
        let b = helix_position(21);
        assert!((a[0] - b[0]).abs() < 1e-9 && (a[1] - b[1]).abs() < 1e-9);
        assert!((b[2] - 21.0 * RISE).abs() < 1e-12);
    }
}
