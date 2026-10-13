//! A story rewritten as a strand of sixty-four levels (phiplm::prose::strand,
//! docs/142): `TTT` the whole story, `AAA` the word, and between them the
//! levels the story's own information divides it into — each level the story
//! told again in fewer, larger beats, each beat in the words it says beyond
//! the beat above it. Nothing is generated; every word is the story's.
//!
//! `cargo run --release --bin strand -- <story.txt> [--prior <dir of .txt>] [--out <dir>] [--show TTT,TGA,…] [--words N]`
//!
//! `strand --bench [--trials N] [--seed S] [--len L] [--tol T] [--kmin K] [--chapters dir] [--out dir]`
//! tests it against texts whose true boundaries are known (docs/143);
//! `strand --synthetic [--trials N] [--seed S] [--len L]` against text whose only structure is known, and
//! `strand --cross [--trials N] [--seed S] [--out dir]` against joins of different kinds of text (docs/144);
//! `strand --read [--trials N] [--seed S] [--folds F] [--out dir]` reads held-out words in the strand's units (docs/145);
//! `strand --crossfit [--trials N] [--seed S]` counts the tree's weights on tokens it was not divided by (docs/148).
//!
//! `--out` writes `story.dna` (every level, every unit: span, signature),
//! `levels.tsv` (codon · units · nats gained · nats per token) and
//! `story.md` (the levels shown).

mod bench;
mod cross;
mod crossfit;
mod read;
mod readout;
mod score;
mod suites;
mod synth;

use phistd::codon::{codon, codon_index, hexagram_glyph, CODONS};
use phiplm::prose::strand::{Prior, Strand};
use std::fmt::Write;
use std::path::PathBuf;

use phiano::lab::flag;

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    if let Some(i) = a.iter().position(|x| x == "--bench") {
        a.remove(i);
        let num = |a: &mut Vec<String>, f: &str, d: u64| flag(a, f).and_then(|s| s.parse().ok()).unwrap_or(d);
        let (trials, seed, len, tol) = (num(&mut a, "--trials", 60) as usize, num(&mut a, "--seed", 7), num(&mut a, "--len", 120) as usize, num(&mut a, "--tol", 10) as usize);
        let kmin = num(&mut a, "--kmin", 2) as usize;
        let dir = flag(&mut a, "--chapters").unwrap_or_else(|| "crates/phiano/data/text/emma".into());
        let out = flag(&mut a, "--out");
        let (t, tsv) = bench::run(std::path::Path::new(&dir), trials, seed, len, tol, kmin);
        print!("{}", bench::report(&t, tol));
        if let Some(o) = out {
            std::fs::create_dir_all(&o).expect("out");
            std::fs::write(std::path::Path::new(&o).join("bench.tsv"), tsv).expect("tsv");
        }
        return;
    }
    if let Some(i) = a.iter().position(|x| x == "--cross") {
        a.remove(i);
        let num = |a: &mut Vec<String>, f: &str, d: u64| flag(a, f).and_then(|s| s.parse().ok()).unwrap_or(d);
        let (trials, seed, len) = (num(&mut a, "--trials", 60) as usize, num(&mut a, "--seed", 7), num(&mut a, "--len", 120) as usize);
        let report = cross::run(std::path::Path::new("."), trials, seed, len, 10);
        print!("{}", report.lines().take(3).collect::<Vec<_>>().join("\n") + "\n");
        if let Some(o) = flag(&mut a, "--out") {
            std::fs::create_dir_all(&o).expect("out");
            std::fs::write(std::path::Path::new(&o).join("cross.tsv"), report.lines().skip(3).collect::<Vec<_>>().join("\n") + "\n").expect("tsv");
        }
        return;
    }
    if let Some(i) = a.iter().position(|x| x == "--crossfit") {
        a.remove(i);
        let num = |a: &mut Vec<String>, f: &str, d: u64| flag(a, f).and_then(|s| s.parse().ok()).unwrap_or(d);
        let (trials, seed, folds) = (num(&mut a, "--trials", 40) as usize, num(&mut a, "--seed", 7), num(&mut a, "--folds", 5) as usize);
        for (name, texts) in suites::draw(trials, seed) {
            let fs: Vec<crossfit::Fit> = texts.iter().map(|(t, truth, p)| crossfit::text(t, truth, p, folds)).collect();
            print!("{}", crossfit::summary(&name, &fs));
        }
        return;
    }
    if let Some(i) = a.iter().position(|x| x == "--read") {
        a.remove(i);
        let num = |a: &mut Vec<String>, f: &str, d: u64| flag(a, f).and_then(|s| s.parse().ok()).unwrap_or(d);
        let (trials, seed, folds) = (num(&mut a, "--trials", 40) as usize, num(&mut a, "--seed", 7), num(&mut a, "--folds", 5) as usize);
        reading(trials, seed, folds, flag(&mut a, "--out"));
        return;
    }
    if let Some(i) = a.iter().position(|x| x == "--synthetic") {
        a.remove(i);
        let num = |a: &mut Vec<String>, f: &str, d: u64| flag(a, f).and_then(|s| s.parse().ok()).unwrap_or(d);
        let (trials, seed, len) = (num(&mut a, "--trials", 60) as usize, num(&mut a, "--seed", 7), num(&mut a, "--len", 120) as usize);
        let dir = flag(&mut a, "--chapters").unwrap_or_else(|| "crates/phiano/data/text/emma".into());
        let prior = Prior::read(std::path::Path::new(&dir));
        print!("{}", synth::sweep(&prior, trials, seed, len, 10, &[0.05, 0.1, 0.2, 0.4]));
        return;
    }
    let prior_dir = flag(&mut a, "--prior").unwrap_or_else(|| "crates/phiano/data/text/emma".into());
    let out = flag(&mut a, "--out").map(PathBuf::from);
    let words: usize = flag(&mut a, "--words").and_then(|s| s.parse().ok()).unwrap_or(6);
    let show: Vec<usize> = flag(&mut a, "--show")
        .map(|s| s.split(',').filter_map(codon_index).collect())
        .unwrap_or_else(|| vec![63, 62, 61, 59, 56, 52, 47, 40, 32]);
    let Some(path) = a.first() else {
        eprintln!("strand <story.txt> [--prior dir] [--out dir] [--show TTT,TTG] [--words N]");
        std::process::exit(2);
    };
    let text = std::fs::read_to_string(path).expect("story");
    let prior = Prior::read(std::path::Path::new(&prior_dir));
    let strand = Strand::new(&text, prior);
    let n = strand.tokens.len();
    println!("{} tokens · prior {} tokens ({}) · {} divisions", n, strand.prior.total, prior_dir, strand.cut.steps.len());

    // The ladder: codon, units, information gained over the level above.
    let mut tsv = String::from("k\tcodon\thexagram\tunits\tnats\tnats_per_token\n");
    let mut dna = String::new();
    for k in (0..CODONS).rev() {
        let g = strand.gain(k);
        writeln!(tsv, "{k}\t{}\t{}\t{}\t{g:.3}\t{:.5}", codon(k), hexagram_glyph(k), strand.units[k], g / n as f64).ok();
        writeln!(dna, "{} {} · level {k} · {} units · +{g:.2} nats", codon(k), hexagram_glyph(k), strand.units[k]).ok();
        for s in strand.level(k) {
            let sp = &strand.cut.spans[s];
            writeln!(dna, "  {}-{}\t{}", sp.start, sp.end, strand.signature(s, words).join(" ")).ok();
        }
    }
    let level = strand.informative_level();
    let gained = strand.cut.spans[0].value;
    println!(
        "the story's information supports {} units: dividing it so shortens its description by {gained:.1} nats; the finest level within that is {} {} with {} units; below it a division costs more to say than it saves",
        strand.informative_units(),
        codon(level),
        hexagram_glyph(level),
        strand.units[level]
    );

    let mut md = String::new();
    for &k in &show {
        writeln!(md, "\n### {} {} — {} units (+{:.1} nats over the level above)\n", codon(k), hexagram_glyph(k), strand.units[k], strand.gain(k)).ok();
        for (i, line) in strand.told_at(k, words).iter().enumerate() {
            writeln!(md, "{}. {line}", i + 1).ok();
        }
    }
    print!("{md}");
    if let Some(dir) = out {
        std::fs::create_dir_all(&dir).expect("out");
        std::fs::write(dir.join("story.dna"), dna).expect("dna");
        std::fs::write(dir.join("levels.tsv"), tsv).expect("tsv");
        std::fs::write(dir.join("story.md"), md).expect("md");
        println!("\nwritten: {}/{{story.dna, levels.tsv, story.md}}", dir.display());
    }
}

/// Every suite read in the strand's units: Emma joins, cross joins, synthetic topics, the talk.
fn reading(trials: usize, seed: u64, folds: usize, out: Option<String>) {
    for (name, texts) in suites::draw(trials, seed) {
        let rs: Vec<read::Reading> = texts.iter().map(|(t, truth, p)| read::text(t, truth, p, folds)).collect();
        print!("{}", readout::summary(&name, &rs));
        if let Some(o) = &out {
            std::fs::create_dir_all(o).expect("out");
            let file = name.replace([' ', '='], "_").replace('λ', "lambda");
            std::fs::write(std::path::Path::new(o).join(format!("read_{file}.tsv")), readout::curve(&rs)).expect("curve");
            std::fs::write(std::path::Path::new(o).join(format!("read_{file}_texts.tsv")), readout::texts(&rs)).expect("texts");
        }
    }
}
