#[allow(unused_imports)]
use super::*;

/// Repeat at further seeds and report the spread of the two headline
/// conditions. Only two, because each seed is a full retrain plus fifteen
/// compositions, and an interval on the numbers actually being claimed is
/// worth more than a wide, thin sweep.
pub(super) fn across_seeds(s: &Setup) {
    let runs = s.runs;
    if runs <= 1 {
        return;
    }
    let (split, entries, rounds) = (&s.split, &s.entries, s.rounds);
    println!("\n=== across {} seeds ===", runs);
    let mut base_mrr: Vec<f64> = Vec::new();
    let mut best_mrr: Vec<f64> = Vec::new();
    let mut base_pair: Vec<f64> = Vec::new();
    let mut best_pair: Vec<f64> = Vec::new();

    for r in 0..runs {
        let seed = s.base_seed.wrapping_add(r as u64);
        let t = Trainer::new(LEARNING_RATE).with_seed(seed);
        let f0 = Harness::train_ranking_only(split, &t, 4);
        let b = measure("baseline", &f0, &split.valid);

        let mut f1 = f0.clone();
        Conception::compose_graded(
            &mut f1,
            entries,
            rounds,
            HEAD_STEP,
            BETA_STRONG,
            BETA_STRONG,
            false,
            None,
        );
        let c = measure("composed", &f1, &split.valid);

        println!(
            "  seed {:<4} baseline MRR {:.4}  composed MRR {:.4}  (pair {:.1}% -> {:.1}%)",
            seed,
            b.analogy_mrr,
            c.analogy_mrr,
            b.pair_vs_random * 100.0,
            c.pair_vs_random * 100.0
        );
        base_mrr.push(b.analogy_mrr);
        best_mrr.push(c.analogy_mrr);
        base_pair.push(b.pair_vs_random);
        best_pair.push(c.pair_vs_random);
    }

    let (bm, cm) = (Stat::of(&base_mrr), Stat::of(&best_mrr));
    let (bp, cp) = (Stat::of(&base_pair), Stat::of(&best_pair));
    println!(
        "\n  analogy MRR   baseline {:.4} +/- {:.4}   composed {:.4} +/- {:.4}   n={}",
        bm.mean, bm.sd, cm.mean, cm.sd, cm.n
    );
    println!(
        "  pair/random   baseline {:.1}% +/- {:.1}   composed {:.1}% +/- {:.1}",
        bp.mean * 100.0,
        bp.sd * 100.0,
        cp.mean * 100.0,
        cp.sd * 100.0
    );
    println!(
        "\n  VERDICT: composition {} the baseline by more than the two spreads \
         combined on MRR, and {} on pair/random.",
        if cm.separated_from(&bm) { "clears" } else { "does NOT clear" },
        if cp.separated_from(&bp) { "clears" } else { "does not" }
    );
    println!(
        "  (Spread-sum separation is a crude test and is labelled as one: it is \n\
         \x20 not a t-test, and n={} is small.)",
        cm.n
    );
}
