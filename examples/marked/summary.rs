#[allow(unused_imports)]
use super::*;

/// Everything accumulated across markers: the four scorers, their four nulls,
/// the paired own-versus-rival outcome, and the probe count.
#[derive(Default)]
pub(super) struct Totals {
    pub(super) wins: usize,
    pub(super) losses: usize,
    pub(super) t_own: f64,
    pub(super) t_riv: f64,
    pub(super) t_all: f64,
    pub(super) t_shp: f64,
    pub(super) n_own: f64,
    pub(super) n_riv: f64,
    pub(super) n_all: f64,
    pub(super) n_shp: f64,
    pub(super) total: usize,
}

/// The ALL row, the decoy null row, and the sign test that decides whether a
/// channel belongs to the relation it is named after.
pub(super) fn print_summary(t: &Totals) {
    if t.total == 0 {
        return;
    }
    let f = t.total as f64;
    let ratio = |x: f64, y: f64| x / y.max(f64::MIN_POSITIVE);
    println!(
        "  {:<10} {:>6} {:>7} {:>9.5} {:>6.1}x {:>7.5} {:>6.1}x {:>7.5} {:>6.1}x {:>7.5} {:>6.1}x",
        "ALL",
        "",
        t.total,
        t.t_own / f,
        ratio(t.t_own, t.n_own),
        t.t_riv / f,
        ratio(t.t_riv, t.n_riv),
        t.t_all / f,
        ratio(t.t_all, t.n_all),
        t.t_shp / f,
        ratio(t.t_shp, t.n_shp),
    );
    println!(
        "  {:<10} {:>6} {:>7} {:>9.5} {:>7} {:>7.5} {:>7} {:>7.5} {:>7} {:>7.5}",
        "decoy null",
        "",
        t.total,
        t.n_own / f,
        "",
        t.n_riv / f,
        "",
        t.n_all / f,
        "",
        t.n_shp / f,
    );
    // Two-sided sign test on the paired own-versus-rival outcome.
    let (wins, losses) = (t.wins, t.losses);
    let m = wins + losses;
    let c = |n: usize, k: usize| -> f64 {
        (0..k).fold(1.0f64, |acc, i| acc * (n - i) as f64 / (i + 1) as f64)
    };
    let tail: f64 = (wins.max(losses)..=m).map(|i| c(m, i)).sum();
    let p = (2.0 * tail / 2f64.powi(m as i32)).min(1.0);
    println!(
        "\n  own beats rival on {wins} of {m} markers, sign test p = {p:.3}"
    );
    println!(
        "\n  The column that decides it is own-channel against rival-channels. \
         Those are\n  the same probes, the same pool and the same scorer, \
         differing only in which\n  dimension the step is taken in. A gap there \
         means the relation owns the\n  dimension. No gap means the model \
         learned which words go together and the\n  assignment of relations to \
         channels carries nothing."
    );
}
