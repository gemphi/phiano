//! Code as a language of relations (docs/108). Rust sources are read by
//! their own grammar (syn) into rows — what a function calls, takes and
//! returns; what a type has, holds and implements; what belongs to what —
//! and a relation model learns them. Held out: a fifth of the rows, read
//! against the prior, and again under the compiler's law for type slots
//! (the answer to `takes`, `returns`, `holds`, `implements` must be a type).
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c). In code the conditions are exact: the grammar says them.
//!
//! `cargo run --release --bin code_relations -- <dir>… [--out rows.tsv] [--r 64] [--epochs 30] [--seed 7]`
//! `--held-out file` holds out whole files instead (new functions, docs/124);
//! `--r 8,16,32,64` sweeps the knob and `--md out.md` keeps the boards.

mod counts;
mod newfn;
mod readings;
mod owner;

use phiano::relate::{held_out, kind_token, RelationModel, Triple};
use phiecs::quest::Row;
use phistd::Adam;
use phical::{relate, FnoConfig, Tie};
use std::collections::{BTreeMap, HashMap, HashSet};

const TYPE_SLOTS: [&str; 4] = ["takes", "returns", "holds", "implements"];

use phiano::lab::flag;

pub fn run(a: Vec<String>) {
    let mut a = a;
    let out = flag(&mut a, "--out").unwrap_or_else(|| "code_relations.tsv".into());
    let rs: Vec<usize> = flag(&mut a, "--r").unwrap_or_else(|| "64".into()).split(',').filter_map(|s| s.trim().parse().ok()).collect();
    let r = rs[0];
    let epochs: usize = flag(&mut a, "--epochs").and_then(|s| s.parse().ok()).unwrap_or(30);
    let seed: u64 = flag(&mut a, "--seed").and_then(|s| s.parse().ok()).unwrap_or(7);
    let held = flag(&mut a, "--held-out").unwrap_or_default();
    let md = flag(&mut a, "--md");
    if held == "file" || held == "family" {
        return new_functions(&a, held == "family", &rs, epochs, seed, md.as_deref());
    }
    let (mut reader, mut files) = (phiano::code::Reader::default(), 0usize);
    for d in &a {
        phiano::code::read_dir(std::path::Path::new(d), &mut reader, &mut files);
    }
    // One row per distinct relation, its count as weight.
    let mut counted: BTreeMap<(String, String, String), f64> = BTreeMap::new();
    for x in &reader.rows {
        *counted.entry((x.about.clone(), x.kind.clone(), x.other.clone())).or_default() += 1.0;
    }
    let rows: Vec<Row> = counted.into_iter().filter(|((a, _, o), _)| a != o).map(|((about, kind, other), weight)| Row { about, kind, other, weight }).collect();
    let tsv: String = rows.iter().map(|x| format!("{}\t{}\t{}\t{:.0}\n", x.about, x.kind, x.other, x.weight)).collect();
    std::fs::write(&out, tsv).expect("write rows");
    let mut per: BTreeMap<&str, usize> = BTreeMap::new();
    rows.iter().for_each(|x| *per.entry(x.kind.as_str()).or_default() += 1);
    println!("{files} files · {} relation sightings · {} distinct relations → {out}", reader.rows.len(), rows.len());
    println!("by kind: {}", per.iter().map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(" · "));

    let cfg = FnoConfig { space: r, width: 16, layers: 2, modes: (r / 4).min(16), margin: 0.3, lr: 5e-3, tie: Tie::Circle };
    let mut m = RelationModel::new(&rows, None, cfg, seed);
    let (test_rows, told_rows): (Vec<&Row>, Vec<&Row>) = rows.iter().partition(|x| held_out(x));
    let told: Vec<Triple> = told_rows.iter().filter_map(|x| m.triple(x)).collect();
    let test: Vec<Triple> = test_rows.iter().filter_map(|x| m.triple(x)).collect();
    let all: Vec<Triple> = told.iter().chain(&test).copied().collect();
    let (pool, by_kind) = relate::pools(&all);
    // Kinds learned against answers of their own kind (docs/103, trial 2c).
    let mut groups: HashMap<usize, Vec<_>> = HashMap::new();
    for t in &told {
        groups.entry(t.kind).or_default().push(relate::example(t));
    }
    let (mut ao, mut at) = (Adam::new(cfg.lr), Adam::new(cfg.lr * 0.5));
    for e in 0..epochs {
        for (k, ex) in &groups {
            m.joint.train_epoch(ex, &by_kind[k], 32, &mut ao, &mut at, seed ^ e as u64 ^ (*k as u64) << 24);
        }
    }
    // The compiler's law for type slots: the answer is a type.
    let types: HashSet<String> = reader.types.iter().cloned().chain(told_rows.iter().filter(|x| TYPE_SLOTS.contains(&x.kind.as_str())).map(|x| x.other.clone())).collect();
    let type_ids: Vec<usize> = pool.iter().copied().filter(|&i| types.contains(&m.names[i])).collect();
    let mut law = by_kind.clone();
    for k in TYPE_SLOTS {
        if let Some(&ki) = m.index.get(&kind_token(k)) {
            law.insert(ki, type_ids.clone());
        }
    }
    let p = relate::prior(&told, &test, &pool, &by_kind);
    let s = relate::score(&mut m.joint, &test, &pool, &by_kind);
    let lawful = relate::score(&mut m.joint, &test, &pool, &law);
    let lawful_prior = relate::prior(&told, &test, &pool, &law);
    let moved = movement(&mut m, &told, &test, &pool, &by_kind);
    let moved_law = movement(&mut m, &told, &test, &pool, &law);
    println!("\nR = {r} · {} told · {} held out · pool {}", told.len(), test.len(), pool.len());
    println!("movement (the kind's count × the name's function): held out MRR {:.3} · hit@5 {:.1}% · within kind {:.3} · under the type law {:.3}", moved.mrr, 100.0 * moved.hit5, moved.within, moved_law.within);
    println!("held out MRR {:.3} (prior {:.3}) · hit@5 {:.1}% · within kind {:.3} (prior {:.3}) · under the type law {:.3} (prior {:.3})", s.mrr, p.mrr, 100.0 * s.hit5, s.within, p.within, lawful.within, lawful_prior.within);
    for (k, n, mrr, within) in &s.per_kind {
        let pk = p.per_kind.iter().find(|x| x.0 == *k).map_or(0.0, |x| x.3);
        println!("  {:<12} {n:>5} held out · MRR {mrr:.3} · within kind {within:.3} (prior {pk:.3})", m.names[*k]);
    }
    println!();
    for (w, k) in [("RelationModel", "has"), ("train", "calls"), ("Episode", "has"), ("score", "returns"), ("Tally", "implements")] {
        let answers = m.answers(w, k, &by_kind.get(&m.index.get(&kind_token(k)).copied().unwrap_or(0)).cloned().unwrap_or_default(), 5);
        println!("{w} · {k} ⇒ {}", answers.iter().map(|(n, c)| format!("{n} ({c:.2})")).collect::<Vec<_>>().join(", "));
    }
    let under = owner::run(&rows, cfg, epochs, seed);
    println!("{under}");
    let model = out.trim_end_matches(".tsv").to_string() + ".relate";
    if m.save(&model).is_ok() {
        println!("saved {model} (ask it: relate_ask {model} <name> <kind>)");
    }
}

/// New functions held out by file, the knob swept (docs/124).
fn new_functions(dirs: &[String], family: bool, rs: &[usize], epochs: usize, seed: u64, md: Option<&str>) {
    let s = newfn::split(dirs, family, seed);
    let unit = if family { "by family" } else { "by file" };
    let mut out = format!("{} files, {} held out {unit} · {} told rows · {} test rows\n\n", s.files, s.held, s.told.len(), s.test.len());
    let sample: Vec<String> = s.test.iter().map(|x| x.about.clone()).collect::<HashSet<_>>().into_iter().collect();
    for &r in rs {
        let cfg = FnoConfig { space: r, width: 16, layers: 2, modes: (r / 4).min(16), margin: 0.3, lr: 5e-3, tie: Tie::Circle };
        let mut t = newfn::train(&s, cfg, epochs, seed);
        let board = newfn::board(&mut t, r);
        println!("{board}");
        out += &board;
        if r == *rs.last().unwrap() {
            let mut names: Vec<&str> = sample.iter().map(String::as_str).collect();
            names.sort();
            let read = readings::readings(&mut t, &names[..names.len().min(8)], 5);
            println!("{read}");
            out += &format!("\n```\n{read}```\n");
        }
        out += "\n";
    }
    if let Some(p) = md {
        std::fs::write(p, out).expect("write md");
    }
}

/// Temperature for the functions' cosines, as in the builders' game (docs/103).
const TEMP: f64 = 0.1;

/// D4 on code: the kind's own count says what is likely (the prior), the
/// name's function moves it — `ln P(v | k) + cos(F(w + k), v) / TEMP`.
fn movement(m: &mut RelationModel, told: &[Triple], test: &[Triple], pool: &[usize], by_kind: &HashMap<usize, Vec<usize>>) -> relate::Score {
    let mut count: HashMap<(usize, usize), f64> = HashMap::new();
    let mut total: HashMap<usize, f64> = HashMap::new();
    for t in told {
        *count.entry((t.kind, t.other)).or_default() += 1.0;
        *total.entry(t.kind).or_default() += 1.0;
    }
    let table = m.joint.functions();
    let inputs: Vec<phistd::Function> = test
        .iter()
        .map(|t| {
            let mut f: phistd::Function = table[t.about].iter().zip(&table[t.kind]).map(|(x, y)| x + y).collect();
            phistd::normalise(&mut f);
            f
        })
        .collect();
    let preds = m.joint.fno.apply(&inputs);
    let n = pool.len() as f64;
    relate::score::tally(test, pool, by_kind, |i, v| {
        let k = test[i].kind;
        let p = (count.get(&(k, v)).copied().unwrap_or(0.0) + 0.5) / (total.get(&k).copied().unwrap_or(0.0) + 0.5 * n);
        p.ln() + phistd::cosine(&preds[i], &table[v]) / TEMP
    })
}
