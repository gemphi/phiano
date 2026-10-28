//! Raising the operator's own ranking on Austen — the ceiling docs/76 named.
//! Arms differ in one thing each: objective (hinge vs full-table softmax),
//! size (8k vs 50k parameters), optimiser (Adam vs AdamW + cosine schedule).
//! The best is saved as `data/austen_operator.fno` so `bin/decode_eval` reads
//! it.
//!
//! A softmax epoch over the 204k examples takes a quarter of an hour on two
//! cores, so the sweep is resumable: every quarter-epoch it checkpoints the
//! operator and the optimiser (`data/sweep/<arm>.ckpt`, format v2, moments
//! and step count included) and stops when its time budget is spent; the
//! next invocation continues from the checkpoint. Finished arms are rows in
//! `data/sweep/results.md` and are not re-run. The arithmetic is the same as
//! one uninterrupted run: an epoch's order is fixed by its seed, and the
//! quarters are consecutive slices of it.
//!
//! `cargo run --release --bin operator_sweep [epochs] [budget_s] [train_limit]`
mod arms;
mod checkpoint;
mod setup;
#[allow(unused_imports)]
use arms::*;
#[allow(unused_imports)]
use checkpoint::*;
#[allow(unused_imports)]
use setup::*;

use phient::net::space::WordFunction;
use phical::{rank, Example};

fn main() {
    let arg = |k: usize| std::env::args().nth(k).and_then(|s| s.parse::<usize>().ok());
    let epochs = arg(1).unwrap_or(4);
    let budget = std::time::Duration::from_secs(arg(2).unwrap_or(480) as u64);
    let data = load(arg(3));
    let started = std::time::Instant::now();
    std::fs::create_dir_all("data/sweep").expect("data/sweep");
    let done = finished_arms();
    println!("pool {} · chance MRR {:.4} · train {} · identity valid {:.1}× test {:.1}× hit@10 {:.3}", data.pool.len(), data.chance, data.train.len(), data.id_valid / data.chance, data.id_test / data.chance, data.id_hit);
    println!("{} arm(s) already finished; budget {} s\n", done.len(), budget.as_secs());
    let mut last_quarter = std::time::Duration::ZERO;
    for arm in arms(&data, epochs) {
        if done.contains(&arm.name.to_string()) {
            continue;
        }
        let out = run_arm(&arm, &data, epochs, started, budget, &mut last_quarter);
        match out {
            Outcome::Finished(row) => {
                append_result(&row);
                println!("{row}");
            }
            Outcome::Paused { epoch, quarter } => {
                println!("paused `{}` at epoch {epoch} quarter {quarter} ({:.0} s used) — run again to continue", arm.name, started.elapsed().as_secs_f64());
                return;
            }
        }
    }
    save_best(&data);
}

/// One arm's training loop, resumed from its checkpoint when there is one.
fn run_arm(arm: &Arm, data: &Data, epochs: usize, started: std::time::Instant, budget: std::time::Duration, last_quarter: &mut std::time::Duration) -> Outcome {
    let (mut fno, mut adam, mut st) = load_checkpoint(arm, epochs);
    let vin: Vec<WordFunction> = data.valid.iter().map(|e| e.input.clone()).collect();
    let n = data.train.len();
    while st.epoch < epochs {
        while st.quarter < QUARTERS {
            // A quarter is not started unless one of the same length fits.
            if started.elapsed() + *last_quarter > budget {
                return Outcome::Paused { epoch: st.epoch, quarter: st.quarter };
            }
            let order = epoch_order(n, st.epoch as u64);
            let (a, b) = (st.quarter * n / QUARTERS, (st.quarter + 1) * n / QUARTERS);
            let slice: Vec<Example> = order[a..b].iter().map(|&i| data.train[i].clone()).collect();
            let seed = (st.epoch * QUARTERS + st.quarter) as u64;
            let q0 = std::time::Instant::now();
            if arm.softmax {
                fno.train_epoch_softmax(data.space.table(), &slice, 8.0, 64, &mut adam, seed);
            } else {
                fno.train_epoch(data.space.table(), &slice, &data.pool, 64, &mut adam, seed);
            }
            st.quarter += 1;
            *last_quarter = q0.elapsed();
            save_checkpoint(arm, &fno, &adam, &st);
        }
        let (m, _) = rank(data.space.table(), &fno.apply(&vin), &data.valid, &data.pool);
        st.epoch += 1;
        st.quarter = 0;
        st.seconds += started.elapsed().as_secs_f64() - st.mark;
        st.mark = started.elapsed().as_secs_f64();
        if m > st.best {
            st.best = m;
            st.best_epoch = st.epoch;
            std::fs::write(best_path(arm), fno.to_bytes()).expect("write best");
        }
        save_checkpoint(arm, &fno, &adam, &st);
        println!("  {} epoch {} valid {:.1}× (best {:.1}× at {})", arm.name, st.epoch, m / data.chance, st.best / data.chance, st.best_epoch);
    }
    let tin: Vec<WordFunction> = data.test.iter().map(|e| e.input.clone()).collect();
    let mut best = phical::Fno::from_bytes(&std::fs::read(best_path(arm)).expect("best")).expect("decode");
    let (tm, th) = rank(data.space.table(), &best.apply(&tin), &data.test, &data.pool);
    Outcome::Finished(format!("| {} | {} | {} | {} | {} | {:.1}× | {:.1}× | {th:.3} | {:.0} |", arm.name, best.parameter_count(), arm.objective(), arm.optimiser(), st.best_epoch, st.best / data.chance, tm / data.chance, st.seconds))
}

pub(crate) enum Outcome {
    Finished(String),
    Paused { epoch: usize, quarter: usize },
}
