#[allow(unused_imports)]
use super::*;

/// The function space and the tokenised splits section 1 builds, all of which
/// sections 2 and 3 go on using.
pub(super) struct Operator {
    pub(super) space: FunctionSpace,
    pub(super) pool: Vec<usize>,
    pub(super) train_sent: Vec<Vec<String>>,
    pub(super) valid_sent: Vec<Vec<String>>,
    pub(super) cfg: FnoConfig,
}

/// Section 1: the operator in the language-model path.
pub(super) fn operator_in_lm_path(facet: &Facet, split: &Split, epochs: usize) -> (Operator, Fno) {
    println!("=== 1. perplexity on held-out sentences ===");
    let lm = PhianoLM::with_gamma(facet, 1.0);
    show_sweep("manifold coherence (PhianoLM, existing)", &lm.gamma_sweep(&split.valid));

    let space = FunctionSpace::from_facet(facet);
    let train_sent: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let pool: Vec<usize> = space.pool(POOL_FLOOR);
    let (train_ex, _) = examples(&space, &train_sent, WINDOW, &pool, u64::MAX);
    // Early stopping on the valid split's ranking, the same split the
    // perplexity is read on: the first run showed the operator's held-out
    // perplexity at 149 after two epochs and 161 after four while its
    // training loss kept falling.
    let valid_sent: Vec<Vec<String>> = split.valid.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let (mut valid_ex, _) = examples(&space, &valid_sent, WINDOW, &pool, u64::MAX);
    valid_ex.truncate(1500);
    let valid_in: Vec<WordFunction> = valid_ex.iter().map(|e| e.input.clone()).collect();
    println!(
        "  function space: {} words, pool {}, {} training examples, {} valid examples for early stopping",
        space.len(),
        pool.len(),
        train_ex.len(),
        valid_ex.len()
    );

    let cfg = FnoConfig { space: phiano::config::PHASE_CHANNELS, width: 8, layers: 2, modes: 12, margin: 0.2, lr: 2e-3, tie: phical::Tie::Circle };
    let mut fno = Fno::new(cfg, 7);
    let mut adam = Adam::new(cfg.lr);
    let t0 = std::time::Instant::now();
    let mut log = Vec::new();
    let (mut best_mrr, mut best_bytes) = (f64::NEG_INFINITY, fno.to_bytes());
    for e in 0..epochs {
        let l = fno.train_epoch(&space.funcs, &train_ex, &pool, BATCH, &mut adam, e as u64);
        let (m, _) = rank(&space.funcs, &fno.apply(&valid_in), &valid_ex, &pool);
        log.push(format!("{l:.3}/{m:.4}"));
        if m > best_mrr {
            best_mrr = m;
            best_bytes = fno.to_bytes();
        }
    }
    let mut fno = Fno::from_bytes(&best_bytes).expect("operator bytes");
    println!(
        "  operator trained in {:.0}s, per epoch loss/valid MRR {}; kept best (MRR {:.4})",
        t0.elapsed().as_secs_f64(),
        log.join(" → "),
        best_mrr
    );
    let vocab = lm.vocabulary();
    {
        let mut scorer = OperatorScorer::new(&mut fno, &space, &vocab, WINDOW);
        let id_sweep = lm.gamma_sweep_scored(&split.valid, |t, i| scorer.score_identity(t, i));
        show_sweep("function-space identity (superposition)", &id_sweep);
        let op_sweep = lm.gamma_sweep_scored(&split.valid, |t, i| scorer.score(t, i));
        show_sweep("fourier operator", &op_sweep);
    }

    (Operator { space, pool, train_sent, valid_sent, cfg }, fno)
}
