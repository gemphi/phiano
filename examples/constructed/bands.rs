#[allow(unused_imports)]
use super::*;

/// Co-occurrence bands: cluster words on the company they keep, using the
/// trained manifold only to describe neighbourhoods, not to place anything.
pub(super) fn cooc_bands(learned: &Facet, train_toks: &[Vec<String>]) -> HashMap<String, u64> {
    let mut neighbours: HashMap<&str, Vec<(f64, f64)>> = HashMap::new();
    for toks in train_toks {
        for i in 0..toks.len() {
            if learned.lexicon.get(&toks[i]).is_none_or(|p| p.count < COUNT_FLOOR) {
                continue;
            }
            let e = neighbours
                .entry(toks[i].as_str())
                .or_insert_with(|| vec![(0.0, 0.0); PHASE_CHANNELS]);
            for (j, tok) in toks.iter().enumerate().take(i + 4).skip(i.saturating_sub(3)) {
                if j == i {
                    continue;
                }
                if let Some(p) = learned.lexicon.get(tok) {
                    for (k, a) in e.iter_mut().enumerate() {
                        let t = p.theta(k);
                        a.0 += t.cos();
                        a.1 += t.sin();
                    }
                }
            }
        }
    }
    // 64 bands, to match the bound the rest of the project argues for.
    let mut m = HashMap::new();
    for (w, acc) in &neighbours {
        // A cheap, deterministic band: the sector of the neighbourhood's
        // resultant on the first two channels. Not k-means — the point is
        // that the band be a category, not that it be optimal.
        let s0 = acc[0].1.atan2(acc[0].0).rem_euclid(TWO_PI);
        let s1 = acc[1].1.atan2(acc[1].0).rem_euclid(TWO_PI);
        let b = ((s0 / TWO_PI * 8.0) as u64) * 8 + (s1 / TWO_PI * 8.0) as u64;
        m.insert(w.to_string(), b);
    }
    m
}

/// Character bands: the last three characters, which is where `-ing`, `-ed`,
/// `-ly` and `-tion` come from — straight out of the alphabet.
pub(super) fn suffix_band(w: &str) -> u64 {
    let s: String = w.chars().rev().take(3).collect();
    fnv1a(&s) % 512
}
