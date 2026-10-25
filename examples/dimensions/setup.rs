#[allow(unused_imports)]
use super::*;

/// Load the corpus named on the command line and train the facet everything is
/// measured on: bottom-up ranking, then one downward pass.
pub(super) fn train_facet() -> Facet {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/prose_corpus.txt".to_string());
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = raw
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| Tokenizer::tokenize(l).len() >= 5)
        .collect();
    if corpus.len() < 1000 {
        eprintln!("corpus too small at {path}. Run `bin/prepare_prose` first.");
        std::process::exit(1);
    }

    let split = Harness::split(corpus, 42);
    let base = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let train_toks: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let mut facet = base.clone();
    Descent::cycle(&mut facet, &train_toks, Up::Bag, 4, 0.25, true);
    facet
}
