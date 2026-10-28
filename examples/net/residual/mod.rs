//! The residual loss, and depth as a curriculum.
//! `cargo run --release -p phiano --example net -- residual [corpus] [epochs]`
//!
//! 1. **Train on what is measured.** Every LM operator so far was trained by
//!    hinge against a decoy and then *read* through `softmax(β·cos)` mixed with
//!    a unigram under the trigram affine. The mixture kept preferring γ = 0.
//!    `Fno::train_epoch_mixture` trains the operator on that mixture's own
//!    negative log-likelihood, with each position's affine and unigram held
//!    constant, so the gradient reaches the operator only through the
//!    probability mass the n-gram components fail to place on the true word.
//!    Same architecture, same data, same reader; a different loss.
//!
//! 2. **Depth as a curriculum.** Levels 1..K added one at a time as training
//!    proceeds, against the same total epochs with all three from the start.

mod types;
mod sweep;
#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use sweep::*;

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phient::metrics::harness::{Harness, PhianoLM};
use phient::metrics::baseline::kn::KneserNey;
use phient::net::compose::Composition;
use phient::net::lm::OperatorScorer;
use phient::net::space::{examples, Example, FunctionSpace, WordFunction};
use phistd::Adam;
use phical::{Fno, FnoConfig};
use phiano::tokenizer::Tokenizer;
use phient::train::trainer::Trainer;
use phical::{rank, MixtureExample};

pub fn run(a: Vec<String>) {
    let args: Vec<String> = std::iter::once(String::from("example")).chain(a).collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let epochs: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(5);

    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 3)
        .collect();
    let split = Harness::split(corpus, 42);
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let kn = KneserNey::train(&split.train);
    println!(
        "corpus: {path}: {} train / {} valid sentences, {} words, {} epochs; Kneser-Ney {:.2}",
        split.train.len(),
        split.valid.len(),
        facet.vocabulary_size(),
        epochs,
        kn.perplexity(&split.valid)
    );

    let lm = PhianoLM::with_gamma(&facet, 1.0);
    let vocab = lm.vocabulary();
    let space = FunctionSpace::from_facet(&facet);
    assert_eq!(vocab, space.words, "LM index and function space must align");
    let train_sent: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let valid_sent: Vec<Vec<String>> = split.valid.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let pool: Vec<usize> = space.pool(POOL_FLOOR);
    let cfg = FnoConfig { space: PHASE_CHANNELS, width: 8, layers: 2, modes: 12, margin: 0.2, lr: 2e-3, tie: phical::Tie::Circle };

    // ── 1. the residual loss ────────────────────────────────────────────
    println!("\n=== 1. hinge against a decoy vs the mixture likelihood ===");
    show_sweep("manifold coherence (existing)", &lm.gamma_sweep(&split.valid));

    // Mixture examples: every position i ≥ 2 of every training sentence, the
    // context as a function, the target, and the n-gram constants there.
    let mut mix_ex: Vec<MixtureExample> = Vec::new();
    for toks in &train_sent {
        if toks.len() < 3 {
            continue;
        }
        let ids: Vec<usize> = toks.iter().filter_map(|w| space.index.get(w).copied()).collect();
        if ids.len() != toks.len() {
            continue;
        }
        for i in 2..toks.len() {
            let (idx, k, coef, other) = match lm.mixture_terms(&toks[i - 2], &toks[i - 1], &toks[i]) {
                Some(t) => t,
                None => continue,
            };
            if let Some(input) = space.context(&ids, i, WINDOW) {
                mix_ex.push(MixtureExample { input, target: idx, k, coef, other });
            }
        }
    }
    let (train_ex, _) = examples(&space, &train_sent, WINDOW, &pool, u64::MAX);
    let (mut valid_ex, _) = examples(&space, &valid_sent, WINDOW, &pool, u64::MAX);
    valid_ex.truncate(1500);
    let valid_in: Vec<WordFunction> = valid_ex.iter().map(|e| e.input.clone()).collect();
    println!("  {} mixture positions, {} hinge examples", mix_ex.len(), train_ex.len());

    // Hinge, as before.
    let hinge = {
        let mut fno = Fno::new(cfg, 7);
        let mut adam = Adam::new(cfg.lr);
        let (mut best, mut bytes) = (f64::NEG_INFINITY, fno.to_bytes());
        for e in 0..epochs {
            fno.train_epoch(&space.funcs, &train_ex, &pool, 64, &mut adam, e as u64);
            let (m, _) = rank(&space.funcs, &fno.apply(&valid_in), &valid_ex, &pool);
            if m > best {
                best = m;
                bytes = fno.to_bytes();
            }
        }
        Fno::from_bytes(&bytes).unwrap()
    };
    let mut hinge = hinge;
    let sweep = {
        let mut scorer = OperatorScorer::new(&mut hinge, &space, &vocab, WINDOW);
        lm.gamma_sweep_scored(&split.valid, |t, i| scorer.score(t, i))
    };
    show_sweep("hinge against uniform decoys", &sweep);

    // Mixture likelihood at a few training γ, each early-stopped on its own
    // held-out perplexity at γ = 1 (the term alone) — the scorer's metric.
    for &gamma in &[1.0f64, 0.5, 0.2] {
        let t0 = std::time::Instant::now();
        let mut fno = Fno::new(cfg, 7);
        let mut adam = Adam::new(cfg.lr);
        let mut log = Vec::new();
        let (mut best_ppl, mut bytes) = (f64::INFINITY, fno.to_bytes());
        for e in 0..epochs {
            let l = fno.train_epoch_mixture(&space.funcs, &mix_ex, gamma, PhianoLM::BETA, BATCH, &mut adam, e as u64);
            let ppl = {
                let mut scorer = OperatorScorer::new(&mut fno, &space, &vocab, WINDOW);
                let sw = lm.gamma_sweep_scored(&split.valid, |t, i| scorer.score(t, i));
                sw.iter().find(|(g, _)| (*g - 1.0).abs() < 1e-9).map(|x| x.1).unwrap_or(f64::NAN)
            };
            log.push(format!("{:.1}/{ppl:.1}", l.exp()));
            if ppl < best_ppl {
                best_ppl = ppl;
                bytes = fno.to_bytes();
            }
        }
        let mut fno = Fno::from_bytes(&bytes).unwrap();
        println!(
            "  mixture γ_train={gamma}: {:.0}s, per epoch train-ppl/valid-alone {}",
            t0.elapsed().as_secs_f64(),
            log.join(" → ")
        );
        let sweep = {
            let mut scorer = OperatorScorer::new(&mut fno, &space, &vocab, WINDOW);
            lm.gamma_sweep_scored(&split.valid, |t, i| scorer.score(t, i))
        };
        show_sweep(&format!("mixture likelihood, γ_train = {gamma}"), &sweep);
        let (m, _) = rank(&space.funcs, &fno.apply(&valid_in), &valid_ex, &pool);
        println!("    (valid MRR on the frequent pool: {m:.4})");
        if (gamma - 0.5).abs() < 1e-9 {
            let _ = std::fs::write("data/net_residual.fno", fno.to_bytes());
        }
    }

    // ── 2. depth as a curriculum ────────────────────────────────────────
    println!("\n=== 2. depth as a curriculum ===");
    let comp = Composition::from_corpus(&space, &train_sent, &pool);
    let n = comp.words.len();
    let (train_w, test_w) = comp.words.split_at(n * 4 / 5);
    let chance = (1..=n).map(|k| 1.0 / k as f64).sum::<f64>() / n as f64;
    let cpool: Vec<usize> = comp.words.clone();
    let ccfg = FnoConfig { space: PHASE_CHANNELS, width: 8, layers: 2, modes: 16, margin: 0.3, lr: 3e-3, tie: phical::Tie::Circle };
    let level_sets: Vec<(Vec<Example>, FunctionSpace)> = (1..=3)
        .map(|l| {
            let (pairs, targets) = comp.cleanup_examples(&space, train_w, l);
            (pairs.iter().map(|(q, f)| Example { input: q.clone(), target: *f }).collect(), targets)
        })
        .collect();
    let total = epochs * 3;
    let mut all_at_once = Fno::new(ccfg, 11);
    let mut curriculum = Fno::new(ccfg, 11);
    let (mut a1, mut a2) = (Adam::new(ccfg.lr), Adam::new(ccfg.lr));
    for e in 0..total {
        for (ex, sp) in &level_sets {
            all_at_once.train_epoch(&sp.funcs, ex, &cpool, 32, &mut a1, 700 + e as u64);
        }
        // Curriculum: level 1 for the first third, levels 1–2 for the second,
        // all three for the last — the same number of epochs, spent on fewer
        // levels early.
        let k = 1 + (3 * e) / total;
        for (ex, sp) in level_sets.iter().take(k) {
            curriculum.train_epoch(&sp.funcs, ex, &cpool, 32, &mut a2, 800 + e as u64);
        }
    }
    println!("  {n} composable words, {total} epochs each; chance MRR {chance:.4}");
    println!("  {:<6} {:>18} {:>18}", "level", "all levels at once", "curriculum 1→2→3");
    for level in 1..=4 {
        let (m1, _) = comp.recover(&space, level, test_w, Some(&mut all_at_once));
        let (m2, _) = comp.recover(&space, level, test_w, Some(&mut curriculum));
        println!("  {:<6} {:>11.4} {:>5.0}x {:>11.4} {:>5.0}x", level, m1, m1 / chance, m2, m2 / chance);
    }
}
