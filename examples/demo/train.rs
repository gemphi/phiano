//! Train a relation model on told rows (inheritance where the personas
//! declare it) and read it on any rows against the prior.

use phiano::relate::{kind_token, RelationModel};
use phiecs::quest::Row;
use phical::relate::{self, inherit_epoch, prior, score, score_inherit, Inherit, Score, Triple};
use phistd::Adam;
use std::collections::HashMap;

pub struct Reading {
    pub told: Score,
    pub held: Score,
    pub prior: Score,
}

pub fn triples(m: &RelationModel, rows: &[&Row]) -> Vec<Triple> {
    rows.iter().filter_map(|r| m.triple(r)).collect()
}

fn inherit(m: &RelationModel, told: &[Triple], kinds: &[String]) -> Option<Inherit> {
    let isa = *m.index.get(&kind_token("is-a"))?;
    let ks: Vec<usize> = kinds.iter().filter_map(|k| m.index.get(&kind_token(k)).copied()).collect();
    (!ks.is_empty()).then(|| Inherit::from_told(told, isa, ks, 1.0))
}

pub fn train(m: &mut RelationModel, told: &[Triple], inherit_kinds: &[String], epochs: usize, seed: u64) {
    let (pool, _) = relate::pools(told);
    let inh = inherit(m, told, inherit_kinds);
    let examples: Vec<_> = told.iter().map(relate::example).collect();
    let lr = m.joint.fno.config.lr;
    let (mut a, mut b) = (Adam::new(lr), Adam::new(lr * 0.5));
    for e in 0..epochs {
        let s = seed ^ e as u64;
        match &inh {
            Some(i) => {
                inherit_epoch(&mut m.joint, told, i, &pool, &mut a, &mut b, s);
            }
            None => {
                m.joint.train_epoch(&examples, &pool, 32, &mut a, &mut b, s);
            }
        }
    }
}

/// `told` and `held` read against a pool of every answer in `all`.
pub fn read(m: &mut RelationModel, all: &[Triple], told: &[Triple], held: &[Triple], inherit_kinds: &[String]) -> Reading {
    let (pool, by_kind): (Vec<usize>, HashMap<usize, Vec<usize>>) = relate::pools(all);
    let inh = inherit(m, told, inherit_kinds);
    let mut go = |rows: &[Triple]| match &inh {
        Some(i) => score_inherit(&mut m.joint, rows, i, &pool, &by_kind),
        None => score(&mut m.joint, rows, &pool, &by_kind),
    };
    let (t, h) = (go(told), go(held));
    Reading { told: t, held: h, prior: prior(told, held, &pool, &by_kind) }
}
