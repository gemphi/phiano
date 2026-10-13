#[allow(unused_imports)]
use super::*;

/// Section 2: operator and lexicon trained together.
pub(super) fn joint_training(facet: &Facet, split: &Split, op: &Operator, fno: Fno, epochs: usize) {
    let Operator { space, pool, train_sent, valid_sent, cfg } = op;
    println!("\n=== 2. operator and lexicon trained together ===");
    // Every word is a target here, not only the frequent pool. The first
    // joint run trained targets from the pool alone: ranking inside the pool
    // improved (valid MRR 0.1425 → 0.1508) while perplexity over the whole
    // vocabulary got worse (149 → 168), because the 5,500 words outside the
    // pool moved only as context and drifted into the softmax's way.
    let all: Vec<usize> = (0..space.len()).collect();
    let (train_idx, _) = index_examples(space, train_sent, WINDOW, &all, u64::MAX);
    let mut joint = Joint::new(fno, &space.funcs, WINDOW).with_tether(JOINT_TETHER);
    let mut adam_op = Adam::new(cfg.lr);
    let mut adam_words = Adam::new(cfg.lr * 0.25);
    let t0 = std::time::Instant::now();
    let mut log = Vec::new();
    let mut best: Option<(f64, Vec<u8>, Vec<WordFunction>)> = None;
    for e in 0..epochs {
        let l = joint.train_epoch(&train_idx, &all, BATCH, &mut adam_op, &mut adam_words, 100 + e as u64);
        // Validate on the learned functions: contexts and targets both move.
        let funcs = joint.functions();
        let mut sp = FunctionSpace::from_facet(facet);
        sp.funcs = funcs.clone();
        let (vex, _) = examples(&sp, valid_sent, WINDOW, pool, u64::MAX);
        let vex: Vec<Example> = vex.into_iter().take(1500).collect();
        let vin: Vec<WordFunction> = vex.iter().map(|e| e.input.clone()).collect();
        let (m, _) = rank(&sp.funcs, &joint.fno.apply(&vin), &vex, pool);
        log.push(format!("{l:.3}/{m:.4}"));
        if best.as_ref().is_none_or(|b| m > b.0) {
            best = Some((m, joint.fno.to_bytes(), funcs));
        }
    }
    let (best_m, best_bytes, learned) = best.expect("at least one epoch");
    println!(
        "  joint trained in {:.0}s, per epoch loss/valid MRR {}; kept best (MRR {:.4})",
        t0.elapsed().as_secs_f64(),
        log.join(" → "),
        best_m
    );
    let mut facet2 = facet.clone();
    let written = write_back(&mut facet2, space, &learned);
    println!("  wrote {written} word functions back into a copy of the facet");
    let lm2 = PhianoLM::with_gamma(&facet2, 1.0);
    show_sweep("manifold coherence on written-back facet", &lm2.gamma_sweep(&split.valid));
    let space2 = FunctionSpace::from_facet(&facet2);
    drop(joint);
    let mut fno = Fno::from_bytes(&best_bytes).expect("joint operator bytes");
    {
        let vocab2 = lm2.vocabulary();
        let mut scorer = OperatorScorer::new(&mut fno, &space2, &vocab2, WINDOW);
        let id_sweep = lm2.gamma_sweep_scored(&split.valid, |t, i| scorer.score_identity(t, i));
        show_sweep("identity on learned functions", &id_sweep);
        let op_sweep = lm2.gamma_sweep_scored(&split.valid, |t, i| scorer.score(t, i));
        show_sweep("jointly trained operator", &op_sweep);
    }
    if std::fs::write("data/phinet_joint.fno", fno.to_bytes()).is_ok() {
        println!("  saved jointly trained operator to data/phinet_joint.fno");
    }
}
