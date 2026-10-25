#[allow(unused_imports)]
use super::*;

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

/// Per-channel agreement for a family, over the pairs that have evidence there.
///
/// The averaging matters. Dividing by the whole family size scores a channel as
/// zero wherever a pair has never been seen — so a sparse marker channel where
/// three pairs agree perfectly reports 3/22, and only channels every pair
/// populates can ever come first. The first version did that, and duly reported
/// all ten families peaking on offset channels, which is what the measurement
/// could say rather than what the data said.
///
/// Here each channel is averaged over its own covered pairs, and channels with
/// too little coverage are excluded rather than diluted — a channel two pairs
/// agree on is not evidence that the family lives there.
pub(super) fn profile(f: &RelationalFacet, pairs: &[(String, String)]) -> (Vec<f64>, Vec<usize>, usize) {
    let mut acc = vec![0.0f64; PHASE_CHANNELS];
    let mut cov = vec![0usize; PHASE_CHANNELS];
    let mut n = 0usize;
    for (a, b) in pairs {
        let (pa, pb) = match (f.words.get(a), f.words.get(b)) {
            (Some(x), Some(y)) => (x, y),
            _ => continue,
        };
        for k in 0..PHASE_CHANNELS {
            if pa.mag[k] > 0.0 && pb.mag[k] > 0.0 {
                acc[k] += (pa.theta[k] - pb.theta[k]).cos();
                cov[k] += 1;
            }
        }
        n += 1;
    }
    for k in 0..PHASE_CHANNELS {
        acc[k] = match cov[k] >= MIN_COVERAGE {
            true => acc[k] / cov[k] as f64,
            // Not "zero agreement" — no measurement. Excluded from the maximum.
            false => f64::NEG_INFINITY,
        };
    }
    (acc, cov, n)
}

pub(super) fn argmax(v: &[f64]) -> (usize, f64) {
    v.iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, x)| (i, *x))
        .unwrap_or((0, 0.0))
}

pub(super) fn pair_vs_random<F: Fn(&str, &str) -> f64>(
    pool: &[String],
    families: &[RelationFamily],
    known: &dyn Fn(&str) -> bool,
    sim: F,
) -> f64 {
    let (mut wins, mut n) = (0.0f64, 0usize);
    for fam in families {
        for (i, p) in fam.pairs.iter().enumerate() {
            if !known(&p.a) || !known(&p.b) {
                continue;
            }
            let target = sim(&p.a, &p.b);
            let (mut hits, mut draws) = (0usize, 0usize);
            for d in 0..DRAWS {
                let r = (fnv1a(&p.a) ^ ((i as u64) << 32) ^ (d as u64).wrapping_mul(0x9E37_79B9))
                    as usize;
                let cand = &pool[r % pool.len().max(1)];
                if cand == &p.a || cand == &p.b {
                    continue;
                }
                draws += 1;
                if target > sim(&p.a, cand) {
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
