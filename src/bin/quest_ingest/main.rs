//! Ingest a phiecs quest into a phiano facet, and measure what came through.
//!
//! In Phient's world, a word (w) is nothing but what it relates to, under what
//! conditions (c). A quest (phiecs) is a table of exactly that — `about`,
//! `kind`, `other`, `weight` — asked of an oracle one word at a time. This
//! binary is the bridge back: every row becomes the document `about kind
//! other`, the kinds become marker channels because they stand between the
//! pair, and `Trainer::couple_relations` moves each pair on its kind's channel
//! alone. Then the facet is asked to give the table back, and the answer is a
//! number, not a hope: the rank of the true `other` on the kind's channel,
//! against the same reading on the wrong channel and on the offset channels.
//!
//! `cargo run --release --bin quest_ingest [relations.tsv] [epochs] [out.md]`
//!
//! `PHIANO_QUEST_HOLDOUT=0.2` keeps a fifth of the rows out of training and
//! reads them one hop (chance, honestly) and two hops. The facet is saved
//! beside the table as `<table>.chroma`, which `bin/quest_ask` opens.
mod board;
mod categories;
mod holdout;
mod liskov;
mod repel;
mod report;
#[allow(unused_imports)]
use board::*;
#[allow(unused_imports)]
use categories::*;
#[allow(unused_imports)]
use repel::*;
#[allow(unused_imports)]
use report::*;
mod measure;
mod rows;
#[allow(unused_imports)]
use liskov::*;
#[allow(unused_imports)]
use holdout::*;
#[allow(unused_imports)]
use measure::*;
#[allow(unused_imports)]
use rows::*;

use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::phasor::Rounding;
use phiano::storage::Storage;
use phiano::trainer::Trainer;
use std::time::Instant;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let path = a.first().cloned().unwrap_or_else(|| "data/quest/synthetic.tsv".to_string());
    let epochs: usize = a.get(1).and_then(|s| s.parse().ok()).unwrap_or(8);
    let out = a.get(2).cloned();

    let all_rows = read_rows(&path);
    let holdout: f64 = std::env::var("PHIANO_QUEST_HOLDOUT").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let (rows, held) = split(&all_rows, holdout);
    let docs = documents(&rows);
    let pool = target_pool(&rows);
    let kinds = kinds(&rows);
    println!(
        "{path}: {} rows ({} held out) · {} kinds · {} target words · {} documents (weight × {WEIGHT_REPEATS})",
        rows.len(),
        held.len(),
        kinds.len(),
        pool.len(),
        docs.len()
    );

    let t0 = Instant::now();
    let mut facet = Facet::new();
    let markers = facet.learn_relations(&docs);
    for d in &docs {
        for w in d {
            facet.get_or_init(w);
        }
    }
    // The arms: negatives on/off × rounding nearest/dither. The shipped
    // trainer is negatives on, nearest; the quest ships attraction-only,
    // dithered (docs/83 for why each).
    let negatives = std::env::var("PHIANO_QUEST_NEGATIVES").map(|v| v != "0").unwrap_or(false);
    let rounding = match std::env::var("PHIANO_QUEST_ROUNDING").as_deref() {
        Ok("nearest") => Rounding::Nearest,
        _ => Rounding::Dither,
    };
    let trainer = match negatives {
        true => Trainer::new(LEARNING_RATE),
        false => Trainer::attraction_only(LEARNING_RATE),
    }
    .with_seed(7)
    .with_rounding(rounding);
    // Repulsion among the answers of one kind (`repel.rs`); 0 = off.
    let repel: f64 = std::env::var("PHIANO_QUEST_REPEL").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    // Or spread the answers once, before training, and never move them again
    // by repulsion: `PHIANO_QUEST_SPREAD=<passes>`.
    let spread: usize = std::env::var("PHIANO_QUEST_SPREAD").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
    for _ in 0..spread {
        repel_answers(&mut facet, &all_rows, 0.5);
    }
    println!("trainer: negatives {negatives} · rounding {rounding:?} · answer repulsion {repel} · spread before training {spread} passes");
    let mut report = String::new();
    report.push_str("| epoch | kind MRR | hit@1 | hit@5 | within kind MRR | wrong channel MRR | offset MRR |\n|---|---|---|---|---|---|---|\n");
    // The scoreboard: every `board_every` epochs, the three operators over
    // every word, filed by hexagram, printed and appended to
    // `<table>.board.tsv` — the quest watched while it learns.
    let board_every: usize = std::env::var("PHIANO_QUEST_BOARD").ok().and_then(|v| v.parse().ok()).unwrap_or(8).max(1);
    let board_path = format!("{}.board.tsv", path.trim_end_matches(".tsv"));
    let mut board_tsv = String::from("epoch\tkind\thexagram\tid\twords\tbetter\tworse\tloss\n");
    let mut boards = String::new();
    let mut previous: Option<Labels> = None;
    for epoch in 1..=epochs {
        let mut order: Vec<usize> = (0..docs.len()).collect();
        shuffle(&mut order, 7 + epoch as u64);
        for i in order {
            trainer.couple_relations(&mut facet, &docs[i]);
        }
        repel_answers(&mut facet, &rows, repel);
        let r = measure(&facet, &rows);
        let line = format!(
            "| {epoch} | {:.3} | {:.1}% | {:.1}% | {:.3} | {:.3} | {:.3} |",
            r.kind.mrr(),
            100.0 * r.kind.hit1(),
            100.0 * r.kind.hit5(),
            r.within.mrr(),
            r.wrong.mrr(),
            r.offset.mrr()
        );
        println!("{line}");
        report.push_str(&line);
        report.push('\n');
        if epoch % board_every == 0 || epoch == epochs {
            let scores = score_words(&facet, &all_rows);
            let b = board(&facet, &scores, &all_rows, previous.as_ref());
            let bl = b.line(epoch);
            println!("{bl}");
            boards.push_str(&bl);
            boards.push('\n');
            board_tsv.push_str(&board_rows(epoch, &scores, &b.labels));
            previous = Some(b.labels);
        }
    }
    std::fs::write(&board_path, &board_tsv).unwrap_or_else(|e| eprintln!("cannot write {board_path}: {e}"));
    report.push_str(&format!("\n### Scoreboard (every {board_every} epochs; per-hexagram rows in {board_path})\n\n```\n{boards}```\n"));
    let secs = t0.elapsed().as_secs_f64();
    summary_report(&facet, &rows, &pool, markers, secs, &mut report);
    if !held.is_empty() {
        holdout_report(&facet, &kinds, &held, &all_rows, &mut report);
    }
    // The three operators over every word, filed under its hexagram.
    operators_report(&facet, &all_rows, &path, &mut report);
    let chroma = format!("{}.chroma", path.trim_end_matches(".tsv"));
    match Storage::save(&facet, &chroma) {
        Ok(()) => println!("saved {chroma}"),
        Err(e) => eprintln!("could not save {chroma}: {e}"),
    }
    if let Some(out) = out {
        std::fs::write(&out, report).unwrap_or_else(|e| eprintln!("cannot write {out}: {e}"));
    }
}

/// Fisher–Yates on a splitmix stream, so an epoch's order is fixed by its seed.
pub(crate) fn shuffle(v: &mut [usize], mut seed: u64) {
    for i in (1..v.len()).rev() {
        seed = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = seed;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        v.swap(i, (z % (i as u64 + 1)) as usize);
    }
}
