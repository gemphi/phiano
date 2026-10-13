#[allow(unused_imports)]
use super::*;

/// Per-channel agreement of a family's pairs: `cos(theta_k(b) - theta_k(a))`,
/// averaged over the pairs that survive the floor.
pub(super) fn channel_profile(facet: &Facet, pairs: &[(String, String)]) -> (Vec<f64>, usize) {
    let mut acc = vec![0.0f64; PHASE_CHANNELS];
    let mut n = 0usize;
    for (a, b) in pairs {
        let (pa, pb) = match (facet.lexicon.get(a), facet.lexicon.get(b)) {
            (Some(x), Some(y)) => (x, y),
            _ => continue,
        };
        for (k, s) in acc.iter_mut().enumerate() {
            *s += (pb.theta(k) - pa.theta(k)).cos();
        }
        n += 1;
    }
    if n > 0 {
        for s in acc.iter_mut() {
            *s /= n as f64;
        }
    }
    (acc, n)
}

/// Same pairs, fillers permuted. Destroys the relation, keeps the vocabulary and
/// the pair count — so the null pays the same maximum-over-64 bias the signal
/// does.
pub(super) fn permute(pairs: &[(String, String)], seed: u64) -> Vec<(String, String)> {
    let mut fillers: Vec<String> = pairs.iter().map(|p| p.1.clone()).collect();
    let mut r = seed | 1;
    for i in (1..fillers.len()).rev() {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        fillers.swap(i, (r % (i as u64 + 1)) as usize);
    }
    pairs.iter().zip(fillers).map(|(p, f)| (p.0.clone(), f)).collect()
}

pub(super) fn argmax(v: &[f64]) -> (usize, f64) {
    v.iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, x)| (i, *x))
        .unwrap_or((0, 0.0))
}

/// Pair-versus-random under a similarity, chance 50%.
pub(super) fn pair_vs_random<F: Fn(&Facet, &str, &str) -> f64>(
    facet: &Facet,
    pool: &[&String],
    families: &[RelationFamily],
    sim: F,
) -> f64 {
    let (mut wins, mut n) = (0.0f64, 0usize);
    for fam in families {
        for (i, p) in fam.pairs.iter().enumerate() {
            if !facet.lexicon.contains_key(&p.a) || !facet.lexicon.contains_key(&p.b) {
                continue;
            }
            let target = sim(facet, &p.a, &p.b);
            let (mut hits, mut draws) = (0usize, 0usize);
            for d in 0..DRAWS {
                let r = (fnv1a(&p.a) ^ ((i as u64) << 32) ^ (d as u64).wrapping_mul(0x9E37_79B9))
                    as usize;
                let cand = pool[r % pool.len().max(1)];
                if cand == &p.a || cand == &p.b {
                    continue;
                }
                draws += 1;
                if target > sim(facet, &p.a, cand) {
                    hits += 1;
                }
            }
            if draws > 0 {
                wins += hits as f64 / draws as f64;
                n += 1;
            }
        }
    }
    match n {
        0 => 0.5,
        _ => wins / n as f64,
    }
}

pub(super) fn mean_sim(facet: &Facet, a: &str, b: &str) -> f64 {
    match (facet.lexicon.get(a), facet.lexicon.get(b)) {
        (Some(x), Some(y)) => x.resonance(y),
        _ => 0.0,
    }
}

/// The best single dimension, rather than the average of all of them.
pub(super) fn max_sim(facet: &Facet, a: &str, b: &str) -> f64 {
    match (facet.lexicon.get(a), facet.lexicon.get(b)) {
        (Some(x), Some(y)) => (0..PHASE_CHANNELS)
            .map(|k| (y.theta(k) - x.theta(k)).cos())
            .fold(f64::NEG_INFINITY, f64::max),
        _ => 0.0,
    }
}

/// The mean of the best eight dimensions — between the two, in case a relation
/// occupies a few channels rather than exactly one.
pub(super) fn top8_sim(facet: &Facet, a: &str, b: &str) -> f64 {
    match (facet.lexicon.get(a), facet.lexicon.get(b)) {
        (Some(x), Some(y)) => {
            let mut v: Vec<f64> = (0..PHASE_CHANNELS)
                .map(|k| (y.theta(k) - x.theta(k)).cos())
                .collect();
            v.sort_by(|p, q| q.partial_cmp(p).unwrap_or(std::cmp::Ordering::Equal));
            v[..8].iter().sum::<f64>() / 8.0
        }
        _ => 0.0,
    }
}
