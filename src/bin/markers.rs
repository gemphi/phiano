//! What words does `RelationVocab` decide are relations, and on what evidence?
//! `cargo run --release --bin markers [corpus]`
//!
//! Marker selection is the weakest part of `relational`, and it is weak in a way
//! the aggregate scores hide: the composition table can look healthy while every
//! channel it used was an offset channel and no named relation ever fired. Three
//! criteria were tried and discarded here, so this prints the ranking rather than
//! leaving it to be argued about.
//!
//! ```text
//! by absolute linking count   the and of to in a s was by on for with that as
//! by share of occurrences     nautical stoppani cu facto botanist dans shp
//! ```
//!
//! The first list is the inventory of English relational markers. The second is
//! a list of words that occur a dozen times each. A ratio rewards a small
//! denominator, and the words that name relations have the largest denominators
//! in the language — so the ranking is by count, and telling `the` from `of` is
//! left to the rarest-wins rule in `RelationVocab::marker`, where it belongs.
use phiano::relational::{RelationVocab, MARKER_MAX_GAP, OFFSET_CHANNELS};
use phiano::tokenizer::Tokenizer;
use std::collections::HashMap;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "data/prose_corpus.txt".into());
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let docs: Vec<Vec<String>> = raw
        .lines()
        .map(|l| Tokenizer::tokenize(l))
        .filter(|t| t.len() >= 5)
        .collect();

    let mut total: HashMap<&str, usize> = HashMap::new();
    let mut link: HashMap<&str, usize> = HashMap::new();
    for d in &docs {
        for t in d {
            *total.entry(t.as_str()).or_insert(0) += 1;
        }
        for i in 0..d.len() {
            for j in (i + 2)..(i + 1 + MARKER_MAX_GAP).min(d.len()) {
                for m in (i + 1)..j {
                    *link.entry(d[m].as_str()).or_insert(0) += 1;
                }
            }
        }
    }

    let vocab = RelationVocab::learn(&docs);
    println!(
        "{} sentences, {} candidate markers, {} given channels\n",
        docs.len(),
        link.len(),
        vocab.markers()
    );

    println!("  {:<4} {:<14} {:>9} {:>9}", "ch", "marker", "links", "total");
    for k in OFFSET_CHANNELS..(OFFSET_CHANNELS + vocab.markers()).min(64) {
        let name = vocab.name(k);
        println!(
            "  {:<4} {:<14} {:>9} {:>9}",
            k,
            name,
            link.get(name.as_str()).copied().unwrap_or(0),
            total.get(name.as_str()).copied().unwrap_or(0)
        );
    }
    println!(
        "\n  Channels run most general to most specific, and \
         `RelationVocab::marker` picks\n  the highest-numbered candidate between \
         a pair — the rarest, so the most\n  informative. `the` holds channel {} \
         and therefore almost never wins one.",
        OFFSET_CHANNELS
    );
}
