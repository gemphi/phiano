#[allow(unused_imports)]
use super::*;

/// The fixed basis.
///
/// Bentov's standing waves: a bounded medium sustains only integer harmonics,
/// so a 64-channel torus has exactly 64 modes and the bound is the channel
/// count rather than a number anyone picked. The experimental point is that
/// this basis fits NOTHING — no centroids, no k-means, no seed, identical on
/// every corpus — while the swept rows fit 64 rotations of 64 channels each,
/// 4,096 free numbers, to these very pairs.
pub(super) fn harmonic_basis(facet: &Facet, descended: &Facet, pairs: &[(String, String)]) {
    println!("\n  --- fixed harmonic basis: 0 fitted parameters vs 4096 above ---");
    println!(
        "  {:<14} {:<16} {:>10} {:>10} {:>10} {:>9}",
        "manifold", "offset", "held-out", "shuffled", "gain", "g/noise"
    );
    for (label, f) in [("bottom-up", facet), ("after descent", descended)] {
        for est in OffsetEstimator::ALL {
            let p = KindTable::harmonic_saturation(f, pairs, est);
            println!(
                "  {:<14} {:<16} {:>10.4} {:>10.4} {:>10.4} {:>8.1}x",
                label,
                est.label(),
                p.held_out,
                p.shuffled,
                p.gain(),
                match p.noise() > 1e-9 {
                    true => p.gain() / p.noise(),
                    false => f64::INFINITY,
                }
            );
        }
    }
}
