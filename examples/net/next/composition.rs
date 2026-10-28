#[allow(unused_imports)]
use super::*;

/// Section 3: recursive composition, unbinding through the operator.
pub(super) fn recursive_composition(op: &Operator, epochs: usize) {
    let Operator { space, pool, train_sent, .. } = op;
    println!("\n=== 3. recursive composition: unbind through the operator ===");
    let comp = Composition::from_corpus(space, train_sent, pool);
    let n = comp.words.len();
    let (train_w, test_w) = comp.words.split_at(n * 4 / 5);
    let chance = {
        let m = n as f64;
        (1..=n).map(|k| 1.0 / k as f64).sum::<f64>() / m
    };
    println!(
        "  {} composable words ({} roles each), {} to train the cleanup, {} held out; chance MRR {:.4}",
        n,
        phient::net::compose::ROLES,
        train_w.len(),
        test_w.len(),
        chance
    );
    let cpool: Vec<usize> = comp.words.clone();
    let ccfg = FnoConfig { space: phiano::config::PHASE_CHANNELS, width: 8, layers: 2, modes: 16, margin: 0.3, lr: 3e-3, tie: phical::Tie::Circle };
    // Two cleanups: one that has only seen level 1, and one trained on
    // levels 1 and 2 together. Level 3 is unseen by both.
    let levels: Vec<(Vec<Example>, FunctionSpace)> = (1..=2)
        .map(|l| {
            let (pairs, targets) = comp.cleanup_examples(space, train_w, l);
            (pairs.iter().map(|(q, f)| Example { input: q.clone(), target: *f }).collect(), targets)
        })
        .collect();
    let mut cleanup_l1 = Fno::new(ccfg, 11);
    let mut cleanup_mix = Fno::new(ccfg, 11);
    let (mut a1, mut a2) = (Adam::new(ccfg.lr), Adam::new(ccfg.lr));
    let t0 = std::time::Instant::now();
    let mut last = (0.0, 0.0);
    for e in 0..(epochs * 3) {
        last.0 = cleanup_l1.train_epoch(&levels[0].1.funcs, &levels[0].0, &cpool, 32, &mut a1, 500 + e as u64);
        for (ex, sp) in &levels {
            last.1 = cleanup_mix.train_epoch(&sp.funcs, ex, &cpool, 32, &mut a2, 700 + e as u64);
        }
    }
    println!(
        "  cleanups trained in {:.0}s (final loss: level-1 only {:.3}, levels 1+2 {:.3})",
        t0.elapsed().as_secs_f64(),
        last.0,
        last.1
    );
    println!("  {:<6} {:>20} {:>22} {:>22}", "level", "phase unbind", "+ cleanup (level 1)", "+ cleanup (levels 1+2)");
    println!(
        "  {:<6} {:>9} {:>10} {:>10} {:>11} {:>10} {:>11}",
        "", "MRR", "hit@1", "MRR", "hit@1", "MRR", "hit@1"
    );
    for level in 1..=3 {
        let (m0, h0) = comp.recover(space, level, test_w, None);
        let (m1, h1) = comp.recover(space, level, test_w, Some(&mut cleanup_l1));
        let (m2, h2) = comp.recover(space, level, test_w, Some(&mut cleanup_mix));
        println!(
            "  {:<6} {:>9.4} {:>9.1}% {:>10.4} {:>10.1}% {:>10.4} {:>10.1}%   ({:.0}x / {:.0}x / {:.0}x chance)",
            level,
            m0,
            h0 * 100.0,
            m1,
            h1 * 100.0,
            m2,
            h2 * 100.0,
            m0 / chance,
            m1 / chance,
            m2 / chance
        );
    }
}
