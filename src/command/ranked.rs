//! A ranked list of words with their distance, printed one way.
//!
//! `synonym` and `resonance` ask different questions - one word's
//! neighbours, a sentence's - and printed the answer with two copies of the
//! same loop and two wordings of the same empty line.

/// Prints each word with its rank and distance, or one line when the search
/// found nothing.
pub(super) fn ranked(found: &[(String, f64)]) {
    match found.is_empty() {
        true => println!("  No resonant words found."),
        false => {
            for (rank, (w, delta)) in found.iter().enumerate() {
                println!("  Rank {}: {:<15} ΔC = {:.8}", rank + 1, w, delta);
            }
        }
    }
}
