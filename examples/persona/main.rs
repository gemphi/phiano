//! Does a persona condition composition? One shared lexicon (`data/austen.chroma`,
//! three authors), three personas, the same held-out Austen prompts, and the
//! generator run once per persona with `Generator::with_persona` at several
//! weights. The judge is `Styles::match_text` — the attribution reader that
//! scores held-out paragraphs at ~64–70% (docs/74–76) — applied to what was
//! composed. Chance is one in three; a persona that conditions nothing reads
//! the same author whatever it is asked for.
//!
//! `cargo run --release --bin persona_eval [prompts] [tokens]`
#[allow(dead_code)]
use phiano::lab as util;
mod world;

use phiecs::phasor::facet::Facet;
use phiano::generate::{ContextWaveBuffer, Decode, Generator};
use phient::metrics::harness::Harness;
use phient::metrics::baseline::kn::KneserNey;
use phiecs::phasor::facet::storage::Storage;
use phiano::tokenizer::Tokenizer;
use util::dir_sentences;

fn main() {
    let arg = |k: usize| std::env::args().nth(k).and_then(|s| s.parse::<usize>().ok());
    let (n_prompts, tokens) = (arg(1).unwrap_or(24), arg(2).unwrap_or(30));
    let austen = dir_sentences("data/corpora/austen");
    let contrast = dir_sentences("data/corpora/contrast");
    let austen_all: Vec<String> = austen.iter().flat_map(|(_, s)| s.clone()).collect();
    let split = Harness::split(austen_all, 7);
    let facet: Facet = Storage::load("data/austen.chroma").expect("run bin/austen first");
    let (world, pfacet) = world::build(&facet, &split, &contrast);
    let kn = KneserNey::train(&split.train);
    // Prompts from every author's held-out sentences in equal number, so the
    // prompt's own author cannot decide the verdict.
    let prompts: Vec<String> = world::held_prompts(&split, &contrast, n_prompts);
    let mut names: Vec<String> = world.personas.keys().cloned().collect();
    names.sort();
    println!("shared lexicon {} words · {} personas · {} prompts (all three authors' held-out sentences) × {} tokens · judge: Styles::match_text (chance {:.0}%)\n", pfacet.vocabulary_size(), names.len(), prompts.len(), tokens, 100.0 / names.len() as f64);
    println!("| asked for | weight | read as Austen | Chesterton | Carroll | as asked | KN (Austen) | distinct |\n|---|---|---|---|---|---|---|---|");
    let mut summary = Vec::new();
    println!("weights are (sector prior, persona bigram) — see `generate::persona`.\n");
    for (weight, ngram) in [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0), (0.0, 2.0)] {
        let mut hit_total = (0usize, 0usize);
        for name in &names {
            let fp = &world.personas[name].fingerprint;
            let mut picks = std::collections::BTreeMap::new();
            let (mut lp, mut d, mut n) = (0.0, 0.0, 0usize);
            for (i, prompt) in prompts.iter().enumerate() {
                let g = Generator::deterministic(tokens, 0.3, 900 + i as u64).with_decode(Decode::Trigram).with_persona_examples(&pfacet, fp, &world.personas[name].examples, weight, ngram);
                let mut buf = ContextWaveBuffer::new(64);
                let text = g.generate(&pfacet, &mut buf, prompt);
                let toks = Tokenizer::tokenize(&text);
                if toks.len() < 3 {
                    continue;
                }
                let full = format!("{prompt} {text}");
                if let Some(m) = world.match_text(&pfacet, &text) {
                    *picks.entry(m.best_name).or_insert(0usize) += 1;
                }
                lp += kn.perplexity(&[full]).ln();
                let uniq: std::collections::HashSet<&String> = toks.iter().collect();
                d += uniq.len() as f64 / toks.len() as f64;
                n += 1;
                if i == 0 && weight == 1.0 && ngram == 1.0 {
                    summary.push(format!("- *{name}* · *{prompt}* → {text}"));
                }
            }
            let ok = picks.get(name).copied().unwrap_or(0);
            hit_total.0 += ok;
            hit_total.1 += n;
            let nf = n.max(1) as f64;
            println!("| {name} | {weight}, {ngram} | {} | {} | {} | {:.0}% | {:.1} | {:.3} |", picks.get("Jane Austen").unwrap_or(&0), picks.get("G. K. Chesterton").unwrap_or(&0), picks.get("Lewis Carroll").unwrap_or(&0), 100.0 * ok as f64 / nf, (lp / nf).exp(), d / nf);
        }
        println!("| **all** | {weight}, {ngram} | | | | **{:.0}%** | | |", 100.0 * hit_total.0 as f64 / hit_total.1.max(1) as f64);
    }
    println!("\n{}", summary.join("\n"));
}
