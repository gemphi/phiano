//! Decoding from the operator versus the trigram sampler, on Austen.
//! `cargo run --release --bin decode_eval [tokens] [k]`
use phiano::eval::Evaluator;
use phiano::facet::Facet;
use phiano::generate::{ContextWaveBuffer, Decode, Generator, OperatorGuide};
use phiano::learn::GenWeights;
use phiano::metrics::harness::Harness;
use phiano::metrics::baseline::kn::KneserNey;
use phiano::phinet::space::Sides;
use phiano::storage::Storage;
use phiano::tokenizer::Tokenizer;

use phiano::lab::sentences_of;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let max_tokens: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(30);
    let k: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(32);
    let mut all = Vec::new();
    for f in ["emma", "persuasion", "sense_and_sensibility"] { all.extend(sentences_of(&format!("data/corpora/austen/{f}.txt"))); }
    let split = Harness::split(all, 7);
    let facet: Facet = Storage::load("data/austen_voice.chroma").expect("run bin/austen first");
    let kn = KneserNey::train(&split.train);
    let ev = Evaluator::new();
    let learned = GenWeights::load("data/learned_austen.json").map(|w| w.for_decoding());
    let prompts: Vec<String> = split.test.iter().filter(|s| Tokenizer::tokenize(s).len() >= 8).take(40).map(|s| Tokenizer::tokenize(s)[..3].join(" ")).collect();
    let lambda: f64 = std::env::var("PHIANO_FORCING").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0);
    let arms: Vec<(&str, Decode)> = vec![
        ("trigram sampler", Decode::Trigram),
        ("operator nearest-k", Decode::Operator { k }),
        ("union", Decode::Union { k }),
        ("forcing (Liskov, λ)", Decode::Forcing { k, lambda }),
    ];
    println!("| arm | n | KN perplexity (geo. mean) | evaluator | distinct | mean tokens | novel bigrams | shared 4-grams |\n|---|---|---|---|---|---|---|---|");
    let mut shown = String::new();
    for (name, decode) in &arms {
        let (mut lp, mut e, mut d, mut len, mut novel, mut n) = (0.0, 0.0, 0.0, 0usize, 0.0, 0usize);
        let mut grams: Vec<std::collections::HashSet<String>> = Vec::new();
        for (i, prompt) in prompts.iter().enumerate() {
            let guide = OperatorGuide::load("data/austen_operator.fno", &facet, Sides::Directed, 6).expect("operator");
            let g = Generator::deterministic(max_tokens, 0.3, 900 + i as u64).with_weights(learned.clone()).with_operator(guide).with_decode(*decode);
            let mut buf = ContextWaveBuffer::new(64);
            let text = g.generate(&facet, &mut buf, prompt);
            let toks = Tokenizer::tokenize(&text);
            if toks.len() < 3 { continue; }
            let full = format!("{prompt} {text}");
            lp += kn.perplexity(std::slice::from_ref(&full)).ln();
            e += ev.eval(&facet, &full).overall;
            let uniq: std::collections::HashSet<&String> = toks.iter().collect();
            d += uniq.len() as f64 / toks.len() as f64;
            len += toks.len();
            // bigrams the tables have never seen: what decoding outside the tables produces
            let pairs = toks.windows(2).count().max(1);
            let unseen = toks.windows(2).filter(|w| !facet.next_word_candidates(&w[0]).iter().any(|(x, _)| *x == w[1])).count();
            novel += unseen as f64 / pairs as f64;
            grams.push(toks.windows(4).map(|w| w.join(" ")).collect());
            n += 1;
            if i < 3 { shown.push_str(&format!("- {name} · *{prompt}* → {text}\n")); }
        }
        let nf = n.max(1) as f64;
        // the attractor: 4-grams of one continuation that also occur in another prompt's
        let shared: f64 = grams.iter().enumerate().map(|(i, g)| {
            let hit = g.iter().filter(|x| grams.iter().enumerate().any(|(j, h)| j != i && h.contains(*x))).count();
            hit as f64 / g.len().max(1) as f64
        }).sum::<f64>() / nf;
        println!("| {name} (k={k}) | {n} | {:.1} | {:.3} | {:.3} | {:.1} | {:.1}% | {:.1}% |", (lp / nf).exp(), e / nf, d / nf, len as f64 / nf, 100.0 * novel / nf, 100.0 * shared);
    }
    println!("\n{shown}");
}
