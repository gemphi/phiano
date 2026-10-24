//! The demo: one knob, and learning that keeps.
//!
//! In Phient's world, a word (w) is nothing but what it relates to, under what
//! conditions (c). Two claims, measured on the personas' quest folders:
//!
//! 1. **One knob.** The number of relations a word holds — samples of its
//!    function, R — is the only thing turned; operator shape, epochs and
//!    data stay fixed. Size and accuracy are printed for each R.
//! 2. **Learning that keeps.** The personas are learned one after another
//!    into one model that grows (`RelationModel::extend`): after each, every
//!    persona learned so far is read again. Two ways: *naive* (train on the
//!    new persona only) and *keep* (the new persona plus every earlier told
//!    row). One `.relate` is saved per step, so every stage stays on disk.
//!
//! `cargo run --release --bin learn_demo -- <root> <persona.md>… [--sweep 16,32,64,128] [--epochs 30] [--seed 7] [--only knob|stages] [--share 1.0]`
//! `--share f` trains the knob on that share of the told rows (the held-out rows stay the same).
//! writes `<root>/learn/report.md` and `<root>/learn/step<i>_<slug>.relate`.

mod report;
mod train;

use phiano::relate::{held_out, read_quests, RelationModel};
use phiecs::persona::Persona;
use phical::{FnoConfig, Tie};
use std::path::Path;

fn config(r: usize) -> FnoConfig {
    FnoConfig { space: r, width: 16, layers: 2, modes: (r / 4).min(16), margin: 0.3, lr: 5e-3, tie: Tie::Circle }
}

/// True for the `share` of told rows the knob trains on (same held set).
fn kept(r: &phiecs::quest::Row, share: f64) -> bool {
    let h = phistd::mix64(phiano::phasor::fnv1a(&format!("share\t{}\t{}\t{}", r.about, r.kind, r.other)));
    ((h >> 11) as f64 / (1u64 << 53) as f64) < share
}

use phiano::lab::flag;

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let sweep: Vec<usize> = flag(&mut a, "--sweep").unwrap_or_else(|| "16,32,64,128".into()).split(',').filter_map(|s| s.trim().parse().ok()).collect();
    let epochs: usize = flag(&mut a, "--epochs").and_then(|s| s.parse().ok()).unwrap_or(30);
    let seed: u64 = flag(&mut a, "--seed").and_then(|s| s.parse().ok()).unwrap_or(7);
    let only = flag(&mut a, "--only").unwrap_or_default();
    let (knob_only, stages_only) = (only == "knob", only == "stages");
    let share: f64 = flag(&mut a, "--share").and_then(|s| s.parse().ok()).unwrap_or(1.0);
    let Some(root) = a.first().map(|s| Path::new(s).to_path_buf()) else {
        eprintln!("usage: learn_demo <root> <persona.md>… [--sweep 16,32,64,128] [--epochs 30]");
        std::process::exit(2);
    };
    let mut personas = Vec::new();
    for f in &a[1..] {
        let p = Persona::from_md(&std::fs::read_to_string(f).unwrap_or_else(|e| panic!("{f}: {e}"))).unwrap_or_else(|e| panic!("{f}: {e}"));
        let (rows, _) = read_quests(&root.join(p.slug()));
        println!("{}: {} rows", p.name, rows.len());
        if !rows.is_empty() {
            personas.push((p, rows));
        }
    }
    if personas.is_empty() {
        eprintln!("no quest rows under {}", root.display());
        std::process::exit(1);
    }
    let out = root.join("learn");
    let _ = std::fs::create_dir_all(&out);
    let inherit: Vec<String> = personas.iter().flat_map(|(p, _)| p.inherit.clone()).collect();
    let mut rep = report::Report::new(&personas.iter().map(|(p, _)| p.name.clone()).collect::<Vec<_>>(), epochs);

    // 1. One knob: every persona's rows, R relations per word.
    let rows: Vec<&phiecs::quest::Row> = personas.iter().flat_map(|(_, r)| r.iter()).collect();
    let owned: Vec<phiecs::quest::Row> = rows.iter().map(|r| (*r).clone()).collect();
    for &r in sweep.iter().filter(|_| !stages_only) {
        let t0 = std::time::Instant::now();
        let mut m = RelationModel::new(&owned, None, config(r), seed);
        let (mut told_rows, held_rows): (Vec<&phiecs::quest::Row>, Vec<_>) = rows.iter().copied().partition(|r| !held_out(r));
        told_rows.retain(|r| kept(r, share));
        let (told, held) = (train::triples(&m, &told_rows), train::triples(&m, &held_rows));
        let all: Vec<_> = told.iter().chain(&held).copied().collect();
        train::train(&mut m, &told, &inherit, epochs, seed);
        let reading = train::read(&mut m, &all, &told, &held, &inherit);
        let (op, tab) = m.size();
        rep.knob(r, op + tab, &reading, t0.elapsed().as_secs_f64());
    }

    // 2. Learning that keeps: personas one after another, naive and keep.
    for keep in [false, true].into_iter().filter(|_| !knob_only) {
        let mut m: Option<RelationModel> = None;
        let mut seen: Vec<&phiecs::quest::Row> = Vec::new();
        for (i, (p, rows)) in personas.iter().enumerate() {
            let owned: Vec<phiecs::quest::Row> = rows.clone();
            let model = m.get_or_insert_with(|| RelationModel::new(&owned, None, config(64), seed));
            model.extend(&owned, seed + i as u64);
            let new_told: Vec<&phiecs::quest::Row> = rows.iter().filter(|r| !held_out(r)).collect();
            let train_rows: Vec<&phiecs::quest::Row> = match keep {
                true => seen.iter().copied().chain(new_told.iter().copied()).collect(),
                false => new_told.clone(),
            };
            let told = train::triples(model, &train_rows);
            train::train(model, &told, &p.inherit, epochs, seed + 4 + i as u64);
            seen.extend(new_told);
            for (q, qrows) in personas.iter().take(i + 1) {
                let (t, h): (Vec<_>, Vec<_>) = qrows.iter().partition(|r| !held_out(r));
                let (t, h) = (train::triples(model, &t), train::triples(model, &h));
                let all: Vec<_> = t.iter().chain(&h).copied().collect();
                let reading = train::read(model, &all, &t, &h, &q.inherit);
                rep.stage(keep, i, &p.name, &q.name, &reading);
            }
            let file = out.join(format!("step{}_{}{}.relate", i + 1, p.slug(), if keep { "" } else { "_naive" }));
            let _ = model.save(&file.to_string_lossy());
        }
    }
    let path = out.join(if knob_only { format!("knob_seed{seed}_share{share}.md") } else if stages_only { format!("stages_seed{seed}.md") } else { "report.md".into() });
    std::fs::write(&path, rep.markdown()).expect("write report");
    println!("\n{}\nwrote {}", rep.markdown(), path.display());
}
