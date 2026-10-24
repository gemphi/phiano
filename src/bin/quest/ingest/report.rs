//! What a run prints and writes when it ends: the channels, the held-out
//! rows, and the three operators filed by hexagram.
#[allow(unused_imports)]
use super::*;

/// Expected MRR of a uniformly random ranking over `n` candidates.
fn chance_mrr(n: usize) -> f64 {
    (1..=n).map(|r| 1.0 / r as f64).sum::<f64>() / n.max(1) as f64
}

/// The per-kind table and a dozen rows read back from the facet.
pub fn summary_report(facet: &Facet, rows: &[Row], pool: &[String], markers: usize, secs: f64, report: &mut String) {
    let r = measure(facet, rows);
    let chance = chance_mrr(pool.len());
    println!(
        "\n{} marker channels · chance MRR {chance:.4} · {secs:.1}s\nkind channel {:.3} ({:.0}× chance) · within kind {:.3} · wrong channel {:.3} · offset {:.3}",
        markers,
        r.kind.mrr(),
        r.kind.mrr() / chance,
        r.within.mrr(),
        r.wrong.mrr(),
        r.offset.mrr()
    );
    report.push_str(&format!("\nchance MRR {chance:.4} · {} marker channels · {secs:.1}s\n\n| kind | channel | rows | pool | MRR | hit@1 | hit@5 | within: MRR | hit@1 | chance |\n|---|---|---|---|---|---|---|---|---|---|\n", markers));
    for (k, s, w) in &r.per_kind {
        let ch = facet.marker_channel(k).map(|c| facet.relation_name(c)).unwrap_or_default();
        let pool_k = kind_pool(rows, k).len();
        let line = format!(
            "| {k} | {ch} | {} | {} | {:.3} | {:.1}% | {:.1}% | {:.3} | {:.1}% | {:.3} |",
            s.n,
            pool_k,
            s.mrr(),
            100.0 * s.hit1(),
            100.0 * s.hit5(),
            w.mrr(),
            100.0 * w.hit1(),
            chance_mrr(pool_k)
        );
        println!("{line}");
        report.push_str(&line);
        report.push('\n');
    }
    report.push_str("\n| about | kind | quest said | facet reads back (whole pool) | within kind |\n|---|---|---|---|---|\n");
    println!();
    for row in rows.iter().step_by((rows.len() / 12).max(1)).take(12) {
        let Some(k) = facet.marker_channel(&row.kind) else { continue };
        let (w, s) = best_on(facet, pool, &row.about, k).unwrap_or_default();
        let (wk, sk) = best_on(facet, &kind_pool(rows, &row.kind), &row.about, k).unwrap_or_default();
        let line = format!("| {} | {} | {} | {w} ({s:.2}) | {wk} ({sk:.2}) |", row.about, row.kind, row.other);
        println!("{line}");
        report.push_str(&line);
        report.push('\n');
    }
}

/// Held-out rows, read one hop and two.
pub fn holdout_report(facet: &Facet, kinds: &[String], held: &[Row], all_rows: &[Row], report: &mut String) {
    let pool_all = target_pool(all_rows);
    let mut line = format!(
        "\nheld out {} rows · pool {} · chance MRR {:.3}\n\n| kind | held | one hop MRR | hit@5 | two hops MRR | hit@5 |\n|---|---|---|---|---|---|\n",
        held.len(),
        pool_all.len(),
        chance_mrr(pool_all.len())
    );
    for (k, one, two) in measure_holdout(facet, kinds, held, &pool_all) {
        line.push_str(&format!(
            "| {k} | {} | {:.3} | {:.1}% | {:.3} | {:.1}% |\n",
            one.n,
            one.mrr(),
            100.0 * one.hit5(),
            two.mrr(),
            100.0 * two.hit5()
        ));
    }
    print!("{line}");
    report.push_str(&line);
}

/// Liskov's three operators over every word, filed under its hexagram.
pub fn operators_report(facet: &Facet, all_rows: &[Row], path: &str, report: &mut String) {
    let scores = score_words(facet, all_rows);
    let (tsv, md) = liskov_report(&scores);
    let fb = board(facet, &scores, all_rows, None);
    let coh: Vec<String> = fb.kinds.iter().map(|(k, o, c, top)| format!("{k} {c:.1}× (a perfect filing {top:.1}×) over {o}/64")).collect();
    let scores_path = format!("{}.scores.tsv", path.trim_end_matches(".tsv"));
    std::fs::write(&scores_path, tsv).unwrap_or_else(|e| eprintln!("cannot write {scores_path}: {e}"));
    let mean = |f: fn(&WordScore) -> f64| scores.iter().map(f).sum::<f64>() / scores.len().max(1) as f64;
    let line = format!(
        "\nLiskov's three operators over {} words (margin {MARGIN}; better and loss against the expected decoy): mean better {:.3} · mean worse {:.3} · mean loss {:.3} · hexagram coherence by kind {} · scores in {scores_path}\n\n{md}",
        scores.len(),
        mean(|s| s.better),
        mean(|s| s.worse),
        mean(|s| s.loss),
        coh.join(", ")
    );
    print!("{line}");
    report.push_str(&line);
    let table = categories_report(facet, all_rows, 200);
    let line = format!("\n### Sixty-four categories: which filing carries the quest (coherence vs 200 shuffles)\n\n{table}");
    print!("{line}");
    report.push_str(&line);
}
