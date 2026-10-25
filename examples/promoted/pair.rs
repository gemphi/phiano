#[allow(unused_imports)]
use super::*;

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

/// Reciprocal rank of `target` among `cand` under `score`, excluding the probe.
pub(super) fn rr<F: Fn(&str) -> f64>(cand: &[&String], target: &str, exclude: &[&str], score: F) -> f64 {
    let t = score(target);
    let r = 1 + cand
        .iter()
        .filter(|w| w.as_str() != target && !exclude.contains(&w.as_str()))
        .filter(|w| score(w) > t)
        .count();
    1.0 / r as f64
}

/// Per-channel agreement for a family over the pairs that have evidence there,
/// as `bin/relational` computes it: each channel averaged over its own covered
/// pairs, thin channels excluded rather than diluted.
pub(super) fn profile(f: &Facet, pairs: &[(String, String)]) -> Vec<f64> {
    let mut acc = vec![0.0f64; PHASE_CHANNELS];
    let mut cov = vec![0usize; PHASE_CHANNELS];
    for (a, b) in pairs {
        let (pa, pb) = match (f.lexicon.get(a), f.lexicon.get(b)) {
            (Some(x), Some(y)) => (x, y),
            _ => continue,
        };
        for k in 0..PHASE_CHANNELS {
            if pa.evidence_count(k) > 0 && pb.evidence_count(k) > 0 {
                acc[k] += pa.on(pb, k);
                cov[k] += 1;
            }
        }
    }
    for k in 0..PHASE_CHANNELS {
        acc[k] = match cov[k] >= MIN_COVERAGE {
            true => acc[k] / cov[k] as f64,
            false => f64::NEG_INFINITY,
        };
    }
    acc
}

pub(super) fn argmax(v: &[f64]) -> (usize, f64) {
    v.iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, x)| (i, *x))
        .unwrap_or((0, 0.0))
}

pub(super) fn dense_query(f: &Facet, a: &str, b: &str, c: &str) -> phiano::phasor::SpectralPhasor {
    let (pa, pb, pc) = (&f.lexicon[a], &f.lexicon[b], &f.lexicon[c]);
    let mut q = *pc;
    for k in 0..PHASE_CHANNELS {
        q.set_theta(k, pc.theta(k) + pb.theta(k) - pa.theta(k));
    }
    q.sync_phase();
    q
}
