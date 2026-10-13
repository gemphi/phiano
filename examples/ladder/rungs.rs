//! The ladder's rungs: what each one reads, and how a rung is scored.
#[allow(unused_imports)]
use super::*;

pub(super) use phiano::lab::flag;

/// The scores for `truth`'s rank: 0 is first.
pub(super) fn rank(scores: &[f64], at: usize) -> usize {
    scores.iter().filter(|s| **s > scores[at]).count()
}

#[derive(Default, Clone)]
pub(super) struct Tally {
    n: usize,
    top1: usize,
    rr: f64,
}
impl Tally {
    pub(super) fn add(&mut self, scores: &[f64], at: usize) {
        let r = rank(scores, at);
        self.n += 1;
        self.top1 += (r == 0) as usize;
        self.rr += 1.0 / (r + 1) as f64;
    }
    pub(super) fn cell(&self) -> String {
        let n = self.n.max(1) as f64;
        format!("{:.1}% · {:.3}", 100.0 * self.top1 as f64 / n, self.rr / n)
    }
}

/// The level sets the reader builds for `said`, outermost first, as
/// `Knowing::next` builds them: the path's scopes; the last word and the word
/// under each inner scope; the last pair and the pair under each inner scope.
pub(super) fn levels_for(k: &Knowing, said: &[String]) -> (Vec<Vec<String>>, Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut recent: Vec<String> = said.iter().rev().take(6).filter(|w| w.chars().any(char::is_alphanumeric)).cloned().collect();
    recent.extend(said.windows(2).rev().take(3).map(|p| format!("pair:{} {}", p[0], p[1])));
    let last_pairs: Vec<String> = said.windows(2).rev().take(3).map(|p| format!("pair:{} {}", p[0], p[1])).collect();
    let cast = k.tree.cast_under(&k.counts, &k.within, &recent, f64::MIN, 2);
    let path = cast
        .into_iter()
        .next()
        .filter(|c| c.own >= 2 && c.path.last().is_some_and(|leaf| last_pairs.iter().all(|p| k.counts.count(Some(leaf), SAYS, p) >= 1.0)))
        .map(|c| c.path)
        .unwrap_or_default();
    let scope_levels: Vec<Vec<String>> = path.iter().map(|s| vec![s.clone()]).collect();
    let inner: Vec<&String> = path.iter().skip(1).collect();
    let mut word_levels = Vec::new();
    if let Some(last) = said.last() {
        word_levels.push(vec![last.clone()]);
        word_levels.extend(inner.iter().map(|s| vec![format!("{s}|{last}")]));
    }
    let mut pair_levels = Vec::new();
    if said.len() >= 2 {
        let pair = format!("pair:{} {}", said[said.len() - 2], said[said.len() - 1]);
        pair_levels.push(vec![pair.clone()]);
        pair_levels.extend(inner.iter().map(|s| vec![format!("{s}|{pair}")]));
    }
    (scope_levels, word_levels, pair_levels)
}

pub(super) const RUNGS: [&str; 13] = [
    "0 · prior (frequency)",
    "1 · + path scopes",
    "2 · + last word",
    "3 · + last pair",
    "4 · + kin base",
    "5 · + dictionary (WordNet)",
    "6 · + frame",
    "7 · + both (levels on the frame)",
    "8 · mix (levels + frame)",
    "9 · router (confident⟶levels, else⟶frame)",
    "10 · real-world prior (wordfreq)",
    "11 · real prior × frame",
    "12 · WORKING (told⟶levels, open⟶real×frame)",
];

/// Spread each value's probability to its WordNet neighbours (synonyms,
/// hypernyms, hyponyms): mass told to a word flows to the words that mean
/// nearly the same, so a word never told after the last can be read through a
/// synonym that was. `near[t]` is value `t`'s neighbours as `(index, weight)`.
pub(super) fn smooth(log_scores: &[f64], near: &[Vec<(usize, f64)>], alpha: f64) -> Vec<f64> {
    let p: Vec<f64> = log_scores.iter().map(|s| s.exp()).collect();
    let mut q = p.clone();
    for (t, pt) in p.iter().enumerate() {
        if *pt > 1e-9 {
            for (c, wt) in &near[t] {
                q[*c] += alpha * wt * pt;
            }
        }
    }
    let z: f64 = q.iter().sum();
    q.iter().map(|x| (x / z).max(1e-300).ln()).collect()
}
