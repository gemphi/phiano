#[allow(unused_imports)]
use super::*;

impl Stat {
    pub(super) fn of(xs: &[f64]) -> Self {
        let n = xs.len();
        if n == 0 {
            return Self::default();
        }
        let mean = xs.iter().sum::<f64>() / n as f64;
        let sd = match n {
            0 | 1 => 0.0,
            _ => (xs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64).sqrt(),
        };
        Self { mean, sd, n }
    }

    /// True when this is separated from `other` by more than the two spreads
    /// together — the crude test, stated as crude, that an effect clears its
    /// own noise.
    pub(super) fn separated_from(&self, other: &Stat) -> bool {
        (self.mean - other.mean).abs() > self.sd + other.sd
    }
}

pub(super) fn measure(name: &str, facet: &Facet, valid: &[String]) -> Row {
    let families = RelationBenchmark::default_families();
    let r = RelationBenchmark::evaluate(facet, &families);
    Row {
        name: name.to_string(),
        usable_pairs: r.families.iter().map(|f| f.usable_pairs).sum(),
        pair_vs_random: r.overall_pair_vs_random,
        neighbour_top10: r
            .families
            .iter()
            .map(|f| f.neighbour_top10)
            .sum::<f64>()
            / r.families.len().max(1) as f64,
        analogy_top1: r.overall_analogy_top1,
        analogy_mrr: r.families.iter().map(|f| f.analogy_mrr).sum::<f64>()
            / r.families.len().max(1) as f64,
        dispersion: facet.phase_dispersion(),
        valid_ppl: Harness::perplexity_no_phase(facet, valid),
    }
}

/// Measures a condition, keeps the facet if it is the best seen.
///
/// Only the best facet is retained — holding all fifteen would be about a
/// gigabyte at this vocabulary size — and it is what the per-family breakdown
/// is computed from.
pub(super) fn record(
    rows: &mut Vec<Row>,
    best: &mut (f64, Facet),
    name: &str,
    f: Facet,
    valid: &[String],
) {
    let r = measure(name, &f, valid);
    if r.analogy_mrr > best.0 {
        *best = (r.analogy_mrr, f);
    }
    rows.push(r);
}
