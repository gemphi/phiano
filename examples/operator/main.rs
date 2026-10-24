//! A long run of the operator (docs/150): phinet's Joint FNO trained for
//! hours on the told paragraphs' `word next word` rows, with a checkpoint
//! and a measurement every `--every` epochs. The counts learn at once; the
//! operator is the slow speed, and the only part of the model that trains.
//! Its place, if it has one, is the open stratum of docs/137–138: the 23%
//! of held-out positions whose word was never told after the last word or
//! pair, where nothing yet beats frequency (3.4% top-1). The run asks
//! whether hours of epochs move the operator there, and writes each
//! checkpoint's strata into `log.md` so the curve can be read while it runs.
//!
//! `cargo run --release -p phiano --example operator -- data/text --in text/emma [--held-out 0.05] [--seed 7] [--r 64] [--epochs 96] [--every 8] [--ahead 1] [--tie circle|phase] [--out data/runs/operator]`
//!
//! Every epoch is saved as `latest.relate` (its epoch in `latest.txt`), and
//! every `--every` epochs as `epoch_N.relate` with its measurement. A run
//! that stops is resumed by running the same command: the latest save in
//! `--out` is loaded and training goes on from its epoch (the optimiser's
//! moments start afresh).
//!
//! `--dev 0.10` trains on the paragraphs outside a larger held-out share and
//! fits the reader's mixture (`waves/strata.rs`) on the paragraphs between the two:
//! rows the operator never saw (docs/152). `--measure FILE` trains nothing: it loads a saved operator and reads the
//! held-out strata, forward and backward, as a wave (`waves/`, docs/153).

#[allow(dead_code)]
#[path = "../story/operator.rs"]
mod operator;
#[allow(dead_code)]
#[path = "../story/strata.rs"]
mod strata;
#[path = "../waves/strata.rs"]
mod waves;

use phiano::prose::{heldout, Knowing};
use phiano::relate::RelationModel;
use phical::{FnoConfig, Tie};
use phistd::Adam;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use phiano::lab::flag;

/// The latest save in `dir`: its epoch and path.
fn latest(dir: &Path) -> Option<(u64, PathBuf)> {
    let every_epoch = std::fs::read_to_string(dir.join("latest.txt")).ok().and_then(|s| s.trim().parse().ok()).map(|e| (e, dir.join("latest.relate")));
    let measured = std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            let epoch = name.strip_prefix("epoch_")?.strip_suffix(".relate")?.parse().ok()?;
            Some((epoch, e.path()))
        })
        .max_by_key(|x| x.0);
    every_epoch.into_iter().chain(measured).max_by_key(|x| x.0)
}

fn log(out: &Path, line: &str) {
    print!("{line}");
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(out.join("log.md")).expect("log");
    f.write_all(line.as_bytes()).expect("log");
}

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let num = |a: &mut Vec<String>, f: &str, d: f64| flag(a, f).and_then(|s| s.parse().ok()).unwrap_or(d);
    let within = flag(&mut a, "--in").unwrap_or_else(|| "text/emma".into());
    let (share, seed) = (num(&mut a, "--held-out", 0.05), num(&mut a, "--seed", 7.0) as u64);
    let (space, epochs, every) = (num(&mut a, "--r", 64.0) as usize, num(&mut a, "--epochs", 96.0) as u64, num(&mut a, "--every", 8.0) as u64);
    // `--ahead 3`: the words two and three places on as kinds of their own (docs/151).
    let ahead = num(&mut a, "--ahead", 1.0) as usize;
    // `--tie phase`: the operator read as a wave, no biases, phase-preserving activation (docs/154).
    let tie = if flag(&mut a, "--tie").as_deref() == Some("phase") { Tie::Phase } else { Tie::Circle };
    let measure = flag(&mut a, "--measure");
    let dev = num(&mut a, "--dev", share).max(share);
    let out = PathBuf::from(flag(&mut a, "--out").unwrap_or_else(|| format!("data/runs/operator_r{space}_s{seed}")));
    let root = PathBuf::from(a.first().map(String::as_str).unwrap_or("data/text"));
    std::fs::create_dir_all(&out).expect("out");

    let held = heldout::held(share, seed);
    let (mut story, _) = Knowing::learn_except(&root, &held);
    story.within = phiano::relate::scopes(&within, 0);
    let unseen = heldout::held(dev, seed);
    let (told, _) = operator::rows_ahead(&root, &unseen, ahead);
    // The rows the reader's mixture is fitted on: the dev paragraphs when there are any, else the operator's own.
    let fit = if dev > share { operator::rows(&root, &|p: &str| !unseen(p) || held(p)).0 } else { told.clone() };
    if let Some(file) = measure {
        let mut m = RelationModel::load(&file).expect("operator");
        let defects = phiano::relate::FnoWave::new(&mut m).defects(1000).unwrap_or_default();
        println!("symmetry defects over 1,000 words: global phase {:.3}, conjugation {:.3}\n", defects.0, defects.1);
        print!("{}\n{}", strata::strata(&mut story, Some(&mut m)), waves::measure(&mut story, &mut phiano::relate::FnoWave::new(&mut m), ahead, &fit));
        return;
    }
    let cfg = FnoConfig { space, width: 16, layers: 2, modes: (space / 4).min(16), margin: 0.3, lr: 5e-3, tie };
    let (mut m, mut from) = match latest(&out) {
        Some((e, p)) => (RelationModel::load(p.to_str().unwrap_or_default()).expect("checkpoint"), e),
        None => (RelationModel::new(&told, None, cfg, seed), 0),
    };
    let triples: Vec<_> = told.iter().filter_map(|r| m.triple(r)).collect();
    let examples: Vec<_> = triples.iter().map(phical::relate::example).collect();
    let (pool, _) = phical::relate::pools(&triples);
    let (mut ao, mut at) = (Adam::new(cfg.lr), Adam::new(cfg.lr * 0.5));
    if from == 0 {
        let n: usize = story.held.iter().map(|x| x.1.len()).sum();
        log(&out, &format!("# Operator run: R = {space}, {epochs} epochs, a checkpoint every {every}, words up to {ahead} ahead\n\n{} told rows · {} held-out paragraphs ({n} words) · seed {seed} · the mixture fitted on {} rows {}\n", told.len(), story.held.len(), fit.len(), if dev > share { "the operator never saw" } else { "it trained on" }));
    } else {
        log(&out, &format!("\n_resumed from epoch {from}_\n"));
    }
    let start = Instant::now();
    while from < epochs {
        let t = Instant::now();
        let loss = m.joint.train_epoch(&examples, &pool, 32, &mut ao, &mut at, from);
        from += 1;
        let save = |m: &RelationModel, name: &str| {
            if let Err(e) = m.save(out.join(name).to_str().unwrap_or_default()) {
                eprintln!("could not save {name}: {e}");
            }
        };
        save(&m, "latest.relate");
        std::fs::write(out.join("latest.txt"), from.to_string()).expect("latest");
        println!("epoch {from} · hinge loss {loss:.4} · {:.0} s", t.elapsed().as_secs_f64());
        if from % every == 0 || from == epochs {
            save(&m, &format!("epoch_{from}.relate"));
            let t = Instant::now();
            let table = format!("{}\n{}", strata::strata(&mut story, Some(&mut m)), waves::measure(&mut story, &mut phiano::relate::FnoWave::new(&mut m), ahead, &fit));
            log(&out, &format!("\n## Epoch {from} · hinge loss {loss:.4} · {:.0} s measuring · {:.1} h this sitting\n\n{table}", t.elapsed().as_secs_f64(), start.elapsed().as_secs_f64() / 3600.0));
        }
    }
}
