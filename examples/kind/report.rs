//! One line per reading.

use phical::relate::Score;

pub fn line(label: &str, s: &Score, name: &dyn Fn(usize) -> String) {
    let per: Vec<String> = s.per_kind.iter().map(|(k, n, m, w)| format!("{} {m:.3}/{w:.3} ({n})", name(*k))).collect();
    println!("{label:<10} MRR {:.3} · within kind {:.3} · hit@5 {:.1}% · {}", s.mrr, s.within, 100.0 * s.hit5, per.join(" · "));
}
