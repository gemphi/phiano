//! The counters the learner is measured against, and the corpus reader.

use phient::prose::tokens;
use phient::study::Score;
use std::collections::HashMap;

/// The file's lines as sentences: cut at the full stops, tokenised, and a
/// token with a digit in it dropped - the guide is a grammar book, and its
/// unit and exercise numbers ("103", "84c") were read as words and said
/// back (docs/235). A sentence is one of at least four tokens before the
/// drop, so the dev and test cut (one line in ten, one in twenty) falls on
/// the same sentences as before the drop.
pub fn sentences(path: &str) -> Vec<Vec<String>> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .split(|c| matches!(c, '.' | '!' | '?' | '\n'))
        .map(tokens)
        .map(|w| w.into_iter().filter(|t| t.chars().any(char::is_alphanumeric)).collect::<Vec<_>>())
        .filter(|w| w.len() >= 4)
        .map(|w| w.into_iter().filter(|t| !not_a_word(t)).collect())
        .collect()
}

/// A path's file name: what a register's row is named by.
pub fn name_of(path: &str) -> String {
    std::path::Path::new(path).file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default()
}

/// What the reader does not take for a word: a token with a digit in it.
/// The guide is a grammar book, and its unit and exercise numbers were
/// read as words and said back (docs/235).
pub fn not_a_word(t: &str) -> bool {
    t.chars().any(|c| c.is_ascii_digit())
}

/// Where `truth` stands in a model's ranked answers (1 is first), for
/// phient's `Score`; `None` when it is not among them.
pub fn rank(ranked: &[String], truth: &str) -> Option<usize> {
    ranked.iter().position(|w| w == truth).map(|i| i + 1)
}

/// The best of counting: interpolated Witten–Bell over two words back,
/// add-one at the bottom, normalised over the words it was told.
#[derive(Default)]
pub struct WittenBell {
    pub uni: HashMap<String, u64>,
    pub n: u64,
    pub bi: HashMap<String, HashMap<String, u64>>,
    pub tri: HashMap<(String, String), HashMap<String, u64>>,
}

impl WittenBell {
    pub fn tell(&mut self, line: &[String]) {
        let pad = |j: usize, d: usize| if j >= d { line[j - d].clone() } else { "<s>".to_string() };
        for j in 0..line.len() {
            let w = line[j].clone();
            *self.uni.entry(w.clone()).or_default() += 1;
            self.n += 1;
            *self.bi.entry(pad(j, 1)).or_default().entry(w.clone()).or_default() += 1;
            *self.tri.entry((pad(j, 2), pad(j, 1))).or_default().entry(w).or_default() += 1;
        }
    }

    pub fn p1(&self, w: &str) -> f64 {
        (self.uni.get(w).copied().unwrap_or(0) as f64 + 1.0) / (self.n + self.uni.len() as u64) as f64
    }

    fn mix(seen: Option<&HashMap<String, u64>>, w: &str, lower: f64) -> f64 {
        let Some(m) = seen else { return lower };
        let (c, t) = (m.values().sum::<u64>() as f64, m.len() as f64);
        (m.get(w).copied().unwrap_or(0) as f64 + t * lower) / (c + t)
    }

    pub fn p(&self, h2: &str, h1: &str, w: &str) -> f64 {
        let p2 = Self::mix(self.bi.get(h1), w, self.p1(w));
        Self::mix(self.tri.get(&(h2.to_string(), h1.to_string())), w, p2)
    }

    /// The five likeliest: among what followed the two words, what followed
    /// the one, and the most told — nothing outside those can outrank them.
    pub fn top5(&self, h2: &str, h1: &str, common: &[String]) -> Vec<String> {
        let mut c: Vec<&String> = common.iter().collect();
        if let Some(m) = self.bi.get(h1) {
            c.extend(m.keys());
        }
        if let Some(m) = self.tri.get(&(h2.to_string(), h1.to_string())) {
            c.extend(m.keys());
        }
        c.sort();
        c.dedup();
        let mut s: Vec<(f64, &String)> = c.into_iter().map(|w| (self.p(h2, h1, w), w)).collect();
        s.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.1.cmp(b.1)));
        s.into_iter().take(5).map(|(_, w)| w.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::sentences;

    #[test]
    fn a_number_is_not_a_word_and_the_cut_does_not_move() {
        let dir = std::env::temp_dir().join("phiano_learner_sentences");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("guide.txt");
        std::fs::write(&path, "103 enough and too\nI have been waiting for you.\nUnit 84C enough and too + adjective\n").unwrap();
        let s = sentences(path.to_str().unwrap());
        assert_eq!(s.len(), 3, "a line of four tokens stays a sentence after its number is dropped");
        assert!(s.iter().flatten().all(|t| !t.chars().any(|c| c.is_ascii_digit())), "{s:?}");
        assert_eq!(s[0], ["enough", "and", "too"]);
        assert_eq!(s[1].len(), 6);
    }
}

/// The three rows counting scores at the test set's positions: the most
/// told word, the word before counted, and the smoothed trigram. The
/// `Count` is the caller's, since the sayings go on from it after the rows
/// are read.
pub fn scored(
    train: &[Vec<String>],
    test: &[Vec<String>],
    count: &phient::lineup::Count,
) -> Vec<(String, Score, Option<f64>)> {
    let mut wb = WittenBell::default();
    for l in train {
        wb.tell(l);
    }
    let mut common: Vec<(&String, &u64)> = wb.uni.iter().collect();
    common.sort_by(|x, y| y.1.cmp(x.1).then_with(|| x.0.cmp(y.0)));
    let common: Vec<String> = common.into_iter().take(10).map(|(w, _)| w.clone()).collect();
    let (mut floor, mut counted, mut smooth) = (Score::default(), Score::default(), Score::default());
    let (mut nll, mut known) = (0.0f64, 0usize);
    for l in test {
        for j in 1..l.len() {
            floor.rank(rank(&common[..common.len().min(5)], &l[j]));
            let mut after = count.choices(&l[j - 1]);
            after.sort_by(|x, y| y.1.cmp(&x.1).then_with(|| x.0.cmp(y.0)));
            counted.rank(rank(&after.into_iter().take(5).map(|(w, _)| w.clone()).collect::<Vec<_>>(), &l[j]));
            let h2 = match j >= 2 {
                true => l[j - 2].as_str(),
                false => "<s>",
            };
            smooth.rank(rank(&wb.top5(h2, &l[j - 1], &common), &l[j]));
            match wb.uni.contains_key(&l[j]) {
                true => {
                    nll -= wb.p(h2, &l[j - 1], &l[j]).ln();
                    known += 1;
                }
                false => {}
            }
        }
    }
    let ppl = (nll / known.max(1) as f64).exp();
    vec![
        ("the most told word (floor)".to_string(), floor, None),
        ("phient counts (the word before)".to_string(), counted, None),
        ("Witten–Bell trigram (counting, smoothed)".to_string(), smooth, Some(ppl)),
    ]
}
