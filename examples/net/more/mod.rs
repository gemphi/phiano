//! The three things `docs/66_phinet_in_the_loop.md` left open, measured.
//! `cargo run --release -p phiano --example net -- more [corpus] [epochs]`
//!
//! 1. **The mixture.** The operator's phase term (134) and the unigram (126)
//!    combined to 125.5 — they predict nearly the same thing. A uniform decoy
//!    teaches the operator to beat a random word; the unigram already beats a
//!    random word. Decoys drawn in proportion to word frequency teach it to
//!    beat the *likely* words instead, which is the only thing a term mixed
//!    with a unigram can add. Same operator, same data, one change.
//!
//! 2. **Depth three.** The cleanup recovered every level it was trained on and
//!    fell to chance one deeper. Train it on levels 1–3 and read levels 1–4.
//!
//! 3. **One operator.** The LM operator and the cleanup have the same
//!    architecture. Train one network on both objectives, alternating, and
//!    read both tasks against the two specialists.

mod types;
mod sweep;
#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use sweep::*;

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::metrics::harness::{Harness, PhianoLM};
use phiano::metrics::baseline::kn::KneserNey;
use phiano::phinet::compose::Composition;
use phiano::phinet::lm::OperatorScorer;
use phiano::phinet::space::{examples, Example, FunctionSpace, WordFunction};
use phiano::phinet::{Adam, Fno, FnoConfig};
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;
use phical::{rank, FrequencySampler};

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
    let train_sent: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let valid_sent: Vec<Vec<String>> = split.valid.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let pool: Vec<usize> = space.pool(POOL_FLOOR);
    let (train_ex, _) = examples(&space, &train_sent, WINDOW, &pool, u64::MAX);
    let (mut valid_ex, _) = examples(&space, &valid_sent, WINDOW, &pool, u64::MAX);
    valid_ex.truncate(1500);
    let cfg = FnoConfig { space: PHASE_CHANNELS, width: 8, layers: 2, modes: 12, margin: 0.2, lr: 2e-3, tie: phical::Tie::Circle };

    // ── 1. decoys the unigram would pick ────────────────────────────────
    println!("\n=== 1. what the operator is trained to beat ===");
    let uniform = |t: usize, r: u64| phistd::decoy(&pool, t, r);
    let counts: Vec<f64> = pool.iter().map(|&i| space.counts[i] as f64).collect();
    let by_freq = FrequencySampler::new(pool.clone(), &counts, 1.0);
    let by_freq_soft = FrequencySampler::new(pool.clone(), &counts, 0.75);
    let soft = |t: usize, r: u64| by_freq_soft.draw(t, r);
    let hard = |t: usize, r: u64| by_freq.draw(t, r);
    let arms: Vec<(&str, Sampler)> = vec![
        ("uniform decoys (as before)", &uniform),
        ("frequency^0.75 decoys", &soft),
        ("frequency decoys", &hard),
    ];
    let mut lm_ops: Vec<(String, Fno)> = Vec::new();
    for (name, sampler) in arms {
        let t0 = std::time::Instant::now();
        let (mut fno, mrr, log) = train_lm(cfg, 7, &LmData { space: &space, train: &train_ex, valid: &valid_ex, pool: &pool }, sampler, epochs);
        println!("  {name}: {:.0}s, loss/valid-MRR per epoch {log}; kept MRR {mrr:.4}", t0.elapsed().as_secs_f64());
        let sweep = {
            let mut scorer = OperatorScorer::new(&mut fno, &space, &vocab, WINDOW);
            lm.gamma_sweep_scored(&split.valid, |t, i| scorer.score(t, i))
        };
        show_sweep(name, &sweep);
        lm_ops.push((name.to_string(), fno));
    }

    // ── 2. depth three ──────────────────────────────────────────────────
    println!("\n=== 2. recursion depth ===");
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
    println!("  {n} composable words; cleanups trained on levels 1..=K for K = 1, 2, 3; chance MRR {chance:.4}");
    let mut cleanups: Vec<Fno> = Vec::new();
    for k in 1..=3 {
        let mut c = Fno::new(ccfg, 11);
        let mut a = Adam::new(ccfg.lr);
        for e in 0..(epochs * 3) {
            for (ex, sp) in level_sets.iter().take(k) {
                c.train_epoch(&sp.funcs, ex, &cpool, 32, &mut a, 700 + e as u64);
            }
        }
        cleanups.push(c);
    }
    println!("  {:<6} {:>16} {:>16} {:>16} {:>16}", "level", "phase only", "cleanup K=1", "cleanup K=2", "cleanup K=3");
    for level in 1..=4 {
        let (m0, _) = comp.recover(&space, level, test_w, None);
        let ms: Vec<f64> = cleanups.iter_mut().map(|c| comp.recover(&space, level, test_w, Some(c)).0).collect();
        println!(
            "  {:<6} {:>9.4} {:>5.0}x {:>9.4} {:>5.0}x {:>9.4} {:>5.0}x {:>9.4} {:>5.0}x",
            level,
            m0,
            m0 / chance,
            ms[0],
            ms[0] / chance,
            ms[1],
            ms[1] / chance,
            ms[2],
            ms[2] / chance
        );
    }

    // ── 3. one operator for both ────────────────────────────────────────
    println!("\n=== 3. one operator, both objectives ===");
    let valid_in: Vec<WordFunction> = valid_ex.iter().map(|e| e.input.clone()).collect();
    // Specialists: the best LM arm from section 1 and the K=3 cleanup.
    let (best_name, mut lm_spec) = lm_ops
        .into_iter()
        .max_by(|a, b| {
            let ma = rank(&space.funcs, &a.1.clone_apply(&valid_in), &valid_ex, &pool).0;
            let mb = rank(&space.funcs, &b.1.clone_apply(&valid_in), &valid_ex, &pool).0;
            ma.partial_cmp(&mb).unwrap()
        })
        .expect("arms");
    let (m_spec, _) = rank(&space.funcs, &lm_spec.apply(&valid_in), &valid_ex, &pool);
    let mut clean_spec = cleanups.pop().expect("K=3 cleanup");
    let c_spec: Vec<f64> = (1..=3).map(|l| comp.recover(&space, l, test_w, Some(&mut clean_spec)).0).collect();

    // The shared one: LM epochs with the best sampler, interleaved with the
    // three cleanup levels, same total epochs as each specialist.
    let mut shared = Fno::new(ccfg, 21);
    let mut a = Adam::new(ccfg.lr);
    let (mut best_m, mut best_bytes) = (f64::NEG_INFINITY, shared.to_bytes());
    for e in 0..epochs {
        shared.train_epoch_with(&space.funcs, &train_ex, &|t, r| by_freq_soft.draw(t, r), BATCH, &mut a, e as u64);
        for _ in 0..3 {
            for (ex, sp) in &level_sets {
                shared.train_epoch(&sp.funcs, ex, &cpool, 32, &mut a, 900 + e as u64);
            }
        }
        let (m, _) = rank(&space.funcs, &shared.apply(&valid_in), &valid_ex, &pool);
        if m > best_m {
            best_m = m;
            best_bytes = shared.to_bytes();
        }
    }
    let mut shared = Fno::from_bytes(&best_bytes).expect("bytes");
    let c_shared: Vec<f64> = (1..=3).map(|l| comp.recover(&space, l, test_w, Some(&mut shared)).0).collect();
    println!("  {:<28} {:>12} {:>12} {:>12} {:>12}", "", "LM valid MRR", "recover L1", "recover L2", "recover L3");
    println!(
        "  {:<28} {:>12.4} {:>12.4} {:>12.4} {:>12.4}",
        format!("specialists ({best_name})"),
        m_spec,
        c_spec[0],
        c_spec[1],
        c_spec[2]
    );
    println!("  {:<28} {:>12.4} {:>12.4} {:>12.4} {:>12.4}", "one shared operator", best_m, c_shared[0], c_shared[1], c_shared[2]);
    let sweep = {
        let mut scorer = OperatorScorer::new(&mut shared, &space, &vocab, WINDOW);
        lm.gamma_sweep_scored(&split.valid, |t, i| scorer.score(t, i))
    };
    show_sweep("one shared operator, perplexity", &sweep);
}
