#[allow(unused_imports)]
use super::*;

/// C as a frame: tag each pair with the class of its gloss and subtract the
/// class's mean offset. Each row is run against a shuffled partition of the
/// same shape, because centering within any partition removes variance.
pub(super) fn mean_shift_table(d: &Data) {
    let (facet, pairs, sources) = (&d.facet, &d.pairs, &d.sources);

    // ---- the sources of C ----
    let none: Vec<u16> = vec![0; pairs.len()];

    let syntactic: Vec<u16> = sources
        .iter()
        .map(|g| phiton::p64::topology::SentenceType::classify(g).base as u16)
        .collect();

    let disc8 = discover_classes(facet, sources, 8, 8);
    let disc64 = discover_classes(facet, sources, 64, 8);

    // A class per head: the strongest possible "context", included as a ceiling
    // rather than a proposal. It removes the head effect by construction, so its
    // head-domination is what perfect conditioning looks like — and its
    // agreement column says what that costs.
    let per_head_class: Vec<u16> = pairs
        .iter()
        .map(|p| (mix64(fnv1a(&p.0)) % 60_000) as u16)
        .collect();

    println!(
        "\n  {:<26} {:>8} {:>12} {:>11} {:>12}",
        "context C", "classes", "agreement", "head-dom", "vs shuffled"
    );

    let rows: Vec<(&str, Vec<u16>)> = vec![
        ("none (every earlier run)", none),
        ("syntactic (p64)", syntactic),
        ("discovered, K=8", disc8),
        ("discovered, K=64", disc64),
        ("one class per head", per_head_class),
    ];

    for (label, classes) in rows {
        let (classes, used) = collapse_small(&classes);
        let table = OffsetTable::build_contextual(facet, pairs, &classes);
        let (agree, dom) = (table.harmonic_agreement(), table.head_domination());

        // The same transform on classes of the same sizes, assigned at random.
        // Subtracting a per-class mean removes variance whatever the classes
        // are; only the gap to this row says C carried information.
        let shuffled = shuffle_classes(&classes, 0x9E37_79B9_7F4A_7C15);
        let st = OffsetTable::build_contextual(facet, pairs, &shuffled);
        let (s_agree, s_dom) = (st.harmonic_agreement(), st.head_domination());

        println!(
            "  {:<26} {:>8} {:>12.4} {:>10.1}% {:>+11.4}",
            label,
            used,
            agree,
            dom * 100.0,
            agree - s_agree
        );
        println!(
            "  {:<26} {:>8} {:>12.4} {:>10.1}%   (shuffled C)",
            "", used, s_agree, s_dom * 100.0
        );
    }
}
