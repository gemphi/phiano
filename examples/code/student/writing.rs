//! The student writing: a signature's conditions, the bodies it writes for
//! them (a told body said again with the words exchanged), and the
//! proportion that names the exchanges (docs/124).

use super::rows::{given, rows_of};
use super::student::{Student, EXCHANGED, TAU};
use std::collections::HashMap;

impl Student {
    /// The conditions of a signature: the name's words and its types.
    pub fn conditions(sig: &str) -> Vec<String> {
        let Some((rows, name)) = rows_of(&format!("{sig} {{}}")) else { return Vec::new() };
        let (words, takes, returns) = given(&rows, &name);
        words.iter().chain(&takes).chain(&returns).cloned().collect()
    }

    /// Bodies for the signature `sig`, best first, each with the told
    /// function it says again and the exchanges made.
    pub fn write(&self, sig: &str, k: usize) -> Vec<(Vec<String>, String, Vec<String>)> {
        let Some((rows, name)) = rows_of(&format!("{sig} {{}}")) else { return Vec::new() };
        let (words, takes, returns) = given(&rows, &name);
        let conds: Vec<String> = words.iter().chain(&takes).chain(&returns).cloned().collect();
        let ranked = self.said_again(&conds);
        let mut out = Vec::new();
        for (src_name, _) in ranked.iter().take(k) {
            let Some(src) = self.told.iter().find(|t| t.key == *src_name) else { continue };
            let mut map: HashMap<String, String> = HashMap::new();
            map.insert(src.name.clone(), name.clone());
            for (a, b) in src.takes.iter().zip(&takes).chain(src.returns.iter().zip(&returns)) {
                if a != b {
                    map.insert(a.clone(), b.clone());
                }
            }
            let mine: Vec<String> = words.iter().filter(|w| !src.words.contains(w)).cloned().collect();
            let theirs: Vec<String> = src.words.iter().filter(|w| !words.contains(w)).cloned().collect();
            let shared: Vec<String> = words.iter().filter(|w| src.words.contains(w)).cloned().collect();
            // The types stand in proportion too, each by its place in the
            // signature (`i32 : 0 :: f64 : 0.0`, `i32 : max :: f64 : max_by`),
            // under the words.
            let mut pairs: Vec<(Vec<String>, Vec<String>, Vec<String>)> = vec![(theirs.clone(), mine.clone(), shared.clone())];
            for (a, b) in src.takes.iter().zip(&takes).chain(src.returns.iter().zip(&returns)).filter(|(a, b)| a != b) {
                if !pairs.iter().any(|p| p.0 == [a.clone()]) {
                    pairs.push((vec![a.clone()], vec![b.clone()], words.clone()));
                }
            }
            let mut why = Vec::new();
            for kind in EXCHANGED {
                for (theirs, mine, shared) in &pairs {
                    for (old, new) in self.proportion(shared, theirs, mine, kind, &src.body) {
                        why.push(format!("{old} ⇒ {new} ({kind}: {} : {old} :: {} : {new})", theirs.join(" "), mine.join(" ")));
                        map.entry(old).or_insert(new);
                    }
                }
            }
            let body: Vec<String> = src.body.iter().map(|t| map.get(t).cloned().unwrap_or_else(|| t.clone())).collect();
            out.push((body, src.key.clone(), why));
        }
        out
    }

    /// The exchanges of `kind` the proportion licenses between the
    /// conditions only the source has (`theirs`) and those only the new
    /// name has (`mine`): `old` in the body is what `theirs` moves to beyond
    /// `mine` and beyond the shared conditions; `new` is what `mine` moves
    /// to beyond `theirs` and the shared, the furthest first. To move to a
    /// value is to double its odds. Each new token stands for one old.
    fn proportion(&self, shared: &[String], theirs: &[String], mine: &[String], kind: &str, body: &[String]) -> Vec<(String, String)> {
        if mine.is_empty() || theirs.is_empty() {
            return Vec::new();
        }
        let values = self.counts.values(kind);
        let prior: Vec<f64> = self.counts.prior(kind, &values).iter().map(|p| p.ln()).collect();
        let moves = |conds: &[String]| -> Vec<f64> { self.counts.read(conds, kind, &values, TAU).0.iter().zip(&prior).map(|(s, p)| s - p).collect() };
        let (by_shared, by_theirs, by_mine) = (moves(shared), moves(theirs), moves(mine));
        let doubles = std::f64::consts::LN_2;
        let at = |v: &str| values.iter().position(|x| x == v);
        let mut out: Vec<(String, String)> = Vec::new();
        for old in body.iter().filter(|t| at(t).is_some()) {
            let i = at(old).unwrap();
            if by_theirs[i] - by_mine[i].max(by_shared[i]) < doubles {
                continue;
            }
            let moved = |j: usize| by_mine[j] - by_theirs[j].max(by_shared[j]) >= doubles;
            let best = (0..values.len()).filter(|&j| j != i && moved(j)).max_by(|&a, &b| (by_mine[a] - by_theirs[a]).total_cmp(&(by_mine[b] - by_theirs[b])));
            if let Some(j) = best {
                if !out.iter().any(|(o, n)| *o == *old || *n == values[j]) {
                    out.push((old.clone(), values[j].clone()));
                }
            }
        }
        out
    }
}
