#[allow(unused_imports)]
use super::*;

pub(super) fn show_sweep(label: &str, sweep: &[(f64, f64)]) {
    let alone = sweep.iter().find(|(g, _)| (*g - 1.0).abs() < 1e-9).map(|x| x.1).unwrap_or(f64::NAN);
    let (bg, best) = sweep
        .iter()
        .cloned()
        .fold((0.0, f64::INFINITY), |a, b| if b.1 < a.1 { b } else { a });
    let off = sweep.first().map(|x| x.1).unwrap_or(f64::NAN);
    println!(
        "  {:<44} alone (γ=1) {:>8.2}   best mix {:>8.2} at γ={:.1}   no phase {:>8.2}",
        label, alone, best, bg, off
    );
}

/// Trains with early stopping on `data.valid`, returning the best operator and its MRR.
pub(super) fn train_lm(cfg: FnoConfig, seed: u64, data: &LmData, sampler: Sampler, epochs: usize) -> (Fno, f64, String) {
    let LmData { space, train, valid, pool } = *data;
    let valid_in: Vec<WordFunction> = valid.iter().map(|e| e.input.clone()).collect();
    let mut fno = Fno::new(cfg, seed);
    let mut adam = Adam::new(cfg.lr);
    let mut log = Vec::new();
    let (mut best_mrr, mut best_bytes) = (f64::NEG_INFINITY, fno.to_bytes());
    for e in 0..epochs {
        let l = fno.train_epoch_with(&space.funcs, train, sampler, BATCH, &mut adam, e as u64);
        let (m, _) = rank(&space.funcs, &fno.apply(&valid_in), valid, pool);
        log.push(format!("{l:.3}/{m:.4}"));
        if m > best_mrr {
            best_mrr = m;
            best_bytes = fno.to_bytes();
        }
    }
    (Fno::from_bytes(&best_bytes).expect("bytes"), best_mrr, log.join(" → "))
}

impl CloneApply for Fno {
    fn clone_apply(&self, inputs: &[WordFunction]) -> Vec<WordFunction> {
        Fno::from_bytes(&self.to_bytes()).expect("bytes").apply(inputs)
    }
}
