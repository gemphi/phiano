//! Writing by family resemblance (docs/118, PI §§65–67). A new signature is
//! set beside the signatures of the answers already learned. Where two are
//! alike token for token except for some words (`fn product(v: &[i32]) -> i32`
//! and `fn sum(v: &[i64]) -> i64`), those words are taken as standing for
//! each other (product ↔ sum, i32 ↔ i64). The learned body is then said again
//! with them exchanged: `v.iter().product()` becomes `v.iter().sum()`. That is
//! the relation "said in the signature ⇒ said again in the body" (sig.rs),
//! carried by a whole answer instead of token by token.
//!
//! In Phient's world, a word (w) is nothing but what it relates to, under what
//! conditions (c): `sum` stands where `product` stood, under the same
//! signature.

use std::collections::HashMap;

/// A learned answer: its signature tokens (up to the first `{`) and body.
pub struct Answer {
    pub sig: Vec<String>,
    pub body: Vec<String>,
}

/// A candidate body under its rank key (exchanged non-type words, whether it
/// does not say its own name, likeness), marked `true` for a type sibling.
type Ranked = ((usize, bool, usize), Vec<String>, bool);

/// The answer in a file: its last top-level function is the task's; any
/// item before it (a helper such as `gcd` in `lcm`'s answer) goes after the
/// body, where Rust is as happy to find it.
pub fn answer(toks: &[String]) -> Answer {
    let (mut depth, mut last) = (0i32, 0usize);
    for (i, t) in toks.iter().enumerate() {
        match t.as_str() {
            "{" => depth += 1,
            "}" => depth -= 1,
            "fn" if depth == 0 => last = i,
            _ => {}
        }
    }
    let open = toks[last..].iter().position(|t| t == "{").map_or(toks.len(), |i| last + i);
    let mut body = toks[open..].to_vec();
    body.extend(toks[..last].iter().cloned());
    Answer { sig: toks[last..open].to_vec(), body }
}

/// The words exchanged between two signatures of the same shape, and how many
/// tokens they share; `None` when the shapes differ or a word would have to
/// stand for two.
fn exchange(from: &[String], to: &[String]) -> Option<(HashMap<String, String>, usize)> {
    if from.len() != to.len() {
        return None;
    }
    let mut map: HashMap<String, String> = HashMap::new();
    let mut same = 0;
    for (a, b) in from.iter().zip(to) {
        if a == b {
            same += 1;
            continue;
        }
        let ident = |s: &str| s.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_');
        if !ident(a) || !ident(b) || map.get(a).is_some_and(|x| x != b) {
            return None;
        }
        map.insert(a.clone(), b.clone());
    }
    Some((map, same))
}

const TYPES: [&str; 6] = ["i32", "i64", "u32", "u64", "f64", "usize"];

/// Bodies for `sig`, by the most alike learned answers first, each marked
/// `true` when the two signatures differ in type words only (the same task
/// in another type: the answer itself, said again).
pub fn write_tagged(sig: &[String], learned: &[Answer], k: usize) -> Vec<(Vec<String>, bool)> {
    // Ranked by how many non-type words are exchanged (fewest first: a type
    // sibling before a name sibling), then by whether the learned answer
    // says its own name again in its body (`v.iter().min()` in `min`, so
    // the exchange carries the new name into the body), then by likeness.
    let mut alike: Vec<Ranked> = learned
        .iter()
        .filter_map(|a| {
            let (map, same) = exchange(&a.sig, sig)?;
            let non_type = map.keys().filter(|w| !TYPES.contains(&w.as_str())).count();
            let name = a.sig.iter().skip_while(|t| *t != "fn").nth(1);
            let says_name = name.is_some_and(|n| map.contains_key(n) && a.body.contains(n));
            let body = a.body.iter().map(|t| map.get(t).cloned().unwrap_or_else(|| t.clone())).collect();
            Some(((non_type, !says_name, usize::MAX - same), body, non_type == 0))
        })
        .collect();
    alike.sort_by_key(|x| x.0);
    alike.into_iter().take(k).map(|x| (x.1, x.2)).collect()
}

pub fn write(sig: &[String], learned: &[Answer], k: usize) -> Vec<Vec<String>> {
    write_tagged(sig, learned, k).into_iter().map(|x| x.0).collect()
}

