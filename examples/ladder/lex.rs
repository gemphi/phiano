//! What the ladder reads beside the story: WordNet neighbours, the frame
//! buckets (POS or supersense, function words their own), and the
//! real-world prior (wordfreq). Loaded once, by value index.
#[allow(unused_imports)]
use super::*;

pub(super) struct Lex {
    /// value index -> [(neighbour index, weight)]
    pub near: Vec<Vec<(usize, f64)>>,
    pub near_rows: usize,
    /// value index -> frame bucket id
    pub bucket_code: Vec<usize>,
    pub n_buckets: usize,
    /// prior mass per bucket
    pub global_pos: Vec<f64>,
    /// the real-world prior per value, normalised
    pub real_prior: Vec<f64>,
}

impl Lex {
    pub(super) fn load(root: &std::path::Path, near_file: &str, frame: &str, values: &[String], prior: &[f64]) -> Self {
        let at: std::collections::HashMap<&str, usize> = values.iter().enumerate().map(|(i, v)| (v.as_str(), i)).collect();
    // WordNet neighbours, restricted to the vocabulary: value index -> [(index, weight)].
    let mut near: Vec<Vec<(usize, f64)>> = vec![Vec::new(); values.len()];
    let mut near_rows = 0usize;
    if let Ok(text) = std::fs::read_to_string(root.join(&near_file)).or_else(|_| std::fs::read_to_string(&near_file)) {
        for line in text.lines() {
            let mut it = line.split('\t');
            if let (Some(a0), Some(b0), Some(w0)) = (it.next(), it.next(), it.next()) {
                if let (Some(&i), Some(&j), Ok(wt)) = (at.get(a0), at.get(b0), w0.parse::<f64>()) {
                    near[i].push((j, wt));
                    near_rows += 1;
                }
            }
        }
    }

    // The frame bucket per value. `--frame pos` uses WordNet's 6 POS classes;
    // `--frame sense` uses its ~45 supersenses (noun.location, verb.motion…),
    // a finer grammatical/semantic frame. Function words (the closed-class
    // core) are their own bucket "f"; a word with no entry is "u".
    let funcs: std::collections::HashSet<String> = std::fs::read_to_string(root.join("data/lex/function.txt"))
        .or_else(|_| std::fs::read_to_string("data/lex/function.txt"))
        .unwrap_or_default()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let lex_file = if frame == "pos" { "data/lex/pos.tsv" } else { "data/lex/sense.tsv" };
    let mut cat_of: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    if let Ok(text) = std::fs::read_to_string(root.join(lex_file)).or_else(|_| std::fs::read_to_string(lex_file)) {
        for line in text.lines() {
            if let Some((wd, c)) = line.split_once('\t') {
                // pos.tsv may list several POS letters; take the first.
                let cat = if frame == "pos" { c.chars().next().map(|c| c.to_string()).unwrap_or_else(|| "u".into()) } else { c.to_string() };
                cat_of.insert(wd.to_string(), cat);
            }
        }
    }
    // Assign each distinct category an id; "f" and "u" always present.
    let mut cat_id: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let bucket_id = |c: &str, m: &mut std::collections::HashMap<String, usize>| -> usize {
        let n = m.len();
        *m.entry(c.to_string()).or_insert(n)
    };
    bucket_id("f", &mut cat_id);
    bucket_id("u", &mut cat_id);
    let bucket_code: Vec<usize> = values
        .iter()
        .map(|v| {
            let c = if funcs.contains(v) { "f".to_string() } else { cat_of.get(v).cloned().unwrap_or_else(|| "u".into()) };
            bucket_id(&c, &mut cat_id)
        })
        .collect();
    let n_buckets = cat_id.len();
    let mut global_pos = vec![0.0f64; n_buckets];
    for (j, p) in prior.iter().enumerate() {
        global_pos[bucket_code[j]] += p;
    }
    // The real-world prior: wordfreq's frequencies (Wikipedia, subtitles, news,
    // books — billions of words), so a word Emma never said still has a rank.
    let mut real_prior = vec![0.0f64; values.len()];
    if let Ok(text) = std::fs::read_to_string(root.join("data/lex/freq.tsv")).or_else(|_| std::fs::read_to_string("data/lex/freq.tsv")) {
        let fr: std::collections::HashMap<&str, f64> = text.lines().filter_map(|l| l.split_once('\t')).filter_map(|(w, f)| Some((w, f.parse::<f64>().ok()?))).collect();
        for (j, v) in values.iter().enumerate() {
            real_prior[j] = *fr.get(v.as_str()).unwrap_or(&1e-9);
        }
    } else {
        real_prior = prior.to_vec();
    }
    let z: f64 = real_prior.iter().sum();
    real_prior.iter_mut().for_each(|p| *p /= z);

        Self { near, near_rows, bucket_code, n_buckets, global_pos, real_prior }
    }
}
