//! The Recorder's tests, split by shape to stay under the line cap: the two
//! named properties in `prop`, the integration unit facts and the read-only
//! directory test (R2.9) in `unit`.

mod figures;
mod prop;
mod unit;

use crate::bars::{Bar, Judged, Sense, Unit};
use crate::record::Measured;
use phistd::gen::Seeded;

/// The run's declared seed, matching the harness's other structural targets.
pub(crate) const SEED: u64 = 0x5241_425f_5048_494e;

/// The six units, for a generator to pick from.
pub(crate) const UNITS: [Unit; 6] = [
    Unit::Percentage,
    Unit::Perplexity,
    Unit::Count,
    Unit::ZScore,
    Unit::F1,
    Unit::Slope,
];

/// Draw one `Measured` with random name, value, unit, Bar presence, control
/// presence and verdict. The verdict is consistent with the Bar's presence so
/// the page is well formed; Property 6 (in the Bars tests) owns the judging
/// rule, so this only exercises the round trip and the partition.
pub(crate) fn draw_measured(g: &mut Seeded, i: usize) -> Measured {
    let unit = UNITS[g.below(UNITS.len() as u64) as usize];
    let name = format!("n.{i}.{}", g.below(1_000));
    let value = (g.below(100_000) as f64) / 100.0;
    let has_bar = g.below(2) == 0;
    let bar = if has_bar {
        let sense = if g.below(2) == 0 { Sense::AtLeast } else { Sense::AtMost };
        Some(Bar { value: (g.below(100_000) as f64) / 100.0, sense })
    } else {
        None
    };
    let verdict = if !has_bar {
        Judged::Unjudged
    } else if g.below(2) == 0 {
        Judged::Pass
    } else {
        Judged::Miss { shortfall: (g.below(10_000) as f64) / 100.0 - 50.0 }
    };
    let control =
        if g.below(2) == 0 { Some((format!("c.{i}"), (g.below(100_000) as f64) / 100.0)) } else { None };
    Measured { name, value, unit, bar, control, verdict }
}

/// Count the three verdicts over a set of numbers.
pub(crate) fn count_verdicts(numbers: &[Measured]) -> (usize, usize, usize) {
    let (mut p, mut mi, mut u) = (0, 0, 0);
    for m in numbers {
        match m.verdict {
            Judged::Pass => p += 1,
            Judged::Miss { .. } => mi += 1,
            Judged::Unjudged => u += 1,
        }
    }
    (p, mi, u)
}

/// Two floats equal, treating NaN as equal to NaN so the generator's
/// non-finite cases round-trip.
pub(crate) fn same(a: f64, b: f64) -> bool {
    (a.is_nan() && b.is_nan()) || a == b
}
