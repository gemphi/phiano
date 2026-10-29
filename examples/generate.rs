//! Does the operator make generation better? Measured, not asserted.
//! `cargo run --release --bin generate_eval [corpus] [n_prompts]`
//!
//! Trains a v6 lexicon on the corpus, saves and reloads it (exercising the
//! format), then generates a continuation for `n` held-out prompts (the first
//! three words of a validation sentence) three ways: unguided, guided by the
//! directed operator in `data/phinet_generate.fno`, and — the honest control —
//! guided by an *untrained* operator of the same shape. Each continuation is
//! scored by a Kneser-Ney model trained on the same split (a judge neither
//! system can see), by the engine's own `Evaluator`, and by its distinct-token
//! ratio. Same seed for every arm.

use phiano::config::LEARNING_RATE;
use phient::eval::Evaluator;
use phient::generate::{ContextWaveBuffer, Generator, OperatorGuide};
use phient::metrics::harness::Harness;
use phient::metrics::baseline::kn::KneserNey;
use phient::net::space::Sides;
use phiecs::phasor::facet::storage::Storage;
use phiano::tokenizer::Tokenizer;
use phient::train::trainer::Trainer;

/// An arm: (name, operator, weight, prior-corrected proposals, widen, learned?, relational proposals).
type Arm = (&'static str, Option<&'static str>, f64, usize, usize, bool, usize);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let n: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(60);
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 3)
        .collect();
    let split = Harness::split(corpus, 42);
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let kn = KneserNey::train(&split.train);

    // Round-trip through the v6 format.
    Storage::save(&facet, "data/rust_book_v6.chroma").expect("save");
    let facet = Storage::load("data/rust_book_v6.chroma").expect("load");
    println!(
        "lexicon: {} words, {} evidenced, saved and reloaded as v6 ({} bytes)",
        facet.vocabulary_size(),
        facet.evidenced_words(),
        std::fs::metadata("data/rust_book_v6.chroma").map(|m| m.len()).unwrap_or(0)
    );

    let prompts: Vec<String> = split
        .valid
        .iter()
        .filter(|s| Tokenizer::tokenize(s).len() >= 6)
        .take(n)
        .map(|s| Tokenizer::tokenize(s)[..3].join(" "))
        .collect();
    println!("{} prompts from held-out sentences; judge: Kneser-Ney on the training split", prompts.len());

    let _untrained = {
        let space = phient::net::space::FunctionSpace::from_facet_sides(&facet, Sides::Directed);
        let cfg = phical::FnoConfig { space: space.space(), width: 8, layers: 2, modes: 12, margin: 0.2, lr: 2e-3, tie: phical::Tie::Circle };
        let fno = phical::Fno::new(cfg, 99);
        std::fs::write("data/phinet_untrained.fno", fno.to_bytes()).expect("write");
        "data/phinet_untrained.fno"
    };

    let learned = phient::generate::learn::GenWeight::load_default();
    println!("learned weights: {}", if learned.is_some() { "data/learned.json" } else { "none (run bin/learn)" });
    let arms: [Arm; 8] = [
        ("hand scorer, unguided", None, 0.0, 0, 0, false, 0),
        ("hand scorer, directed operator w=0.5", Some("data/phinet_generate.fno"), 0.5, 0, 0, false, 0),
        ("hand, directed, 64 candidates w=1", Some("data/phinet_generate.fno"), 1.0, 0, 64, false, 0),
        ("learned, no operator", None, 0.0, 0, 0, true, 0),
        ("learned + operator", Some("data/phinet_generate.fno"), 0.0, 0, 0, true, 0),
        ("learned + operator, 64 candidates", Some("data/phinet_generate.fno"), 0.0, 0, 64, true, 0),
        ("learned, 8 relational proposals", None, 0.0, 0, 0, true, 8),
        ("learned + operator, 8 relational proposals", Some("data/phinet_generate.fno"), 0.0, 0, 0, true, 8),
    ];
    let ev = Evaluator::new();
    println!("\n  {:<32} {:>10} {:>9} {:>9} {:>8}", "arm", "KN ppl", "eval", "distinct", "tokens");
    for (name, op, weight, proposals, widen, use_learned, rel) in arms {
        let (mut ppl_sum, mut ev_sum, mut dist_sum, mut tok_sum, mut cnt) = (0.0, 0.0, 0.0, 0usize, 0usize);
        for (i, prompt) in prompts.iter().enumerate() {
            let mut gen = Generator::deterministic(24, 0.15, 1000 + i as u64);
            if use_learned {
                gen = gen.with_weights(learned.clone());
            }
            gen.relational_proposals = rel;
            if let Some(p) = op {
                match OperatorGuide::load(p, &facet, Sides::Directed, 6) {
                    Ok(mut g) => {
                        g.weight = weight;
                        g.proposals = proposals;
                        g.widen = widen;
                        gen = gen.with_operator(g)
                    }
                    Err(e) => {
                        println!("  {name}: {e}");
                        break;
                    }
                }
            }
            let mut buf = ContextWaveBuffer::new(64);
            let text = gen.generate(&facet, &mut buf, prompt);
            let toks = Tokenizer::tokenize(&text);
            if toks.len() < 4 {
                continue;
            }
            let full = format!("{prompt} {text}");
            ppl_sum += kn.perplexity(std::slice::from_ref(&full)).ln();
            ev_sum += ev.eval(&facet, &full).overall;
            let uniq: std::collections::HashSet<&String> = toks.iter().collect();
            dist_sum += uniq.len() as f64 / toks.len() as f64;
            tok_sum += toks.len();
            cnt += 1;
            if i < 3 {
                println!("    [{name}] {prompt} → {text}");
            }
        }
        if cnt > 0 {
            println!(
                "  {:<32} {:>10.1} {:>9.3} {:>9.3} {:>8.1}   (n = {cnt})",
                name,
                (ppl_sum / cnt as f64).exp(),
                ev_sum / cnt as f64,
                dist_sum / cnt as f64,
                tok_sum as f64 / cnt as f64
            );
        }
    }
    println!("\n  KN ppl is the geometric mean over continuations (lower = more like the corpus); eval is the engine's own score; distinct = unique/total tokens.");
}
