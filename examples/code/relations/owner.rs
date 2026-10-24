//! The condition code lacks (docs/108 §2): a method is read under its owner.
//! `f part-of T` makes T a condition of f; what T's methods call, take and
//! return is what T relates to (rows derived from the told rows only), and
//! f inherits it — `calls(f) = own(f) ⊕ λ · calls(T)` — the inheritance
//! structure of docs/99, along `part-of` instead of `is-a`.

use phiano::relate::{held_out, kind_token, RelationModel, Triple};
use phiecs::quest::Row;
use phical::relate::{self, inherit_epoch, score_inherit, Inherit};
use phistd::Adam;
use phical::FnoConfig;
use std::collections::{BTreeMap, HashMap};

pub const INHERITED: [&str; 3] = ["calls", "takes", "returns"];

/// The owner's rows: for every told `f part-of T` and told `f k v` with k
/// inherited, one `T k v`, counted.
pub fn owner_rows(told: &[&Row]) -> Vec<Row> {
    let mut owner: HashMap<&str, Vec<&str>> = HashMap::new();
    for r in told.iter().filter(|r| r.kind == "part-of") {
        owner.entry(r.about.as_str()).or_default().push(r.other.as_str());
    }
    let mut out: BTreeMap<(String, String, String), f64> = BTreeMap::new();
    for r in told.iter().filter(|r| INHERITED.contains(&r.kind.as_str())) {
        for t in owner.get(r.about.as_str()).into_iter().flatten() {
            *out.entry((t.to_string(), r.kind.clone(), r.other.clone())).or_default() += r.weight;
        }
    }
    out.into_iter().map(|((about, kind, other), weight)| Row { about, kind, other, weight }).collect()
}

/// Tally under the owner: a held-out `f k ?` ranked by its owner's counts
/// (what its sibling methods relate to), then by the kind's count. f's own
/// told answers get no extra weight: every learner here is scored unfiltered.
fn counted(m: &RelationModel, told: &[&Row], owners: &[Row], test: &[Triple], pool: &[usize], by_kind: &HashMap<usize, Vec<usize>>) -> relate::Score {
    let mut c: HashMap<(usize, usize, usize), f64> = HashMap::new();
    let mut kind: HashMap<(usize, usize), f64> = HashMap::new();
    let mut parent: HashMap<usize, Vec<usize>> = HashMap::new();
    for r in told.iter().copied().chain(owners.iter()) {
        if let Some(t) = m.triple(r) {
            *c.entry((t.about, t.kind, t.other)).or_default() += r.weight;
            if r.kind == "part-of" {
                parent.entry(t.about).or_default().push(t.other);
            }
            *kind.entry((t.kind, t.other)).or_default() += r.weight;
        }
    }
    relate::score::tally(test, pool, by_kind, |i, v| {
        let t = test[i];
        let up: f64 = parent.get(&t.about).into_iter().flatten().map(|p| c.get(&(*p, t.kind, v)).copied().unwrap_or(0.0)).sum();
        (1.0 + up).ln() * 10.0 + (1.0 + kind.get(&(t.kind, v)).copied().unwrap_or(0.0)).ln() * 0.01
    })
}

pub fn run(rows: &[Row], cfg: FnoConfig, epochs: usize, seed: u64) -> String {
    let told_rows: Vec<&Row> = rows.iter().filter(|r| !held_out(r)).collect();
    let owners = owner_rows(&told_rows);
    let mut all_rows: Vec<Row> = rows.to_vec();
    all_rows.extend(owners.iter().cloned());
    let mut m = RelationModel::new(&all_rows, None, cfg, seed);
    let told: Vec<Triple> = told_rows.iter().copied().chain(owners.iter()).filter_map(|r| m.triple(r)).collect();
    let test: Vec<Triple> = rows.iter().filter(|r| held_out(r)).filter_map(|r| m.triple(r)).collect();
    let every: Vec<Triple> = told.iter().chain(&test).copied().collect();
    let (pool, by_kind) = relate::pools(&every);
    let counts = counted(&m, &told_rows, &owners, &test, &pool, &by_kind);
    let Some(&part_of) = m.index.get(&kind_token("part-of")) else { return String::from("no part-of rows\n") };
    let kinds = INHERITED.iter().filter_map(|k| m.index.get(&kind_token(k)).copied()).collect();
    let inh = Inherit::from_told(&told, part_of, kinds, 1.0);
    let (mut a, mut b) = (Adam::new(cfg.lr), Adam::new(cfg.lr * 0.5));
    for e in 0..epochs {
        inherit_epoch(&mut m.joint, &told, &inh, &pool, &mut a, &mut b, seed ^ e as u64);
    }
    let s = score_inherit(&mut m.joint, &test, &inh, &pool, &by_kind);
    let p = relate::prior(&told, &test, &pool, &by_kind);
    let line = |name: &str, x: &relate::Score| format!("{name}: held out MRR {:.3} · hit@5 {:.1}% · within kind {:.3}\n", x.mrr, 100.0 * x.hit5, x.within);
    let mut out = format!("\nUnder the owner ({} owner rows from the told rows):\n", owners.len());
    out += &line("  prior (with owner rows)", &p);
    out += &line("  counts under the owner (siblings', then the kind's)", &counts);
    out += &line("  phinet, inheriting along part-of", &s);
    for (k, n, mrr, within) in &counts.per_kind {
        out += &format!("    counts · {:<12} {n:>5} · MRR {mrr:.3} · within {within:.3}\n", m.names[*k]);
    }
    out
}
