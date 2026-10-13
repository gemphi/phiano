//! Does `Impersonator::impersonate` write as the persona asked for? Same
//! lexicon, personas, prompts and judge as `bin/persona_eval`, but through the
//! impersonation path the REPL and the server expose — so the number here is
//! the number a user of `persona impersonate` gets.
//!
//! `cargo run --release --bin impersonate_eval [prompts]`
#[allow(dead_code)]
use phiano::lab as util;
#[path = "../persona_eval/world.rs"]
mod world;

use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::metrics::baseline::kn::KneserNey;
use phiano::persona::Impersonator;
use phiano::storage::Storage;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;
use util::dir_sentences;

fn main() {
    let n_prompts = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(12);
    let austen = dir_sentences("data/corpora/austen");
    let contrast = dir_sentences("data/corpora/contrast");
    let austen_all: Vec<String> = austen.iter().flat_map(|(_, s)| s.clone()).collect();
    let split = Harness::split(austen_all, 7);
    let facet: Facet = Storage::load("data/austen.chroma").expect("run bin/austen first");
    let (world, pfacet) = world::build(&facet, &split, &contrast);
    let kn = KneserNey::train(&split.train);
    let prompts = world::held_prompts(&split, &contrast, n_prompts);
    let mut names: Vec<String> = world.personas.keys().cloned().collect();
    names.sort();
    let trainer = Trainer::new(LEARNING_RATE).with_seed(1);
    println!("| asked for | read as Austen | Chesterton | Carroll | as asked | KN (Austen) | distinct | mean tokens |\n|---|---|---|---|---|---|---|---|");
    let (mut hit, mut total) = (0usize, 0usize);
    let mut shown = String::new();
    for name in &names {
        let persona = &world.personas[name];
        let mut picks = std::collections::BTreeMap::new();
        let (mut lp, mut d, mut len, mut n) = (0.0, 0.0, 0usize, 0usize);
        for (i, prompt) in prompts.iter().enumerate() {
            let mut work = pfacet.clone();
            let imp = Impersonator::new();
            let r = imp.impersonate_with(&mut work, &trainer, &persona.fingerprint, &persona.examples, name, prompt);
            let toks = Tokenizer::tokenize(&r.text);
            if toks.len() < 3 {
                continue;
            }
            if let Some(m) = world.match_text(&pfacet, &r.text) {
                *picks.entry(m.best_name).or_insert(0usize) += 1;
            }
            lp += kn.perplexity(std::slice::from_ref(&r.text)).ln();
            let uniq: std::collections::HashSet<&String> = toks.iter().collect();
            d += uniq.len() as f64 / toks.len() as f64;
            len += toks.len();
            n += 1;
            if i == 0 {
                shown.push_str(&format!("- *{name}* · *{prompt}* → {}\n", r.text));
            }
        }
        let ok = picks.get(name).copied().unwrap_or(0);
        hit += ok;
        total += n;
        let nf = n.max(1) as f64;
        println!("| {name} | {} | {} | {} | {:.0}% | {:.1} | {:.3} | {:.1} |", picks.get("Jane Austen").unwrap_or(&0), picks.get("G. K. Chesterton").unwrap_or(&0), picks.get("Lewis Carroll").unwrap_or(&0), 100.0 * ok as f64 / nf, (lp / nf).exp(), d / nf, len as f64 / nf);
    }
    println!("| **all** | | | | **{:.0}%** (chance 33%) | | | |\n\n{shown}", 100.0 * hit as f64 / total.max(1) as f64);
}
