//! The routes to a body, and their vote (docs/121). A told expression
//! (exprs.rs) counts 2, a proportion (proportion.rs) or a token gloss
//! (gloss.rs) 1, a look-alike (analog.rs) 0 — or 5 when it is the same
//! task in another type, the answer itself said again; the body most routes arrive at
//! goes first, the first that compiles is kept. Agreement among routes is
//! the student's agreement in judgments (§242) with itself.

use super::{analog, exprs, gloss, proportion, types};
use std::collections::HashMap;

/// Every body the routes arrive at, with its vote and the routes' names.
pub fn explain(row: usize, sig: &[String], learned: &[analog::Answer], learned_toks: &[Vec<String>], ex: &HashMap<String, Vec<String>>, gl: &HashMap<String, Vec<String>>) -> Vec<(Vec<String>, usize, Vec<&'static str>)> {
    let mut routes: Vec<(Vec<String>, usize, &'static str)> = Vec::new();
    if row == 7 {
        routes.extend(exprs::write(sig, learned_toks, ex, 16).into_iter().map(|b| (b, 2, "expression told by Phi-4")));
    }
    if row >= 5 {
        routes.extend(proportion::write(sig, learned_toks, 16).into_iter().map(|b| (b, 1, "proportion between names")));
        routes.extend(types::write(sig, learned_toks, 8).into_iter().map(|b| (b, 2, "proportion between types")));
    }
    if row >= 6 {
        routes.extend(gloss::write(sig, learned_toks, gl, 16).into_iter().map(|b| (b, 1, "gloss told by Phi-4")));
    }
    routes.extend(analog::write_tagged(sig, learned, 5).into_iter().map(|(b, sibling)| (b, if sibling { 5 } else { 0 }, if sibling { "the same task in another type" } else { "a look-alike answer" })));
    let mut score: Vec<(Vec<String>, usize, Vec<&'static str>, usize)> = Vec::new();
    for (i, (b, w, why)) in routes.into_iter().enumerate() {
        match score.iter_mut().find(|x| x.0 == b) {
            Some(x) => {
                x.1 += w;
                if !x.2.contains(&why) {
                    x.2.push(why);
                }
            }
            None => score.push((b, w, vec![why], i)),
        }
    }
    score.sort_by(|x, y| y.1.cmp(&x.1).then(x.3.cmp(&y.3)));
    score.into_iter().map(|x| (x.0, x.1, x.2)).collect()
}

pub fn vote(row: usize, sig: &[String], learned: &[analog::Answer], learned_toks: &[Vec<String>], ex: &HashMap<String, Vec<String>>, gl: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    explain(row, sig, learned, learned_toks, ex, gl).into_iter().map(|x| x.0).collect()
}
