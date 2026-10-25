#[allow(unused_imports)]
use super::*;

/// The arms, in the order they are reported: the learned manifold, then each
/// constructed banding paired with the controls that isolate what the band and
/// what the ordering are each worth.
pub(super) fn build_rows(s: &Setup) -> Vec<(String, HashMap<String, Point>)> {
    let cooc_band = cooc_bands(&s.learned, &s.train_toks);
    let cooc = |w: &str| -> u64 { cooc_band.get(w).copied().unwrap_or(0) };
    let suffix = |w: &str| -> u64 { suffix_band(w) };

    let mut rows: Vec<(String, HashMap<String, Point>)> = vec![(
        "learned (trained manifold)".into(),
        learned_points(&s.learned, &s.vocab),
    )];
    rows.push((
        "constructed: suffix + alphabetical".into(),
        construct(&s.vocab, &suffix, true, false),
    ));
    rows.push((
        "  control: suffix + shuffled order".into(),
        construct(&s.vocab, &suffix, false, false),
    ));
    rows.push((
        "  control: shuffled band + alpha".into(),
        construct(&s.vocab, &suffix, true, true),
    ));
    rows.push((
        "constructed: co-occur + alphabetical".into(),
        construct(&s.vocab, &cooc, true, false),
    ));
    rows.push((
        "  control: co-occur + shuffled order".into(),
        construct(&s.vocab, &cooc, false, false),
    ));
    rows
}
