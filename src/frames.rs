//! Statistics shared by the context experiments.
//!
//! Extracted from `bin/context.rs` when a second experiment needed them.
//! `predictive_gain` in particular is load-bearing and subtle — a held-out split
//! keyed on content rather than position, Laplace smoothing so an unseen
//! (class, mode) costs a finite amount — and two copies of it would drift.

use crate::config::PHASE_CHANNELS;
use phiecs::phasor::facet::Facet;
use crate::phasor::{fnv1a, mix64};
use crate::tokenizer::Tokenizer;
use phiecs::phasor::wave::Wave;
use std::collections::HashMap;

/// Classes with fewer members than this cannot support a mean, and centering
/// them is subtracting a pair from itself.
pub const MIN_CLASS: usize = 8;

/// Assigns each pair a class, keeping the class-size distribution of `classes`
/// but destroying which pair belongs to which.
pub fn shuffle_classes(classes: &[u16], seed: u64) -> Vec<u16> {
    let mut out = classes.to_vec();
    let mut r = seed | 1;
    for i in (1..out.len()).rev() {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        out.swap(i, (r % (i as u64 + 1)) as usize);
    }
    out
}

/// K-means over gloss phase states, returning one class per gloss.
///
/// Farthest-point seeded and run to a fixed round count, so two runs on the same
/// data give the same classes.
pub fn discover_classes(facet: &Facet, texts: &[String], k: usize, rounds: usize) -> Vec<u16> {
    let states: Vec<Vec<f64>> = texts
        .iter()
        .map(|t| {
            let words = Tokenizer::tokenize(t);
            Wave::sentence_channels(facet, &words, false)
                .iter()
                .map(|z| z.arg())
                .collect()
        })
        .collect();
    if states.len() < k || k == 0 {
        return vec![0; texts.len()];
    }

    let agree = |a: &[f64], b: &[f64]| -> f64 {
        a.iter().zip(b.iter()).map(|(x, y)| (x - y).cos()).sum::<f64>() / PHASE_CHANNELS as f64
    };

    let mut best_sim = vec![f64::NEG_INFINITY; states.len()];
    let mut seeds: Vec<usize> = Vec::with_capacity(k);
    let mut next = 0usize;
    loop {
        seeds.push(next);
        if seeds.len() >= k {
            break;
        }
        let chosen = states[next].clone();
        for (b, s) in best_sim.iter_mut().zip(states.iter()) {
            let a = agree(s, &chosen);
            if a > *b {
                *b = a;
            }
        }
        best_sim[next] = f64::INFINITY;
        match best_sim
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
        {
            Some(i) if best_sim[i].is_finite() => next = i,
            _ => break,
        }
    }

    let mut centroids: Vec<Vec<f64>> = seeds.iter().map(|s| states[*s].clone()).collect();
    let mut assign = vec![0usize; states.len()];
    for _ in 0..rounds {
        let mut moved = false;
        for (i, s) in states.iter().enumerate() {
            let best = centroids
                .iter()
                .enumerate()
                .max_by(|a, b| {
                    agree(s, a.1).partial_cmp(&agree(s, b.1)).unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(c, _)| c)
                .unwrap_or(0);
            if assign[i] != best {
                assign[i] = best;
                moved = true;
            }
        }
        for (c, cen) in centroids.iter_mut().enumerate() {
            for kk in 0..PHASE_CHANNELS {
                let (mut x, mut y) = (0.0f64, 0.0f64);
                for (i, s) in states.iter().enumerate() {
                    if assign[i] == c {
                        x += s[kk].cos();
                        y += s[kk].sin();
                    }
                }
                if x.hypot(y) > 1e-12 {
                    cen[kk] = y.atan2(x);
                }
            }
        }
        if !moved {
            break;
        }
    }
    assign.into_iter().map(|a| a as u16).collect()
}

/// Merges classes too small to have a usable mean into one bucket.
pub fn collapse_small(classes: &[u16]) -> (Vec<u16>, usize) {
    let mut counts: HashMap<u16, usize> = HashMap::new();
    for c in classes {
        *counts.entry(*c).or_insert(0) += 1;
    }
    let out: Vec<u16> = classes
        .iter()
        .map(|c| match counts[c] >= MIN_CLASS {
            true => *c,
            false => u16::MAX,
        })
        .collect();
    let used = out.iter().collect::<std::collections::HashSet<_>>().len();
    (out, used)
}

/// Held-out log-loss of predicting a pair's harmonic mode from its context class,
/// and the gain over the context-free marginal.
///
/// Fitted on half the pairs and scored on the other half, split by content hash
/// rather than position — a per-head extractor emits pairs in an order that
/// index parity would stratify. Laplace smoothing, so an unseen (class, mode)
/// costs a finite amount rather than infinity.
///
/// Returns `(log_loss, marginal_log_loss - log_loss)`: positive gain means the
/// context helped.
pub fn predictive_gain(classes: &[u16], modes: &[usize]) -> (f64, f64) {
    let n = classes.len().min(modes.len());
    let side = |i: usize| -> bool {
        mix64(fnv1a(&format!("{}:{}", classes[i], i))) >> 63 == 0
    };

    const ALPHA: f64 = 0.5;
    let mut joint: HashMap<(u16, usize), f64> = HashMap::new();
    let mut per_class: HashMap<u16, f64> = HashMap::new();
    let mut marginal = vec![0.0f64; PHASE_CHANNELS];
    let mut fit_n = 0.0f64;

    for i in 0..n {
        if !side(i) {
            continue;
        }
        *joint.entry((classes[i], modes[i])).or_insert(0.0) += 1.0;
        *per_class.entry(classes[i]).or_insert(0.0) += 1.0;
        marginal[modes[i].min(PHASE_CHANNELS - 1)] += 1.0;
        fit_n += 1.0;
    }
    if fit_n < 1.0 {
        return (0.0, 0.0);
    }

    let m = PHASE_CHANNELS as f64;
    let (mut ll, mut ll_marg, mut held) = (0.0f64, 0.0f64, 0.0f64);
    for i in 0..n {
        if side(i) {
            continue;
        }
        let mode = modes[i].min(PHASE_CHANNELS - 1);
        let c = classes[i];
        let num = joint.get(&(c, mode)).copied().unwrap_or(0.0) + ALPHA;
        let den = per_class.get(&c).copied().unwrap_or(0.0) + ALPHA * m;
        ll -= (num / den).ln();
        ll_marg -= ((marginal[mode] + ALPHA) / (fit_n + ALPHA * m)).ln();
        held += 1.0;
    }
    match held < 1.0 {
        true => (0.0, 0.0),
        false => (ll / held, (ll_marg - ll) / held),
    }
}
