//! Ingest every persona from every sentence of their books, and save them.
//!
//! In Phient's world a persona is which conditions a writer puts words under,
//! aggregated. Until now that aggregate was 600 sampled sentences per writer
//! (`bin/austen`), because the personas were rebuilt on every run; this
//! ingests each writer's whole training split and writes the result to
//! `data/personas.chroma` + `data/personas.json`, so `bin/persona_chat` can
//! open a dialogue without re-reading a corpus. The lexicon is trained on the
//! three writers' *training* splits only, so held-out text is held out of the
//! manifold as well as the fingerprint.
//!
//! `cargo run --release --bin persona_ingest [passes] [baseline_sample]`
#[allow(dead_code)]
use phiano::lab as util;
mod corpus;
mod measure;
#[allow(unused_imports)]
use corpus::*;
#[allow(unused_imports)]
use measure::*;

use phiano::config::LEARNING_RATE;
use phiecs::phasor::facet::Facet;
use phiano::metrics::harness::{Harness, Split};
use phiano::persona::{PersonaStore, Styles};
use phiano::storage::Storage;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;
use util::dir_sentences;

const LEXICON: &str = "data/personas.chroma";
const STORE: &str = "data/personas.json";

pub fn run(a: Vec<String>) {
    let arg = |k: usize| a.get(k - 1).and_then(|s| s.parse::<usize>().ok());
    let (passes, baseline_n) = (arg(1).unwrap_or(3), arg(2).unwrap_or(600));
    let t0 = std::time::Instant::now();
    let authors = authors();
    println!("| author | books | sentences | train | held out | words |\n|---|---|---|---|---|---|");
    for a in &authors {
        let words: usize = a.split.train.iter().map(|s| Tokenizer::tokenize(s).len()).sum();
        println!("| {} | {} | {} | {} | {} | {} |", a.name, a.files.len(), a.split.train.len() + a.split.valid.len() + a.split.test.len(), a.split.train.len(), a.split.test.len(), words);
    }

    // ── the shared lexicon: every author's training split, nothing else ──
    let train: Vec<String> = authors.iter().flat_map(|a| a.split.train.clone()).collect();
    let valid: Vec<String> = authors.iter().flat_map(|a| a.split.valid.clone()).collect();
    let test: Vec<String> = authors.iter().flat_map(|a| a.split.test.clone()).collect();
    let split = Split { train, valid, test };
    println!("\ntraining the shared lexicon on {} sentences, {passes} passes…", split.train.len());
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), passes);
    Storage::save(&facet, LEXICON).expect("save lexicon");
    println!("  {} words, {} with relation evidence, {} marker channels → {LEXICON} ({:.0} s)", facet.vocabulary_size(), facet.evidenced_words(), facet.relations.markers(), t0.elapsed().as_secs_f64());

    // ── the personas: the sampled baseline, then every sentence ──
    let trainer = Trainer::new(LEARNING_RATE).with_seed(1);
    let mut base_facet = facet.clone();
    let mut base = Styles::new();
    for a in &authors {
        base.add_persona_quiet(&a.name, &sample(&a.split.train, baseline_n, 11), &mut base_facet, &trainer);
    }
    let (base_acc, base_rows) = attribution(&base, &base_facet, &authors, 150);
    drop(base_facet);

    let mut pfacet = facet.clone();
    let mut world = Styles::new();
    for a in &authors {
        let t = std::time::Instant::now();
        world.add_persona_quiet(&a.name, &a.split.train, &mut pfacet, &trainer);
        println!("  ingested {} from {} sentences ({:.0} s)", a.name, a.split.train.len(), t.elapsed().as_secs_f64());
    }
    Storage::save(&pfacet, LEXICON).expect("save lexicon");
    PersonaStore::save(&world, LEXICON, STORE).expect("save personas");

    // ── what the full ingestion bought ──
    let (full_acc, full_rows) = attribution(&world, &pfacet, &authors, 150);
    println!("\n## Attribution on held-out paragraphs (chance {:.0}%)\n", 100.0 / authors.len() as f64);
    println!("| persona | ingested from | sampled {baseline_n} | all sentences |\n|---|---|---|---|");
    for (i, a) in authors.iter().enumerate() {
        println!("| {} | {} sentences | {:.0}% | {:.0}% |", a.name, a.split.train.len(), 100.0 * base_rows[i].1 as f64 / base_rows[i].2.max(1) as f64, 100.0 * full_rows[i].1 as f64 / full_rows[i].2.max(1) as f64);
    }
    println!("| **all** | | **{base_acc:.0}%** | **{full_acc:.0}%** |");
    println!("\n## The personas\n");
    println!("| persona | samples | mean length | entropy | dominant sectors |\n|---|---|---|---|---|");
    let mut names: Vec<&String> = world.personas.keys().collect();
    names.sort();
    for n in &names {
        let fp = &world.personas[*n].fingerprint;
        println!("| {} | {} | {:.1} | {:.3} | {} |", n, fp.sample_count, fp.avg_length, fp.diversity, fp.dominant_sectors(5).iter().map(|(s, w)| format!("{s}:{w:.3}")).collect::<Vec<_>>().join(" "));
    }
    println!("\n{STORE} written; {:.0} s total. `cargo run --release --bin persona_chat -- \"<persona>\"` opens a dialogue.", t0.elapsed().as_secs_f64());
}
