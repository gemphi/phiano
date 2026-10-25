#[allow(unused_imports)]
use super::*;

/// How to read the table: what the kernel column means and what the prediction
/// under test would have to look like.
pub(super) fn closing() {
    println!(
        "\n  Low bands are slow with long memory, high bands fast with short\n\
         \x20 memory — that is what the kernel's |z| column shows. The prediction\n\
         \x20 under test is that sentences live low and words high, with overlapping\n\
         \x20 but distinctly peaked profiles. A flat profile, or two peaks in the\n\
         \x20 same band, says the channel index carries no hierarchy and the\n\
         \x20 per-channel kernel is decoration."
    );
}
