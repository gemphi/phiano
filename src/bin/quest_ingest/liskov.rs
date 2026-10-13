//! The three operators over a quest, and the hexagram each word falls under.
//!
//! Liskov (1968) played endgames with two comparison functions, better and
//! worse, coordinated by a forcing tree, and no evaluation number. A quest
//! is scored the same way: for every row, *better* is how far the truth beats
//! the best wrong word on the kind's channel; *worse* is the refutation, how
//! far the truth's own strongest onward reading falls below its fit here (a
//! word that leads nowhere); *loss* is the hinge over better. Each word is
//! then filed under the hexagram its 64-channel function is (`phistd::
//! hexagram::hexagram_of`), so a quest reports itself in sixty-four
//! categories and the patterns that emerge are per hexagram, not per word.
#[allow(unused_imports)]
use super::*;

use phiano::wave::c64;
use phistd::hexagram::hexagram_of;

pub const MARGIN: f64 = 0.1;

#[derive(Clone, Debug, Default)]
pub struct WordScore {
    pub word: String,
    pub hexagram: u8,
    pub rows: usize,
    pub better: f64,
    pub worse: f64,
    pub loss: f64,
}

/// A word's 64-channel function: its source angles as unit phasors on the
/// channels that carry evidence, zero elsewhere, normalised.
pub fn function_of(facet: &Facet, word: &str) -> Option<Vec<c64>> {
    let p = facet.lexicon.get(word)?;
    let mut f: Vec<c64> = (0..64)
        .map(|k| match p.evidence_count(k) > 0 {
            true => c64::from_polar(1.0, p.theta(k)),
            false => c64::new(0.0, 0.0),
        })
        .collect();
    phistd::normalise(&mut f).then_some(f)
}

/// Score every `about` word of the table.
pub fn score_words(facet: &Facet, rows: &[Row]) -> Vec<WordScore> {
    let kinds = kinds(rows);
    let pool = target_pool(rows);
    let mut out: Vec<WordScore> = Vec::new();
    let mut words: Vec<&str> = rows.iter().map(|r| r.about.as_str()).collect();
    words.sort();
    words.dedup();
    for w in words {
        let Some(f) = function_of(facet, w) else { continue };
        let mut s = WordScore { word: w.to_string(), hexagram: hexagram_of(&f), ..Default::default() };
        for r in rows.iter().filter(|r| r.about == w) {
            let Some(k) = facet.marker_channel(&r.kind) else { continue };
            let fit = facet.on(w, &r.other, k);
            // Better against the *expected* wrong word, and the loss the
            // trainer's hinge sees in expectation over a random decoy. The
            // first version compared against the single best wrong word,
            // which on a 256-quantum channel ties almost always and read
            // flat (docs/86 §3).
            let wrongs: Vec<f64> = pool.iter().filter(|x| **x != r.other).map(|x| facet.on(w, x, k)).collect();
            let nw = wrongs.len().max(1) as f64;
            let better = fit - wrongs.iter().sum::<f64>() / nw;
            let hinge = wrongs.iter().map(|o| (MARGIN - (fit - o)).max(0.0)).sum::<f64>() / nw;
            // The truth's strongest onward reading under any kind.
            let onward = kinds.iter().flat_map(|kk| facet.under(&r.other, kk, &pool).into_iter().map(|(_, v)| v).next()).fold(f64::NEG_INFINITY, f64::max);
            let worse = match onward.is_finite() {
                true => fit - onward,
                false => 0.0,
            };
            s.rows += 1;
            s.better += better;
            s.worse += worse;
            s.loss += hinge;
        }
        if s.rows > 0 {
            let n = s.rows as f64;
            s.better /= n;
            s.worse /= n;
            s.loss /= n;
            out.push(s);
        }
    }
    out
}

/// The scores as a table, and the sixty-four categories as a summary.
pub fn liskov_report(scores: &[WordScore]) -> (String, String) {
    let mut tsv = String::from("word\thexagram\tid\trows\tbetter\tworse\tloss\n");
    for s in scores {
        tsv.push_str(&format!("{}\t{}\t{}\t{}\t{:.3}\t{:.3}\t{:.3}\n", s.word, glyph(s.hexagram), s.hexagram, s.rows, s.better, s.worse, s.loss));
    }
    let mut by = vec![(0usize, 0.0f64, 0.0f64, 0.0f64); 64];
    for s in scores {
        let b = &mut by[s.hexagram as usize];
        b.0 += 1;
        b.1 += s.better;
        b.2 += s.worse;
        b.3 += s.loss;
    }
    let mut md = String::from("| hexagram | id | words | mean better | mean worse | mean loss |\n|---|---|---|---|---|---|\n");
    for (id, (n, b, w, l)) in by.iter().enumerate() {
        if *n == 0 {
            continue;
        }
        let nf = *n as f64;
        md.push_str(&format!("| {} | {id} | {n} | {:.3} | {:.3} | {:.3} |\n", glyph(id as u8), b / nf, w / nf, l / nf));
    }
    let occupied = by.iter().filter(|b| b.0 > 0).count();
    md.push_str(&format!("\n{occupied} of 64 hexagrams occupied by {} words.\n", scores.len()));
    (tsv, md)
}

pub fn glyph(id: u8) -> String {
    char::from_u32(0x4DC0 + id as u32).map(String::from).unwrap_or_default()
}
