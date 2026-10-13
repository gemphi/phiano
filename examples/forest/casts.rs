//! The story in the scope tree: its own place, the bounding volumes of
//! *Emma* that hold each of its words, and where each stretch of it is
//! cast — the scope tree read as a bounding-volume hierarchy, a stretch of
//! words a ray drawn down it (`relate::tree`, docs/126).

use phiano::prose::Knowing;
use phiano::relate::SAYS;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

/// The heaviest `n` things scope `s` says, words only.
fn says(k: &Knowing, s: &str, n: usize) -> Vec<(String, f64)> {
    let mut v: Vec<(String, f64)> = k.counts.said(Some(s), SAYS).into_iter().filter(|(w, _)| !w.starts_with("pair:") && w.chars().any(char::is_alphanumeric)).collect();
    v.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    v.truncate(n);
    v
}

/// What a scope says beyond its parent: its words by `c · ln(p_scope / p_parent)`.
fn beyond(k: &Knowing, s: &str, parent: &str, n: usize) -> Vec<(String, f64)> {
    let (ts, tp) = (k.counts.total(Some(s), SAYS).max(1.0), k.counts.total(Some(parent), SAYS).max(1.0));
    let mut v: Vec<(String, f64)> = k
        .counts
        .said(Some(s), SAYS)
        .into_iter()
        .filter(|(w, _)| !w.starts_with("pair:") && w.chars().any(char::is_alphanumeric))
        .map(|(w, c)| {
            let p = k.counts.count(Some(parent), SAYS, &w).max(0.5) / tp;
            let gain = c * ((c / ts) / p).ln();
            (w, gain)
        })
        .collect();
    v.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    v.truncate(n);
    v
}

pub fn scopes(with: &Knowing, without: &Knowing, story: &str, toks: &[String]) -> Value {
    let own = format!("scope:{story}/p1");
    let parent = format!("scope:{}", story.rsplit_once('/').map_or(story, |x| x.0));
    let tree: Vec<Value> = with.tree.roots().iter().map(|r| node(with, r, 0)).collect();
    // Bounding volumes: the Emma paragraphs and chapters that say each word.
    let mut paras: HashMap<&str, usize> = HashMap::new();
    let mut chapters: HashMap<&str, HashSet<String>> = HashMap::new();
    for (path, words) in &without.paragraphs {
        let ch = path.rsplit_once('/').map_or(path.clone(), |x| x.0.to_string());
        for w in words.iter().collect::<HashSet<_>>() {
            if toks.contains(w) {
                *paras.entry(w.as_str()).or_default() += 1;
                chapters.entry(w.as_str()).or_default().insert(ch.clone());
            }
        }
    }
    let mut seen = HashSet::new();
    let volumes: Vec<Value> = toks
        .iter()
        .filter(|t| t.chars().any(char::is_alphanumeric) && seen.insert(t.as_str()))
        .map(|t| json!({ "word": t, "paragraphs": paras.get(t.as_str()).copied().unwrap_or(0), "chapters": chapters.get(t.as_str()).map_or(0, HashSet::len) }))
        .collect();
    json!({
        "tree": tree,
        "own": { "scope": own, "says": says(with, &own, 12), "beyond_parent": beyond(with, &own, &parent, 12) },
        "emma_paragraphs": without.paragraphs.len(),
        "volumes": volumes,
        "casts": casts(without, toks),
    })
}

/// A scope and its children, down to chapters (paragraphs are counted, not listed).
fn node(k: &Knowing, s: &str, depth: usize) -> Value {
    let kids = k.tree.children(s);
    let leafy = kids.iter().all(|c| k.tree.children(c).is_empty());
    let children: Vec<Value> = if leafy || depth >= 3 { Vec::new() } else { kids.iter().map(|c| node(k, c, depth + 1)).collect() };
    json!({ "scope": s, "told": k.counts.total(Some(s), SAYS), "children": children, "paragraphs": if leafy { kids.len() } else { 0 }, "says": says(k, s, 6) })
}

/// Each stretch of six words, three apart, cast into *Emma*'s tree alone.
fn casts(k: &Knowing, toks: &[String]) -> Vec<Value> {
    let words: Vec<&String> = toks.iter().filter(|t| t.chars().any(char::is_alphanumeric)).collect();
    (0..words.len().saturating_sub(5))
        .step_by(3)
        .map(|i| {
            let w: Vec<String> = words[i..i + 6].iter().map(|s| s.to_string()).collect();
            let mut conds = w.clone();
            conds.extend(w.windows(2).map(|p| format!("pair:{} {}", p[0], p[1])));
            let found = k.tree.cast_under(&k.counts, &[], &conds, f64::MIN, 2);
            let top: Vec<Value> = found.iter().take(3).map(|c| json!({ "path": c.path, "said": c.said, "own": c.own, "attraction": c.attraction })).collect();
            let leaf = found.first().and_then(|c| c.path.last().cloned()).unwrap_or_default();
            json!({ "words": w.join(" "), "found": top, "leaf_says": says(k, &leaf, 8) })
        })
        .collect()
}

/// The story gone on with: from its last words inside its own chapter, and
/// its opening read by *Emma*'s language alone. Generated, and marked so.
pub fn go(with: &Knowing, without: &Knowing, toks: &[String]) -> Value {
    let tail: Vec<String> = toks.iter().rev().take(12).rev().cloned().collect();
    let head: Vec<String> = toks.iter().take(6).cloned().collect();
    json!({
        "from_the_end": { "from": tail.join(" "), "said": with.go(&tail.join(" "), 40, true) },
        "opening_in_emma": { "from": head.join(" "), "said": without.go(&head.join(" "), 40, true) },
    })
}
