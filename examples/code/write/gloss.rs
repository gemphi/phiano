//! Writing by gloss (docs/120). Proportion (docs/119) learns what a word
//! changes only from two learned answers that differ in it; `below` has no
//! such pair when `count_below` is the one held out. Phi-4 is asked instead,
//! word by word, which code token says the word (`code_quest --glosses`):
//! `above` ⇒ `>`, `below` ⇒ `<`. A learned answer whose name differs from
//! the new one in such words is said again with each word's token exchanged
//! for the other's (`count_above`'s `y > x` becomes `count_below`'s `y < x`).
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c): a gloss is one relation of the word, taught by Phi-4, under
//! the condition "in code".

use super::analog::{answer, Answer};
use std::collections::HashMap;

pub fn load(path: &std::path::Path) -> HashMap<String, Vec<String>> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.split_once('\t'))
        .filter(|(_, t)| *t != "none")
        .map(|(w, t)| (w.to_string(), super::lex::tokens(t).unwrap_or_default()))
        .filter(|(_, t)| !t.is_empty())
        .collect()
}

fn replace_all(body: &[String], from: &[String], to: &[String]) -> Option<Vec<String>> {
    let (mut out, mut i, mut hit) = (Vec::new(), 0, false);
    while i < body.len() {
        if body[i..].starts_with(from) {
            out.extend(to.iter().cloned());
            i += from.len();
            hit = true;
        } else {
            out.push(body[i].clone());
            i += 1;
        }
    }
    hit.then_some(out)
}

pub fn write(sig: &[String], learned: &[Vec<String>], glosses: &HashMap<String, Vec<String>>, k: usize) -> Vec<Vec<String>> {
    let fname = |s: &[String]| s.iter().skip_while(|t| *t != "fn").nth(1).cloned();
    let Some(target) = fname(sig) else { return vec![] };
    let mut out = Vec::new();
    for c in learned.iter().map(|t| answer(t)).filter(|c: &Answer| c.sig.len() == sig.len()) {
        let Some(nc) = fname(&c.sig) else { continue };
        let (p, q): (Vec<&str>, Vec<&str>) = (nc.split('_').collect(), target.split('_').collect());
        if p.len() != q.len() || nc == target {
            continue;
        }
        let mut body = Some(c.body.clone());
        for (a, b) in p.iter().zip(&q).filter(|(a, b)| a != b) {
            body = match (glosses.get(*a), glosses.get(*b), body) {
                (Some(ga), Some(gb), Some(x)) if ga != gb => replace_all(&x, ga, gb),
                _ => None,
            };
        }
        // Types as the signature has them: the learned answer's type words exchanged.
        let types: HashMap<&String, &String> = c.sig.iter().zip(sig).filter(|(x, y)| x != y).collect();
        if let Some(b) = body {
            out.push(b.iter().map(|t| if *t == nc { target.clone() } else { types.get(t).map_or(t.clone(), |x| (*x).clone()) }).collect());
        }
    }
    out.truncate(k);
    out
}
