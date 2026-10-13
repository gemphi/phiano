#[allow(unused_imports)]
use super::*;

/// Every arm, scored on pair-versus-random twice — phase-only as shipped, and
/// with magnitude restored — plus the gender family called out on its own.
pub(super) fn score_rows(rows: &[(String, HashMap<String, Point>)], vocab: &[(String, u32)]) {
    let families = RelationBenchmark::default_families();
    let pool: Vec<&String> = vocab.iter().map(|(w, _)| w).collect();

    println!(
        "\n  {:<34} {:>12} {:>12} {:>10} {:>8}",
        "positions", "phase-only", "complex", "gender", "pairs"
    );

    for (label, points) in rows {
        let (ph, n) = pair_vs_random(points, &pool, &families, phase_similarity, None);
        let (cx, _) = pair_vs_random(points, &pool, &families, complex_similarity, None);
        let (gender, _) =
            pair_vs_random(points, &pool, &families, complex_similarity, Some("gender"));
        println!(
            "  {:<34} {:>11.1}% {:>11.1}% {:>9.1}% {:>8}",
            label,
            ph * 100.0,
            cx * 100.0,
            gender * 100.0,
            n
        );
    }

    println!(
        "\n  Chance is 50.0% in every column. `phase-only` is what this project\n\
         \x20 actually measures today — `resonance` reads the packed angles and never\n\
         \x20 touches amplitude — and `complex` is the same comparison with magnitude\n\
         \x20 restored. The gap between those two columns is the cost of carrying a\n\
         \x20 complex representation and scoring half of it.\n\n\
         \x20 For the constructed rows, read them against their own controls, not\n\
         \x20 against each other. Alphabetical ordering earns its place only by\n\
         \x20 beating the shuffled-order row with the same bands; the band earns its\n\
         \x20 place only by beating the shuffled-band row with the same ordering."
    );
}
