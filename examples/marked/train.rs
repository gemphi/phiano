#[allow(unused_imports)]
use super::*;

/// The corpus in token form and the two representations compared below.
pub(super) struct Trained {
    pub(super) toks: Vec<Vec<String>>,
    pub(super) shipped: Facet,
    pub(super) vocab: RelationVocab,
    pub(super) rel: RelationalFacet,
}

/// Load the corpus named on the command line and train both representations.
pub(super) fn train() -> Trained {
    let path = std::env::args()
        .nth(1)
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
    let toks: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();

    let base = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let mut shipped = base.clone();
    Descent::cycle(&mut shipped, &toks, Up::Bag, 4, 0.25, true);

    let vocab = RelationVocab::learn(&toks);
    let rel = RelationalFacet::train(&toks, &vocab, 2);

    Trained { toks, shipped, vocab, rel }
}
