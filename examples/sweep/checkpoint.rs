#[allow(unused_imports)]
use super::*;
use phistd::Adam;
use phical::Fno;

/// Where an arm is: epochs and quarters done, the best validation MRR so far
/// and when it happened, and the training seconds spent (wall clock, this
/// arm only).
#[derive(Clone, Copy, Default)]
pub(crate) struct State {
    pub epoch: usize,
    pub quarter: usize,
    pub best: f64,
    pub best_epoch: usize,
    pub seconds: f64,
    /// Elapsed seconds of the current process when the arm's timer last
    /// advanced; not persisted.
    pub mark: f64,
}

pub(crate) fn best_path(arm: &Arm) -> String {
    format!("data/sweep/{}.best.fno", arm.slug)
}

fn ckpt_path(arm: &Arm) -> String {
    format!("data/sweep/{}.ckpt", arm.slug)
}

fn state_path(arm: &Arm) -> String {
    format!("data/sweep/{}.state", arm.slug)
}

pub(crate) fn save_checkpoint(arm: &Arm, fno: &Fno, adam: &Adam, st: &State) {
    std::fs::write(ckpt_path(arm), fno.to_bytes_with(adam)).expect("write ckpt");
    std::fs::write(state_path(arm), format!("{} {} {} {} {}\n", st.epoch, st.quarter, st.best, st.best_epoch, st.seconds)).expect("write state");
}

/// The arm's operator, optimiser and state — from disk when it was paused,
/// fresh otherwise.
pub(crate) fn load_checkpoint(arm: &Arm, epochs: usize) -> (Fno, Adam, State) {
    let fresh = || {
        let (f, a) = arm.fresh(epochs);
        (f, a, State::default())
    };
    let Ok(text) = std::fs::read_to_string(state_path(arm)) else { return fresh() };
    let v: Vec<f64> = text.split_whitespace().filter_map(|x| x.parse().ok()).collect();
    if v.len() < 5 {
        return fresh();
    }
    let Some(bytes) = std::fs::read(ckpt_path(arm)).ok() else { return fresh() };
    let Some((fno, Some(adam))) = Fno::from_bytes_with(&bytes) else { return fresh() };
    let st = State { epoch: v[0] as usize, quarter: v[1] as usize, best: v[2], best_epoch: v[3] as usize, seconds: v[4], mark: 0.0 };
    println!("  resuming `{}` at epoch {} quarter {}", arm.name, st.epoch, st.quarter);
    (fno, adam, st)
}

const RESULTS: &str = "data/sweep/results.md";
const HEADER: &str = "| arm | params | objective | optimiser | best epoch | valid MRR ×chance | test MRR ×chance | test hit@10 | s |\n|---|---|---|---|---|---|---|---|---|\n";

/// Names of the arms that already have a row.
pub(crate) fn finished_arms() -> Vec<String> {
    std::fs::read_to_string(RESULTS).unwrap_or_default().lines().filter(|l| l.starts_with("| ") && !l.starts_with("| arm ")).filter_map(|l| l.split('|').nth(1).map(|s| s.trim().to_string())).collect()
}

pub(crate) fn append_result(row: &str) {
    let mut text = std::fs::read_to_string(RESULTS).unwrap_or_default();
    if text.is_empty() {
        text.push_str(HEADER);
    }
    text.push_str(row);
    text.push('\n');
    std::fs::write(RESULTS, text).expect("write results");
}

/// Copies the arm with the best validation MRR to `data/austen_operator.fno`.
pub(crate) fn save_best(data: &Data) {
    let mut best: Option<(f64, String, String)> = None;
    for arm in arms(data, 0) {
        let Ok(text) = std::fs::read_to_string(state_path(&arm)) else { continue };
        let v: Vec<f64> = text.split_whitespace().filter_map(|x| x.parse().ok()).collect();
        if v.len() >= 5 && best.as_ref().is_none_or(|b| v[2] > b.0) {
            best = Some((v[2], best_path(&arm), arm.name.to_string()));
        }
    }
    if let Some((m, path, name)) = best {
        std::fs::copy(&path, "data/austen_operator.fno").expect("copy best");
        println!("\nsaved the best arm ({name}, valid {:.1}× chance) as data/austen_operator.fno", m / data.chance);
    }
}
