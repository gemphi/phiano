#[allow(unused_imports)]
use super::*;

/// Summed reciprocal ranks for one family: the three scorers on the true
/// target, and the same three on a decoy that supplies the empirical null.
#[derive(Default)]
pub(super) struct Scores {
    pub(super) ch: f64,
    pub(super) all: f64,
    pub(super) ship: f64,
    pub(super) nch: f64,
    pub(super) nall: f64,
    pub(super) nship: f64,
    pub(super) n: usize,
}

/// Score `a : b :: c : ?` over a deterministic subset of ordered pairs-of-pairs
/// drawn from `pairs`, ranking inside `cand` on the family's channel `k`.
pub(super) fn score_family(
    rel: &RelationalFacet,
    shipped: &Facet,
    pairs: &[(String, String)],
    cand: &[&String],
    k: usize,
) -> Scores {
    // A deterministic subset of ordered pairs-of-pairs, so the probe set is
    // identical between the three scorers and between runs.
    let stride = (pairs.len() / 6).max(1);
    let mut s = Scores::default();
    for (i, (a, b)) in pairs.iter().enumerate() {
        for (j, (c, d)) in pairs.iter().enumerate() {
            if i == j || j % stride != i % stride {
                continue;
            }
            // The decoy: a candidate that is not the answer and not a term of
            // the probe. Picked by position so it is fixed across runs and
            // across scorers, and drawn from the same set being ranked.
            let decoy = {
                let mut pick: Option<&String> = None;
                for t in 0..cand.len() {
                    let w = cand[(i * 7919 + j * 104_729 + t) % cand.len()];
                    if w != d && w != a && w != b && w != c {
                        pick = Some(w);
                        break;
                    }
                }
                match pick {
                    Some(w) => w,
                    None => continue,
                }
            };
            let exclude = [a.as_str(), b.as_str(), c.as_str()];

            // One channel: the step is an angle on k, applied to c.
            let want = rel.words[c].theta[k] + rel.words[b].theta[k] - rel.words[a].theta[k];
            let rr_ch = |target: &String| -> f64 {
                let t = (rel.words[target].theta[k] - want).cos();
                let r = 1 + cand
                    .iter()
                    .filter(|w| **w != target && !exclude.contains(&w.as_str()))
                    .filter(|w| (rel.words[**w].theta[k] - want).cos() > t)
                    .count();
                1.0 / r as f64
            };

            // All channels, evidence-weighted: the same step taken in every
            // dimension at once.
            let score_all = |w: &str| -> f64 {
                let (pw, pa, pb, pc) =
                    (&rel.words[w], &rel.words[a], &rel.words[b], &rel.words[c]);
                let (mut dot, mut nw) = (0.0f64, 0.0f64);
                for kk in 0..PHASE_CHANNELS {
                    let m = pw.mag[kk] * pa.mag[kk].min(pb.mag[kk]).min(pc.mag[kk]);
                    if m > 0.0 {
                        let target = pc.theta[kk] + pb.theta[kk] - pa.theta[kk];
                        dot += m * (pw.theta[kk] - target).cos();
                        nw += m;
                    }
                }
                match nw > 0.0 {
                    true => dot / nw,
                    false => f64::NEG_INFINITY,
                }
            };
            let rr_all = |target: &String| -> f64 {
                let t = score_all(target);
                let r = 1 + cand
                    .iter()
                    .filter(|w| **w != target && !exclude.contains(&w.as_str()))
                    .filter(|w| score_all(w) > t)
                    .count();
                1.0 / r as f64
            };

            // The shipped manifold, all 64 channels, unweighted.
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
            let rr_ship = |target: &String| -> f64 {
                let t = sq.resonance(&shipped.lexicon[target]);
                let r = 1 + cand
                    .iter()
                    .filter(|w| **w != target && !exclude.contains(&w.as_str()))
                    .filter(|w| sq.resonance(&shipped.lexicon[**w]) > t)
                    .count();
                1.0 / r as f64
            };

            s.ch += rr_ch(d);
            s.all += rr_all(d);
            s.ship += rr_ship(d);
            s.nch += rr_ch(decoy);
            s.nall += rr_all(decoy);
            s.nship += rr_ship(decoy);
            s.n += 1;
        }
    }
    s
}
