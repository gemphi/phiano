//! Descriptive Logic III, §§65–88, read off Phi-4's own answers (docs/105,
//! paper 21): family resemblance (§§65–67) and Moses (§79).
//!
//! In Phient's world, a word (w) is nothing but what it relates to, under what
//! conditions (c). If that is so, a category should hold together by
//! overlapping relations rather than one shared one, and a word should
//! survive the loss of any one of its relations.
//!
//! `cargo run --release --bin family -- <relations.tsv> [out.md] [--epochs 30] [--seed 7]`

mod data;
mod moses;
mod resemblance;
mod rough;

use data::Table;
use phiano::relate::{held_out, RelationModel};
use phistd::Adam;
use phical::{relate, FnoConfig, Tie};

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let arg = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)?.parse::<u64>().ok());
    let (epochs, seed) = (arg("--epochs").unwrap_or(30) as usize, arg("--seed").unwrap_or(7));
    let path = a.first().cloned().unwrap_or_else(|| "data/quest/night_2026-09-29/all.tsv".into());
    let out = a.get(1).filter(|x| !x.starts_with("--")).cloned().unwrap_or_else(|| "family.md".into());
    let t = Table::read(&path);
    // `--rough`: only the contradictions (§§107, 125, docs/106).
    if a.iter().any(|x| x == "--rough") {
        let md = rough::report(&t, epochs, &[7, 8, 9, 10]);
        std::fs::write(&out, &md).expect("write report");
        println!("{md}\nwrote {out}");
        return;
    }
    let mut md = format!("# Family resemblance and Moses in Phi-4's answers\n\n{path}: {} rows, {} words. Seed {seed}.\n\n", t.rows.len(), t.rel.len());

    // §§65–67
    let cats = t.categories(5);
    let pool: Vec<String> = t.rel.keys().filter(|w| t.properties(w).len() >= 3).cloned().collect();
    let mut d = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    md += "## Family resemblance (§§65–67)\n\n| category | members | longest fibre | mean overlap (Jaccard) | thread | random sets: fibre / overlap / thread |\n|---|---|---|---|---|---|\n";
    let mut sums = [0.0f64; 6];
    for (c, members) in &cats {
        let f = resemblance::family(&t, members);
        let (mut rf, mut rj, mut rt) = (0.0, 0.0, 0.0);
        for _ in 0..200 {
            let mut set: Vec<String> = Vec::new();
            while set.len() < members.len() {
                d = phistd::mix64(d);
                let w = &pool[(d % pool.len() as u64) as usize];
                if !set.contains(w) {
                    set.push(w.clone());
                }
            }
            let r = resemblance::family(&t, &set);
            rf += r.longest_fibre / 200.0;
            rj += r.mean_jaccard / 200.0;
            rt += r.thread / 200.0;
        }
        md += &format!("| {c} | {} | {:.0}% | {:.3} | {:.0}% | {:.0}% / {:.3} / {:.0}% |\n", members.len(), 100.0 * f.longest_fibre, f.mean_jaccard, 100.0 * f.thread, 100.0 * rf, rj, 100.0 * rt);
        for (s, v) in sums.iter_mut().zip([f.longest_fibre, f.mean_jaccard, f.thread, rf, rj, rt]) {
            *s += v / cats.len() as f64;
        }
    }
    let essence = cats.iter().filter(|(_, m)| resemblance::family(&t, m).longest_fibre >= 1.0).count();
    md += &format!("| **mean over {} categories** | | **{:.0}%** | **{:.3}** | **{:.0}%** | {:.0}% / {:.3} / {:.0}% |\n\nCategories with one relation every member holds: {essence} of {}.\n\n", cats.len(), 100.0 * sums[0], sums[1], 100.0 * sums[2], 100.0 * sums[3], sums[4], 100.0 * sums[5], cats.len());

    // §79: a model on the told rows.
    let told_rows: Vec<_> = t.rows.iter().filter(|r| !held_out(r)).cloned().collect();
    let config = FnoConfig { space: 64, width: 16, layers: 2, modes: 16, margin: 0.3, lr: 5e-3, tie: Tie::Circle };
    let mut m = RelationModel::new(&t.rows, None, config, seed);
    let told: Vec<_> = told_rows.iter().filter_map(|r| m.triple(r)).collect();
    let (pool_ix, _) = relate::pools(&told);
    let examples: Vec<_> = told.iter().map(relate::example).collect();
    let (mut ao, mut at) = (Adam::new(config.lr), Adam::new(config.lr * 0.5));
    for e in 0..epochs {
        m.joint.train_epoch(&examples, &pool_ix, 32, &mut ao, &mut at, seed ^ e as u64);
    }
    let words: Vec<String> = t.rel.keys().cloned().collect();
    let told_src: Vec<&_> = t.rows.iter().filter(|r| !held_out(r)).collect();
    let held_src: Vec<&_> = t.rows.iter().filter(|r| held_out(r)).collect();
    md += &format!("## Moses (§79): which word do some of its relations pick out? ({} candidates, chance top-1 {:.2}%)\n\n| relations kept from | dropped | counts: top-1 / MRR | phinet functions: top-1 / MRR | words |\n|---|---|---|---|---|\n", words.len(), 100.0 / words.len() as f64);
    for (name, src) in [("told", &told_src), ("held out (never told)", &held_src)] {
        let drops: &[f64] = if name == "told" { &[0.0, 0.5, 0.75, 0.9] } else { &[0.0] };
        for &drop in drops {
            let c = moses::identify(&t, &mut m, &words, src, drop, false);
            let f = moses::identify(&t, &mut m, &words, src, drop, true);
            md += &format!("| {name} | {:.0}% | {:.1}% / {:.3} | {:.1}% / {:.3} | {} |\n", 100.0 * drop, 100.0 * c.top1, c.mrr, 100.0 * f.top1, f.mrr, c.n);
            println!("{name} drop {drop}: counts {:.3} functions {:.3}", c.mrr, f.mrr);
        }
    }
    std::fs::write(&out, &md).expect("write report");
    println!("\n{md}\nwrote {out}");
}
