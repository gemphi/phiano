//! The gold-act figures: round-trip and force purity over each cut of the gold.

use super::one_decimal;
use crate::bars::{Bars, Unit};
use crate::record::Measured;

/// The round-trip and force-purity figures over a set of gold `(Act, text)`
/// pairs, named `ladder.act.round_trip{suffix}` and
/// `ladder.act.force_purity{suffix}`. `suffix` is empty for the all-150 cut
/// and `_reachable` for the subset that drops the Unreachable_Frames, so both
/// cuts are reported side by side (R2.2). A cut with nothing to score emits no
/// record, matching the all-150 behaviour it replaced.
pub(super) fn gold_figures(suffix: &str, pairs: &[(&phisor::ir::Act, &str)], bars: &Bars) -> Vec<Measured> {
    let mut out = Vec::new();
    let (rt_matched, rt_total, _) =
        phisor::ladder::round_trip(pairs.iter().map(|&(a, s)| (a, s)));
    if rt_total > 0 {
        let rate = 100.0 * rt_matched as f64 / rt_total as f64;
        let name = format!("ladder.act.round_trip{suffix}");
        out.push(Measured::judged(&name, one_decimal(rate), Unit::Percentage, bars));
    }
    let (fp_matched, fp_covered) = phisor::ladder::force_purity(pairs.iter().map(|&(a, s)| (a, s)));
    if fp_covered > 0 {
        let purity = 100.0 * fp_matched as f64 / fp_covered as f64;
        let name = format!("ladder.act.force_purity{suffix}");
        out.push(Measured::judged(&name, one_decimal(purity), Unit::Percentage, bars));
    }
    out
}

/// `ladder.act.round_trip_realisable`: the strict gold round trip over the 150
/// minus the Unrealisable_Frames, the frames whose Act has no field for a word
/// of the sentence (R1.4, R1.8). The gold file is untouched; the exclusion
/// lives only in this figure.
pub(super) fn realisable_figure(gold: &phisor::gold::Gold, bars: &Bars) -> Option<Measured> {
    let realisable: Vec<_> = gold
        .entries
        .iter()
        .enumerate()
        .filter(|(i, _)| phisor::gold::unrealisable(*i).is_none())
        .map(|(_, e)| (&e.act, e.sentence.as_str()))
        .collect();
    let (matched, total, _) = phisor::ladder::round_trip(realisable);
    (total > 0).then(|| {
        let rate = 100.0 * matched as f64 / total as f64;
        Measured::judged("ladder.act.round_trip_realisable", one_decimal(rate), Unit::Percentage, bars)
    })
}

/// `ladder.act.force_purity_decidable`: force purity with only the `BareWill`
/// frames removed. A bare `will` is a Commit or a prediction and the text does
/// not say which, so no Force can be read for it; the `NameCase` frames stay
/// in, since a proper name cannot change a Force (R2.1). The gold file is
/// untouched; the exclusion lives only in this figure.
pub(super) fn decidable_figure(gold: &phisor::gold::Gold, bars: &Bars) -> Option<Measured> {
    use phisor::gold::Reason;
    let decidable: Vec<_> = gold
        .entries
        .iter()
        .enumerate()
        .filter(|(i, _)| phisor::gold::unreachable(*i) != Some(Reason::BareWill))
        .map(|(_, e)| (&e.act, e.sentence.as_str()))
        .collect();
    let (matched, covered) = phisor::ladder::force_purity(decidable);
    (covered > 0).then(|| {
        let purity = 100.0 * matched as f64 / covered as f64;
        Measured::judged("ladder.act.force_purity_decidable", one_decimal(purity), Unit::Percentage, bars)
    })
}
