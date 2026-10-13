//! The story told to a phiecs world (docs/155): each word an entity, each
//! pair an edge under `before` / `after`, words said together in one
//! stretch `near`. Every word carries a phasor — 64 relation channels, an
//! angle and a count on each — and two words that were related move toward
//! each other on that channel alone. A word has no position: what it has is
//! its angles, and they are only ever moved by what it was seen with.

use phiecs::phasor::Phasor;
use phiecs::prelude::*;
use serde_json::{json, Value};
use std::collections::HashSet;

/// The story's own relations, told the way a reader meets them.
pub fn world(toks: &[String]) -> Value {
    let words: Vec<&String> = toks.iter().filter(|t| t.chars().any(char::is_alphanumeric)).collect();
    let mut w = World::new();
    let (before, after, near) = (w.declare("before").unwrap(), w.declare("after").unwrap(), w.declare("near").unwrap());
    for pair in words.windows(2) {
        let (a, b) = (w.coin_bare(pair[0]), w.coin_bare(pair[1]));
        w.relate(a, before, b);
        w.relate(b, after, a);
    }
    // `near`: the words of one stretch of six, each to each, as a paragraph
    // relates everything said in it.
    for stretch in words.chunks(6) {
        for (i, x) in stretch.iter().enumerate() {
            for y in &stretch[i + 1..] {
                let (a, b) = (w.coin_bare(x), w.coin_bare(y));
                w.relate(a, near, b);
                w.relate(b, near, a);
            }
        }
    }
    // The condition the story turns under: what each of its two halves says.
    let half = words.len() / 2;
    for (name, part) in [("rising", &words[..half]), ("resolving", &words[half..])] {
        for pair in part.windows(2) {
            let (a, b) = (w.coin_bare(pair[0]), w.coin_bare(pair[1]));
            w.relate_under(name, a, b, 1.0);
        }
    }
    let mut seen = HashSet::new();
    let unique: Vec<&String> = words.iter().filter(|t| seen.insert(t.as_str())).copied().collect();
    let entities: Vec<Value> = unique
        .iter()
        .map(|t| {
            let id = w.lexicon.get(t).expect("told");
            let p = w.read::<Phasor>(id).expect("phasor");
            let kinds: Vec<Value> = p.kinds().into_iter().map(|(k, n)| json!({ "kind": w.relations.name(k), "channel": k.channel(), "theta": p.theta(k), "amplitude": p.amplitude(k), "evidence": n })).collect();
            let mut edges: Vec<Value> = Vec::new();
            for (name, kind) in [("before", before), ("after", after), ("near", near)] {
                for (to, weight) in w.calls_for(id, kind).into_iter().take(4) {
                    edges.push(json!({ "kind": name, "to": w.spelling(to), "weight": weight }));
                }
            }
            let under: Vec<Value> = w
                .under
                .conditions_of(id)
                .iter()
                .map(|c| json!({ "condition": w.spelling(*c), "calls_for": w.under.calls_for(*c, id).iter().take(3).map(|(to, n)| json!([w.spelling(*to), n])).collect::<Vec<_>>() }))
                .collect();
            json!({ "word": t, "channels": kinds, "edges": edges, "under": under })
        })
        .collect();
    // Who resonates with whom: the cosine over every channel both have moved.
    let mut pairs: Vec<Value> = Vec::new();
    for (i, x) in unique.iter().enumerate() {
        for y in unique.iter().skip(i + 1) {
            let (a, b) = (w.lexicon.get(x).expect("told"), w.lexicon.get(y).expect("told"));
            let (pa, pb) = (w.read::<Phasor>(a).expect("phasor"), w.read::<Phasor>(b).expect("phasor"));
            pairs.push(json!({ "a": x, "b": y, "resonance": pa.resonance(pb) }));
        }
    }
    pairs.sort_by(|a, b| b["resonance"].as_f64().unwrap_or(0.0).total_cmp(&a["resonance"].as_f64().unwrap_or(0.0)));
    pairs.truncate(24);
    json!({ "words": entities.len(), "edges": w.edges.len(), "under": w.under.len(), "entities": entities, "resonant_pairs": pairs })
}
