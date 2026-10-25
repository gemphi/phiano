#[allow(unused_imports)]
use super::*;

/// The table: one row per marker channel, own channel against rivals against
/// the two dense readings, on identical probes.
pub(super) fn run_benchmark(t: &Trained, m: &Mined) {
    let (rel, shipped, vocab) = (&t.rel, &t.shipped, &t.vocab);
    let (mined, pool) = (&m.mined, &m.pool);

    println!(
        "  {:<10} {:>6} {:>7}   {:>15} {:>15} {:>15} {:>15}",
        "", "", "", "own channel", "rival channels", "all 64 blended", "shipped"
    );
    println!(
        "  {:<10} {:>6} {:>7}   {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7}",
        "marker", "pairs", "probes", "MRR", "vs null", "MRR", "vs null", "MRR", "vs null", "MRR",
        "vs null"
    );

    let markers: Vec<usize> = (OFFSET_CHANNELS..PHASE_CHANNELS)
        .filter(|k| mined.get(k).map(|m| m.len()).unwrap_or(0) >= MIN_PAIRS)
        .take(MARKERS_TESTED)
        .collect();

    // Own-versus-rival is a paired comparison per marker, so the honest summary
    // of it is a sign test and not a ratio of two averages: one marker with a lot
    // of probes can carry a mean on its own.
    let mut tot = Totals::default();

    for &k in &markers {
        let mut pairs: Vec<((String, String), usize)> =
            mined[&k].iter().map(|(p, c)| (p.clone(), *c)).collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        pairs.truncate(MAX_PAIRS);
        let pairs: Vec<(String, String)> = pairs.into_iter().map(|(p, _)| p).collect();

        // The rivals: other marker channels, spread across the range so they are
        // not all neighbours of `k`, and deterministic.
        let rivals: Vec<usize> = markers
            .iter()
            .copied()
            .filter(|&r| r != k)
            .enumerate()
            .filter(|(i, _)| i % (markers.len() / RIVALS).max(1) == 0)
            .map(|(_, r)| r)
            .take(RIVALS)
            .collect();

        let s = score_marker(rel, shipped, pool, &pairs, k, &rivals);
        if s.n == 0 {
            continue;
        }
        let f = s.n as f64;
        let ratio = |x: f64, y: f64| x / y.max(f64::MIN_POSITIVE);
        println!(
            "  {:<10} {:>6} {:>7} {:>9.5} {:>6.1}x {:>7.5} {:>6.1}x {:>7.5} {:>6.1}x {:>7.5} {:>6.1}x",
            vocab.name(k),
            pairs.len(),
            s.n,
            s.own / f,
            ratio(s.own, s.nown),
            s.riv / f,
            ratio(s.riv, s.nriv),
            s.all / f,
            ratio(s.all, s.nall),
            s.shp / f,
            ratio(s.shp, s.nshp),
        );
        match s.own / f > s.riv / f {
            true => tot.wins += 1,
            false => tot.losses += 1,
        }
        tot.t_own += s.own;
        tot.t_riv += s.riv;
        tot.t_all += s.all;
        tot.t_shp += s.shp;
        tot.n_own += s.nown;
        tot.n_riv += s.nriv;
        tot.n_all += s.nall;
        tot.n_shp += s.nshp;
        tot.total += s.n;
    }

    print_summary(&tot);
}
