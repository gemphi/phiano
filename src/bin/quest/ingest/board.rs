//! The scoreboard: the three operators over every word, filed under each
//! kind's sixty-four hexagrams, reduced to one line a person watches while
//! training runs.
//!
//! Each kind of the quest files its words by their source angle on that
//! kind's channel, in 64 sectors (`categories::Filing::Kind`). That filing,
//! not the word's centroid, is where the quest's structure shows (docs/86
//! §3): a kind whose words share answers should see those words share a
//! hexagram, and *coherence* is how many times more often they do than two
//! words at random. Kinds where no two words share an answer have nothing to
//! cohere and are left out of the line.
#[allow(unused_imports)]
use super::*;

use std::collections::HashMap;

/// A word's hexagram under each kind: `(word, kind) → hexagram`.
pub type Labels = HashMap<(String, String), u8>;

pub struct Board {
    pub better: f64,
    pub worse: f64,
    pub loss: f64,
    /// Per kind with shared answers: (kind, occupied hexagrams, coherence,
    /// ceiling). The ceiling is the coherence of a perfect filing — one
    /// hexagram per answer, no two answers sharing one.
    pub kinds: Vec<(String, usize, f64, f64)>,
    /// Share of (word, kind) filings unchanged since the previous board.
    pub stable: Option<f64>,
    pub labels: Labels,
}

/// The kinds under which at least two words share an answer.
pub fn shared_kinds(rows: &[Row]) -> Vec<String> {
    let mut seen: HashMap<(&str, &str), usize> = HashMap::new();
    for r in rows {
        *seen.entry((r.kind.as_str(), r.other.as_str())).or_default() += 1;
    }
    kinds(rows).into_iter().filter(|k| seen.iter().any(|((kk, _), n)| kk == k && *n > 1)).collect()
}

pub fn board(facet: &Facet, scores: &[WordScore], rows: &[Row], previous: Option<&Labels>) -> Board {
    let n = scores.len().max(1) as f64;
    let mean = |f: fn(&WordScore) -> f64| scores.iter().map(f).sum::<f64>() / n;
    let mut labels = Labels::new();
    let mut per_kind = Vec::new();
    for kind in shared_kinds(rows) {
        let filing = Filing::Kind(kind.clone());
        let l: HashMap<&str, u8> = scores.iter().filter_map(|s| file(facet, &s.word, &filing).map(|h| (s.word.as_str(), h))).collect();
        let occupied = l.values().collect::<std::collections::HashSet<_>>().len();
        let (c, _) = coherence(&l, rows, Some(&kind));
        per_kind.push((kind.clone(), occupied, c, ceiling(&l, rows, &kind)));
        for (w, h) in l {
            labels.insert((w.to_string(), kind.clone()), h);
        }
    }
    let stable = previous.map(|prev| {
        let kept = labels.iter().filter(|(k, h)| prev.get(*k) == Some(*h)).count();
        kept as f64 / labels.len().max(1) as f64
    });
    Board { better: mean(|s| s.better), worse: mean(|s| s.worse), loss: mean(|s| s.loss), kinds: per_kind, stable, labels }
}

/// Coherence of the perfect filing for `kind`: words that share an answer,
/// directly or through a chain (cat and mammal share `has fur`, mammal and
/// pet `has mammary-glands`), in one hexagram, every such group its own.
/// Then every pair sharing an answer shares a hexagram, and the base rate is
/// the chance two words land in one group. With one answer per word (the
/// synthetic quest) a group is an answer's family, as before; with several
/// (Phi-4) a word would otherwise be counted under only its last answer.
pub fn ceiling(labels: &HashMap<&str, u8>, rows: &[Row], kind: &str) -> f64 {
    let words: Vec<&str> = labels.keys().copied().collect();
    let index: HashMap<&str, usize> = words.iter().enumerate().map(|(i, w)| (*w, i)).collect();
    let mut parent: Vec<usize> = (0..words.len()).collect();
    fn root(p: &mut [usize], mut i: usize) -> usize {
        while p[i] != i {
            p[i] = p[p[i]];
            i = p[i];
        }
        i
    }
    let mut first: HashMap<&str, usize> = HashMap::new();
    for r in rows.iter().filter(|r| r.kind == kind) {
        let Some(&w) = index.get(r.about.as_str()) else { continue };
        let f = *first.entry(r.other.as_str()).or_insert(w);
        let (a, b) = (root(&mut parent, f), root(&mut parent, w));
        parent[a] = b;
    }
    let mut size: HashMap<usize, usize> = HashMap::new();
    for i in 0..words.len() {
        *size.entry(root(&mut parent, i)).or_default() += 1;
    }
    let n = words.len() as f64;
    let same: f64 = size.values().map(|&c| (c * c.saturating_sub(1) / 2) as f64).sum();
    let base = same / (n * (n - 1.0) / 2.0).max(1.0);
    if base > 0.0 { 1.0 / base } else { 0.0 }
}

impl Board {
    pub fn line(&self, epoch: usize) -> String {
        let kinds: Vec<String> = self.kinds.iter().map(|(k, o, c, top)| format!("{k} {o}/64 {c:.1}× (perfect {top:.1}×)")).collect();
        let stable = self.stable.map(|s| format!(" · {:.0}% kept their hexagram", 100.0 * s)).unwrap_or_default();
        format!(
            "  ▸ board @ epoch {epoch}: better {:+.3} · worse {:+.3} · loss {:.3} · coherence {}{stable}",
            self.better,
            self.worse,
            self.loss,
            kinds.join(" · ")
        )
    }
}

/// Rows for `<table>.board.tsv`: one per kind per occupied hexagram.
pub fn board_rows(epoch: usize, scores: &[WordScore], labels: &Labels) -> String {
    let by_word: HashMap<&str, &WordScore> = scores.iter().map(|s| (s.word.as_str(), s)).collect();
    let mut cells: HashMap<(&str, u8), Vec<&WordScore>> = HashMap::new();
    for ((w, k), h) in labels {
        if let Some(s) = by_word.get(w.as_str()) {
            cells.entry((k.as_str(), *h)).or_default().push(s);
        }
    }
    let mut keys: Vec<(&str, u8)> = cells.keys().copied().collect();
    keys.sort();
    let mut out = String::new();
    for key in keys {
        let ws = &cells[&key];
        let n = ws.len() as f64;
        out.push_str(&format!(
            "{epoch}\t{}\t{}\t{}\t{}\t{:.4}\t{:.4}\t{:.4}\n",
            key.0,
            glyph(key.1),
            key.1,
            ws.len(),
            ws.iter().map(|s| s.better).sum::<f64>() / n,
            ws.iter().map(|s| s.worse).sum::<f64>() / n,
            ws.iter().map(|s| s.loss).sum::<f64>() / n
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(a: &str, k: &str, o: &str) -> Row {
        Row { about: a.into(), kind: k.into(), other: o.into(), weight: 1.0 }
    }

    #[test]
    fn a_ceiling_counts_every_answer_a_word_gives() {
        let labels: HashMap<&str, u8> = [("cat", 0), ("mammal", 1), ("pet", 2), ("fish", 3)].into_iter().collect();
        // One answer per word: cat and mammal share one; 1 of 6 pairs → 6×.
        let one = vec![row("cat", "has", "fur"), row("mammal", "has", "fur"), row("pet", "has", "owner"), row("fish", "has", "gills")];
        assert!((ceiling(&labels, &one, "has") - 6.0).abs() < 1e-9);
        // Several: cat's second answer is not what counts; mammal–pet chain in.
        let many = vec![
            row("cat", "has", "fur"),
            row("cat", "has", "tail"),
            row("mammal", "has", "fur"),
            row("mammal", "has", "glands"),
            row("pet", "has", "glands"),
            row("fish", "has", "gills"),
        ];
        assert!((ceiling(&labels, &many, "has") - 2.0).abs() < 1e-9, "{}", ceiling(&labels, &many, "has"));
    }
}
