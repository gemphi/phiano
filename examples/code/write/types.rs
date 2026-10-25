//! Proportion over types (docs/122): as `max(i32)` is to `max(f64)`, so is
//! `min(i32)` to `min(f64)`. A sibling's body does not always carry to a new
//! type — `f64` has no `%` and no `.max()`, `u32` has no `.abs()` — but Phi-4's
//! own variants show the change: the difference between its `max_i32` and
//! `max_f64` bodies is the edit `i32 → f64` makes. The edit most names
//! agree on is applied to the target's sibling in the source type.
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c): here the type is the condition, and the edit is what the
//! same word does under it.

use super::analog::{answer, Answer};
use super::proportion::{apply, diff, Edit};
use std::collections::HashMap;

const TYPES: [&str; 5] = ["i32", "i64", "u32", "u64", "f64"];

fn name(a: &Answer) -> Option<&String> {
    a.sig.iter().skip_while(|t| *t != "fn").nth(1)
}

fn ty(sig: &[String]) -> Option<&str> {
    sig.iter().find_map(|t| TYPES.iter().find(|x| **x == t).copied())
}

/// Bodies for `sig` from its siblings in other types, each carried over by
/// the edits learned from other names' pairs of those two types.
pub fn write(sig: &[String], learned: &[Vec<String>], k: usize) -> Vec<Vec<String>> {
    let answers: Vec<Answer> = learned.iter().map(|t| answer(t)).collect();
    let (Some(target), Some(to)) = (sig.iter().skip_while(|t| *t != "fn").nth(1), ty(sig)) else { return vec![] };
    // Edits per (from type, to type), from pairs of one name.
    let mut edits: HashMap<(String, String), Vec<Edit>> = HashMap::new();
    for x in &answers {
        for y in &answers {
            let (Some(nx), Some(ny), Some(tx), Some(ty_)) = (name(x), name(y), ty(&x.sig), ty(&y.sig)) else { continue };
            if nx != ny || tx == ty_ || ty_ != to {
                continue;
            }
            let bx: Vec<String> = x.body.iter().map(|t| if t == tx { ty_.to_string() } else { t.clone() }).collect();
            edits.entry((tx.to_string(), ty_.to_string())).or_default().push(diff(&bx, &y.body));
        }
    }
    let mut out: Vec<(Vec<String>, usize)> = Vec::new();
    for s in answers.iter().filter(|a| name(a) == Some(target) && ty(&a.sig).is_some_and(|t| t != to)) {
        let from = ty(&s.sig).unwrap();
        let base: Vec<String> = s.body.iter().map(|t| if t == from { to.to_string() } else { t.clone() }).collect();
        for e in edits.get(&(from.to_string(), to.to_string())).into_iter().flatten() {
            if let Some(b) = apply(&base, e) {
                match out.iter_mut().find(|x| x.0 == b) {
                    Some(x) => x.1 += 1,
                    None => out.push((b, 1)),
                }
            }
        }
    }
    out.sort_by_key(|x| std::cmp::Reverse(x.1));
    out.into_iter().take(k).map(|x| x.0).collect()
}
