//! A wave measured on held-out text (docs/151–153), on the strata of
//! docs/137: positions whose word was told after the last word or pair, and
//! positions where it never was.
//!
//! - **Forward:** the word `s` places back read `s` steps on, each alone;
//!   their real parts summed (the coherent sum, docs/151's "offsets
//!   summed"); the last word's modulus alone (amplitude without phase); and
//!   the readings and the prior mixed, weights counted on `fit`.
//! - **Backward:** the word before, read from the word after by the same
//!   relation run backwards, against frequency; split by whether the pair
//!   was told.

use super::operator::Reader;
use phiano::relate::Wave;
use phiano::prose::Knowing;
use phical::c64;
use phiecs::quest::Row;

#[derive(Default)]
struct Tally {
    n: usize,
    top1: Vec<usize>,
    rr: Vec<f64>,
}

impl Tally {
    fn add(&mut self, ranks: &[Option<usize>]) {
        self.n += 1;
        self.top1.resize(ranks.len(), 0);
        self.rr.resize(ranks.len(), 0.0);
        for (i, r) in ranks.iter().enumerate() {
            if let Some(r) = r {
                self.top1[i] += (*r == 0) as usize;
                self.rr[i] += 1.0 / (*r + 1) as f64;
            }
        }
    }

    fn row(&self, name: &str) -> String {
        let n = self.n.max(1) as f64;
        let cells: Vec<String> = self.top1.iter().zip(&self.rr).map(|(t, r)| format!("{:.1}% · {:.3}", 100.0 * *t as f64 / n, r / n)).collect();
        format!("| {name} | {} | {} |\n", self.n, cells.join(" | "))
    }
}

/// Where `truth` ranks under `scores` (0 is first).
fn rank(at: Option<usize>, scores: &[f64]) -> Option<usize> {
    let at = at?;
    Some(scores.iter().filter(|s| **s > scores[at]).count())
}

fn parts(c: &Option<Vec<c64>>, f: fn(&c64) -> f64) -> Option<Vec<f64>> {
    c.as_ref().map(|c| c.iter().map(f).collect())
}

fn table(first: &str, head: &[String], known: &Tally, unknown: &Tally, rows: [&str; 2]) -> String {
    let md = format!("| {first} | positions | {} |\n|---|---|{}\n", head.join(" | "), "---|".repeat(head.len()));
    md + &known.row(rows[0]) + &unknown.row(rows[1])
}

/// Both tables for `wave`, reading up to `steps` words back.
pub fn measure(k: &mut Knowing, wave: &mut dyn Wave, steps: usize, fit: &[Row]) -> String {
    let both = k.counts.values_and_prior("next");
    let (values, prior) = (&both.0, &both.1);
    let at: std::collections::HashMap<&str, usize> = values.iter().enumerate().map(|(i, v)| (v.as_str(), i)).collect();
    let next: Vec<Row> = fit.iter().filter(|r| r.kind == "next").cloned().collect();
    let names: Vec<String> = (1..=steps).map(|s| wave.under(s)).collect();
    let reader = {
        let mut real = |w: &str, d: usize| parts(&wave.ahead(w, d + 1, values), |z| z.re);
        Reader::fit_with(&mut real, names.clone(), &next, values, prior, 2000)
    };
    let (mut known, mut unknown, mut told_back, mut new_back) = (Tally::default(), Tally::default(), Tally::default(), Tally::default());
    let (mut nats, mut asked) = (0.0, 0usize);
    for (_, words) in k.held.clone() {
        for i in 1..words.len() {
            let (last, truth) = (&words[i - 1], &words[i]);
            // Backward: the word before, from the word after.
            let (b, t) = (wave.back(truth, values), at.get(last.as_str()).copied());
            let ranks = [parts(&b, |z| z.re).and_then(|c| rank(t, &c)), parts(&b, |z| z.norm()).and_then(|c| rank(t, &c)), rank(t, prior)];
            match k.counts.count(Some(last), "next", truth) > 0.0 {
                true => told_back.add(&ranks),
                false => new_back.add(&ranks),
            }
            if i < 2 {
                continue;
            }
            let pair = format!("pair:{} {}", words[i - 2], last);
            let told = k.counts.count(Some(last), "next", truth) > 0.0 || k.counts.count(Some(&pair), "next", truth) > 0.0;
            let t = at.get(truth.as_str()).copied();
            let each: Vec<Option<Vec<c64>>> = (1..=steps).map(|s| if i >= s { wave.ahead(&words[i - s], s, values) } else { None }).collect();
            let mut ranks: Vec<Option<usize>> = each.iter().map(|c| parts(c, |z| z.re).and_then(|c| rank(t, &c))).collect();
            let summed = each.iter().flatten().fold(None, |s: Option<Vec<f64>>, c| Some(c.iter().enumerate().map(|(j, z)| z.re + s.as_ref().map_or(0.0, |s| s[j])).collect()));
            ranks.push(summed.and_then(|s| rank(t, &s)));
            ranks.push(parts(&each[0], |z| z.norm()).and_then(|c| rank(t, &c)));
            let mut real = |w: &str, d: usize| parts(&wave.ahead(w, d + 1, values), |z| z.re);
            let mixed = reader.read_with(&mut real, &words[..i]);
            if let (Some(d), Some(t)) = (&mixed, t) {
                nats -= d[t].max(1e-300).ln();
                asked += 1;
            }
            ranks.push(mixed.and_then(|c| rank(t, &c)));
            match told {
                true => known.add(&ranks),
                false => unknown.add(&ranks),
            }
        }
    }
    let mut head: Vec<String> = names.iter().enumerate().map(|(s, n)| format!("{} back · {n}", s + 1)).collect();
    head.extend(["the readings summed".into(), "1 back, amplitude only".into(), "the readings and the prior, mixed".into()]);
    let mut md = table("the word that came was…", &head, &known, &unknown, ["told after the last word or pair", "never told after either"]);
    md += &format!("\nmixed: {} · perplexity of the mixed reading alone {:.1} over {asked} positions\n\n**Backward** — the word before, read from the word after:\n\n", reader.note(), (nats / asked.max(1) as f64).exp());
    let head = ["the relation read backwards".to_string(), "backwards, amplitude only".into(), "frequency".into()];
    md + &table("the word before…", &head, &told_back, &new_back, ["the pair was told", "the pair was never told"])
}
