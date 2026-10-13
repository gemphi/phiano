#[allow(unused_imports)]
use super::*;

/// One author: every sentence of their books, split 80/10/10 by sentence
/// hash (seed 7) so that held-out text is held out of the lexicon too.
pub(crate) struct Author {
    pub name: String,
    pub files: Vec<String>,
    pub split: Split,
}

/// Which author a file in `data/corpora/` belongs to.
fn author_of(dir: &str, file: &str) -> String {
    match (dir, file) {
        ("austen", _) => "Jane Austen".into(),
        (_, f) if f.starts_with("chesterton") => "G. K. Chesterton".into(),
        (_, f) if f.starts_with("carroll") => "Lewis Carroll".into(),
        (_, f) => f.replace('_', " "),
    }
}

/// Every author under `data/corpora/{austen,contrast}`, each with its own
/// split. A book is read whole and then split, so no sentence is in two
/// places.
pub(crate) fn authors() -> Vec<Author> {
    let mut by_author: std::collections::BTreeMap<String, (Vec<String>, Vec<String>)> = std::collections::BTreeMap::new();
    for dir in ["austen", "contrast"] {
        for (file, sents) in dir_sentences(&format!("data/corpora/{dir}")) {
            let a = author_of(dir, &file);
            let e = by_author.entry(a).or_default();
            e.0.push(file);
            e.1.extend(sents);
        }
    }
    by_author
        .into_iter()
        .map(|(name, (files, sents))| Author { name, files, split: Harness::split(sents, 7) })
        .collect()
}

/// A deterministic sample of `n` sentences — the old ingestion, kept as the
/// baseline the full one is measured against.
pub(crate) fn sample(v: &[String], n: usize, seed: u64) -> Vec<String> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    let mut s = seed | 1;
    for i in (1..idx.len()).rev() {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let j = (s % (i as u64 + 1)) as usize;
        idx.swap(i, j);
    }
    idx.into_iter().take(n.min(v.len())).map(|i| v[i].clone()).collect()
}
