//! The three personas exactly as `bin/austen` builds them (600 sampled
//! training sentences each, seeds 11/13, persona trainer seed 1) on a clone of
//! the shared lexicon.
use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::metrics::harness::{Harness, Split};
use phiano::persona::World;
use phiano::trainer::Trainer;

fn sample(v: &[String], n: usize, seed: u64) -> Vec<String> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    let mut s = seed | 1;
    for i in (1..idx.len()).rev() {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let j = (s % (i as u64 + 1)) as usize;
        idx.swap(i, j);
    }
    idx.into_iter().take(n).map(|i| v[i].clone()).collect()
}

pub fn build(facet: &Facet, split: &Split, contrast: &[(String, Vec<String>)]) -> (World, Facet) {
    let mut world = World::new();
    let mut pfacet = facet.clone();
    let trainer = Trainer::new(LEARNING_RATE).with_seed(1);
    world.add_persona("Jane Austen", &sample(&split.train, 600, 11), &mut pfacet, &trainer);
    let mut by_author: std::collections::BTreeMap<&str, Vec<String>> = std::collections::BTreeMap::new();
    for (n, s) in contrast {
        let author = if n.starts_with("chesterton") { "G. K. Chesterton" } else { "Lewis Carroll" };
        by_author.entry(author).or_default().extend(s.iter().cloned());
    }
    for (author, sents) in &by_author {
        let sp = Harness::split(sents.clone(), 7);
        world.add_persona(author, &sample(&sp.train, 600, 13), &mut pfacet, &trainer);
    }
    (world, pfacet)
}

/// The first `n / 3` usable held-out sentences of each author, reduced to
/// their first three words.
pub fn held_prompts(split: &Split, contrast: &[(String, Vec<String>)], n: usize) -> Vec<String> {
    use phiano::tokenizer::Tokenizer;
    let mut by_author: std::collections::BTreeMap<&str, Vec<String>> = std::collections::BTreeMap::new();
    for (name, s) in contrast {
        let author = if name.starts_with("chesterton") { "G. K. Chesterton" } else { "Lewis Carroll" };
        by_author.entry(author).or_default().extend(s.iter().cloned());
    }
    let mut sets: Vec<Vec<String>> = vec![split.test.clone()];
    for sents in by_author.values() {
        sets.push(Harness::split(sents.clone(), 7).test);
    }
    let per = (n / sets.len()).max(1);
    let mut out = Vec::new();
    for set in sets {
        out.extend(set.iter().filter(|s| Tokenizer::tokenize(s).len() >= 8).take(per).map(|s| Tokenizer::tokenize(s)[..3].join(" ")));
    }
    out
}
