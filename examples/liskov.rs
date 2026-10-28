//! The Liskov readings, measured on Austen.
//!
//! 1. Substitution: a word may stand in for another wherever every relation
//!    the context demands still holds (`Facet::relational_fit`). Cloze on
//!    held-out sentences — recover the masked content word from a pool —
//!    and substitution proper: put the best stand-in in her sentence and let
//!    a Kneser-Ney model of her say how much it minds.
//! 2. Endgame: compose toward a goal word (`Generator::with_goal`), a sweep of
//!    pull weights; reach rate, tokens to reach, and perplexity of the text.
//!
//! `cargo run --release --bin liskov`

use phiecs::phasor::facet::Facet;
use phiano::generate::{ContextWaveBuffer, Generator};
use phiano::learn::GenWeight;
use phient::metrics::harness::Harness;
use phient::metrics::baseline::kn::KneserNey;
use phiecs::phasor::facet::storage::Storage;
use phiano::tokenizer::Tokenizer;
use std::fmt::Write as _;

use phiano::lab::sentences_of;

fn dir_sentences(dir: &str) -> Vec<String> {
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        let mut paths: Vec<_> = rd.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.extension().is_some_and(|x| x == "txt")).collect();
        paths.sort();
        for p in paths {
            if p.file_stem().unwrap().to_string_lossy().starts_with("README") {
                continue;
            }
            out.extend(sentences_of(p.to_str().unwrap()));
        }
    }
    out
}

fn main() {
    let austen = dir_sentences("data/corpora/austen");
    let split = Harness::split(austen, 7);
    let facet: Facet = Storage::load("data/austen_voice.chroma").expect("run bin/austen first");
    let kn = KneserNey::train(&split.train);
    let mut md = String::new();
    writeln!(md, "## The Liskov readings (`bin/liskov`, Austen voice lexicon)\n").unwrap();

    // ── pool: content words seen ≥ 20 times ──
    let mut pool: Vec<(String, u32)> = facet
        .lexicon
        .iter()
        .filter(|(w, p)| p.count >= 40 && !Tokenizer::is_function_word(w) && w.chars().all(|c| c.is_ascii_alphabetic()))
        .map(|(w, p)| (w.clone(), p.count))
        .collect();
    pool.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let chance = { let n = pool.len() as f64; (1..=pool.len()).map(|k| 1.0 / k as f64).sum::<f64>() / n };
    let in_pool: std::collections::HashSet<&str> = pool.iter().map(|(w, _)| w.as_str()).collect();

    // ── 1. cloze and substitution ──
    let mut positions: Vec<(Vec<String>, usize)> = Vec::new();
    for s in &split.test {
        let t = Tokenizer::tokenize(s);
        if t.len() < 6 {
            continue;
        }
        for i in 2..t.len() - 2 {
            if in_pool.contains(t[i].as_str()) {
                positions.push((t.clone(), i));
                break;
            }
        }
        if positions.len() >= 200 {
            break;
        }
    }
    let mrr = |rank_of: &dyn Fn(&[String], usize) -> Option<usize>| -> f64 {
        let mut s = 0.0;
        for (t, i) in &positions {
            if let Some(r) = rank_of(t, *i) {
                s += 1.0 / (r as f64 + 1.0);
            }
        }
        s / positions.len() as f64
    };
    let rank_by = |t: &[String], i: usize, score: &dyn Fn(&str) -> f64| -> Option<usize> {
        let mut sc: Vec<(f64, &str)> = pool.iter().map(|(w, _)| (score(w), w.as_str())).collect();
        sc.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        sc.iter().position(|(_, w)| *w == t[i])
    };
    let mrr_count = mrr(&|t, i| rank_by(t, i, &|w| facet.lexicon[w].count as f64));
    let mrr_left = mrr(&|t, i| rank_by(t, i, &|w| facet.relational_score(&t[..i], w).0));
    let mrr_fit = mrr(&|t, i| rank_by(t, i, &|w| facet.relational_fit(t, i, w).0));
    let mrr_fit_n = mrr(&|t, i| rank_by(t, i, &|w| { let (m, n) = facet.relational_fit(t, i, w); m * (n as f64).ln_1p() }));
    // The learned scorer's own trade between the store (count) and the
    // Difference-Engine reading (relational), reused as fit: w[0]·ln count + w[6]·fit.
    let lw = GenWeight::load("data/learned_austen.json").map(|w| w.w).unwrap_or([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
    let mrr_mix = mrr(&|t, i| rank_by(t, i, &|w| lw[0] * (facet.lexicon[w].count as f64).ln() + lw[6] * facet.relational_fit(t, i, w).0));
    writeln!(md, "**Cloze** — recover a masked content word from a pool of {} (seen ≥ 40 times), {} held-out positions, chance MRR {:.4}:\n", pool.len(), positions.len(), chance).unwrap();
    writeln!(md, "| reading | MRR | × chance |\n|---|---|---|").unwrap();
    for (name, m) in [("most frequent word", mrr_count), ("relational, left context only (`relational_score`)", mrr_left), ("relational, both sides (`relational_fit`)", mrr_fit), ("`relational_fit` × ln(1 + pairs)", mrr_fit_n), ("learned trade: w_count·ln count + w_relational·fit (weights from `learned_austen.json`)", mrr_mix)] {
        writeln!(md, "| {name} | {m:.4} | {:.1}× |", m / chance).unwrap();
    }

    // substitution proper
    let judge = |t: &[String]| kn.perplexity(&[t.join(" ")]);
    let (mut p_orig, mut p_fit, mut p_sim, mut p_rand, mut n) = (0.0, 0.0, 0.0, 0.0, 0usize);
    let mut examples: Vec<String> = Vec::new();
    let mut seed = 99u64;
    for (t, i) in positions.iter().take(200) {
        let orig = &t[*i];
        let best = |score: &dyn Fn(&str) -> f64| -> String {
            pool.iter().filter(|(w, _)| w != orig).map(|(w, _)| (score(w), w)).max_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).map(|(_, w)| w.clone()).unwrap()
        };
        let fit = best(&|w| facet.relational_fit(t, *i, w).0);
        let sim = best(&|w| facet.similarity(orig, w));
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let rnd = pool[(seed >> 33) as usize % pool.len()].0.clone();
        let sub = |w: &str| { let mut u = t.clone(); u[*i] = w.to_string(); u };
        p_orig += judge(t).ln();
        p_fit += judge(&sub(&fit)).ln();
        p_sim += judge(&sub(&sim)).ln();
        p_rand += judge(&sub(&rnd)).ln();
        n += 1;
        if examples.len() < 8 {
            examples.push(format!("- {} → **{}** (fit) / {} (similar) — *{}*", orig, fit, sim, t.join(" ")));
        }
    }
    let g = |x: f64| (x / n as f64).exp();
    writeln!(md, "\n**Substitution** — the best stand-in (never the original) put into her sentence, judged by Kneser-Ney (geo. mean over {n} sentences; lower = she minds less):\n").unwrap();
    writeln!(md, "| sentence | perplexity |\n|---|---|\n| as written | {:.1} |\n| `relational_fit` stand-in (Liskov: every demanded relation holds) | {:.1} |\n| most similar word (`similarity`: the same sides, a point-like reading) | {:.1} |\n| random pool word | {:.1} |\n", g(p_orig), g(p_fit), g(p_sim), g(p_rand)).unwrap();
    writeln!(md, "{}\n", examples.join("\n")).unwrap();

    // ── 2. endgame ──
    let learned = GenWeight::load("data/learned_austen.json").map(|w| w.for_decoding());
    let mut games: Vec<(String, String)> = Vec::new();
    for s in &split.test {
        let t = Tokenizer::tokenize(s);
        if t.len() < 10 {
            continue;
        }
        if let Some(goal) = t[6..].iter().find(|w| in_pool.contains(w.as_str()) && !t[..3].contains(w)) {
            games.push((t[..3].join(" "), goal.clone()));
        }
        if games.len() >= 40 {
            break;
        }
    }
    writeln!(md, "**Endgame** — {} prompts (three words of a held-out sentence) with a goal word that sentence reaches later; 30 tokens, temperature 0.3, learned scorer; the pull is `weight · calls_for(cand, goal)` added to every logit until the goal is said:\n", games.len()).unwrap();
    writeln!(md, "| pull weight | reached | tokens to goal (mean, when reached) | KN perplexity | distinct |\n|---|---|---|---|---|").unwrap();
    let mut shown = String::new();
    for w in [0.0, 0.5, 1.0, 2.0, 4.0] {
        let (mut reached, mut steps, mut lp, mut dist, mut n) = (0usize, 0usize, 0.0, 0.0, 0usize);
        for (gi, (prompt, goal)) in games.iter().enumerate() {
            let mut g = Generator::deterministic(30, 0.3, 700 + gi as u64).with_weights(learned.clone());
            if w > 0.0 {
                g = g.with_goal(goal, w);
            }
            let mut buf = ContextWaveBuffer::new(64);
            let text = g.generate(&facet, &mut buf, prompt);
            let toks = Tokenizer::tokenize(&text);
            if toks.len() < 3 {
                continue;
            }
            if let Some(p) = toks.iter().position(|t| t == goal) {
                reached += 1;
                steps += p + 1;
            }
            lp += kn.perplexity(&[format!("{prompt} {text}")]).ln();
            let uniq: std::collections::HashSet<&String> = toks.iter().collect();
            dist += uniq.len() as f64 / toks.len() as f64;
            n += 1;
            if gi < 3 && (w == 0.0 || w == 2.0) {
                writeln!(shown, "- pull {w} · *{prompt}* → {goal}: {text}").unwrap();
            }
        }
        writeln!(md, "| {w} | {reached}/{n} | {} | {:.1} | {:.3} |", if reached > 0 { format!("{:.1}", steps as f64 / reached as f64) } else { "—".into() }, (lp / n.max(1) as f64).exp(), dist / n.max(1) as f64).unwrap();
    }
    writeln!(md, "\n{shown}").unwrap();
    std::fs::write("docs/papers/liskov_section.md", &md).expect("write");
    print!("{md}");
}
