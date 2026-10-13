//! Each word of the story as the forest holds it (docs/155): how often the
//! language has heard it, what it goes on to in *Emma* and in this story,
//! its kin, how far it moves the next word from the prior (D2's post), and,
//! for a word the language never heard, what the forest would put in its
//! place — read from the words on both sides of it.

use phiano::prose::Knowing;
use phiano::relate::SAYS;
use serde_json::{json, Value};
use std::collections::HashSet;

fn top(mut v: Vec<(String, f64)>, n: usize) -> Vec<(String, f64)> {
    v.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    v.truncate(n);
    v
}

/// KL (nats) of what follows `c` from what follows anything: D2's post.
fn post(k: &Knowing, c: &str) -> f64 {
    let told = k.counts.said(Some(c), "next");
    let n: f64 = told.iter().map(|x| x.1).sum();
    let all = k.counts.total(None, "next").max(1.0);
    told.iter().map(|(w, m)| (m / n) * ((m / n) / (k.counts.count(None, "next", w).max(0.5) / all)).ln()).sum()
}

/// The words that fit between `left` (everything said before) and `right`
/// (the word after): `ln p(v | left) + ln p(right | v)`, the second with a
/// half count for what was never seen.
fn between(k: &Knowing, left: &[String], right: Option<&String>) -> Vec<(String, f64)> {
    let (_, ranked, _) = k.next(left);
    let fits: Vec<(String, f64)> = ranked
        .into_iter()
        .take(300)
        .filter(|(v, _)| v.chars().any(char::is_alphanumeric))
        .map(|(v, s)| {
            let after = right.map_or(0.0, |r| ((k.counts.count(Some(&v), "next", r) + 0.5) / (k.counts.total(Some(&v), "next") + 1.0)).ln());
            (v, s + after)
        })
        .collect();
    top(fits, 6)
}

pub fn words(with: &Knowing, without: &Knowing, story: &str, toks: &[String]) -> Vec<Value> {
    let own = format!("scope:{story}/p1");
    let mut seen = HashSet::new();
    toks.iter()
        .enumerate()
        .filter(|(_, t)| t.chars().any(char::is_alphanumeric))
        .filter(|(_, t)| seen.insert(t.as_str()))
        .map(|(i, w)| {
            // How often *Emma* says the word at all (`has-word` is a given
            // kind — a condition, not a counted value — so `says` is what counts).
            let heard = without.counts.count(None, SAYS, w);
            let here = with.counts.said(Some(&format!("{own}|{w}")), "next");
            let kin: Vec<(String, f64)> = without.counts.kin(w).iter().take(6).cloned().collect();
            let slot = (heard == 0.0).then(|| {
                json!({
                    "left_only": top(without.next(&toks[..i]).1.into_iter().filter(|(v, _)| v.chars().any(char::is_alphanumeric)).collect(), 6),
                    "both_sides": between(without, &toks[..i], toks.get(i + 1)),
                })
            });
            json!({
                "word": w,
                "at": i,
                "heard_in_emma": heard,
                "next_in_emma": top(without.counts.said(Some(w), "next"), 6),
                "next_here": top(here, 4),
                "post_nats": if heard > 0.0 { post(without, w) } else { 0.0 },
                "kin": kin,
                "never_heard": slot,
            })
        })
        .collect()
}
