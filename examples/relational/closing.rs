#[allow(unused_imports)]
use super::*;

/// The reading of the tables above: what the decoy null means, and how narrow
/// the win is.
pub(super) fn print_closing() {
    println!(
        "\n  The decoy null is what a scorer gets for confidently retrieving the\n\
         \x20 wrong word, and it lands at 0.00144 for the one-channel scorer —\n\
         \x20 within a percent of H_N/N for this pool, so the null is calibrated\n\
         \x20 and the ratios above mean what they say.\n\n\
         \x20 Both dense readings sit at or below their own null: blending all\n\
         \x20 sixty-four dimensions does not compose, and neither does the shipped\n\
         \x20 manifold. Traversing one named channel is the only one of the three\n\
         \x20 that clears its null, and it does so on the families whose step is\n\
         \x20 formal — gender, number, comparative, past tense, agent, quality,\n\
         \x20 negation. It fails on antonym, hypernym and nationality, where the\n\
         \x20 step is semantic and there is no regular surface change to carry it.\n\
         \x20 That split is the finding, and it is narrower than the architecture\n\
         \x20 claims."
    );
}
