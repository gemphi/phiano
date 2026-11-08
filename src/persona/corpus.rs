//! The writers' books, read once: which author a file belongs to, each
//! author's 80/10/10 split, a seeded sample, and the three personas the
//! evaluations are built on.
use crate::config::LEARNING_RATE;
use crate::lab::dir_sentences;
use crate::persona::Styles;
use phiecs::phasor::facet::Facet;
use phient::metrics::harness::{Harness, Split};
use phient::train::trainer::Trainer;

/// One author: every sentence of their books, split 80/10/10 by sentence
/// hash (seed 7) so that held-out text is held out of the lexicon too.
pub struct Author {
    pub name: String,
    pub files: Vec<String>,
    pub split: Split,
}

/// Which author a file in `data/corpora/` belongs to.
pub fn author_of(dir: &str, file: &str) -> String {
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
pub fn authors() -> Vec<Author> {
    let mut by_author: std::collections::BTreeMap<String, (Vec<String>, Vec<String>)> = std::collections::BTreeMap::new();
    for dir in ["austen", "contrast"] {
        for (file, sents) in dir_sentences(&crate::data::path(&format!("corpora/{dir}"))) {
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

/// A deterministic sample of `n` sentences (clamped to `v.len()`).
pub fn sample(v: &[String], n: usize, seed: u64) -> Vec<String> {
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

/// The three personas exactly as `bin/austen` builds them (600 sampled
/// training sentences each, seeds 11/13, persona trainer seed 1) on a clone
/// of the shared lexicon.
pub fn three_personas(facet: &Facet, split: &Split, contrast: &[(String, Vec<String>)]) -> (Styles, Facet) {
    let mut world = Styles::new();
    let mut pfacet = facet.clone();
    let trainer = Trainer::new(LEARNING_RATE).with_seed(1);
    world.add_persona("Jane Austen", &sample(&split.train, 600, 11), &mut pfacet, &trainer);
    let mut by_author: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
    for (file, sents) in contrast {
        by_author.entry(author_of("contrast", file)).or_default().extend(sents.iter().cloned());
    }
    for (author, sents) in &by_author {
        let sp = Harness::split(sents.clone(), 7);
        world.add_persona(author, &sample(&sp.train, 600, 13), &mut pfacet, &trainer);
    }
    (world, pfacet)
}
