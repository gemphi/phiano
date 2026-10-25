//! §§107, 125: the rough ground. The ideal of a consistent calculus against
//! what the teacher actually said: Phi-4's answers contain rows that
//! contradict one another under the kinds' ordinary reading (a is-a b and b
//! is-a a; a has b and a part-of b; a part-of b and b part-of a). Does the
//! contradiction stop the game? Train on the told rows with and without the
//! contradicting rows; read the same held-out rows.

use super::data::Table;
use phiano::relate::{held_out, RelationModel};
use phiecs::quest::Row;
use phistd::Adam;
use phical::{relate, FnoConfig, Tie};
use std::collections::HashSet;

/// Each contradicting row, with the kind of contradiction it takes part in.
pub fn contradictions(t: &Table) -> Vec<(&'static str, &Row)> {
    let has: HashSet<(&str, &str, &str)> = t.rows.iter().map(|r| (r.about.as_str(), r.kind.as_str(), r.other.as_str())).collect();
    let mut out = Vec::new();
    for r in &t.rows {
        let (a, k, o) = (r.about.as_str(), r.kind.as_str(), r.other.as_str());
        let why = match k {
            "is-a" if has.contains(&(o, "is-a", a)) => Some("a is-a b and b is-a a"),
            "part-of" if has.contains(&(o, "part-of", a)) => Some("a part-of b and b part-of a"),
            "has" if has.contains(&(a, "part-of", o)) => Some("a has b and a part-of b"),
            "part-of" if has.contains(&(a, "has", o)) => Some("a has b and a part-of b"),
            _ => None,
        };
        if let Some(w) = why {
            out.push((w, r));
        }
    }
    out
}

/// Held-out MRR and within-kind MRR, trained on `told`.
pub fn train_and_read(t: &Table, told: &[Row], held: &[Row], epochs: usize, seed: u64) -> (f64, f64) {
    let config = FnoConfig { space: 64, width: 16, layers: 2, modes: 16, margin: 0.3, lr: 5e-3, tie: Tie::Circle };
    let mut m = RelationModel::new(&t.rows, None, config, seed);
    let tt: Vec<_> = told.iter().filter_map(|r| m.triple(r)).collect();
    let ht: Vec<_> = held.iter().filter_map(|r| m.triple(r)).collect();
    let all: Vec<_> = t.rows.iter().filter_map(|r| m.triple(r)).collect();
    let (pool, by_kind) = relate::pools(&all);
    let examples: Vec<_> = tt.iter().map(relate::example).collect();
    let (mut ao, mut at) = (Adam::new(config.lr), Adam::new(config.lr * 0.5));
    for e in 0..epochs {
        m.joint.train_epoch(&examples, &pool, 32, &mut ao, &mut at, seed ^ e as u64);
    }
    let s = relate::score(&mut m.joint, &ht, &pool, &by_kind);
    (s.mrr, s.within)
}

pub fn report(t: &Table, epochs: usize, seeds: &[u64]) -> String {
    let bad = contradictions(t);
    let bad_rows: HashSet<(&str, &str, &str)> = bad.iter().map(|(_, r)| (r.about.as_str(), r.kind.as_str(), r.other.as_str())).collect();
    let is_bad = |r: &Row| bad_rows.contains(&(r.about.as_str(), r.kind.as_str(), r.other.as_str()));
    let mut s = format!("## The rough ground (§§107, 125): contradictions in Phi-4's answers\n\n{} of {} rows take part in a contradiction:\n\n| contradiction | rows |\n|---|---|\n", bad_rows.len(), t.rows.len());
    for kind in ["a is-a b and b is-a a", "a has b and a part-of b", "a part-of b and b part-of a"] {
        let v: Vec<String> = bad.iter().filter(|(w, _)| *w == kind).map(|(_, r)| format!("{} {} {}", r.about, r.kind, r.other)).collect();
        s += &format!("| {kind} | {} (e.g. {}) |\n", v.len(), v.iter().take(4).cloned().collect::<Vec<_>>().join("; "));
    }
    let told_all: Vec<Row> = t.rows.iter().filter(|r| !held_out(r)).cloned().collect();
    let told_clean: Vec<Row> = told_all.iter().filter(|r| !is_bad(r)).cloned().collect();
    let held: Vec<Row> = t.rows.iter().filter(|r| held_out(r) && !is_bad(r)).cloned().collect();
    s += &format!("\nTrained on the told rows with the contradicting ones ({}) and without ({}); read on the {} held-out rows that contradict nothing.\n\n| seed | with contradictions: MRR / within | without: MRR / within |\n|---|---|---|\n", told_all.len(), told_clean.len(), held.len());
    for &seed in seeds {
        let (a, b) = train_and_read(t, &told_all, &held, epochs, seed);
        let (c, d) = train_and_read(t, &told_clean, &held, epochs, seed);
        s += &format!("| {seed} | {a:.3} / {b:.3} | {c:.3} / {d:.3} |\n");
        println!("rough seed {seed}: with {a:.3}/{b:.3} without {c:.3}/{d:.3}");
    }
    s
}
