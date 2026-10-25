//! One operator for every relation kind, and the laws the kinds obey —
//! phiecs's quest table, phinet's operator, phiano's names (docs/92–93).
//!
//! In Phient's world, a word (w) is nothing but what it relates to, under what
//! conditions (c). A word is a learned function on the circle, a kind a row
//! of the same table, and one shared operator maps `unit(word + kind)` to the
//! answer. A fifth of the rows is held out by hash and read back three ways:
//! one hop, two hops through every told neighbour, and two hops only through
//! neighbours a counted law names. Every reading is printed beside the prior.
//!
//! `cargo run --release -p phiano --example kind -- <relations.tsv> [epochs] [arms]`
//!
//! arms: comma list of `fno`, `pointwise` (modes 0), `aniso` (sphere tie),
//! each optionally `+closure` (train the laws found with confidence ≥
//! `PHIANO_LAW_MIN`, default 0.8)
//! `+inverse` (also train the rows kept inverse laws imply), `+inherit`
//! (kinds in `PHIANO_INHERIT` take their is-a parents' readings) and `+facet`
//! (start words from `<table>.chroma`, the quest facet), `+transitive`
//! (kinds in `PHIANO_TRANSITIVE`, default is-a,part-of, read their own
//! told parents) and `+converse` (pairs in `PHIANO_CONVERSE`, default
//! has>part-of, are read both ways — docs/101). The
//! last arm is saved as `<table>.relate`.

mod report;

use phiano::relate::{find_laws, kind_token, RelationModel, Triple};
use phiano::storage::Storage;
use phical::relate::{self, chance, closure_epoch, closure_pairs, find_inverses, implied, laws_epoch, score_laws, Converse, Inherit};
use phistd::Adam;
use phical::{FnoConfig, Tie};

fn config(arm: &str) -> FnoConfig {
    let base = FnoConfig { space: 64, width: 16, layers: 2, modes: 16, margin: 0.3, lr: 5e-3, tie: Tie::Circle };
    match arm.split('+').next().unwrap_or("fno") {
        "pointwise" => FnoConfig { modes: 0, ..base },
        "aniso" => FnoConfig { modes: 8, tie: Tie::Anisotropic, ..base },
        _ => base,
    }
}

fn held_out(r: &phiecs::quest::Row) -> bool {
    let h = phistd::mix64(phiano::phasor::fnv1a(&format!("{}\t{}\t{}", r.about, r.kind, r.other)));
    ((h >> 11) as f64 / (1u64 << 53) as f64) < 0.2
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let path = a.first().cloned().unwrap_or_else(|| "data/quest/synthetic.tsv".into());
    let epochs: usize = a.get(1).and_then(|s| s.parse().ok()).unwrap_or(30);
    let arms: Vec<String> = a.get(2).map_or(vec!["fno".into(), "fno+closure".into()], |s| s.split(',').map(String::from).collect());
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
    let rows = phiecs::quest::parse_tsv(&text);
    let chroma = path.trim_end_matches(".tsv").to_string() + ".chroma";
    let facet = Storage::load(&chroma).ok();

    let probe = RelationModel::new(&rows, None, config("fno"), 1);
    let (test_rows, told_rows): (Vec<_>, Vec<_>) = rows.iter().partition(|r| held_out(r));
    let told: Vec<Triple> = told_rows.iter().filter_map(|r| probe.triple(r)).collect();
    let test: Vec<Triple> = test_rows.iter().filter_map(|r| probe.triple(r)).collect();
    let all: Vec<Triple> = told.iter().chain(&test).copied().collect();
    let (pool, by_kind) = relate::pools(&all);
    let name = |i: usize| probe.names[i].trim_matches(|c| c == '⟨' || c == '⟩').to_string();
    println!(
        "{path}: {} rows ({} held out) · {} kinds · {} words · pool {} · chance MRR {:.4} · facet {}",
        rows.len(),
        test.len(),
        probe.kinds.len(),
        probe.names.len() - probe.kinds.len(),
        pool.len(),
        chance(pool.len()),
        if facet.is_some() { chroma.as_str() } else { "none" }
    );
    let laws = find_laws(&told, 10);
    println!("\nlaws in the told rows (k₁ ⇒ k₂: w k₁ v, and w, v share a k₂ answer), support ≥ 10:");
    for l in laws.iter().take(8) {
        println!("  {:>10} ⇒ {:<10} {:>5.1}% of {}", name(l.k1), name(l.k2), 100.0 * l.confidence(), l.support);
    }
    // PHIANO_LAW_MIN: the confidence a law needs to be trained (default 0.8).
    let min: f64 = std::env::var("PHIANO_LAW_MIN").ok().and_then(|s| s.parse().ok()).unwrap_or(0.8);
    let kept: Vec<_> = laws.iter().filter(|l| l.confidence() >= min && l.k1 != l.k2).collect();
    let inverses = find_inverses(&told, 10);
    println!("inverses (w k v ⇒ v k' w), support ≥ 10:");
    for i in inverses.iter().take(4) {
        println!("  {:>10} ⇐⇒ {:<10} {:>5.1}% of {}", name(i.k), name(i.k_inv), 100.0 * i.confidence(), i.support);
    }
    let kept_inv: Vec<_> = inverses.iter().filter(|i| i.confidence() >= min).collect();
    let via = |k2: usize| kept.iter().filter(|l| l.k2 == k2).map(|l| l.k1).collect::<Vec<usize>>();
    report::line("prior", &relate::prior(&told, &test, &pool, &by_kind), &name);

    // PHIANO_SEED: the operator's and the table's seed (default 7).
    let seed: u64 = std::env::var("PHIANO_SEED").ok().and_then(|s| s.parse().ok()).unwrap_or(7);
    for arm in &arms {
        let seeds = facet.as_ref().filter(|_| arm.contains("+facet")).map(|f| f.operator_seeds());
        let mut m = RelationModel::new(&rows, seeds.as_ref().map(|s| s as &dyn Fn(&str) -> Option<phistd::Function>), config(arm), seed);
        // `+inverse`: train the rows the kept inverses imply as well.
        let extra = match arm.contains("+inverse") {
            true => implied(&told, &kept_inv),
            false => Vec::new(),
        };
        let examples: Vec<_> = told.iter().chain(&extra).map(relate::example).collect();
        let pairs = match arm.contains("+closure") {
            true => closure_pairs(&told, &kept),
            false => Vec::new(),
        };
        let words: Vec<usize> = (0..m.names.len()).filter(|&i| !m.kinds.iter().any(|k| m.names[i] == kind_token(k))).collect();
        // `+inherit`: the kinds in PHIANO_INHERIT (default has, made-of, does,
        // used-for) take their is-a parents' readings in the forward pass —
        // declared law as structure (docs/97 §1).
        let inherit_names = std::env::var("PHIANO_INHERIT").unwrap_or_else(|_| "has,made-of,does,used-for".into());
        let isa = m.index.get(&kind_token("is-a")).copied();
        let ids = |v: &str| v.split(',').filter_map(|k| m.index.get(&kind_token(k.trim())).copied()).collect::<Vec<usize>>();
        let env = |k: &str, d: &str| std::env::var(k).unwrap_or_else(|_| d.into());
        let inh = match (arm.contains("+inherit"), arm.contains("+transitive"), isa) {
            (false, false, _) | (_, _, None) => None,
            (i, t, Some(isa)) => {
                let base = Inherit::from_told(&told, isa, if i { ids(&inherit_names) } else { Vec::new() }, 1.0);
                Some(if t { base.with_transitive(&told, &ids(&env("PHIANO_TRANSITIVE", "is-a,part-of"))) } else { base })
            }
        };
        let conv = match arm.contains("+converse") {
            true => {
                let pairs: Vec<(usize, usize)> = env("PHIANO_CONVERSE", "has>part-of").split(',').filter_map(|p| p.split_once('>')).filter_map(|(a, b)| Some((*ids(a).first()?, *ids(b).first()?))).collect();
                Converse::declared(&pairs, 1.0)
            }
            false => Converse::none(),
        };
        let structured = inh.is_some() || !conv.of.is_empty();
        let lr = m.joint.fno.config.lr;
        let (mut a_op, mut a_tab) = (Adam::new(lr), Adam::new(lr * 0.5));
        let t0 = std::time::Instant::now();
        for e in 0..epochs {
            match structured {
                true => {
                    let rows: Vec<Triple> = told.iter().chain(&extra).copied().collect();
                    laws_epoch(&mut m.joint, &rows, inh.as_ref(), &conv, &pool, &mut a_op, &mut a_tab, e as u64);
                }
                false => {
                    m.joint.train_epoch(&examples, &pool, 32, &mut a_op, &mut a_tab, e as u64);
                }
            }
            if !pairs.is_empty() {
                closure_epoch(&mut m.joint, &pairs, &words, 1.0, &mut a_op, &mut a_tab, e as u64 ^ 0xC105);
            }
        }
        let (op, tab) = m.size();
        println!("\n{arm}: {op} operator + {tab} table numbers · {} closure pairs · {} implied rows · {:.1}s", pairs.len(), extra.len(), t0.elapsed().as_secs_f64());
        match structured {
            true => {
                report::line("  told", &score_laws(&mut m.joint, &told, inh.as_ref(), &conv, &pool, &by_kind), &name);
                report::line("  held out", &score_laws(&mut m.joint, &test, inh.as_ref(), &conv, &pool, &by_kind), &name);
            }
            false => {
                report::line("  told", &relate::score(&mut m.joint, &told, &pool, &by_kind), &name);
                report::line("  held out", &relate::score(&mut m.joint, &test, &pool, &by_kind), &name);
            }
        }
        report::line("  two hops", &relate::two_hop(&mut m.joint, &told, &test, &pool, &by_kind, None), &name);
        report::line("  law hops", &relate::two_hop(&mut m.joint, &told, &test, &pool, &by_kind, Some(&via)), &name);
        if Some(arm) == arms.last() {
            let out = path.trim_end_matches(".tsv").to_string() + ".relate";
            match m.save(&out) {
                Ok(()) => println!("saved {out}"),
                Err(e) => eprintln!("could not save {out}: {e}"),
            }
        }
    }
}
