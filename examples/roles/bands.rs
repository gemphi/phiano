#[allow(unused_imports)]
use super::*;

use phient::train::roles::{Coherence, LexicalRule};

/// Coherence of `n` random pairs from the same vocabulary: the floor each band
/// is read against.
pub(super) fn shuffled_null(facet: &Facet, n: usize, salt: u64) -> f64 {
    let vocab: Vec<&String> = facet.lexicon.keys().collect();
    let mut r = 0x2545F4914F6CDD1Du64 ^ salt;
    let mut pairs = Vec::with_capacity(n);
    for _ in 0..n {
        r ^= r << 13; r ^= r >> 7; r ^= r << 17;
        let a = vocab[(r % vocab.len() as u64) as usize].clone();
        r ^= r << 13; r ^= r >> 7; r ^= r << 17;
        let b = vocab[(r % vocab.len() as u64) as usize].clone();
        if a != b { pairs.push((a, b)); }
    }
    Coherence::measure(facet, &pairs)
}

/// Register, grouped by how polysemous their head is.
pub(super) fn sense_bands(
    facet: &Facet,
    lex: &LexicalRule,
    senses: &HashMap<String, usize>,
) -> Vec<(usize, f64)> {
    println!(
        "\n  {:<22} {:>8} {:>11} {:>10} {:>8}",
        "head sense count", "pairs", "coherence", "shuffled", "ratio"
    );

    // The four largest prepositions only: the null-control run showed smaller
    // groups are indistinguishable from noise, so splitting them further by
    // sense count would only compare noise with noise.
    let mut bands: Vec<(&str, Vec<(String, String)>)> = vec![
        ("1 sense (monosemous)", Vec::new()),
        ("2-3 senses", Vec::new()),
        ("4-7 senses", Vec::new()),
        ("8+ senses", Vec::new()),
    ];
    for role in ["of", "to", "in", "with"] {
        for (h, f) in lex.pairs_for_role(role) {
            let n = senses.get(&h).copied().unwrap_or(1);
            let band = match n {
                0 | 1 => 0,
                2..=3 => 1,
                4..=7 => 2,
                _ => 3,
            };
            bands[band].1.push((h, f));
        }
    }

    let mut ratios: Vec<(usize, f64)> = Vec::new();
    for (i, (label, pairs)) in bands.iter().enumerate() {
        if pairs.len() < 50 {
            println!("  {:<22} {:>8} {:>11} {:>10} {:>8}", label, pairs.len(), "-", "-", "too few");
            continue;
        }
        let sample: Vec<(String, String)> = pairs.iter().take(8000).cloned().collect();
        let coh = Coherence::measure(facet, &sample);
        let null = shuffled_null(facet, sample.len(), i as u64);
        let ratio = coh / null.max(1e-9);
        ratios.push((i, ratio));
        println!(
            "  {:<22} {:>8} {:>11.4} {:>10.4} {:>7.2}x",
            label,
            pairs.len(),
            coh,
            null,
            ratio
        );
    }
    ratios
}

/// The disambiguation.
///
/// Polysemous words in a dictionary are also the COMMON words: cat, set,
/// run, head. Common words are trained on far more often, so their phases
/// are shaped by the objective while a hapax's phases are still essentially
/// its hash seed. Binning by frequency instead of by sense count separates
/// the two explanations, and only one of them is actionable.
pub(super) fn frequency_bands(facet: &Facet, lex: &LexicalRule) -> Vec<f64> {
    println!(
        "\n  {:<22} {:>8} {:>11} {:>10} {:>8}",
        "head frequency band", "pairs", "coherence", "shuffled", "ratio"
    );
    let count_of = |w: &str| facet.lexicon.get(w).map(|p| p.count).unwrap_or(0);
    let mut fbands: Vec<(&str, Vec<(String, String)>)> = vec![
        ("count 1-4", Vec::new()),
        ("count 5-24", Vec::new()),
        ("count 25-199", Vec::new()),
        ("count 200+", Vec::new()),
    ];
    for role in ["of", "to", "in", "with"] {
        for (h, f) in lex.pairs_for_role(role) {
            let c = count_of(&h);
            let band = match c {
                0..=4 => 0,
                5..=24 => 1,
                25..=199 => 2,
                _ => 3,
            };
            fbands[band].1.push((h, f));
        }
    }
    let mut fratios: Vec<f64> = Vec::new();
    for (i, (label, pairs)) in fbands.iter().enumerate() {
        if pairs.len() < 50 {
            println!("  {:<22} {:>8} {:>11} {:>10} {:>8}", label, pairs.len(), "-", "-", "too few");
            fratios.push(f64::NAN);
            continue;
        }
        let sample: Vec<(String, String)> = pairs.iter().take(8000).cloned().collect();
        let coh = Coherence::measure(facet, &sample);
        let null = shuffled_null(facet, sample.len(), 100 + i as u64);
        let ratio = coh / null.max(1e-9);
        fratios.push(ratio);
        println!(
            "  {:<22} {:>8} {:>11.4} {:>10.4} {:>7.2}x",
            label, pairs.len(), coh, null, ratio
        );
    }
    fratios
}
