//! Are the strand's units the scopes a reading should use? (docs/145.) In
//! each of five folds a different fifth of the tokens is held out (token `i`
//! is held out in fold `i mod 5`). The strand is built on the other four
//! fifths, with the prior, and each held-out word is read in the scope that
//! holds the token before it. Each reader is run under two rules, which
//! differ only in how much a scope trusts its own counts: Witten–Bell to
//! the scope's parent (the code the strand divides with), and the weight the
//! text counts itself by leaving each token out (`strand::Scopes`). The
//! readers differ only in their scopes:
//!
//! - `prior`: English alone (*Emma*), the text unread;
//! - `whole`: the text as one scope;
//! - `labels`: the true segments (chapters, documents, topics) under the whole;
//! - `equal parts`: as many equal parts as the strand supports, under the whole;
//! - `strand flat`: the strand's supported units, each straight under the whole;
//! - `strand at k`: the strand cut into as many units as there are labels, read down its tree;
//! - `strand`: the units the strand's information supports, read down its tree.
//!
//! Each of the sixty-four levels is read as well, so the level a reading
//! does best at can be set beside the level the strand says the text supports.

use phistd::codon::CODONS;
use phiplm::prose::strand::{Prior, Scopes, Strand};
use std::collections::HashMap;

pub const READERS: [&str; 7] = ["prior", "whole", "labels", "equal parts", "strand flat", "strand at k", "strand"];
pub const RULES: [&str; 2] = ["Witten–Bell", "counted weights"];

/// One text read in every fold: nats summed over its held-out words.
#[derive(Clone)]
pub struct Reading {
    pub held: usize,
    /// `nats[rule][reader]`, `levels[rule][k]`.
    pub nats: [[f64; 7]; 2],
    pub levels: [[f64; CODONS]; 2],
    pub units: [usize; CODONS],
    /// The units supported, summed over the folds.
    pub supported: usize,
    pub folds: usize,
}

/// The scope of `spans` (sorted by start) that holds token `i`.
fn holding(s: &Strand, spans: &[usize], i: usize) -> usize {
    spans[spans.partition_point(|x| s.cut.spans[*x].start <= i).saturating_sub(1)]
}

/// A flat division of `tokens` at `cuts`: each segment's counts and length.
fn segments<'a>(tokens: &'a [String], cuts: &[usize]) -> Vec<(HashMap<&'a str, u32>, usize)> {
    let mut bounds = vec![0];
    bounds.extend(cuts.iter().copied().filter(|c| *c > 0 && *c < tokens.len()));
    bounds.push(tokens.len());
    bounds.windows(2).map(|w| (tokens[w[0]..w[1]].iter().fold(HashMap::new(), |mut m, t| { *m.entry(t.as_str()).or_insert(0) += 1; m }), w[1] - w[0])).collect()
}

/// Witten–Bell: `(c + T · above) / (n + T)`.
fn wb(c: u32, types: usize, n: usize, above: f64) -> f64 {
    (c as f64 + types as f64 * above) / (n as f64 + types as f64)
}

/// Read `tokens` (true segment starts `truth`) in `folds` folds.
pub fn text(tokens: &[String], truth: &[usize], prior: &Prior, folds: usize) -> Reading {
    let mut r = Reading { held: 0, nats: [[0.0; 7]; 2], levels: [[0.0; CODONS]; 2], units: [0; CODONS], supported: 0, folds };
    for fold in 0..folds {
        let train: Vec<String> = tokens.iter().enumerate().filter(|(i, _)| i % folds != fold).map(|(_, t)| t.clone()).collect();
        // Where each original position falls among the training tokens.
        let before = |i: usize| i - (i + folds - 1 - fold) / folds;
        let s = Strand::from_tokens(train.clone(), prior.clone());
        let n = s.tokens.len();
        let u = s.informative_units();
        r.supported += u;
        if fold == 0 {
            r.units = s.units;
        }
        let labels: Vec<usize> = truth.iter().map(|t| before(*t)).collect();
        let eq_cuts = super::score::equal(n, u);
        let (lab, eq) = (segments(&train, &labels), segments(&train, &eq_cuts));
        let (sup, at_k) = (s.cut.units_at(u), s.cut.units_at(truth.len() + 1));
        let levels: Vec<Vec<usize>> = (0..CODONS).map(|k| s.level(k)).collect();
        // The same scopes with counted weights: flat ones under the whole, and the strand's tree.
        let ids: Vec<u32> = s.tokens.iter().map(|t| s.index[t]).collect();
        let under = |cuts: &[usize]| {
            let mut b = vec![0];
            b.extend(cuts.iter().copied().filter(|c| *c > 0 && *c < n));
            b.push(n);
            let mut spans = vec![(0, n, None)];
            spans.extend(b.windows(2).map(|w| (w[0], w[1], Some(0))));
            Scopes::fit(&ids, &spans, &|id| s.prior.p(&s.vocab[id as usize]))
        };
        let sup_cuts: Vec<usize> = sup.iter().map(|x| s.cut.spans[*x].start).collect();
        let (c_lab, c_eq, c_flat, tree) = (under(&labels), under(&eq_cuts), under(&sup_cuts), s.scopes());
        for (i, w) in tokens.iter().enumerate().filter(|(i, _)| i % folds == fold) {
            let a = before(i).saturating_sub(1).min(n - 1);
            let root = s.read(Some(0), w);
            let flat = |segs: &[(HashMap<&str, u32>, usize)], cuts: &[usize]| {
                let (c, len) = &segs[cuts.partition_point(|x| *x <= a)];
                wb(c.get(w.as_str()).copied().unwrap_or(0), c.len(), *len, root)
            };
            let unit = holding(&s, &sup, a);
            let sp = &s.cut.spans[unit];
            let own = s.index.get(w).and_then(|id| sp.counts.get(id)).copied().unwrap_or(0);
            let p = [
                s.read(None, w),
                root,
                flat(&lab, &labels),
                flat(&eq, &eq_cuts),
                wb(own, sp.counts.len(), sp.len(), root),
                s.read(Some(holding(&s, &at_k, a)), w),
                s.read(Some(unit), w),
            ];
            let (id, pr) = (s.index.get(w).copied(), s.prior.p(w));
            let seg = |cuts: &[usize]| 1 + cuts.iter().filter(|c| **c > 0 && **c < n && **c <= a).count();
            let q = [
                pr,
                tree.read(0, id, pr),
                c_lab.read(seg(&labels), id, pr),
                c_eq.read(seg(&eq_cuts), id, pr),
                c_flat.read(seg(&sup_cuts), id, pr),
                tree.read(holding(&s, &at_k, a), id, pr),
                tree.read(unit, id, pr),
            ];
            for (rule, ps) in [p, q].iter().enumerate() {
                for (x, v) in r.nats[rule].iter_mut().zip(ps) {
                    *x -= v.ln();
                }
            }
            for (k, l) in levels.iter().enumerate() {
                let u = holding(&s, l, a);
                r.levels[0][k] -= s.read(Some(u), w).ln();
                r.levels[1][k] -= tree.read(u, id, pr).ln();
            }
            r.held += 1;
        }
    }
    r
}
