//! The hierarchy, one rung at a time (docs/156). The reader of `Knowing::next`
//! is built from levels: the prior, then the path's scopes, then the last
//! word (and the word under each scope), then the last pair, then a base the
//! levels move from — the last word's kin (`Counts::kin_base`, docs/138) or a
//! trained operator. This binary turns the rungs on one at a time and asks,
//! on held-out *Emma*, what each level adds — split by stratum: positions
//! whose word was told after the last word or pair (**told**), and positions
//! where it never was (**open**, the standing problem of docs/152).
//!
//! Nothing is trained here; the counts are learned once and read. The point
//! is to see where each rung helps, and whether the kin rung moves the open
//! stratum past frequency.
//!
//! `cargo run --release --bin ladder -- data/text --in text/emma [--held-out 0.05] [--seed 7] [--n 1500] [--tau 0.5] [--alpha 0.5] [--gate 0.12]`

use phiano::prose::{heldout, Knowing};
use phiano::relate::{scopes, SAYS};

mod lex;
mod rungs;
use lex::Lex;
use rungs::*;

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let num = |a: &mut Vec<String>, f: &str, d: f64| flag(a, f).and_then(|s| s.parse().ok()).unwrap_or(d);
    let within = flag(&mut a, "--in").unwrap_or_else(|| "text/emma".into());
    let (share, seed) = (num(&mut a, "--held-out", 0.05), num(&mut a, "--seed", 7.0) as u64);
    let n = num(&mut a, "--n", 1500.0) as usize;
    let tau = num(&mut a, "--tau", 0.5);
    let alpha = num(&mut a, "--alpha", 0.5);
    let gate = num(&mut a, "--gate", 0.12);
    let frame = flag(&mut a, "--frame").unwrap_or_else(|| "sense".into());
    let near_file = flag(&mut a, "--near").unwrap_or_else(|| "data/lex/near.tsv".into());
    let root = std::path::PathBuf::from(a.first().map(String::as_str).unwrap_or("data/text"));

    let held = heldout::held(share, seed);
    let (mut story, _) = Knowing::learn_except(&root, &held);
    story.within = scopes(&within, 0);
    let vp = story.counts.values_and_prior("next");
    let (values, prior) = (&vp.0, &vp.1);
    let at: std::collections::HashMap<&str, usize> = values.iter().enumerate().map(|(i, v)| (v.as_str(), i)).collect();
    let lex = Lex::load(&root, &near_file, &frame, values, prior);
    let Lex { near, near_rows, bucket_code, global_pos, real_prior, .. } = &lex;
    let n_buckets = lex.n_buckets;
    let mut pos_frame: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();

    // Every readable position across the held-out paragraphs, then sampled.
    // Measure only on held-out Emma; any other scope (the dictionary-use
    // corpus) is training, not a test.
    let mut all: Vec<(usize, usize)> = Vec::new();
    for (pi, (path, words)) in story.held.iter().enumerate() {
        if !path.contains("emma") {
            continue;
        }
        for i in 1..words.len() {
            if words[i].chars().any(char::is_alphanumeric) {
                all.push((pi, i));
            }
        }
    }
    let step = (all.len() / n.max(1)).max(1);
    let keep: Vec<(usize, usize)> = all.into_iter().step_by(step).take(n).collect();

    let mut told = vec![Tally::default(); RUNGS.len()];
    let mut open = vec![Tally::default(); RUNGS.len()];
    let (mut n_told, mut n_open) = (0usize, 0usize);

    for (pi, i) in keep {
        let words = &story.held[pi].1;
        let truth = &words[i];
        let Some(&t) = at.get(truth.as_str()) else { continue };
        let said = &words[..i];
        let last = said.last().cloned().unwrap_or_default();
        let pair = if said.len() >= 2 { format!("pair:{} {}", said[said.len() - 2], said[said.len() - 1]) } else { String::new() };
        let is_told = story.counts.count(Some(&last), "next", truth) > 0.0 || (!pair.is_empty() && story.counts.count(Some(&pair), "next", truth) > 0.0);

        let (scope_levels, word_levels, pair_levels) = levels_for(&story, said);
        let mut lv: Vec<Vec<String>> = Vec::new();
        let mut scores = vec![prior.clone(); 1];
        lv.extend(scope_levels.iter().cloned());
        scores.push(story.counts.read_levels(&lv, "next", values, tau).0);
        lv.extend(word_levels.iter().cloned());
        scores.push(story.counts.read_levels(&lv, "next", values, tau).0);
        lv.extend(pair_levels.iter().cloned());
        scores.push(story.counts.read_levels(&lv, "next", values, tau).0);
        let kin_scores = match story.counts.kin_base(&last, "next", values) {
            Some(base) => story.counts.read_levels_from(&lv, "next", values, tau, base).0,
            None => scores[3].clone(),
        };
        scores.push(kin_scores);
        let dict_scores = smooth(&scores[3], &near, alpha);
        scores.push(dict_scores);
        let frame = pos_frame
            .entry(last.clone())
            .or_insert_with(|| {
                let mut f = vec![0.0f64; n_buckets];
                for (j, v) in values.iter().enumerate() {
                    let n = story.counts.count(Some(&last), "next", v);
                    if n > 0.0 {
                        f[bucket_code[j]] += n;
                    }
                }
                let tot: f64 = f.iter().sum();
                for p in 0..n_buckets {
                    f[p] = (f[p] + 0.5 * global_pos[p]) / (tot + 0.5);
                }
                f
            })
            .clone();
        let pos_scores: Vec<f64> = prior.iter().enumerate().map(|(j, p)| (p * frame[bucket_code[j]]).max(1e-300).ln()).collect();
        scores.push(pos_scores);
        let framed_prior: Vec<f64> = {
            let raw: Vec<f64> = prior.iter().enumerate().map(|(j, p)| p * frame[bucket_code[j]]).collect();
            let z: f64 = raw.iter().sum();
            raw.iter().map(|x| x / z).collect()
        };
        let both = story.counts.read_levels_from(&lv, "next", values, tau, framed_prior).0;
        scores.push(both);
        let mix: Vec<f64> = scores[3].iter().zip(&scores[6]).map(|(a, b)| (0.5 * a.exp() + 0.5 * b.exp()).max(1e-300).ln()).collect();
        scores.push(mix);
        let confidence = scores[3].iter().cloned().fold(f64::MIN, f64::max).exp();
        let routed = if confidence >= gate { scores[7].clone() } else { scores[6].clone() };
        scores.push(routed);
        // Rung 10: the real-world prior alone (wordfreq).
        scores.push(real_prior.iter().map(|p| p.max(1e-300).ln()).collect());
        // Rung 11: the real-world prior re-weighted by the POS frame — a word
        // Emma never said, ranked by how common it is in that grammatical slot.
        let rp_frame: Vec<f64> = real_prior.iter().enumerate().map(|(j, p)| (p * frame[bucket_code[j]]).max(1e-300).ln()).collect();
        scores.push(rp_frame);
        // Rung 12: the working reader. The levels move from the real-world,
        // POS-framed prior — told positions climb, open positions rest on it.
        let real_framed: Vec<f64> = {
            let raw: Vec<f64> = real_prior.iter().enumerate().map(|(j, p)| p * frame[bucket_code[j]]).collect();
            let z: f64 = raw.iter().sum();
            raw.iter().map(|x| x / z).collect()
        };
        let working = story.counts.read_levels_from(&lv, "next", values, tau, real_framed).0;
        scores.push(working);
        let _ = confidence;

        let bucket = if is_told { &mut told } else { &mut open };
        if is_told { n_told += 1 } else { n_open += 1 }
        for (r, s) in scores.iter().enumerate() {
            bucket[r].add(s, t);
        }
    }

    println!("# The ladder of levels, on held-out Emma\n");
    println!("{} positions sampled · {n_told} told, {n_open} open · seed {seed} · tau {tau} · alpha {alpha} · gate {gate} · frame={frame} ({n_buckets} buckets) · {} words · {near_rows} dict links\n", n_told + n_open, values.len());
    println!("Each rung adds one level of the hierarchy. top-1 · MRR, by stratum.\n");
    println!("| rung | told ({n_told}) | open ({n_open}) |");
    println!("|---|---|---|");
    for r in 0..RUNGS.len() {
        println!("| {} | {} | {} |", RUNGS[r], told[r].cell(), open[r].cell());
    }
    println!("\nFrequency is rung 0's open cell; a rung that beats it on the open stratum is reading what was never told.");
}
