//! `novelty_fold` — what folding the evaluator's novelty angle moved
//! (docs/206). The evaluator's novelty is the angle between a sentence's
//! wave and the lexicon's centroid. It used to be `|Δarg|` capped at π, so
//! two directions either side of the ±π seam read as maximally novel; it is
//! now folded into [0, π] (`phical::circle::distance`). For every sentence
//! this reads novelty both ways from the same wave and centroid, and counts
//! what moved: the novelty, the overall score, and the quality class.
//!
//! ```text
//! cargo run --release -p phiano --example novelty_fold -- data/person/murphy.txt data/person/murphy.txt [n]
//! cargo run --release -p phiano --example novelty_fold -- data/austen.chroma data/person/murphy.txt [n]
//! ```

use phiano::config::LEARNING_RATE;
use phiecs::phasor::facet::storage::Storage;
use phiecs::phasor::facet::Facet;
use phient::train::trainer::Trainer;
use phiecs::phasor::wave::Wave;
use phient::eval::constants::NOVELTY_SCALE;
use phient::eval::{Evaluator, Quality};
use phiton::p64::words::Tokenizer;
use std::f64::consts::PI;

/// The novelty from an angle, as the evaluator computes it.
fn novelty(angular: f64) -> f64 {
    let normalized = angular / PI;
    (1.0 - (-(normalized * NOVELTY_SCALE * 5.0).min(20.0)).exp()).clamp(0.0, 1.0)
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let lexicon = a.first().cloned().unwrap_or_else(|| "data/austen.chroma".into());
    let corpus = a.get(1).cloned().unwrap_or_else(|| "data/person/murphy.txt".into());
    let limit: usize = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(2000);
    let text = std::fs::read_to_string(&corpus).expect("read the corpus");
    let all: Vec<&str> = text.split(['.', '!', '?', '\n']).map(str::trim).filter(|s| s.split_whitespace().count() >= 4).collect();
    // A `.txt` lexicon is trained here, on nine sentences in ten of it, and the
    // tenth (never trained on) is what is read; a `.chroma` lexicon is loaded.
    let (facet, sentences): (Facet, Vec<&str>) = match lexicon.ends_with(".txt") {
        true => {
            let own = std::fs::read_to_string(&lexicon).expect("read the lexicon's text");
            let lines: Vec<&str> = own.split(['.', '!', '?', '\n']).map(str::trim).filter(|s| s.split_whitespace().count() >= 4).collect();
            let train: Vec<String> = lines.iter().enumerate().filter(|(i, _)| i % 10 != 0).map(|(_, s)| s.to_string()).collect();
            let mut facet = Facet::new();
            Trainer::new(LEARNING_RATE).train_corpus_with_descent(&mut facet, &train);
            let held: Vec<&str> = all.iter().enumerate().filter(|(i, _)| i % 10 == 0).map(|(_, s)| *s).take(limit).collect();
            eprintln!("lexicon trained on {} sentences of {lexicon}", train.len());
            (facet, held)
        }
        false => (Storage::load(&lexicon).expect("load the lexicon"), all.into_iter().take(limit).collect()),
    };

    let ev = Evaluator::new();
    let centroid = facet.centroid();
    let (mut defined, mut across, mut class_moved) = (0usize, 0usize, 0usize);
    let (mut dn_sum, mut dn_max, mut do_sum, mut do_max) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut shown = Vec::new();
    for s in &sentences {
        let e = ev.eval(&facet, s);
        if e.novelty.is_nan() {
            continue;
        }
        defined += 1;
        let tokens = Tokenizer::tokenize(s);
        let wave = Wave::sentence(&facet, &tokens);
        let raw = (centroid.arg() - wave.arg()).abs();
        if raw <= PI {
            continue;
        }
        across += 1;
        let old_novelty = novelty(raw.min(PI));
        let old_overall = Evaluator::overall(e.coherence, old_novelty, e.resonance);
        let old_class = Quality::from_scores(e.coherence, old_novelty, e.resonance);
        let (dn, dov) = ((e.novelty - old_novelty).abs(), (e.overall - old_overall).abs());
        dn_sum += dn;
        dn_max = dn_max.max(dn);
        do_sum += dov;
        do_max = do_max.max(dov);
        if old_class != e.verdict {
            class_moved += 1;
        }
        if shown.len() < 8 {
            shown.push(format!("| {} | {:.3} → {:.3} | {:.3} → {:.3} | {:?} → {:?} |", s.chars().take(60).collect::<String>(), old_novelty, e.novelty, old_overall, e.overall, old_class, e.verdict));
        }
    }
    println!("# novelty folded — {lexicon} on {corpus}\n");
    println!("the lexicon's centroid points at {:.3} rad (the seam is at ±π = ±{PI:.3})\n", centroid.arg());
    println!("{} sentences read · {defined} with a defined novelty · **{across} across the ±π seam** ({:.1}% of those defined)\n", sentences.len(), 100.0 * across as f64 / defined.max(1) as f64);
    println!("Of those across the seam: novelty moved by {:.3} on average (at most {:.3}); the overall score by {:.3} (at most {:.3}); the quality class changed for {class_moved}.\n", dn_sum / across.max(1) as f64, dn_max, do_sum / across.max(1) as f64, do_max);
    println!("| sentence | novelty, capped → folded | overall | class |\n|---|---|---|---|");
    for r in shown {
        println!("{r}");
    }
}
