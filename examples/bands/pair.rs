#[allow(unused_imports)]
use super::*;

/// Does `a` resonate with `b` more than with a random word, on this band alone?
///
/// Chance is 0.5 by construction, on every band, so the bands are comparable to
/// each other — which a raw resonance would not be, since bands differ in how
/// much they were moved by training.
pub(super) fn pair_vs_random(facet: &Facet, pool: &[&String], lo: usize, hi: usize) -> f64 {
    let families = RelationBenchmark::default_families();
    let (mut wins, mut n) = (0.0f64, 0usize);

    for fam in &families {
        for (i, p) in fam.pairs.iter().enumerate() {
            let (pa, pb) = match (facet.lexicon.get(&p.a), facet.lexicon.get(&p.b)) {
                (Some(a), Some(b)) => (a, b),
                _ => continue,
            };
            let target = pa.resonance_band(pb, lo, hi);
            let (mut hits, mut draws) = (0usize, 0usize);
            for d in 0..DRAWS {
                let r = (fnv1a(&p.a) ^ ((i as u64) << 32) ^ (d as u64).wrapping_mul(0x9E37_79B9))
                    as usize;
                let cand = pool[r % pool.len().max(1)];
                if cand == &p.a || cand == &p.b {
                    continue;
                }
                draws += 1;
                if target > pa.resonance_band(&facet.lexicon[cand], lo, hi) {
                    hits += 1;
                }
            }
            if draws > 0 {
                wins += hits as f64 / draws as f64;
                n += 1;
            }
        }
    }
    match n {
        0 => 0.5,
        _ => wins / n as f64,
    }
}

/// A sentence's state on one band: the circular mean of its words' channels.
pub(super) fn state_band(facet: &Facet, words: &[String], lo: usize, hi: usize) -> Vec<(f64, f64)> {
    let mut acc = vec![(0.0f64, 0.0f64); hi.saturating_sub(lo)];
    for w in words {
        if let Some(p) = facet.lexicon.get(w) {
            for (i, k) in (lo..hi).enumerate() {
                let t = p.theta(k);
                acc[i].0 += t.cos();
                acc[i].1 += t.sin();
            }
        }
    }
    acc
}

pub(super) fn similarity(a: &[(f64, f64)], b: &[(f64, f64)]) -> f64 {
    let (mut s, mut n) = (0.0f64, 0usize);
    for (x, y) in a.iter().zip(b.iter()) {
        let (nx, ny) = (x.0.hypot(x.1), y.0.hypot(y.1));
        if nx > 1e-12 && ny > 1e-12 {
            s += (x.0 * y.0 + x.1 * y.1) / (nx * ny);
            n += 1;
        }
    }
    match n {
        0 => 0.0,
        _ => s / n as f64,
    }
}

/// The identical item set is used for every band. A comparison across bands is
/// only a comparison if the task does not move.
pub(super) fn build_items(held: &[String], seed: u64) -> Vec<Item> {
    let toks: Vec<Vec<String>> = held.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let mut items = Vec::new();
    let mut i = CONTEXT;
    while i < toks.len() && items.len() < ITEMS {
        let truth_toks = toks[i].clone();
        if truth_toks.len() >= 4 {
            let context: Vec<Vec<String>> = toks[i - CONTEXT..i].to_vec();
            let mut chosen: Vec<Vec<String>> = Vec::with_capacity(CANDIDATES);
            let mut r = fnv1a(&i.to_string()) ^ seed;
            let mut seen = std::collections::HashSet::new();
            while chosen.len() + 1 < CANDIDATES {
                r ^= r << 13;
                r ^= r >> 7;
                r ^= r << 17;
                let idx = (r % toks.len() as u64) as usize;
                // Distractors from elsewhere in the corpus, never from this
                // item's own neighbourhood, which would make them plausible
                // continuations rather than distractors.
                if idx.abs_diff(i) < 16 || !seen.insert(idx) || toks[idx].len() < 4 {
                    continue;
                }
                chosen.push(toks[idx].clone());
            }
            let truth = (r % CANDIDATES as u64) as usize;
            chosen.insert(truth.min(chosen.len()), truth_toks);
            items.push(Item { context, candidates: chosen, truth });
        }
        // Stride, so the items are spread over the whole held-out half rather
        // than crowded into its opening.
        i += 7;
    }
    items
}

pub(super) fn sentence_mrr(facet: &Facet, items: &[Item], lo: usize, hi: usize) -> f64 {
    let mut total = 0.0;
    for it in items {
        let mut ctx = vec![(0.0f64, 0.0f64); hi.saturating_sub(lo)];
        for sent in &it.context {
            let s = state_band(facet, sent, lo, hi);
            for (a, b) in ctx.iter_mut().zip(s.iter()) {
                a.0 += b.0;
                a.1 += b.1;
            }
        }
        let score =
            |c: &Vec<String>| similarity(&ctx, &state_band(facet, c, lo, hi));
        let truth_score = score(&it.candidates[it.truth]);
        let beating = it
            .candidates
            .iter()
            .enumerate()
            .filter(|(i, c)| *i != it.truth && score(c) > truth_score)
            .count();
        total += 1.0 / (beating + 1) as f64;
    }
    total / items.len().max(1) as f64
}
