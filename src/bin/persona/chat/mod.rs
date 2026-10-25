//! A dialogue with an ingested persona.
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions. A question names a condition; the persona's manifold says what
//! that condition calls for (X ⇒ Y); the reply is generated toward it, by a
//! generator conditioned on that writer's sector histogram and pair evidence,
//! with the context wave carried across turns (`persona::Dialogue`). Nothing
//! here is a template: the same code path `bin/persona_eval` measures at 53%
//! attribution against 33% chance.
//!
//! `cargo run --release --bin persona_chat -- ["<persona>"] ["question" …]`
//! with no persona, every ingested persona answers the same questions;
//! `--interactive` reads questions from stdin.
mod judge;
mod script;
#[allow(unused_imports)]
use judge::*;
#[allow(unused_imports)]
use script::*;

use phiano::facet::Facet;
use phiano::persona::{Dialogue, PersonaStore, Reply, World};
use phiano::storage::Storage;

const LEXICON: &str = "data/personas.chroma";
const STORE: &str = "data/personas.json";

pub fn run(a: Vec<String>) {
    let args = a;
    let interactive = args.iter().any(|a| a == "--interactive");
    let args: Vec<String> = args.into_iter().filter(|a| a != "--interactive").collect();
    // PHIANO_PERSONAS: another store — e.g. personas learned from quests
    // (bin/persona_from_quest) — read against the lexicon it names.
    let store = std::env::var("PHIANO_PERSONAS").unwrap_or_else(|_| STORE.to_string());
    let lexicon = PersonaStore::lexicon_of(&store).unwrap_or_else(|_| LEXICON.to_string());
    let facet: Facet = Storage::load(&lexicon).expect("run bin/persona_ingest (or persona_from_quest) first");
    let world: World = PersonaStore::load(&store, &facet).expect("run bin/persona_ingest (or persona_from_quest) first");
    let mut names: Vec<String> = world.personas.keys().cloned().collect();
    names.sort();
    let (asked, questions) = match args.first() {
        Some(a) if names.iter().any(|n| n.eq_ignore_ascii_case(a) || n.to_lowercase().contains(&a.to_lowercase())) => {
            let who = names.iter().find(|n| n.eq_ignore_ascii_case(a) || n.to_lowercase().contains(&a.to_lowercase())).unwrap().clone();
            (vec![who], args[1..].to_vec())
        }
        _ => (names.clone(), args.clone()),
    };
    let questions: Vec<String> = match questions.is_empty() {
        true => match std::env::var("PHIANO_CHAT_SCRIPT").as_deref() {
            Ok("long") => SCRIPT_LONG.iter().map(|s| s.to_string()).collect(),
            _ => SCRIPT.iter().map(|s| s.to_string()).collect(),
        },
        false => questions,
    };
    let fps = world.fingerprints(&facet);
    let panel = Panel::train(&world);
    println!("# Dialogue\n\nlexicon {} words · personas {} · two judges read each reply alone: the engine's sector attribution and a Kneser-Ney panel, one model per writer; chance {:.0}%\n", facet.vocabulary_size(), names.join(", "), 100.0 / names.len() as f64);

    let mut dialogues: Vec<Dialogue> = asked.iter().map(|n| Dialogue::open(&facet, &world.personas[n])).collect();
    let mut asked_n = 0usize;
    let (mut hit, mut total, mut kn_hit) = (0usize, 0usize, 0usize);
    // How often a reply reaches the word its topic called for.
    let reached = std::cell::Cell::new((0usize, 0usize));
    let ask = |q: &str, dialogues: &mut Vec<Dialogue>, asked_n: &mut usize, hit: &mut usize, total: &mut usize, kn_hit: &mut usize| {
        for d in dialogues.iter_mut() {
            let name = d.name.clone();
            let r: Reply = d.reply(&facet, q);
            if let Some(goal) = &r.calls_for {
                let (got, of) = reached.get();
                let hit_goal = r.text.to_lowercase().split(|c: char| !c.is_alphanumeric()).any(|w| w == goal);
                reached.set((got + hit_goal as usize, of + 1));
            }
            let verdict = world.match_text_with(&facet, &r.text, &fps).map(|m| {
                *total += 1;
                if m.best_name == name {
                    *hit += 1;
                }
                (m.best_name, m.confidence)
            });
            let kn = panel.read(&r.text).map(|(who, ppl, margin)| {
                if who == name {
                    *kn_hit += 1;
                }
                (who, ppl, margin)
            });
            show(*asked_n + 1, q, &name, &r, verdict, kn);
        }
        *asked_n += 1;
    };
    for q in &questions {
        ask(q, &mut dialogues, &mut asked_n, &mut hit, &mut total, &mut kn_hit);
    }
    if interactive {
        use std::io::Write;
        loop {
            print!("\nyou › ");
            std::io::stdout().flush().ok();
            let mut line = String::new();
            if std::io::stdin().read_line(&mut line).unwrap_or(0) == 0 {
                break;
            }
            let q = line.trim().to_string();
            if q.is_empty() || q == "quit" || q == "exit" {
                break;
            }
            ask(&q, &mut dialogues, &mut asked_n, &mut hit, &mut total, &mut kn_hit);
        }
    }
    let pct = |k: usize| 100.0 * k as f64 / total.max(1) as f64;
    let (got, of) = reached.get();
    println!("\n---\n\n{total} replies · read back as the persona who spoke: sector attribution {} ({:.0}%), Kneser-Ney panel {} ({:.0}%) · chance {:.0}% · reached the word the topic called for {got}/{of} · reading {:?}.", hit, pct(hit), kn_hit, pct(kn_hit), 100.0 / names.len() as f64, phiano::facet::Reading::from_env());
}
