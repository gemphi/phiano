//! Writing by expression exchange (docs/121). A one-token gloss (gloss.rs)
//! cannot tell `even` from `odd` (both `%`): what a word says in code is an
//! expression over a value, `x % 2 == 0` against `x % 2 != 0`, `x > 0`
//! against `x < 0`. Phi-4 is asked for each word's expression over `x` (and
//! a bound `y`), `code_quest --exprs`. A learned answer whose name differs
//! from the new one in such words is said again with the old word's
//! expression replaced by the new one's, the variables aligned to the body's
//! own (`x` to the closure's `x`, `y` to the function's second parameter).
//!
//! In Phient's world, a word (w) is nothing but what it relates to, under what
//! conditions (c): the expression is the word's relation under "in code",
//! at the size the word has.

use super::analog::answer;
use std::collections::HashMap;

pub fn load(path: &std::path::Path) -> HashMap<String, Vec<String>> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.split_once('\t'))
        .filter(|(_, e)| *e != "none")
        .filter_map(|(w, e)| super::lex::tokens(e).map(|t| (w.to_string(), t)))
        .filter(|(_, t)| !t.is_empty())
        .collect()
}

fn is_ident(t: &str) -> bool {
    t.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_')
}

/// The identifiers a body binds: closure parameters and the signature's parameters.
fn bound(sig: &[String], body: &[String]) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for w in body.windows(2) {
        if w[0] == "|" || w[0] == "&" {
            if let Some(n) = w.get(1).filter(|t| is_ident(t) && !["mut", "ref"].contains(&t.as_str())) {
                v.push(n.clone());
            }
        }
    }
    for w in sig.windows(2) {
        if w[1] == ":" && is_ident(&w[0]) {
            v.push(w[0].clone());
        }
    }
    v.dedup();
    v
}

fn rename(e: &[String], x: &str, y: Option<&str>) -> Vec<String> {
    e.iter().map(|t| match t.as_str() { "x" => x.to_string(), "y" => y.unwrap_or("y").to_string(), _ => t.clone() }).collect()
}

fn replace_all(body: &[String], from: &[String], to: &[String]) -> Option<Vec<String>> {
    let (mut out, mut i, mut hit) = (Vec::new(), 0, false);
    while i < body.len() {
        if !from.is_empty() && body[i..].starts_with(from) {
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

/// `body` with the expression of `a` replaced by that of `b`, trying each
/// alignment of the expressions' `x` and `y` to the body's bound names.
fn exchange(sig: &[String], body: &[String], ea: &[String], eb: &[String]) -> Option<Vec<String>> {
    let names = bound(sig, body);
    let needs_y = ea.iter().any(|t| t == "y");
    for x in &names {
        let ys: Vec<Option<&str>> = if needs_y { names.iter().filter(|n| *n != x).map(|n| Some(n.as_str())).collect() } else { vec![None] };
        for y in ys {
            if let Some(b) = replace_all(body, &rename(ea, x, y), &rename(eb, x, y)) {
                return Some(b);
            }
        }
    }
    None
}

pub fn write(sig: &[String], learned: &[Vec<String>], exprs: &HashMap<String, Vec<String>>, k: usize) -> Vec<Vec<String>> {
    let fname = |s: &[String]| s.iter().skip_while(|t| *t != "fn").nth(1).cloned();
    let Some(target) = fname(sig) else { return vec![] };
    let mut out = Vec::new();
    for c in learned.iter().map(|t| answer(t)).filter(|c| c.sig.len() == sig.len()) {
        let Some(nc) = fname(&c.sig) else { continue };
        let (p, q): (Vec<&str>, Vec<&str>) = (nc.split('_').collect(), target.split('_').collect());
        if p.len() != q.len() || nc == target {
            continue;
        }
        let mut body = Some(c.body.clone());
        for (a, b) in p.iter().zip(&q).filter(|(a, b)| a != b) {
            body = match (exprs.get(*a), exprs.get(*b), body) {
                (Some(ea), Some(eb), Some(x)) if ea != eb => exchange(&c.sig, &x, ea, eb),
                _ => None,
            };
        }
        let types: HashMap<&String, &String> = c.sig.iter().zip(sig).filter(|(x, y)| x != y).collect();
        if let Some(b) = body {
            out.push(b.iter().map(|t| if *t == nc { target.clone() } else { types.get(t).map_or(t.clone(), |x| (*x).clone()) }).collect());
        }
    }
    out.truncate(k);
    out
}
