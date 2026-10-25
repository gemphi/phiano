//! Answers of one kind push apart.
//!
//! The quest's attraction pulls every word toward its answer on the kind's
//! channel, so a family condenses around its head (docs/86 §3). Nothing
//! pushes two heads apart: their target angles stay where their seeds put
//! them, and 32 heads seeded at random share 64 sectors with collisions. This
//! is repulsive Kuramoto among the answers of one kind only — each answer's
//! target side moves away from every other answer's, weighted by `sin` of
//! their difference, so the set spreads toward even spacing. Words that are
//! not answers are untouched; they follow their heads by attraction.
#[allow(unused_imports)]
use super::*;

use phiano::phasor::Rounding;

/// One repulsion pass over every kind with shared answers. Returns the number
/// of nudges applied. It reads and moves the *raw* target side: until a word
/// has `TARGET_TIE_FLOOR` observations on a channel its target reading is tied
/// to its source, so a spread made before training must act on the stored
/// value that takes over once evidence arrives.
pub fn repel_answers(facet: &mut Facet, rows: &[Row], rate: f64) -> usize {
    if rate == 0.0 {
        return 0;
    }
    let mut applied = 0;
    for kind in shared_kinds(rows) {
        let Some(k) = facet.marker_channel(&kind) else { continue };
        let mut answers: Vec<String> = rows.iter().filter(|r| r.kind == kind).map(|r| r.other.clone()).collect();
        answers.sort();
        answers.dedup();
        let angles: Vec<f64> = answers.iter().map(|a| facet.lexicon.get(a).map(|p| p.theta_target_raw(k)).unwrap_or(0.0)).collect();
        let n = answers.len().max(2) as f64;
        for (i, a) in answers.iter().enumerate() {
            // Mean repulsive pull from every other answer.
            let push: f64 = angles.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, t)| (angles[i] - t).sin()).sum::<f64>() / (n - 1.0);
            if let Some(p) = facet.lexicon.get_mut(a) {
                p.nudge_target_by(k, rate * push, Rounding::Dither);
                applied += 1;
            }
        }
    }
    applied
}
