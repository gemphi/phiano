//! `learner` — the phasors trained by their error, against the counts, on
//! sentences nobody was told (docs/203).
//!
//! The guide is cut three ways: one sentence in ten is the TEST set, which
//! no model is told or tuned on; one in twenty is the DEV set the learner's
//! epochs are stopped on; the rest is told to everyone. Every model is
//! scored at the test set's positions (each word after a line's first):
//! top-1, top-5, and for the two that give a probability, perplexity over
//! the words they were told (the test words nobody was told are a miss for
//! every model, and are counted as such).
//!
//! `--more a.txt,b.txt` tells every model more English beside the guide's
//! training sentences, cut the same way. The dev and test sets stay the
//! guide's own: the extra text is never scored.
//!
//! | model | what it is |
//! |---|---|
//! | the most told word | the floor |
//! | phient counts | the word before, counted (the arena's line to beat) |
//! | Witten–Bell trigram | the best of counting: two words back, smoothed |
//! | phiecs phasors | the arena's phasor reader: angles moved to the mean, never corrected |
//! | phient learner | the same kind of angles, corrected by their error (`phient::lineup::learner`) |
//!
//! ```text
//! cargo run --release -p phiano --example learner -- crates/phiano/data/person/murphy.txt [--channels 64] [--context 4] [--epochs 16] [--block 1] [--tie] [--dropout 0.1] [--silence 0] [--negatives 128] [--rate 0.5] [--no-pair] [--no-ecs] [--more extra.txt,…]
//! ```

mod counting;

use counting::{rank, sentences, WittenBell};
use phient::study::Score;
use phiano::arena::Ecs;
use phient::lineup::learner::{Config, Learner};
use phient::lineup::Count;
use phient::prose::tokens;
use phient::study::Reader;
use std::time::Instant;
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let Some(path) = a.first().filter(|p| !p.starts_with("--")).cloned() else {
        eprintln!("which corpus?");
        return;
    };
    let flag = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).cloned();
    let num = |k: &str, d: f64| flag(k).and_then(|s| s.parse().ok()).unwrap_or(d);
    let config = Config {
        channels: num("--channels", 64.0) as usize,
        context: num("--context", 4.0) as usize,
        block: num("--block", 1.0) as usize,
        pair: !a.iter().any(|x| x == "--no-pair"),
        tie: a.iter().any(|x| x == "--tie"),
        dropout: num("--dropout", 0.1) as f32,
        silence: num("--silence", 0.0) as f32,
        negatives: num("--negatives", 128.0) as usize,
        rate: num("--rate", 0.5) as f32,
        epochs: num("--epochs", 16.0) as usize,
        seed: num("--seed", 7.0) as u64,
    };

    let lines = sentences(&path);
    let (mut train, mut dev, mut test) = (Vec::new(), Vec::new(), Vec::new());
    for (i, l) in lines.into_iter().enumerate() {
        match (i % 10, i % 20) {
            (0, _) => test.push(l),
            (_, 5) => dev.push(l),
            _ => train.push(l),
        }
    }
    let words = |ls: &[Vec<String>]| ls.iter().map(Vec::len).sum::<usize>();
    let guide_told = train.len();
    for extra in flag("--more").iter().flat_map(|m| m.split(',')).filter(|m| !m.is_empty()) {
        train.extend(sentences(extra));
    }
    println!("told {} sentences ({} words; {guide_told} of them the guide's) · dev {} · test {} ({} positions)\n", train.len(), words(&train), dev.len(), test.len(), words(&test) - test.len());

    let mut rows: Vec<String> = Vec::new();
    let row = |name: &str, h: &Score, ppl: Option<f64>| {
        format!("| {name} | {:.1}% | {:.1}% | {} |", h.top1(), h.top5(), ppl.map_or("—".into(), |p| format!("{p:.1}")))
    };

    // The counters.
    let (mut count, mut wb) = (Count::new(), WittenBell::default());
    for l in &train {
        count.tell(l);
        wb.tell(l);
    }
    let mut common: Vec<(&String, &u64)> = wb.uni.iter().collect();
    common.sort_by(|x, y| y.1.cmp(x.1).then_with(|| x.0.cmp(y.0)));
    let common: Vec<String> = common.into_iter().take(10).map(|(w, _)| w.clone()).collect();
    let (mut floor, mut counted, mut smooth) = (Score::default(), Score::default(), Score::default());
    let (mut nll, mut known) = (0.0f64, 0usize);
    for l in &test {
        for j in 1..l.len() {
            floor.rank(rank(&common[..5], &l[j]));
            let mut after = count.choices(&l[j - 1]);
            after.sort_by(|x, y| y.1.cmp(&x.1).then_with(|| x.0.cmp(y.0)));
            counted.rank(rank(&after.into_iter().take(5).map(|(w, _)| w.clone()).collect::<Vec<_>>(), &l[j]));
            let h2 = if j >= 2 { l[j - 2].as_str() } else { "<s>" };
            smooth.rank(rank(&wb.top5(h2, &l[j - 1], &common), &l[j]));
            if wb.uni.contains_key(&l[j]) {
                nll -= wb.p(h2, &l[j - 1], &l[j]).ln();
                known += 1;
            }
        }
    }
    rows.push(row("the most told word (floor)", &floor, None));
    rows.push(row("phient counts (the word before)", &counted, None));
    rows.push(row("Witten–Bell trigram (counting, smoothed)", &smooth, Some((nll / known.max(1) as f64).exp())));

    // The phasors as they were: moved to the mean, never corrected.
    if !a.iter().any(|x| x == "--no-ecs") {
        let t = Instant::now();
        let mut ecs = Ecs::new();
        for l in &train {
            ecs.tell(l);
        }
        let mut h = Score::default();
        for l in &test {
            for j in 1..l.len() {
                let r: Vec<String> = ecs.ranked(&l[..j]).into_iter().take(5).map(|(w, _)| w).collect();
                h.rank(rank(&r, &l[j]));
            }
        }
        rows.push(row("phiecs phasors (counted: running mean)", &h, None));
        eprintln!("phiecs: {:.1}s", t.elapsed().as_secs_f64());
    }

    // The phasors corrected by their error.
    let t = Instant::now();
    let mut l = Learner::new(config.clone());
    for s in &train {
        l.tell(s);
    }
    let r = l.measure(&dev);
    println!("{config:?}\nvocabulary {}\n", l.vocabulary());
    println!("| epoch | error (train) | dev top-1 | dev perplexity | seconds |\n|---|---|---|---|---|");
    println!("| told once (online) | — | {:.1}% | {:.1} | {:.1} |", r.top1(), r.perplexity(), t.elapsed().as_secs_f64());
    let (mut best, mut best_ppl, mut since) = (l.clone(), r.perplexity(), 0);
    for e in 1..=config.epochs {
        let t = Instant::now();
        let loss = l.epoch();
        let r = l.measure(&dev);
        println!("| {e} | {loss:.3} | {:.1}% | {:.1} | {:.1} |", r.top1(), r.perplexity(), t.elapsed().as_secs_f64());
        match r.perplexity() < best_ppl {
            true => {
                (best, best_ppl, since) = (l.clone(), r.perplexity(), 0);
            }
            false => {
                since += 1;
                if since >= 2 {
                    break;
                }
            }
        }
    }
    let r = best.measure(&test);
    rows.push(row("**phient learner (corrected by its error)**", &r, Some(r.perplexity())));

    println!("\n| model, on the held-out test sentences | top-1 | top-5 | perplexity |\n|---|---|---|---|");
    for r in &rows {
        println!("{r}");
    }

    println!("\n| asked | the learner goes on | the counts go on |\n|---|---|---|");
    for q in ["i have been waiting", "if i had known", "when do we use the present perfect", "she has been working", "i wish i", "the man who lives next door", "we are going to"] {
        let said = tokens(q);
        let mine = best.speak(&said, 12).join(" ");
        let mut chain: Vec<String> = said.clone();
        while chain.len() < said.len() + 12 {
            let Some(next) = count.after(chain.last().map(String::as_str).unwrap_or("")).cloned() else { break };
            let last = chain.last().cloned().unwrap_or_default();
            if chain.windows(2).any(|p| p[0] == last && p[1] == next) {
                break;
            }
            chain.push(next);
        }
        println!("| {q} | {mine} | {} |", chain[said.len()..].join(" "));
    }
}
