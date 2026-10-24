//! Can the facet give the quest back?
#[allow(unused_imports)]
use super::*;

/// What one reading recovers over a set of rows.
#[derive(Default, Clone, Debug)]
pub struct Score {
    pub n: usize,
    pub mrr: f64,
    pub hit1: usize,
    pub hit5: usize,
}

impl Score {
    pub fn add(&mut self, rank: usize) {
        self.n += 1;
        self.mrr += 1.0 / rank as f64;
        self.hit1 += (rank == 1) as usize;
        self.hit5 += (rank <= 5) as usize;
    }

    pub fn mrr(&self) -> f64 {
        if self.n == 0 { 0.0 } else { self.mrr / self.n as f64 }
    }

    pub fn hit1(&self) -> f64 {
        if self.n == 0 { 0.0 } else { self.hit1 as f64 / self.n as f64 }
    }

    pub fn hit5(&self) -> f64 {
        if self.n == 0 { 0.0 } else { self.hit5 as f64 / self.n as f64 }
    }
}

/// Rank of `other` among `pool` when every candidate is scored by `score`,
/// higher first; ties are broken against the truth (a tied truth ranks last
/// among its ties), so a reading that says nothing scores as chance, not
/// better.
pub fn rank_of(pool: &[String], other: &str, score: impl Fn(&str) -> f64) -> usize {
    let truth = score(other);
    1 + pool.iter().filter(|w| w.as_str() != other && score(w) >= truth).count()
}

/// Four readings of the same rows.
///
/// - `kind`: the truth ranked on **its own kind's channel**, `on(about, w, k)`,
///   against every word the table ever named as an `other` — the reading the
///   ingestion is for, and the hard one: a word never named under this kind
///   keeps its seeded target angle on the channel, and a seed can land
///   anywhere, including next to the truth.
/// - `within`: the same reading against the words named under **this kind**
///   only — the quest's own question ("is-a | ?" is answered from the things
///   anything is).
/// - `wrong`: ranked on the channel of the *next* kind in the table. If this
///   scores as well as `kind`, the channels carry adjacency, not kinds.
/// - `offset`: [`Facet::calls_for`], the offset channels only — what the
///   dialogue reads today, which never saw a kind.
pub struct Readings {
    pub kind: Score,
    pub within: Score,
    pub wrong: Score,
    pub offset: Score,
    pub per_kind: Vec<(String, Score, Score)>,
}

pub fn measure(facet: &Facet, rows: &[Row]) -> Readings {
    let pool = target_pool(rows);
    let kinds = kinds(rows);
    let channel: Vec<Option<usize>> = kinds.iter().map(|k| facet.marker_channel(k)).collect();
    let pools: Vec<Vec<String>> = kinds.iter().map(|k| kind_pool(rows, k)).collect();
    let mut r = Readings {
        kind: Score::default(),
        within: Score::default(),
        wrong: Score::default(),
        offset: Score::default(),
        per_kind: kinds.iter().map(|k| (k.clone(), Score::default(), Score::default())).collect(),
    };
    for row in rows {
        let ki = kinds.iter().position(|k| k == &row.kind).unwrap_or(0);
        let Some(k) = channel[ki] else { continue };
        let wrong_k = channel[(ki + 1) % kinds.len()].unwrap_or(k);
        let on = |w: &str, ch: usize| facet.on(&row.about, w, ch);
        let rank = rank_of(&pool, &row.other, |w| on(w, k));
        r.kind.add(rank);
        let within = rank_of(&pools[ki], &row.other, |w| on(w, k));
        r.within.add(within);
        r.per_kind[ki].1.add(rank);
        r.per_kind[ki].2.add(within);
        r.wrong.add(rank_of(&pool, &row.other, |w| on(w, wrong_k)));
        r.offset.add(rank_of(&pool, &row.other, |w| facet.calls_for(&row.about, w)));
    }
    r
}

/// The single word channel `k` calls for from `about` — what the quest would
/// read back as an answer to its own question.
pub fn best_on(facet: &Facet, pool: &[String], about: &str, k: usize) -> Option<(String, f64)> {
    pool.iter()
        .map(|w| (w.clone(), facet.on(about, w, k)))
        .filter(|(_, s)| *s > 0.0)
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
}
