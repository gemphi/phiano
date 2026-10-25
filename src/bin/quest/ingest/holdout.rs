//! Held-out rows: what the facet can say about a relation it was never told.
#[allow(unused_imports)]
use super::*;

/// Split rows by a hash of the row, `fraction` held out. Deterministic.
pub fn split(rows: &[Row], fraction: f64) -> (Vec<Row>, Vec<Row>) {
    let (mut train, mut held) = (Vec::new(), Vec::new());
    for r in rows {
        let key = format!("{}\t{}\t{}", r.about, r.kind, r.other);
        let h = key.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100_0000_01b3));
        match (h >> 11) as f64 / (1u64 << 53) as f64 <= fraction {
            true => held.push(r.clone()),
            false => train.push(r.clone()),
        }
    }
    (train, held)
}

/// One hop through any other kind: `about ⇒ m` under some kind, then
/// `m ⇒ w` under the asked kind, the product of the two readings, best
/// intermediate wins. Nothing here knows the synthetic structure — it is
/// the generic rule that what a word's relatives relate to, it may relate
/// to. Only the three strongest intermediates per kind are followed.
pub fn two_hop(facet: &Facet, kinds: &[String], pool: &[String], about: &str, kind: &str, w: &str) -> f64 {
    let Some(k) = facet.marker_channel(kind) else { return 0.0 };
    let mut best = 0.0f64;
    for k2 in kinds {
        for (m, s1) in facet.under(about, k2, pool).into_iter().take(3) {
            if m == w {
                continue;
            }
            let s = s1 * facet.on(&m, w, k).max(0.0);
            best = best.max(s);
        }
    }
    best
}

/// Held-out rows read one hop (the channel never saw them: chance is the
/// honest expectation) and two hops.
pub fn measure_holdout(facet: &Facet, kinds: &[String], held: &[Row], pool: &[String]) -> Vec<(String, Score, Score)> {
    let mut out: Vec<(String, Score, Score)> =
        kinds.iter().map(|k| (k.clone(), Score::default(), Score::default())).collect();
    out.push(("all".to_string(), Score::default(), Score::default()));
    for row in held {
        let Some(k) = facet.marker_channel(&row.kind) else { continue };
        let ki = kinds.iter().position(|x| x == &row.kind).unwrap_or(0);
        let one = rank_of(pool, &row.other, |w| facet.on(&row.about, w, k));
        let two = rank_of(pool, &row.other, |w| two_hop(facet, kinds, pool, &row.about, &row.kind, w));
        for i in [ki, kinds.len()] {
            out[i].1.add(one);
            out[i].2.add(two);
        }
    }
    out
}
