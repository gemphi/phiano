#[allow(unused_imports)]
use super::*;

/// Question 3: does composition through the pair's own most-evidenced channel
/// beat the dense analogy, against a decoy null? Prints one row per family and
/// the pooled totals.
pub(super) fn experiment_composition(
    control: &Facet,
    promoted: &Facet,
    pool: &[String],
    families: &[RelationFamily],
    known: &dyn Fn(&str) -> bool,
    balance: &[f64],
) {
    println!("\n  === composition: a:b :: c:? ===");
    println!(
        "  {:<12} {:>6} {:>6} {:>17} {:>17} {:>17} {:>17}",
        "", "", "", "family channel", "pair's channel", "dense promoted", "dense control"
    );
    println!(
        "  {:<12} {:>6} {:>6} {:>9} {:>8} {:>9} {:>8} {:>9} {:>8} {:>9} {:>8}",
        "family", "probes", "pool", "MRR", "vs null", "MRR", "vs null", "MRR", "vs null", "MRR", "vs null"
    );
    let pool_ref: Vec<&String> = pool.iter().collect();
    let (mut tot_fc, mut tot_ch, mut tot_pd, mut tot_cd, mut tot_n) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0usize);
    let (mut nul_fc, mut nul_ch, mut nul_pd, mut nul_cd) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);

    for fam in families {
        let pairs: Vec<(String, String)> = fam
            .pairs
            .iter()
            .filter(|p| known(&p.a) && known(&p.b))
            .map(|p| (p.a.clone(), p.b.clone()))
            .collect();
        if pairs.len() < 4 {
            continue;
        }
        // The family's channel: where its pairs agree most, as `bin/relational`
        // chose it. This is a supervised choice — it looks at the family's own
        // pairs — and it is the ceiling the unsupervised per-pair choice is
        // measured against.
        let (fk, fv) = argmax(&profile(promoted, &pairs));
        if !fv.is_finite() {
            continue;
        }
        // Every scorer ranks inside the same candidate set: words the family's
        // channel can place.
        let cand: Vec<&String> = pool_ref
            .iter()
            .copied()
            .filter(|w| promoted.lexicon[*w].evidence_count(fk) > 0)
            .collect();
        if cand.len() < 32 {
            continue;
        }
        let stride = (pairs.len() / 6).max(1);
        let (mut fc, mut ch, mut pd, mut cd, mut n) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0usize);
        let (mut nfc, mut nch, mut npd, mut ncd) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
        let mut chosen: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (i, (a, b)) in pairs.iter().enumerate() {
            for (j, (c, d)) in pairs.iter().enumerate() {
                if i == j || j % stride != i % stride {
                    continue;
                }
                // The pair's own channel: where a and b were most often placed
                // together by the corpus, relative to how much anyone uses it.
                let k = match promoted.lexicon[a]
                    .shared_relation_with(&promoted.lexicon[b], Some(balance))
                {
                    Some(k) => k,
                    None => continue,
                };
                *chosen.entry(k).or_insert(0usize) += 1;
                let decoy = {
                    let mut pick: Option<&String> = None;
                    for t in 0..cand.len() {
                        let w = cand[(i * 7919 + j * 104_729 + t) % cand.len()];
                        if w != d && w != a && w != b && w != c {
                            pick = Some(w);
                            break;
                        }
                    }
                    match pick {
                        Some(w) => w,
                        None => continue,
                    }
                };
                let exclude = [a.as_str(), b.as_str(), c.as_str()];

                // Directed: the relation a→b on a channel is the step from a's
                // source side to b's target side, so d's target side should sit
                // at c's source side plus that step.
                let want_f = promoted.lexicon[c].theta(fk) + promoted.lexicon[b].theta_target(fk)
                    - promoted.lexicon[a].theta(fk);
                let score_fc = |w: &str| (promoted.lexicon[w].theta_target(fk) - want_f).cos();
                let want = promoted.lexicon[c].theta(k) + promoted.lexicon[b].theta_target(k)
                    - promoted.lexicon[a].theta(k);
                let score_ch = |w: &str| (promoted.lexicon[w].theta_target(k) - want).cos();

                let qp = dense_query(promoted, a, b, c);
                let score_pd = |w: &str| qp.resonance(&promoted.lexicon[w]);
                let qc = dense_query(control, a, b, c);
                let score_cd = |w: &str| qc.resonance(&control.lexicon[w]);

                fc += rr(&cand, d, &exclude, score_fc);
                ch += rr(&cand, d, &exclude, score_ch);
                pd += rr(&cand, d, &exclude, score_pd);
                cd += rr(&cand, d, &exclude, score_cd);
                nfc += rr(&cand, decoy, &exclude, score_fc);
                nch += rr(&cand, decoy, &exclude, score_ch);
                npd += rr(&cand, decoy, &exclude, score_pd);
                ncd += rr(&cand, decoy, &exclude, score_cd);
                n += 1;
            }
        }
        if n == 0 {
            continue;
        }
        let f = n as f64;
        let mut top: Vec<(usize, usize)> = chosen.into_iter().collect();
        top.sort_by_key(|x| std::cmp::Reverse(x.1));
        let via: Vec<String> = top
            .iter()
            .take(3)
            .map(|(k, c)| format!("{}×{}", promoted.relation_name(*k), c))
            .collect();
        println!(
            "  {:<12} {:>6} {:>6} {:>9.5} {:>7.1}x {:>9.5} {:>7.1}x {:>9.5} {:>7.1}x {:>9.5} {:>7.1}x   {} / {}",
            fam.name,
            n,
            cand.len(),
            fc / f,
            fc / nfc.max(f64::MIN_POSITIVE),
            ch / f,
            ch / nch.max(f64::MIN_POSITIVE),
            pd / f,
            pd / npd.max(f64::MIN_POSITIVE),
            cd / f,
            cd / ncd.max(f64::MIN_POSITIVE),
            promoted.relation_name(fk),
            via.join(" "),
        );
        tot_fc += fc;
        nul_fc += nfc;
        tot_ch += ch;
        tot_pd += pd;
        tot_cd += cd;
        nul_ch += nch;
        nul_pd += npd;
        nul_cd += ncd;
        tot_n += n;
    }
    if tot_n > 0 {
        let t = tot_n as f64;
        println!(
            "  {:<12} {:>6} {:>6} {:>9.5} {:>7.1}x {:>9.5} {:>7.1}x {:>9.5} {:>7.1}x {:>9.5} {:>7.1}x",
            "ALL",
            tot_n,
            "",
            tot_fc / t,
            tot_fc / nul_fc.max(f64::MIN_POSITIVE),
            tot_ch / t,
            tot_ch / nul_ch.max(f64::MIN_POSITIVE),
            tot_pd / t,
            tot_pd / nul_pd.max(f64::MIN_POSITIVE),
            tot_cd / t,
            tot_cd / nul_cd.max(f64::MIN_POSITIVE),
        );
        println!(
            "  {:<12} {:>6} {:>6} {:>9.5} {:>8} {:>9.5} {:>8} {:>9.5} {:>8} {:>9.5} {:>8}",
            "decoy null", tot_n, "", nul_fc / t, "", nul_ch / t, "", nul_pd / t, "", nul_cd / t, ""
        );
    }
}
