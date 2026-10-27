#[allow(unused_imports)]
use super::*;

pub use phistd::codon::{codon, codon_index, helix_position, hexagram_glyph, BASES, CODONS, PITCH_PER_TURN, RISE};
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
