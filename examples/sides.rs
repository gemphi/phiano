//! The perplexity harness on the two-sided lexicon: which reading?
//! `cargo run --release --bin phinet_sides [corpus] [epochs]`
//!
//! docs/70 measured next-word *ranking* under four readings and found the
//! directed one (source context → target table) best by far. This is the
//! same question asked of held-out *perplexity*: the identity reader, the
//! hinge operator and the mixture-likelihood operator, each on the source
//! reading and on the directed reading, mixed with the n-gram tables at
//! every γ.

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::metrics::harness::{Harness, PhianoLM};
use phiano::metrics::baseline::kn::KneserNey;
use phiano::phinet::lm::OperatorScorer;
use phiano::phinet::space::{examples, FunctionSpace, Sides, WordFunction};
use phiano::phinet::{Adam, Fno, FnoConfig};
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;
use phical::{rank, MixtureExample};

const POOL_FLOOR: u32 = 25;
const WINDOW: usize = 6;
const BATCH: usize = 32;

fn show_sweep(label: &str, sweep: &[(f64, f64)]) {
    let alone = sweep.iter().find(|(g, _)| (*g - 1.0).abs() < 1e-9).map(|x| x.1).unwrap_or(f64::NAN);
    let best = sweep.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).copied().unwrap_or((f64::NAN, f64::NAN));
    println!("  {label:<44} alone {alone:>8.2}   best {:.2} at γ = {}", best.1, best.0);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let epochs: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(3);

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
        split.train.len(), split.valid.len(), facet.vocabulary_size(), epochs, kn.perplexity(&split.valid)
    );
    let lm = PhianoLM::with_gamma(&facet, 1.0);
    let vocab = lm.vocabulary();
    show_sweep("manifold coherence (existing reader)", &lm.gamma_sweep(&split.valid));
    let train_sent: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let valid_sent: Vec<Vec<String>> = split.valid.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let cfg = FnoConfig { space: PHASE_CHANNELS, width: 8, layers: 2, modes: 12, margin: 0.2, lr: 2e-3, tie: phical::Tie::Circle };

    let only_directed = args.get(3).is_some_and(|a| a == "directed");
    for (name, sides) in [("source sides (as docs/65-68)", Sides::Source), ("directed (src ctx -> tgt table)", Sides::Directed)] {
        if only_directed && sides != Sides::Directed {
            continue;
        }
        println!("\n=== {name} ===");
        let space = FunctionSpace::from_facet_sides(&facet, sides);
        assert_eq!(vocab, space.words);
        let pool: Vec<usize> = space.pool(POOL_FLOOR);
        let (train_ex, _) = examples(&space, &train_sent, WINDOW, &pool, u64::MAX);
        let (mut valid_ex, _) = examples(&space, &valid_sent, WINDOW, &pool, u64::MAX);
        valid_ex.truncate(1500);
        let valid_in: Vec<WordFunction> = valid_ex.iter().map(|e| e.input.clone()).collect();

        // identity: no operator, the context function against the table
        {
            let mut dummy = Fno::new(cfg, 0);
            let scorer = OperatorScorer::new(&mut dummy, &space, &vocab, WINDOW);
            let sweep = lm.gamma_sweep_scored(&split.valid, |t, i| scorer.score_identity(t, i));
            let (m, _) = rank(space.table(), &valid_in, &valid_ex, &pool);
            show_sweep(&format!("identity (no operator), valid MRR {m:.4}"), &sweep);
        }

        // hinge operator, early-stopped on valid MRR
        let mut hinge = {
            let mut fno = Fno::new(cfg, 7);
            let mut adam = Adam::new(cfg.lr);
            let (mut best, mut bytes) = (f64::NEG_INFINITY, fno.to_bytes());
            for e in 0..epochs.max(3) {
                fno.train_epoch(space.table(), &train_ex, &pool, 64, &mut adam, e as u64);
                let (m, _) = rank(space.table(), &fno.apply(&valid_in), &valid_ex, &pool);
                if m > best { best = m; bytes = fno.to_bytes(); }
            }
            println!("  hinge operator valid MRR {best:.4}");
            Fno::from_bytes(&bytes).unwrap()
        };
        let sweep = {
            let mut scorer = OperatorScorer::new(&mut hinge, &space, &vocab, WINDOW);
            lm.gamma_sweep_scored(&split.valid, |t, i| scorer.score(t, i))
        };
        show_sweep("hinge operator", &sweep);

        // mixture likelihood, γ_train = 1
        let mut mix_ex: Vec<MixtureExample> = Vec::new();
        for toks in &train_sent {
            if toks.len() < 3 { continue; }
            let ids: Vec<usize> = toks.iter().filter_map(|w| space.index.get(w).copied()).collect();
            if ids.len() != toks.len() { continue; }
            for i in 2..toks.len() {
                if let Some((idx, k, coef, other)) = lm.mixture_terms(&toks[i - 2], &toks[i - 1], &toks[i]) {
                    if let Some(input) = space.context(&ids, i, WINDOW) {
                        mix_ex.push(MixtureExample { input, target: idx, k, coef, other });
                    }
                }
            }
        }
        let t0 = std::time::Instant::now();
        let mut fno = Fno::new(cfg, 7);
        let mut adam = Adam::new(cfg.lr);
        let (mut best_ppl, mut bytes) = (f64::INFINITY, fno.to_bytes());
        let mut log = Vec::new();
        for e in 0..epochs {
            let l = fno.train_epoch_mixture(space.table(), &mix_ex, 1.0, PhianoLM::BETA, BATCH, &mut adam, e as u64);
            let ppl = {
                let mut scorer = OperatorScorer::new(&mut fno, &space, &vocab, WINDOW);
                let sw = lm.gamma_sweep_scored(&split.valid, |t, i| scorer.score(t, i));
                sw.iter().find(|(g, _)| (*g - 1.0).abs() < 1e-9).map(|x| x.1).unwrap_or(f64::NAN)
            };
            log.push(format!("{:.1}/{ppl:.1}", l.exp()));
            if ppl < best_ppl { best_ppl = ppl; bytes = fno.to_bytes(); }
        }
        let mut fno = Fno::from_bytes(&bytes).unwrap();
        println!("  mixture γ_train=1: {:.0}s, per epoch train/valid {}", t0.elapsed().as_secs_f64(), log.join(" → "));
        let sweep = {
            let mut scorer = OperatorScorer::new(&mut fno, &space, &vocab, WINDOW);
            lm.gamma_sweep_scored(&split.valid, |t, i| scorer.score(t, i))
        };
        show_sweep("mixture-likelihood operator", &sweep);
        if sides == Sides::Directed {
            let _ = std::fs::write("data/phinet_generate.fno", fno.to_bytes());
            println!("  saved data/phinet_generate.fno (directed, mixture-trained)");
        }
    }
}
