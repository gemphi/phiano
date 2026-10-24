//! A new function, held out by file (docs/124). The task gives a name
//! (`sum_above`) and asks for a body. In Phient's world the name is nothing
//! but what it relates to: `sum_above has-word sum`, `has-word above`, told
//! from the name alone, and every function already read gave its words
//! their relations. So the new function inherits along `has-word` (docs/97
//! §1, `phical::relate::Inherit`): `calls(sum_above) = own ⊕ λ · mean(
//! calls(sum), calls(above))`, and the same for `uses-op`, `takes`,
//! `returns`. Held out: a fifth of the files by hash of their path; the test
//! rows are their relations not told by any other file. The prior (how often
//! the kind named the answer) is the bar, as everywhere in `phical::relate`.

use phiano::relate::{kind_token, RelationModel, Triple};
use phiecs::quest::Row;
use phical::relate::{self, inherit_epoch, Inherit};
use phistd::Adam;
use phical::FnoConfig;
use std::collections::{BTreeMap, HashMap, HashSet};

/// The kinds a new function inherits from its words.
pub const INHERITED: [&str; 5] = ["calls", "uses-op", "takes", "returns", "part-of"];
/// What the task states about a held-out function: its name's words and its
/// signature (`code_write` is given the same). The body is what is asked.
pub const GIVEN: [&str; 4] = ["has-word", "takes", "returns", "within"];

/// By file: a fifth of the files. By family: a fifth of the directories,
/// so a held-out function's siblings (`above_i32` for `above_i64`) are held
/// out with it.
fn held_out_file(path: &str, family: bool, seed: u64) -> bool {
    let unit = match family {
        true => std::path::Path::new(path).parent().map_or(path, |p| p.to_str().unwrap_or(path)),
        false => path,
    };
    let h = phistd::mix64(phiano::phasor::fnv1a(unit) ^ seed);
    ((h >> 11) as f64 / (1u64 << 53) as f64) < 0.2
}

fn distinct(rows: &[Row]) -> Vec<Row> {
    let mut counted: BTreeMap<(String, String, String), f64> = BTreeMap::new();
    for x in rows {
        *counted.entry((x.about.clone(), x.kind.clone(), x.other.clone())).or_default() += 1.0;
    }
    counted.into_iter().filter(|((a, _, o), _)| a != o).map(|((about, kind, other), weight)| Row { about, kind, other, weight }).collect()
}

fn key(r: &Row) -> (String, String, String) {
    (r.about.clone(), r.kind.clone(), r.other.clone())
}

pub struct Split {
    pub told: Vec<Row>,
    pub test: Vec<Row>,
    pub files: usize,
    pub held: usize,
}

/// Told: every row of the told files, and the `GIVEN` rows of the held-out
/// files (the task states the name and the signature). Test: the held-out
/// files' body rows that no told file states.
pub fn split(dirs: &[String], family: bool, seed: u64) -> Split {
    let mut per_file = Vec::new();
    dirs.iter().for_each(|d| phiano::code::read_files(std::path::Path::new(d), &mut per_file));
    let (mut told, mut test) = (Vec::new(), Vec::new());
    let mut held = 0usize;
    for (path, rows) in &per_file {
        match held_out_file(path, family, seed) {
            true => {
                held += 1;
                for r in rows {
                    match GIVEN.contains(&r.kind.as_str()) {
                        true => told.push(r.clone()),
                        false => test.push(r.clone()),
                    }
                }
            }
            false => told.extend(rows.iter().cloned()),
        }
    }
    let told = distinct(&told);
    let known: HashSet<_> = told.iter().map(key).collect();
    let test: Vec<Row> = distinct(&test).into_iter().filter(|r| !known.contains(&key(r))).collect();
    Split { told, test, files: per_file.len(), held }
}

pub struct Trained {
    pub m: RelationModel,
    pub inh: Inherit,
    pub told: Vec<Triple>,
    pub test: Vec<Triple>,
    pub pool: Vec<usize>,
    pub by_kind: HashMap<usize, Vec<usize>>,
}

/// One model inheriting along `has-word`, trained `epochs` times on the
/// told rows, as `persona_from_quest::relate_one` trains a persona.
pub fn train(s: &Split, cfg: FnoConfig, epochs: usize, seed: u64) -> Trained {
    let all: Vec<Row> = s.told.iter().chain(&s.test).cloned().collect();
    let mut m = RelationModel::new(&all, None, cfg, seed);
    let told: Vec<Triple> = s.told.iter().filter_map(|x| m.triple(x)).collect();
    let test: Vec<Triple> = s.test.iter().filter_map(|x| m.triple(x)).collect();
    let (pool, by_kind) = relate::pools(&told.iter().chain(&test).copied().collect::<Vec<_>>());
    let via = m.index[&kind_token("has-word")];
    let kinds = INHERITED.iter().filter_map(|k| m.index.get(&kind_token(k)).copied()).collect();
    let inh = Inherit::from_told(&told, via, kinds, 1.0);
    let (mut ao, mut at) = (Adam::new(cfg.lr), Adam::new(cfg.lr * 0.5));
    for e in 0..epochs as u64 {
        inherit_epoch(&mut m.joint, &told, &inh, &pool, &mut ao, &mut at, seed ^ e);
    }
    Trained { m, inh, told, test, pool, by_kind }
}

/// The scoreboard for one R: prior, own reading, inherited reading, two
/// hops through the words — overall and per kind.
pub fn board(t: &mut Trained, r: usize) -> String {
    let Trained { m, inh, told, test, pool, by_kind } = t;
    let has_word = m.index[&kind_token("has-word")];
    let p = relate::prior(told, test, pool, by_kind);
    let own = relate::score(&mut m.joint, test, pool, by_kind);
    let inherited = relate::score_inherit(&mut m.joint, test, inh, pool, by_kind);
    let via = |_k: usize| vec![has_word];
    let hops = relate::two_hop(&mut m.joint, told, test, pool, by_kind, Some(&via));
    let (posts, bag) = (super::counts::counts_score(m, told, test, pool, by_kind, 0.5), super::counts::counts_score(m, told, test, pool, by_kind, 0.0));
    let under = super::counts::counts_read(m, told, test, pool, by_kind, 0.5, true, phiano::relate::Weighting::Breadth);
    let counted = super::counts::counts_read(m, told, test, pool, by_kind, 0.5, true, phiano::relate::Weighting::Counted);
    let path = super::counts::counts_read(m, told, test, pool, by_kind, 0.5, true, phiano::relate::Weighting::Path);
    let mut s = format!(
        "R = {r} · {} told · {} held out (new functions) · pool {}\n\n| reading | MRR | hit@5 | within kind |\n|---|---|---|---|\n",
        told.len(),
        test.len(),
        pool.len()
    );
    let readings = [
        ("prior (the kind's count)", &p),
        ("operator: own (the new name's random function)", &own),
        ("operator: inherited from its words (docs/97 §1)", &inherited),
        ("operator: two hops through its words", &hops),
        ("counts: movement by the given words and types, posts at τ = 0.5 (D2)", &posts),
        ("counts: movement, every condition weighs (τ = 0)", &bag),
        ("counts: under the path's scopes, then the words and types (levels, docs/126)", &under),
        ("counts: levels, each level's weight counted by leaving out (docs/147)", &counted),
        ("counts: levels as one path, weights counted for the function (docs/149)", &path),
    ];
    for (name, x) in readings {
        s += &format!("| {name} | {:.3} | {:.1}% | {:.3} |\n", x.mrr, 100.0 * x.hit5, x.within);
    }
    s += "\n| kind | held out | prior | inherited | two hops | counts, posts | counts, τ = 0 | counts, levels |\n|---|---|---|---|---|---|---|---|\n";
    for (k, n, _, w) in &inherited.per_kind {
        let find = |x: &relate::Score| x.per_kind.iter().find(|y| y.0 == *k).map_or(0.0, |y| y.3);
        s += &format!("| {} | {n} | {:.3} | {w:.3} | {:.3} | {:.3} | {:.3} | {:.3} |\n", m.names[*k], find(&p), find(&hops), find(&posts), find(&bag), find(&under));
    }
    // How often was the name's rarest word heard? The test rows by the number
    // of other told functions that said it: a word heard once against three.
    s += "\n| the name's rarest word heard in … other functions | rows | prior | inherited | two hops | counts, posts |\n|---|---|---|---|---|---|\n";
    let heard = heard_in(m.index[&kind_token("has-word")], told);
    for (label, lo, hi) in [("0", 0, 0), ("1", 1, 1), ("2", 2, 2), ("3 or more", 3, usize::MAX)] {
        let sub: Vec<Triple> = test.iter().filter(|t| { let h = heard.get(&t.about).copied().unwrap_or(0); h >= lo && h <= hi }).copied().collect();
        if sub.is_empty() {
            continue;
        }
        let p = relate::prior(told, &sub, pool, by_kind);
        let i = relate::score_inherit(&mut m.joint, &sub, inh, pool, by_kind);
        let h = relate::two_hop(&mut m.joint, told, &sub, pool, by_kind, Some(&via));
        let c = super::counts::counts_score(m, told, &sub, pool, by_kind, 0.5);
        s += &format!("| {label} | {} | {:.3} | {:.3} | {:.3} | {:.3} |\n", sub.len(), p.mrr, i.mrr, h.mrr, c.mrr);
    }
    s
}

/// Per function (row index): the fewest other told functions any of its
/// words was said by.
fn heard_in(has_word: usize, told: &[Triple]) -> HashMap<usize, usize> {
    let mut by_word: HashMap<usize, HashSet<usize>> = HashMap::new();
    for t in told.iter().filter(|t| t.kind == has_word) {
        by_word.entry(t.other).or_default().insert(t.about);
    }
    let mut out: HashMap<usize, usize> = HashMap::new();
    for t in told.iter().filter(|t| t.kind == has_word) {
        let n = by_word[&t.other].iter().filter(|&&f| f != t.about).count();
        let e = out.entry(t.about).or_insert(usize::MAX);
        *e = (*e).min(n);
    }
    out
}

