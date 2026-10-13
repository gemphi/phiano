#[allow(unused_imports)]
use super::*;

/// Reading one: C as a shift. Subtract the frame class's mean offset and see
/// whether what remains agrees better with the fixed harmonic basis.
pub(super) fn shift_section(facet: &Facet, rows: &Rows) {
    let (pairs, context) = (&rows.pairs, &rows.context);
    println!("\n  === C as a shift ===");
    println!(
        "  {:<22} {:>8} {:>12} {:>11} {:>12}",
        "context C", "classes", "agreement", "head-dom", "vs shuffled"
    );
    let none: Vec<u16> = vec![0; pairs.len()];
    let base_table = OffsetTable::build_contextual(facet, pairs, &none);
    println!(
        "  {:<22} {:>8} {:>12.4} {:>10.1}% {:>12}",
        "none",
        1,
        base_table.harmonic_agreement(),
        base_table.head_domination() * 100.0,
        "-"
    );
    for k in [8usize, 64] {
        let (classes, used) = collapse_small(&discover_classes(facet, context, k, 8));
        let t = OffsetTable::build_contextual(facet, pairs, &classes);
        let st = OffsetTable::build_contextual(
            facet,
            pairs,
            &shuffle_classes(&classes, 0x9E37_79B9_7F4A_7C15),
        );
        println!(
            "  {:<22} {:>8} {:>12.4} {:>10.1}% {:>+12.4}",
            format!("discovered, K={k}"),
            used,
            t.harmonic_agreement(),
            t.head_domination() * 100.0,
            t.harmonic_agreement() - st.harmonic_agreement()
        );
    }
}
