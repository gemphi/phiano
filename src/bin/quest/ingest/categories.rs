//! Three ways to file a word under one of sixty-four hexagrams, and whether
//! the filing carries the quest's structure.
//!
//! *Centroid* is the word's mean phase over its evidenced channels
//! (`phistd::hexagram::hexagram_of`). *Lines* is the I Ching reading
//! phinet already gives a function: its energy in six octaves
//! (`hexagram::lines`), a line yang when its octave holds more than an even
//! share. *By kind* is the word's source angle on one kind's channel in 64
//! sectors — each kind of the quest gets its own sixty-four categories.
//!
//! A filing carries structure if words that share an answer under a kind
//! share a hexagram more often than two words drawn at random. That ratio is
//! *coherence*; it is judged against the same statistic with the labels
//! shuffled across words, so a number above one means something only when it
//! stands clear of what shuffling produces.
#[allow(unused_imports)]
use super::*;

use phistd::hexagram::{hexagram_of, lines};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Filing {
    Centroid,
    Lines,
    Kind(String),
}

impl Filing {
    pub fn name(&self) -> String {
        match self {
            Filing::Centroid => "centroid".into(),
            Filing::Lines => "lines".into(),
            Filing::Kind(k) => format!("by kind ({k})"),
        }
    }
}

/// The hexagram `word` falls under in `filing`, if it has one.
pub fn file(facet: &Facet, word: &str, filing: &Filing) -> Option<u8> {
    match filing {
        Filing::Centroid => function_of(facet, word).map(|f| hexagram_of(&f)),
        Filing::Lines => function_of(facet, word).map(|f| {
            let e = lines(&f);
            (0..6).fold(0u8, |id, j| id | (((e[j] > 1.0 / 6.0) as u8) << j))
        }),
        Filing::Kind(kind) => {
            let k = facet.marker_channel(kind)?;
            let p = facet.lexicon.get(word)?;
            (p.evidence_count(k) > 0).then(|| p.theta_byte(k) / 4)
        }
    }
}

/// Coherence of `labels` over the pairs that share an answer — under `kind`
/// only, or under any kind — and the number of such pairs.
pub fn coherence(labels: &HashMap<&str, u8>, rows: &[Row], kind: Option<&str>) -> (f64, usize) {
    let n = labels.len() as f64;
    let mut count = [0usize; 64];
    for h in labels.values() {
        count[*h as usize] += 1;
    }
    let base = count.iter().map(|&c| (c * c.saturating_sub(1) / 2) as f64).sum::<f64>() / (n * (n - 1.0) / 2.0).max(1.0);
    let mut groups: HashMap<(&str, &str), Vec<u8>> = HashMap::new();
    for r in rows.iter().filter(|r| kind.is_none_or(|k| r.kind == k)) {
        if let Some(h) = labels.get(r.about.as_str()) {
            groups.entry((r.kind.as_str(), r.other.as_str())).or_default().push(*h);
        }
    }
    let (mut same, mut pairs) = (0usize, 0usize);
    for hs in groups.values() {
        for i in 0..hs.len() {
            for j in (i + 1)..hs.len() {
                pairs += 1;
                same += (hs[i] == hs[j]) as usize;
            }
        }
    }
    match pairs > 0 && base > 0.0 {
        true => ((same as f64 / pairs as f64) / base, pairs),
        false => (0.0, pairs),
    }
}

/// Mean and standard deviation of coherence with the labels shuffled across
/// words, `shuffles` times, deterministically.
pub fn shuffled(labels: &HashMap<&str, u8>, rows: &[Row], kind: Option<&str>, shuffles: usize) -> (f64, f64) {
    let mut words: Vec<&str> = labels.keys().copied().collect();
    words.sort();
    let values: Vec<u8> = words.iter().map(|w| labels[w]).collect();
    let mut xs = Vec::with_capacity(shuffles);
    for s in 0..shuffles {
        let mut order: Vec<usize> = (0..values.len()).collect();
        shuffle(&mut order, 0xC0DE + s as u64);
        let perm: HashMap<&str, u8> = words.iter().zip(&order).map(|(w, &i)| (*w, values[i])).collect();
        xs.push(coherence(&perm, rows, kind).0);
    }
    let m = xs.iter().sum::<f64>() / xs.len().max(1) as f64;
    let v = xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / xs.len().max(1) as f64;
    (m, v.sqrt())
}

/// Every filing against every kind, as a table: coherence and its z against
/// the shuffled labels.
pub fn categories_report(facet: &Facet, rows: &[Row], shuffles: usize) -> String {
    let kinds = kinds(rows);
    let mut abouts: Vec<&str> = rows.iter().map(|r| r.about.as_str()).collect();
    abouts.sort();
    abouts.dedup();
    let mut filings = vec![Filing::Centroid, Filing::Lines];
    filings.extend(kinds.iter().map(|k| Filing::Kind(k.clone())));
    let mut out = format!("| filing | occupied | {} |\n|---|---|{}\n", kinds.join(" | "), "---|".repeat(kinds.len()));
    for filing in &filings {
        let labels: HashMap<&str, u8> = abouts.iter().filter_map(|w| file(facet, w, filing).map(|h| (*w, h))).collect();
        let occupied = labels.values().collect::<std::collections::HashSet<_>>().len();
        let cells: Vec<String> = kinds
            .iter()
            .map(|k| {
                let (c, pairs) = coherence(&labels, rows, Some(k));
                if pairs == 0 {
                    return "—".into();
                }
                let (m, sd) = shuffled(&labels, rows, Some(k), shuffles);
                let z = if sd > 0.0 { (c - m) / sd } else { 0.0 };
                format!("{c:.2}× (z {z:.1})")
            })
            .collect();
        out.push_str(&format!("| {} | {occupied}/64 | {} |\n", filing.name(), cells.join(" | ")));
    }
    out
}
