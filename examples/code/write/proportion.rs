//! Writing by proportion (docs/119): as `sum_even` is to `sum_odd`, so is
//! `count_even` to `count_odd`. A word's meaning in code is the change it
//! makes: the difference between two learned answers whose names differ in
//! one word (`even` → `odd`) is that word pair's edit (`== 0` → `!= 0`). A
//! new name that differs from a learned one in the same word gets the
//! learned body with the edit made.
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c): here `odd` is what it changes, against `even`, under the
//! condition of a body otherwise the same.

use super::analog::{answer, Answer};
use std::collections::HashMap;

const TYPES: [&str; 5] = ["i32", "i64", "u32", "u64", "f64"];

fn name(a: &Answer) -> Option<&String> {
    a.sig.iter().skip_while(|t| *t != "fn").nth(1)
}

fn ty(sig: &[String]) -> Option<&str> {
    sig.iter().find_map(|t| TYPES.iter().find(|x| **x == t).copied())
}

/// The words two names differ in, as (in `a`, in `b`), place by place.
fn words(a: &str, b: &str) -> Option<Vec<(String, String)>> {
    let (p, q): (Vec<&str>, Vec<&str>) = (a.split('_').collect(), b.split('_').collect());
    if p.len() != q.len() {
        return None;
    }
    Some((0..p.len()).filter(|&i| p[i] != q[i]).map(|i| (p[i].to_string(), q[i].to_string())).collect())
}

/// The one word two names differ in.
fn one_word(a: &str, b: &str) -> Option<(String, String)> {
    words(a, b).filter(|d| d.len() == 1).map(|mut d| d.remove(0))
}

/// The changed stretches from `x` to `y`, each with the token before it.
pub type Edit = Vec<(Option<String>, Vec<String>, Vec<String>)>;

pub fn diff(x: &[String], y: &[String]) -> Edit {
    let (n, m) = (x.len(), y.len());
    let mut l = vec![vec![0usize; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            l[i][j] = if x[i] == y[j] { l[i + 1][j + 1] + 1 } else { l[i + 1][j].max(l[i][j + 1]) };
        }
    }
    let (mut i, mut j, mut out): (usize, usize, Edit) = (0, 0, Vec::new());
    while i < n || j < m {
        if i < n && j < m && x[i] == y[j] {
            i += 1;
            j += 1;
            continue;
        }
        let ctx = (i > 0).then(|| x[i - 1].clone());
        let (i0, j0) = (i, j);
        while (i < n || j < m) && !(i < n && j < m && x[i] == y[j]) {
            if j >= m || (i < n && l[i + 1][j] >= l[i][j + 1]) {
                i += 1;
            } else {
                j += 1;
            }
        }
        out.push((ctx, x[i0..i].to_vec(), y[j0..j].to_vec()));
    }
    out
}

/// `body` with every stretch of the edit changed, or `None` if one is not found.
pub fn apply(body: &[String], edit: &Edit) -> Option<Vec<String>> {
    let mut b = body.to_vec();
    for (ctx, old, new) in edit {
        let at = (0..=b.len()).find(|&k| {
            let before = ctx.as_ref().map_or(k == 0, |c| k > 0 && &b[k - 1] == c);
            before && b.len() >= k + old.len() && b[k..k + old.len()] == old[..]
        })?;
        b.splice(at..at + old.len(), new.iter().cloned());
    }
    Some(b)
}

/// Bodies for the signature `sig`, by proportion with the learned answers.
pub fn write(sig: &[String], learned: &[Vec<String>], k: usize) -> Vec<Vec<String>> {
    let answers: Vec<Answer> = learned.iter().map(|t| answer(t)).collect();
    let Some(target) = sig.iter().skip_while(|t| *t != "fn").nth(1) else { return vec![] };
    let target_ty = ty(sig);
    // Edits per word pair, from learned pairs of one type whose names differ in one word.
    let mut edits: HashMap<(String, String), Vec<Edit>> = HashMap::new();
    for x in &answers {
        for y in &answers {
            let (Some(nx), Some(ny)) = (name(x), name(y)) else { continue };
            if nx == ny || ty(&x.sig) != ty(&y.sig) || x.sig.len() != y.sig.len() {
                continue;
            }
            if let Some(pair) = one_word(nx, ny) {
                let body_y: Vec<String> = y.body.iter().map(|t| if t == ny { nx.clone() } else { t.clone() }).collect();
                edits.entry(pair).or_default().push(diff(&x.body, &body_y));
            }
        }
    }
    // One word changed, then two (each by its own learned edit, in turn):
    // as `count_even` : `count_odd`, and `count` : `sum`, so `count_even` : `sum_odd`.
    let mut out = Vec::new();
    for depth in 1..=2 {
        for c in answers.iter().filter(|c| ty(&c.sig) == target_ty && c.sig.len() == sig.len()) {
            let Some(nc) = name(c) else { continue };
            let Some(pairs) = words(nc, target).filter(|d| d.len() == depth) else { continue };
            let mut bodies = vec![c.body.clone()];
            for pair in &pairs {
                let es = edits.get(pair).map(|v| &v[..if depth == 1 { v.len() } else { v.len().min(3) }]).unwrap_or(&[]);
                bodies = bodies.iter().flat_map(|b| es.iter().filter_map(|e| apply(b, e))).collect();
            }
            for b in bodies {
                out.push(b.iter().map(|t| if t == nc { target.clone() } else { t.clone() }).collect::<Vec<String>>());
            }
        }
    }
    // Many routes, one body: the body most routes arrive at comes first (a
    // vote among the proportions, as agreement in judgments, §242).
    let mut votes: HashMap<Vec<String>, (usize, usize)> = HashMap::new();
    for (i, b) in out.iter().enumerate() {
        votes.entry(b.clone()).or_insert((0, i)).0 += 1;
    }
    let mut ranked: Vec<(Vec<String>, (usize, usize))> = votes.into_iter().collect();
    ranked.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.1 .1.cmp(&b.1 .1)));
    let mut out: Vec<Vec<String>> = ranked.into_iter().map(|x| x.0).collect();
    out.truncate(k);
    out
}
