//! §79, Moses: a name used with no fixed set of descriptions. Take a word's
//! relations, keep only some, and ask which word they pick out. If the
//! meaning were one defining relation, dropping it would lose the word; if
//! it is a cluster, the word should survive the loss of many.
//!
//! Two readers: the counts (how many kept relations a candidate was told)
//! and phinet's functions (Σ cos(F(candidate + kind), other) over the kept
//! relations, from a model trained on the told rows only). And two sources:
//! relations that were told, and held-out relations never told — the
//! descriptions a reader has only heard of.

use super::data::{feature, Table};
use phiano::relate::{held_out, kind_token, RelationModel};
use phiecs::quest::Row;
use phistd::Function;

pub struct Reading {
    pub top1: f64,
    pub mrr: f64,
    pub n: usize,
}

fn rank(scores: &[f64], truth: usize) -> f64 {
    let t = scores[truth];
    1.0 + scores.iter().enumerate().filter(|(i, s)| *i != truth && **s > t).count() as f64 + 0.5 * scores.iter().enumerate().filter(|(i, s)| *i != truth && **s == t).count() as f64
}

/// Keep a `1 − drop` share of each word's relations from `source`
/// (at least one), chosen by a hash; identify the word among `words`.
pub fn identify(t: &Table, m: &mut RelationModel, words: &[String], source: &[&Row], drop: f64, functions: bool) -> Reading {
    let told: Vec<&Row> = t.rows.iter().filter(|r| !held_out(r)).collect();
    let table = m.joint.functions();
    let (mut top1, mut mrr, mut n) = (0.0, 0.0, 0usize);
    for (wi, w) in words.iter().enumerate() {
        let mine: Vec<&&Row> = source.iter().filter(|r| r.about == *w).collect();
        if mine.len() < 4 {
            continue;
        }
        let mut kept: Vec<&&Row> = mine.iter().copied().filter(|r| (phistd::mix64(phiano::phasor::fnv1a(&feature(r))) % 1000) as f64 / 1000.0 >= drop).collect();
        if kept.is_empty() {
            kept.push(mine[0]);
        }
        let scores: Vec<f64> = if functions {
            let mut s = vec![0.0; words.len()];
            for r in &kept {
                let (Some(&k), Some(&o)) = (m.index.get(&kind_token(&r.kind)), m.index.get(&r.other)) else { continue };
                let inputs: Vec<Function> = words
                    .iter()
                    .map(|c| {
                        let i = m.index[c];
                        let mut f: Function = table[i].iter().zip(&table[k]).map(|(x, y)| x + y).collect();
                        phistd::normalise(&mut f);
                        f
                    })
                    .collect();
                for (c, y) in m.joint.fno.apply(&inputs).iter().enumerate() {
                    s[c] += phistd::cosine(y, &table[o]);
                }
            }
            s
        } else {
            words.iter().map(|c| kept.iter().filter(|r| told.iter().any(|x| x.about == *c && x.kind == r.kind && x.other == r.other)).count() as f64).collect()
        };
        let rk = rank(&scores, wi);
        top1 += (rk == 1.0) as usize as f64;
        mrr += 1.0 / rk;
        n += 1;
    }
    Reading { top1: top1 / n.max(1) as f64, mrr: mrr / n.max(1) as f64, n }
}
