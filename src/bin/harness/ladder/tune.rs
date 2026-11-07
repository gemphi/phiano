//! The declared settings grid and the dev-part sweep that picks the settings.

use super::MODE;
use phisor::guide::holdout::HoldoutGuard;
use phisor::guide::{Part, Split};
use phisor::ladder::{Judged, Ladder, Seeds};
use phical::classes::{Settings, MAX_PASSES};

/// The declared settings grid. Each axis is a legitimate knob of the exchange
/// routine; the Bar, the split and the scorer are untouched. The grid is
/// deliberately small and fixed so a run reproduces.
pub(super) fn grid() -> Vec<Settings> {
    let mut out = Vec::new();
    for &passes in &[1usize, 4, 8, MAX_PASSES] {
        for &floor in &[0usize, 2, 4] {
            for &penalty in &[0.0f64, 0.5, 1.0] {
                out.push(Settings {
                    max_passes: passes,
                    min_unit_frequency: floor,
                    split_cost_penalty: penalty,
                    ..Settings::default()
                });
            }
        }
    }
    out
}

/// Sweep the grid and keep the settings with the highest dev function
/// purity (never the Held_Out_Slice). Ties keep the earlier (simpler) settings, so a more complex setting
/// has to earn its place. Returns the best settings and the purity it scored.
pub(super) fn tune(split: &Split) -> (Settings, f64) {
    let mut best = Settings { max_passes: 1, ..Settings::default() };
    let mut best_score = (f64::NEG_INFINITY, f64::NEG_INFINITY);
    for settings in grid() {
        let score = dev_scores_of(split, &settings);
        if score > best_score {
            best_score = score;
            best = settings;
        }
    }
    (best, best_score.0.max(0.0))
}

/// The dev scores a given settings earns, in the order the sweep compares
/// them: function purity first, then terminator purity.
///
/// Dev holds no answer sentences (the Split_Rule sends every Key answer to
/// Test), so dev function purity is the explain share for every setting and
/// never separates them. The terminator mark is read off the text, not the
/// Key, so terminator purity on dev still varies with the Kinds and breaks
/// that tie. It names no held-out label and reads only `Part::Dev`.
/// A setting that could not be scored is negative infinity on both, so it
/// never wins the sweep.
pub(super) fn dev_scores_of(split: &Split, settings: &Settings) -> (f64, f64) {
    let ladder = Ladder::new(split, Seeds::DECLARED);
    let learned = ladder.kinds(phisor::ladder::Arm::Learned, MODE, settings);
    let mut guard = HoldoutGuard::new(vec![]);
    let function = ladder
        .function_purity_on(Part::Dev, &learned, &mut guard)
        .map_or(f64::NEG_INFINITY, |r| value(&r.purity));
    let terminator = ladder
        .terminator_purity_on(Part::Dev, &learned, &mut guard)
        .map_or(f64::NEG_INFINITY, |r| value(&r.purity));
    (function, terminator)
}

/// The number behind a ladder `Measured`, whichever verdict it carries, so
/// the Recorder judges the value against the declared Bar.
pub(super) fn value(m: &phisor::ladder::Measured) -> f64 {
    match m.verdict {
        Judged::Pass => m.value,
        Judged::Miss { value, .. } => value,
    }
}
