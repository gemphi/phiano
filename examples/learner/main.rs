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
//! The learner ACCUMULATES (docs/208): `--save ckpt` writes the best epoch's
//! learner whole, and `--resume ckpt` starts the next run from it instead of
//! the seed, telling it only what is new. `--no-guide` tells it the extra
//! text alone, so a curriculum is two runs: the wide text first and saved,
//! then the guide on top of it. On a resume `--epochs` and `--rate` are this
//! run's; `--forget 0.1` scales the Adagrad accumulators so a sure learner
//! gets its stride back (docs/214). `--record name,lever,doc` appends the run
//! to the curriculum record and `--propose` reads the record and names the
//! next lever (`curriculum`, paper 28).
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
//! cargo run --release -p phiano --example learner -- crates/phiano/data/person/murphy.txt [--channels 64] [--context 4] [--epochs 16] [--block 1] [--tie] [--dropout 0.1] [--silence 0] [--negatives 128] [--rate 0.5] [--no-pair] [--no-ecs] [--more extra.txt,…] [--guide-times 1] [--no-guide] [--test-more held.txt,…] [--resume ckpt] [--widen 128] [--forget 0] [--save ckpt] [--record name,lever,doc] [--propose]
//! ```

mod counting;
mod curriculum;

use counting::{rank, sentences, WittenBell};
use phient::study::Score;
use phient::lineup::Ecs;
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
    // The curriculum read back and the next instalment proposed (paper 28).
    if a.iter().any(|x| x == "--propose") {
        print!("{}", curriculum::propose(&curriculum::read()));
        return;
    }
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
    if a.iter().any(|x| x == "--no-guide") {
        train.clear();
    }
    let guide_told = train.len();
    // The guide's share of the rehearsal (docs/212, 216): told `--guide-times`
    // times, so more text beside it does not dilute it.
    let times = num("--guide-times", 1.0) as usize;
    for _ in 1..times {
        train.extend(train[..guide_told].to_vec());
    }
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
            floor.rank(rank(&common[..common.len().min(5)], &l[j]));
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

    // The phasors corrected by their error: fresh, or carried on from a
    // checkpoint and told only what is new.
    let t = Instant::now();
    let mut l = match flag("--resume") {
        Some(path) => {
            let mut l = Learner::load(&path).expect("read the checkpoint");
            // The shape of the learner is the checkpoint's; how long this
            // instalment trains is this run's.
            if flag("--epochs").is_some() {
                l.config.epochs = config.epochs;
            }
            if flag("--rate").is_some() {
                l.config.rate = config.rate;
            }
            println!("resumed from {path}: {} lines told before, vocabulary {}\n", l.lines_told(), l.vocabulary());
            // More channels for a learner that has already learned (docs/210):
            // the old kept, the new born quiet, nothing learned lost.
            if let Some(wide) = flag("--widen").and_then(|s| s.parse().ok()) {
                let from = l.config.channels;
                match l.widen(wide) {
                    true => println!("widened from {from} to {wide} channels\n"),
                    false => println!("not widened: {wide} is not wider than {from}, or not a multiple of the block {}\n", l.config.block),
                }
            }
            // Its stride back (docs/214): the accumulators scaled, the
            // phasors kept, so a resumed learner can take real steps again.
            if let Some(share) = flag("--forget").and_then(|s| s.parse::<f32>().ok()) {
                l.forget(share);
                println!("forgot the stride: accumulators scaled by {share}
");
            }
            l
        }
        None => Learner::new(config.clone()),
    };
    let config = l.config.clone();
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
    // Held-out sentences of another register (docs/217): the same learner,
    // scored on text it was never told, to tell English learned from the
    // guide learned. The guide's test row above is the bar; these are beside it.
    let mut held = f64::NAN;
    for extra in flag("--test-more").iter().flat_map(|m| m.split(',')).filter(|m| !m.is_empty()) {
        let more = sentences(extra);
        let r = best.measure(&more);
        if held.is_nan() {
            held = r.perplexity();
        }
        let name = std::path::Path::new(extra).file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();
        rows.push(row(&format!("the learner, on {name} ({} sentences, never told)", more.len()), &r, Some(r.perplexity())));
    }
    if let Some(path) = flag("--save") {
        best.save(&path).expect("write the checkpoint");
        println!("\nsaved the best epoch to {path} ({} lines told, vocabulary {})", best.lines_told(), best.vocabulary());
    }
    // `--record name,lever,doc`: this instalment appended to the curriculum.
    if let Some(spec) = flag("--record") {
        let f: Vec<&str> = spec.split(',').collect();
        let stem = |p: &str| std::path::Path::new(p).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| p.to_string());
        let text: Vec<String> = flag("--more").iter().flat_map(|m| m.split(',')).filter(|m| !m.is_empty()).map(stem).collect();
        let instalment = curriculum::Instalment {
            name: f.first().map(|s| s.to_string()).unwrap_or_else(|| flag("--save").map(|p| stem(&p)).unwrap_or_default()),
            from: flag("--resume").map(|p| stem(&p)).unwrap_or_else(|| "-".into()),
            lever: f.get(1).map(|s| s.to_string()).unwrap_or_else(|| "?".into()),
            text: match text.is_empty() {
                true => "-".into(),
                false => text.join("+"),
            },
            guide_times: match a.iter().any(|x| x == "--no-guide") {
                true => 0,
                false => num("--guide-times", 1.0) as usize,
            },
            epochs: config.epochs,
            top1: r.top1(),
            top5: r.top5(),
            ppl: r.perplexity(),
            doc: f.get(2).map(|s| s.to_string()).unwrap_or_else(|| "-".into()),
            held,
        };
        match curriculum::record(&instalment) {
            Ok(true) => println!("recorded {} in {}", instalment.name, curriculum::file().display()),
            Ok(false) => println!("{} was already in {}", instalment.name, curriculum::file().display()),
            Err(e) => println!("could not record {}: {e}", instalment.name),
        }
    }

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
