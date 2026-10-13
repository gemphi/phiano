#[allow(unused_imports)]
use super::*;

/// Composition: traverse a relation, do not blend all of them.
///
/// The similarity rows measure what relation-specific channels cost and not
/// what they are for. Similarity wants every relation at once; composition
/// wants one, named, and traversed. `a : b :: c : ?` is that operation — apply
/// the step from `a` to `b` to `c` — and the architecture's claim is that the
/// step lives in a channel, so applying it should work *better* in that channel
/// alone than smeared across sixty-four.
///
/// Scored per family on its own best channel, against two baselines on
/// identical probes: the same analogy over all channels evidence-weighted, and
/// the shipped manifold's 64-channel version.
pub(super) fn composition_section(
    rel: &RelationalFacet,
    shipped: &Facet,
    pool: &[String],
    families: &[RelationFamily],
) {
    let known = |w: &str| known_word(rel, shipped, w);

    println!("\n  === composition: a:b :: c:? ===");
    println!(
        "  {:<12} {:>6} {:>6} {:>17} {:>17} {:>17}",
        "", "", "", "best channel", "all channels", "shipped"
    );
    println!(
        "  {:<12} {:>6} {:>6} {:>9} {:>8} {:>9} {:>8} {:>9} {:>8}",
        "family", "probes", "pool", "MRR", "vs null", "MRR", "vs null", "MRR", "vs null"
    );

    // Two rules, both learned the hard way earlier in this project.
    //
    // First: all three scorers rank against the *same* candidate set. The
    // one-channel scorer can only rank a word it has evidence for, so if it were
    // allowed to skip words with `mag[k] == 0` while the other two ranked against
    // the full pool, its ranks would come from a pool of a few hundred and theirs
    // from six thousand. That is the `rank_of_above` bug wearing a different hat.
    // So the family's channel decides the candidate set and all three use it.
    //
    // Second: chance for mean-reciprocal-rank is not `1/N`. That is precision@1.
    // A uniformly random ranking scores `H_N / N`, roughly nine times higher at
    // this pool size. Rather than trust the closed form against a pool that has
    // ties and exclusions in it, each scorer is also run against a decoy target —
    // same probe, same pool, same code path, wrong answer. That empirical null is
    // the scale every ratio below is quoted against.
    let pool_ref: Vec<&String> = pool.iter().collect();
    let (mut tot_ch, mut tot_all, mut tot_ship, mut tot_n) = (0.0f64, 0.0f64, 0.0f64, 0usize);
    let (mut nul_ch, mut nul_all, mut nul_ship) = (0.0f64, 0.0f64, 0.0f64);

    for fam in families.iter() {
        let pairs: Vec<(String, String)> = fam
            .pairs
            .iter()
            .filter(|p| known(&p.a) && known(&p.b))
            .map(|p| (p.a.clone(), p.b.clone()))
            .collect();
        if pairs.len() < 4 {
            continue;
        }
        let (prof, _, _) = profile(rel, &pairs);
        let (k, v) = argmax(&prof);
        if !v.is_finite() {
            continue;
        }

        // The candidate set: words this family's channel can actually place.
        // Every scorer ranks inside it, so the three numbers are commensurable.
        let cand: Vec<&String> = pool_ref
            .iter()
            .copied()
            .filter(|w| rel.words[*w].mag[k] > 0.0)
            .collect();
        if cand.len() < 32 {
            continue;
        }

        let s = score_family(rel, shipped, &pairs, &cand, k);
        if s.n == 0 {
            continue;
        }
        let f = s.n as f64;
        println!(
            "  {:<12} {:>6} {:>6} {:>9.5} {:>7.1}x {:>9.5} {:>7.1}x {:>9.5} {:>7.1}x",
            fam.name,
            s.n,
            cand.len(),
            s.ch / f,
            s.ch / s.nch.max(f64::MIN_POSITIVE),
            s.all / f,
            s.all / s.nall.max(f64::MIN_POSITIVE),
            s.ship / f,
            s.ship / s.nship.max(f64::MIN_POSITIVE),
        );
        tot_ch += s.ch;
        tot_all += s.all;
        tot_ship += s.ship;
        nul_ch += s.nch;
        nul_all += s.nall;
        nul_ship += s.nship;
        tot_n += s.n;
    }

    if tot_n > 0 {
        let t = tot_n as f64;
        println!(
            "  {:<12} {:>6} {:>6} {:>9.5} {:>7.1}x {:>9.5} {:>7.1}x {:>9.5} {:>7.1}x",
            "ALL",
            tot_n,
            "",
            tot_ch / t,
            tot_ch / nul_ch.max(f64::MIN_POSITIVE),
            tot_all / t,
            tot_all / nul_all.max(f64::MIN_POSITIVE),
            tot_ship / t,
            tot_ship / nul_ship.max(f64::MIN_POSITIVE),
        );
        println!(
            "  {:<12} {:>6} {:>6} {:>9.5} {:>8} {:>9.5} {:>8} {:>9.5} {:>8}",
            "decoy null",
            tot_n,
            "",
            nul_ch / t,
            "",
            nul_all / t,
            "",
            nul_ship / t,
            "",
        );
    }
}
