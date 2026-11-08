//! The held-out prompts for the persona demo. The personas themselves are built by
//! `phiano::persona::corpus::three_personas`.
use phient::metrics::harness::{Harness, Split};
use phiano::persona::corpus::author_of;

/// The first `n / 3` usable held-out sentences of each author, reduced to
/// their first three words.
pub fn held_prompts(split: &Split, contrast: &[(String, Vec<String>)], n: usize) -> Vec<String> {
    use phiton::p64::words::Tokenizer;
    let mut by_author: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
    for (name, s) in contrast {
        let author = author_of("contrast", name);
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
