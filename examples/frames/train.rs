#[allow(unused_imports)]
use super::*;

/// The prose corpus and the facet trained on it.
pub(super) struct Trained {
    pub(super) corpus: Vec<String>,
    pub(super) facet: Facet,
}

/// Load the prose corpus named on the command line and train the facet.
pub(super) fn train_facet() -> Trained {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/prose_corpus.txt".to_string());

    let raw = match std::fs::read_to_string(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("could not read {path}: {e}\nRun `bin/prepare_prose` first.");
            std::process::exit(1);
        }
    };
    let corpus: Vec<String> = raw
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| Tokenizer::tokenize(l).len() >= 5)
        .collect();
    if corpus.len() < 1000 {
        eprintln!("corpus too small: {} sentences", corpus.len());
        std::process::exit(1);
    }

    let split = Harness::split(corpus.clone(), 42);
    let base = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let train_toks: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let mut facet = base.clone();
    let dr = Descent::cycle(&mut facet, &train_toks, Up::Bag, 4, 0.25, true);

    println!(
        "prose: {} sentences, {} words, descent moved {}{}",
        corpus.len(),
        facet.vocabulary_size(),
        dr.words_moved,
        match dr.rejected {
            true => "  (REJECTED — bottom-up control)",
            false => "",
        }
    );

    Trained { corpus, facet }
}
