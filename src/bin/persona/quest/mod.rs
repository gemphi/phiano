//! Personas learned from quests: every persona file's quest folders become a
//! persona `bin/persona_chat` can talk as, and a relation model it answers
//! from.
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c). A persona file (phiace `personas/*.md`, phiecs
//! `persona::Persona`) plans a quest; `cargo run -p phiecs --example persona_quest` runs it into
//! `<root>/<slug>/<seed>/{relations.tsv,sentences.tsv}`. This reads those:
//!
//! - the sentences, in the persona's voice, are its examples — the same
//!   thing `bin/persona_ingest` reads from a writer's books;
//! - the sentences and the relation rows (as `about kind other`) together
//!   train one shared lexicon;
//! - each persona's rows train a `phiano::relate` model, saved as
//!   `<root>/<slug>/<slug>.relate`, scored on a held-out fifth against the
//!   prior.
//!
//! `cargo run --release --bin persona_from_quest -- <root> <persona.md>…`
//! writes `<root>/personas.chroma` and `<root>/personas.json`; then
//! `PHIANO_PERSONAS=<root>/personas.json cargo run --release --bin persona_chat`.

mod relate;

use phiano::config::LEARNING_RATE;
use phiano::metrics::harness::{Harness, Split};
use phiano::persona::{PersonaStore, Styles};
use phiano::storage::Storage;
use phiano::trainer::Trainer;
use phiano::relate::read_quests;
use phiecs::persona::Persona;
use std::path::Path;

pub fn run(a: Vec<String>) {
    let a = a;
    let Some(root) = a.first() else {
        eprintln!("usage: persona_from_quest <root> <persona.md>…");
        std::process::exit(2);
    };
    let root = Path::new(root);
    let mut personas: Vec<(Persona, Vec<phiecs::quest::Row>, Vec<String>)> = Vec::new();
    for file in &a[1..] {
        let text = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("cannot read {file}: {e}"));
        let p = Persona::from_md(&text).unwrap_or_else(|e| panic!("{file}: {e}"));
        let (rows, sentences) = read_quests(&root.join(p.slug()));
        println!("{}: {} relations, {} sentences", p.name, rows.len(), sentences.len());
        personas.push((p, rows, sentences));
    }
    personas.retain(|(p, rows, s)| {
        let ok = !rows.is_empty() || !s.is_empty();
        if !ok {
            println!("{}: no quest output under {}, skipped", p.name, root.join(p.slug()).display());
        }
        ok
    });
    if personas.is_empty() {
        eprintln!("nothing to learn from: run examples/persona_quest first");
        std::process::exit(1);
    }

    // One lexicon for all: every sentence, and every row read as a sentence.
    let docs: Vec<String> = personas
        .iter()
        .flat_map(|(_, rows, s)| s.iter().cloned().chain(rows.iter().map(|r| format!("{} {} {}", r.about, r.kind, r.other))))
        .collect();
    let facet = Harness::train_ranking_only(&Split { train: docs, valid: Vec::new(), test: Vec::new() }, &Trainer::new(LEARNING_RATE).with_seed(0), 3);
    let lexicon = root.join("personas.chroma").to_string_lossy().to_string();
    let mut pfacet = facet;
    let mut world = Styles::new();
    let trainer = Trainer::new(LEARNING_RATE).with_seed(1);
    for (p, rows, sentences) in &personas {
        if !sentences.is_empty() {
            world.add_persona_quiet(&p.name, sentences, &mut pfacet, &trainer);
        }
        if rows.len() >= 20 {
            let out = root.join(p.slug()).join(format!("{}.relate", p.slug()));
            relate::train(p, rows, &out);
        }
    }
    Storage::save(&pfacet, &lexicon).expect("save lexicon");
    let store = root.join("personas.json").to_string_lossy().to_string();
    PersonaStore::save(&world, &lexicon, &store).expect("save personas");
    println!("\n{} personas → {store} (lexicon {lexicon}, {} words)", world.personas.len(), pfacet.vocabulary_size());
    println!("talk: PHIANO_PERSONAS={store} cargo run --release --bin persona_chat -- <name> \"<question>\"");
}
