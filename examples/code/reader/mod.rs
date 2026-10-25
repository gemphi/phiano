//! A reader of code as a sequence (docs/109): the next Rust token from the
//! tokens before it. docs/108 found that code read as name-level relations
//! is predicted no better than by frequency; here the condition of each
//! token is its place in the sequence — the squares reader's lesson (paper
//! 20) applied to code. Files are split four to one by a hash of the path;
//! a fifth of the training files is a development set, where the movement's
//! weight α and window K are chosen; the test files are read once.
//!
//! In Phient's world, a word (w) is a seed: what it relates to, under what
//! conditions (c). Here c is "d tokens before".
//!
//! docs/110 adds the calibration: a weight per condition kind, learned on
//! dev, and three more readers to measure against (4-gram, the cache reader
//! of Tu et al., and linear mixing).
//!
//! `cargo run --release --bin code_reader -- <dir>… [--sample 20000] [--epochs 3] [--base 3] [--split 0] [--top R] [--by-dir] [--also d1,d2]`

mod calibrate;
mod lex;
mod models;
mod own;
mod score;

use calibrate::{cache, mix, Weighting};
use models::CountModel;
use score::Tally;
use std::collections::HashMap;

pub fn run(a: Vec<String>) {
    let mut a = a;
    let sample: usize = score::flag(&mut a, "--sample").and_then(|s| s.parse().ok()).unwrap_or(20000);
    let epochs: usize = score::flag(&mut a, "--epochs").and_then(|s| s.parse().ok()).unwrap_or(3);
    // `--base 4`: the n-gram under the movement; `--split s`: another split of the files.
    let base: usize = score::flag(&mut a, "--base").and_then(|s| s.parse().ok()).unwrap_or(3);
    let top: usize = score::flag(&mut a, "--top").and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);
    // `--own N`: conditions seen N times earn their own weight (docs/114).
    let own_n: f64 = score::flag(&mut a, "--own").and_then(|s| s.parse().ok()).unwrap_or(64.0);
    let salt: u64 = score::flag(&mut a, "--split").and_then(|s| s.parse().ok()).unwrap_or(0);
    // `--by-dir`: split by a file's folder (a family of Phi-4's tasks, docs/117);
    // `--also d1,d2`: folders that only ever train.
    let by_dir = a.iter().position(|x| x == "--by-dir").map(|i| a.remove(i)).is_some();
    let also: Vec<String> = score::flag(&mut a, "--also").map(|s| s.split(',').map(String::from).collect()).unwrap_or_default();
    let mut files = Vec::new();
    for d in &a {
        lex::files(std::path::Path::new(d), &mut files);
    }
    let mut extra = Vec::new();
    for d in &also {
        lex::files(std::path::Path::new(d), &mut extra);
    }
    let train_only: std::collections::HashSet<String> = extra.iter().map(|(p, _)| p.clone()).collect();
    files.extend(extra);
    let key = |p: &str| if by_dir { std::path::Path::new(p).parent().map_or(p.to_string(), |d| d.to_string_lossy().into_owned()) } else { p.to_string() };
    let part = |p: &str| if train_only.contains(p) { 24 } else { phistd::mix64(phiano::phasor::fnv1a(&key(p)) ^ salt) % 25 }; // 0–4 test, 5–9 dev, rest train
    let enc_vocab = |train: &[&(String, Vec<String>)]| {
        let mut freq: HashMap<&str, usize> = HashMap::new();
        train.iter().for_each(|(_, t)| t.iter().for_each(|x| *freq.entry(x.as_str()).or_default() += 1));
        let mut v: Vec<String> = freq.iter().filter(|(_, &c)| c >= 2).map(|(w, _)| w.to_string()).collect();
        v.sort();
        v.insert(0, "⟨unk⟩".into());
        v
    };
    const K: usize = 8;
    const CACHE: usize = 1000;
    // Development: counts from the core; weights and λs fitted on dev.
    let core: Vec<&(String, Vec<String>)> = files.iter().filter(|(p, _)| part(p) >= 10).collect();
    let dev: Vec<&(String, Vec<String>)> = files.iter().filter(|(p, _)| (5..10).contains(&part(p))).collect();
    let mut w = Weighting::new(K, 0.35);
    let mut own = own::Own { seen: own_n, delta: HashMap::new() };
    let (l_lin, l_cache, l_both);
    {
        let vocab = enc_vocab(&core);
        let (train, test) = (score::encode(&vocab, &core), score::encode(&vocab, &dev));
        let mut c = CountModel::new(&train, vocab.len(), K);
        let uni = c.unigram();
        let posts = score::posts(&c, &uni);
        c.prune(top);
        let pos = score::positions(&test, 8, sample / 2);
        let mut g2 = vec![0.0; w.a.len() + 1];
        for e in 0..epochs {
            let ll: f64 = pos.iter().map(|&(h, t)| w.step(&c.ngram(h, base, &uni), &c.conditions(h, K, &posts), &uni, t, &mut g2, 0.05)).sum();
            eprintln!("epoch {e}: dev perplexity {:.2}", (-ll / pos.len() as f64).exp());
        }
        for e in 0..epochs {
            let ll: f64 = pos.iter().map(|&(h, t)| own.step(&w, &c.ngram(h, base, &uni), h, &c.conditions(h, K, &posts), &vocab, &uni, t, 0.02)).sum();
            eprintln!("own epoch {e}: dev perplexity {:.2}", (-ll / pos.len() as f64).exp());
        }
        let (mut lin, mut cac, mut both) = (Vec::new(), Vec::new(), Vec::new());
        for &(h, t) in &pos {
            let tri = c.ngram(h, base, &uni);
            let conds = c.conditions(h, K, &posts);
            let cc = cache(h, CACHE, vocab.len())[t];
            lin.push((tri[t], c.movement(h, K, &uni, &uni, 1.0, &posts)[t]));
            cac.push((if base == 3 { tri[t] } else { c.ngram(h, 3, &uni)[t] }, cc));
            both.push((w.score(&tri, &conds, &uni)[t], cc));
        }
        (l_lin, l_cache, l_both) = (calibrate::best_lambda(&lin), calibrate::best_lambda(&cac), calibrate::best_lambda(&both));
    }
    // Test: counts from core + dev, the test files read once.
    let train_files: Vec<&(String, Vec<String>)> = files.iter().filter(|(p, _)| part(p) >= 5).collect();
    let test_files: Vec<&(String, Vec<String>)> = files.iter().filter(|(p, _)| part(p) < 5).collect();
    let vocab = enc_vocab(&train_files);
    let (train, test) = (score::encode(&vocab, &train_files), score::encode(&vocab, &test_files));
    let mut c = CountModel::new(&train, vocab.len(), K);
    let uni = c.unigram();
    let posts = score::posts(&c, &uni);
    c.prune(top);
    let names = [
        "unigram (frequency)".to_string(),
        "bigram (discounted)".into(),
        "trigram (discounted)".into(),
        "4-gram (discounted)".into(),
        format!("trigram + cache of the file (Tu et al. 2014), λ = {l_cache}"),
        format!("Phient movement, α = 1, K = {K}"),
        "trigram ⊕ movement, α = 0.35, K = 4 (docs/109)".into(),
        format!("linear mix: {l_lin} · {base}-gram + {:.2} · movement", 1.0 - l_lin),
        format!("{base}-gram ⊕ movement, weights learned per (d, sightings), K = {K}"),
        format!("the same + cache, λ = {l_both}"),
        format!("{base}-gram ⊕ movement, two speeds: own weight after {own_n} sightings (docs/114)"),
    ];
    let mut ts: Vec<Tally> = names.iter().map(|n| Tally::new(n)).collect();
    let mut ids: Vec<Tally> = names.iter().map(|n| Tally::new(n)).collect();
    for (h, truth) in score::positions(&test, 8, sample) {
        let tri = c.ngram(h, 3, &uni);
        let conds = c.conditions(h, K, &posts);
        let cc = cache(h, CACHE, vocab.len());
        let nb = if base == 3 { tri.clone() } else { c.ngram(h, base, &uni) };
        let learned = w.score(&nb, &conds, &uni);
        let ps = [
            uni.clone(),
            c.ngram(h, 2, &uni),
            tri.clone(),
            c.ngram(h, 4, &uni),
            mix(&tri, &cc, l_cache),
            c.movement(h, K, &uni, &uni, 1.0, &posts),
            c.movement(h, 4, &uni, &tri, 0.35, &posts),
            mix(&nb, &c.movement(h, K, &uni, &uni, 1.0, &posts), l_lin),
            learned.clone(),
            mix(&learned, &cc, l_both),
            w.score_with(&nb, &conds, &own.extras(h, &conds, &vocab), &uni),
        ];
        let ident = score::is_identifier(&vocab[truth]);
        for (i, p) in ps.iter().enumerate() {
            ts[i].add(p, truth);
            if ident {
                ids[i].add(p, truth);
            }
        }
    }
    println!(
        "# Reading Rust as a sequence, calibrated\n\n{} files: {} train (of which {} core + {} dev), {} test · {} train tokens · vocabulary {} · {} test positions, {} of them identifiers · weights fitted on dev in {epochs} epochs · base {base}-gram · split {salt} · relations per condition ≤ {}\n",
        files.len(),
        train_files.len(),
        core.len(),
        dev.len(),
        test_files.len(),
        c.total,
        vocab.len(),
        ts[0].n,
        ids[0].n,
        if top == usize::MAX { "all".to_string() } else { top.to_string() }
    );
    println!("## All tokens\n\n| reader | top-1 | top-5 | MRR | perplexity |\n|---|---|---|---|---|");
    ts.iter().for_each(|t| println!("{}", t.row()));
    println!("\n## Identifiers only (names, not keywords or punctuation)\n\n| reader | top-1 | top-5 | MRR | perplexity |\n|---|---|---|---|---|");
    ids.iter().for_each(|t| println!("{}", t.row()));
    let rel: usize = posts.keys().map(|k| c.skip[k].len()).sum();
    let ctx = |n: usize| -> usize { c.ctx.iter().filter(|(h, _)| h.len() == n).map(|(_, m)| m.len()).sum() };
    println!("\n## Size\n\nRelations kept by the movement (posted (d, w) ⇒ next, with counts): {rel} across {} posted conditions, {:.1} per word per distance · n-gram entries: bigram {}, trigram {}, 4-gram {}", posts.len(), posts.len() as f64 / (vocab.len() * K) as f64, ctx(1), ctx(2), ctx(3));
    println!("\n## Two speeds\n\n{} conditions earned their own weight; the most moved: {}", own.delta.len(), own.moved(16));
    println!("\n## The learned weights α[d, sightings]\n\n{}", w.table(K));
}
