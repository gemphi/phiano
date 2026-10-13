#[allow(unused_imports)]
use super::*;

/// Summed reciprocal ranks for one marker: the four scorers on the true target
/// and the same four on a decoy, which supplies the null.
#[derive(Default)]
pub(super) struct MarkerScores {
    pub(super) own: f64,
    pub(super) riv: f64,
    pub(super) all: f64,
    pub(super) shp: f64,
    pub(super) nown: f64,
    pub(super) nriv: f64,
    pub(super) nall: f64,
    pub(super) nshp: f64,
    pub(super) n: usize,
}

/// Score `a : b :: c : ?` over the pairs joined by marker channel `k`, on that
/// channel, on the rival channels, on all sixty-four blended, and on the
/// shipped manifold — identical probes throughout.
pub(super) fn score_marker(
    rel: &RelationalFacet,
    shipped: &Facet,
    pool: &[String],
    pairs: &[(String, String)],
    k: usize,
    rivals: &[usize],
) -> MarkerScores {
    // Rank on one channel: the step is an angle there, applied to `c`.
    let rr_on = |ch: usize, a: &str, b: &str, c: &str, target: &str, ex: &[&str]| -> f64 {
        let want = rel.words[c].theta[ch] + rel.words[b].theta[ch] - rel.words[a].theta[ch];
        let t = (rel.words[target].theta[ch] - want).cos();
        let r = 1 + pool
            .iter()
            .filter(|w| w.as_str() != target && !ex.contains(&w.as_str()))
            .filter(|w| (rel.words[w.as_str()].theta[ch] - want).cos() > t)
            .count();
        1.0 / r as f64
    };

    let mut s = MarkerScores::default();
    let stride = (pairs.len() / 5).max(1);

    for (i, (a, b)) in pairs.iter().enumerate() {
        for (j, (c, d)) in pairs.iter().enumerate() {
            if i == j || j % stride != i % stride {
                continue;
            }
            let ex = [a.as_str(), b.as_str(), c.as_str()];
            let decoy = match pool
                .iter()
                .cycle()
                .skip((i * 7919 + j * 104_729) % pool.len())
                .take(pool.len())
                .find(|w| w.as_str() != d && !ex.contains(&w.as_str()))
            {
                Some(w) => w.as_str(),
                None => continue,
            };

            s.own += rr_on(k, a, b, c, d, &ex);
            s.nown += rr_on(k, a, b, c, decoy, &ex);
            for &r in rivals {
                s.riv += rr_on(r, a, b, c, d, &ex) / rivals.len() as f64;
                s.nriv += rr_on(r, a, b, c, decoy, &ex) / rivals.len() as f64;
            }

            // All sixty-four blended, evidence-weighted.
            let score_all = |w: &str| -> f64 {
                let (pw, pa, pb, pc) =
                    (&rel.words[w], &rel.words[a], &rel.words[b], &rel.words[c]);
                let (mut dot, mut nw) = (0.0f64, 0.0f64);
                for kk in 0..PHASE_CHANNELS {
                    let m = pw.mag[kk] * pa.mag[kk].min(pb.mag[kk]).min(pc.mag[kk]);
                    if m > 0.0 {
                        let tgt = pc.theta[kk] + pb.theta[kk] - pa.theta[kk];
                        dot += m * (pw.theta[kk] - tgt).cos();
                        nw += m;
                    }
                }
                match nw > 0.0 {
                    true => dot / nw,
                    false => f64::NEG_INFINITY,
                }
            };
            let rr_all = |target: &str| -> f64 {
                let t = score_all(target);
                let r = 1 + pool
                    .iter()
                    .filter(|w| w.as_str() != target && !ex.contains(&w.as_str()))
                    .filter(|w| score_all(w) > t)
                    .count();
                1.0 / r as f64
            };
            s.all += rr_all(d);
            s.nall += rr_all(decoy);

            // The shipped manifold.
            let sq = {
                let (pa, pb, pc) =
                    (&shipped.lexicon[a], &shipped.lexicon[b], &shipped.lexicon[c]);
                let mut q = *pc;
                for kk in 0..PHASE_CHANNELS {
                    q.set_theta(kk, pc.theta(kk) + pb.theta(kk) - pa.theta(kk));
                }
                q.sync_phase();
                q
            };
            let rr_ship = |target: &str| -> f64 {
                let t = sq.resonance(&shipped.lexicon[target]);
                let r = 1 + pool
                    .iter()
                    .filter(|w| w.as_str() != target && !ex.contains(&w.as_str()))
                    .filter(|w| sq.resonance(&shipped.lexicon[w.as_str()]) > t)
                    .count();
                1.0 / r as f64
            };
            s.shp += rr_ship(d);
            s.nshp += rr_ship(decoy);
            s.n += 1;
        }
    }
    s
}
