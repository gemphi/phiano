//! Abstract and surface prediction figures for the learned layer, with the
//! two control arms, and the gate that decides whether the layer may be on
//! (task 9.6, design section 5).
//!
//! For each arm (learned, shuffled, frequency) and each level (abstract,
//! surface) the harness records top-1, perplexity, and the predicted and
//! abstained counts as `ladder.act.<level>_<arm>_<field>`. A figure that
//! cannot be computed is a named absence (NaN, no verdict), never a zero.
//!
//! The gate bar is declared in code: the learned arm must beat BOTH control
//! arms at the abstract level, on perplexity (strictly lower) and on top-1
//! (strictly higher). If any of the four figures is uncomputable the layer
//! is not enabled. The verdict is recorded as `ladder.act.abstract_gate_*`.
//! The verdict never flips `Config::default`: the layer is off by default
//! whatever it says. `enabled` in the measuring config below only lets the
//! predict stage run so it can be measured.

use super::{absent, count, percent};
use crate::bars::{Bars, Unit};
use crate::record::Measured;
use phisor::abstraction::{self, Arm, Config, Rule, Scores};
use phisor::guide::Split;

/// Seed of the shuffled control, fixed so a run reproduces.
pub const SEED: u64 = 7;
/// Declared gate bar: the learned arm's abstract perplexity must be lower
/// than each control's by more than this margin.
pub const GATE_PERPLEXITY_MARGIN: f64 = 0.0;
/// Declared gate bar: the learned arm's abstract top-1 (percent) must exceed
/// each control's by more than this many points.
pub const GATE_TOP1_MARGIN_POINTS: f64 = 0.0;

/// The gate's verdict and why.
#[derive(Clone, Debug, PartialEq)]
pub struct Gate {
    pub enabled: bool,
    pub reason: String,
}

/// Whether `learned` beats `control` at the abstract level by the declared
/// margins, or why that cannot be said.
fn beats(learned: &Scores, control: &Scores, name: &str) -> Result<bool, String> {
    let need = |v: Option<f64>, what: &str, who: &str| v.ok_or(format!("{who} {what} is uncomputable"));
    let (lp, cp) = (
        need(learned.abstract_perplexity, "abstract perplexity", "learned")?,
        need(control.abstract_perplexity, "abstract perplexity", name)?,
    );
    let (lt, ct) = (
        need(learned.abstract_top1, "abstract top-1", "learned")?,
        need(control.abstract_top1, "abstract top-1", name)?,
    );
    Ok(cp - lp > GATE_PERPLEXITY_MARGIN && 100.0 * (lt - ct) > GATE_TOP1_MARGIN_POINTS)
}

/// Decide the gate from the three arms' scores.
pub fn decide(learned: &Scores, shuffled: &Scores, frequency: &Scores) -> Gate {
    let mut lost = Vec::new();
    for (name, control) in [("shuffled", shuffled), ("frequency", frequency)] {
        match beats(learned, control, name) {
            Ok(true) => {}
            Ok(false) => lost.push(format!("does not beat the {name} control")),
            Err(why) => lost.push(why),
        }
    }
    if lost.is_empty() {
        Gate { enabled: true, reason: "learned beats both controls on abstract perplexity and top-1".into() }
    } else {
        Gate { enabled: false, reason: format!("learned layer stays off: {}", lost.join("; ")) }
    }
}

/// The eight records of one arm: two levels by four fields. `None` scores
/// (the predict stage did not run) give eight named absences.
fn arm_records(arm: &str, s: Option<&Scores>, bars: &Bars) -> Vec<Measured> {
    let mut out = Vec::new();
    for level in ["abstract", "surface"] {
        let name = |field: &str| format!("ladder.act.{level}_{arm}_{field}");
        let (top1, ppl) = match (level, s) {
            ("abstract", Some(s)) => (s.abstract_top1, s.abstract_perplexity),
            (_, Some(s)) => (s.surface_top1, s.surface_perplexity),
            (_, None) => (None, None),
        };
        out.push(percent(&name("top1"), top1.map(|v| super::one_decimal(100.0 * v)), bars, None));
        out.push(match ppl {
            Some(v) => Measured::judged(&name("perplexity"), v, Unit::Perplexity, bars),
            None => absent(&name("perplexity"), Unit::Perplexity),
        });
        for (field, n) in [("predicted", s.map(|s| s.predicted)), ("abstained", s.map(|s| s.abstained))] {
            out.push(match n {
                Some(n) => count(&name(field), n, bars),
                None => absent(&name(field), Unit::Count),
            });
        }
    }
    out
}

/// The gate records: one per control (1 when beaten, 0 when not, absent when
/// uncomputable) and the verdict itself (1 enabled, 0 not).
fn gate_records(learned: &Scores, shuffled: &Scores, frequency: &Scores, bars: &Bars) -> (Vec<Measured>, Gate) {
    let gate = decide(learned, shuffled, frequency);
    let mut out = Vec::new();
    for (name, control) in [("shuffled", shuffled), ("frequency", frequency)] {
        let flag = format!("ladder.act.abstract_gate_beats_{name}");
        out.push(match beats(learned, control, name) {
            Ok(b) => count(&flag, usize::from(b), bars),
            Err(_) => absent(&flag, Unit::Count),
        });
    }
    out.push(count("ladder.act.abstract_gate_enabled", usize::from(gate.enabled), bars));
    (out, gate)
}

/// Measure the three arms on the Held_Out_Slice and record the figures and
/// the gate verdict. The reason is written to stderr beside the records.
pub fn measure(split: &Split, bars: &Bars) -> Vec<Measured> {
    // Measuring needs the stage on; this is not the layer's default.
    let config = Config { enabled: true, ..Config::default() };
    let run = |arm| abstraction::predict(split, config, arm, Rule::default(), SEED);
    let (learned, shuffled, frequency) = (run(Arm::Learned), run(Arm::Shuffled), run(Arm::Frequency));
    let mut out = Vec::new();
    out.extend(arm_records("learned", learned.as_ref(), bars));
    out.extend(arm_records("shuffled", shuffled.as_ref(), bars));
    out.extend(arm_records("frequency", frequency.as_ref(), bars));
    match (learned, shuffled, frequency) {
        (Some(l), Some(s), Some(f)) => {
            let (records, gate) = gate_records(&l, &s, &f, bars);
            eprintln!("ladder.act.abstract_gate: enabled={} ({})", gate.enabled, gate.reason);
            out.extend(records);
        }
        _ => {
            eprintln!("ladder.act.abstract_gate: layer stays off (the predict stage did not run)");
            out.extend(["beats_shuffled", "beats_frequency", "enabled"].map(|n| {
                absent(&format!("ladder.act.abstract_gate_{n}"), Unit::Count)
            }));
        }
    }
    out
}

#[cfg(test)]
mod tests;
