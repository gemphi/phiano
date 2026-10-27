//! The phase-conjugate mirror trained on held-out *Emma*'s told rows and
//! measured as a wave (docs/153). The mirror (`phiton::conjugate`) reads a
//! relation as an all-pass filter on the circle: forward by `H`, backwards
//! by the conjugate `H̄`, and `n` steps on by `Hⁿ`. The same rows, split,
//! decoys and hinge as the Joint FNO's `operator_run`; the same tables.
//!
//! `cargo run --release --bin mirror_run -- data/text --in text/emma [--held-out 0.05] [--seed 7] [--r 64] [--epochs 32] [--every 8] [--ahead 1] [--pumped] [--dev 0.10] [--out data/runs/mirror_r64_s7]`
//!
//! Every epoch is saved as `latest.mirror` (its epoch in `latest.txt`) and
//! every `--every` epochs as `epoch_N.mirror` with its measurement in
//! `log.md`; the same command resumes (Adam's moments start afresh).
//! `--measure FILE` trains nothing and prints the tables for a saved mirror.

#[allow(dead_code)]
#[path = "../story/operator.rs"]
mod operator;
#[path = "../waves/strata.rs"]
mod waves;

use phiano::prose::{heldout, Knowing};
use phiton::conjugate::{Mirror, Moment, Told};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

use phiano::lab::flag;

fn log(out: &std::path::Path, line: &str) {
    print!("{line}");
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(out.join("log.md")).expect("log");
    f.write_all(line.as_bytes()).expect("log");
}

/// The mirror's tables: offsets as trained, and, when it was trained on
/// more than `next`, the same offsets as powers of `H`.
fn tables(k: &mut Knowing, m: &Mirror, ahead: usize, fit: &[phiecs::quest::Row]) -> String {
    let steps = ahead.max(3);
    let mut md = waves::measure(k, &mut phiano::relate::MirrorWave::new(m.clone(), false), steps, fit);
    if ahead > 1 {
        md += &format!("\n**The same, every offset as a power of `H`:**\n\n{}", waves::measure(k, &mut phiano::relate::MirrorWave::new(m.clone(), true), steps, fit));
    }
    md
}

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let num = |a: &mut Vec<String>, f: &str, d: f64| flag(a, f).and_then(|s| s.parse().ok()).unwrap_or(d);
    let within = flag(&mut a, "--in").unwrap_or_else(|| "text/emma".into());
    let (share, seed) = (num(&mut a, "--held-out", 0.05), num(&mut a, "--seed", 7.0) as u64);
    let (modes, epochs, every) = (num(&mut a, "--r", 64.0) as usize, num(&mut a, "--epochs", 32.0) as u64, num(&mut a, "--every", 8.0) as u64);
    let ahead = num(&mut a, "--ahead", 1.0) as usize;
    // `--pumped`: a gain per mode as well as a phase (docs/154).
    let pumped = a.iter().position(|x| x == "--pumped").map(|i| a.remove(i)).is_some();
    let dev = num(&mut a, "--dev", share).max(share);
    let measure = flag(&mut a, "--measure");
    let out = PathBuf::from(flag(&mut a, "--out").unwrap_or_else(|| format!("data/runs/mirror_r{modes}_s{seed}")));
    let root = PathBuf::from(a.first().map(String::as_str).unwrap_or("data/text"));
    std::fs::create_dir_all(&out).expect("out");

    let held = heldout::held(share, seed);
    let (mut story, _) = Knowing::learn_except(&root, &held);
    story.within = phiano::relate::scopes(&within, 0);
    let unseen = heldout::held(dev, seed);
    let (told, _) = operator::rows_ahead(&root, &unseen, ahead);
    let fit = if dev > share { operator::rows(&root, &|p: &str| !unseen(p) || held(p)).0 } else { told.clone() };
    if let Some(file) = measure {
        let m = Mirror::load(&file).expect("mirror");
        print!("{}", tables(&mut story, &m, ahead, &fit));
        return;
    }
    let mut names: Vec<String> = told.iter().flat_map(|r| [r.about.clone(), r.other.clone()]).collect();
    names.sort();
    names.dedup();
    let kinds: Vec<String> = (1..=ahead).map(|d| if d == 1 { "next".to_string() } else { format!("next{d}") }).collect();
    let latest = std::fs::read_to_string(out.join("latest.txt")).ok().and_then(|s| s.trim().parse::<u64>().ok());
    let (mut m, mut from) = match latest {
        Some(e) => (Mirror::load(out.join("latest.mirror").to_str().unwrap_or_default()).expect("checkpoint"), e),
        None => (Mirror::new(names, kinds, modes, pumped, seed), 0),
    };
    let ix: HashMap<&str, usize> = m.names.iter().enumerate().map(|(i, n)| (n.as_str(), i)).collect();
    let rows: Vec<Told> = told.iter().filter_map(|r| Some(Told { about: *ix.get(r.about.as_str())?, kind: m.kind(&r.kind)?, other: *ix.get(r.other.as_str())? })).collect();
    let mut pool: Vec<usize> = rows.iter().map(|r| r.other).collect();
    pool.sort_unstable();
    pool.dedup();
    let mut mo = Moment::new(&m);
    match from {
        0 => log(&out, &format!("# Mirror run: R = {modes}, {epochs} epochs, a measurement every {every}, words up to {ahead} ahead\n\n{} told rows · {} words · {} held-out paragraphs · seed {seed} · the mixture fitted on {} rows {}\n", rows.len(), m.names.len(), story.held.len(), fit.len(), if dev > share { "the mirror never saw" } else { "it trained on" })),
        e => log(&out, &format!("\n_resumed from epoch {e}_\n")),
    }
    let start = Instant::now();
    while from < epochs {
        let t = Instant::now();
        let loss = m.train_epoch(&rows, &pool, 0.3, 5e-3, &mut mo, from);
        from += 1;
        m.save(out.join("latest.mirror").to_str().unwrap_or_default()).expect("save");
        std::fs::write(out.join("latest.txt"), from.to_string()).expect("latest");
        println!("epoch {from} · hinge loss {loss:.4} · {:.0} s", t.elapsed().as_secs_f64());
        if from % every == 0 || from == epochs {
            m.save(out.join(format!("epoch_{from}.mirror")).to_str().unwrap_or_default()).expect("save");
            let t = Instant::now();
            let md = tables(&mut story, &m, ahead, &fit);
            log(&out, &format!("\n## Epoch {from} · hinge loss {loss:.4} · {:.0} s measuring · {:.1} h this sitting\n\n{md}", t.elapsed().as_secs_f64(), start.elapsed().as_secs_f64() / 3600.0));
        }
    }
}
