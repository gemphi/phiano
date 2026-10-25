#[allow(unused_imports)]
use super::*;

/// Loads the corpus named on the command line and fits the bottom-up manifold
/// the sweep is run against.
pub(super) fn train_facet() -> (Split, Facet) {
    let args: Vec<String> = std::env::args().collect();
    let corpus_path = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "data/dictionary_corpus.txt".to_string());

    let raw = std::fs::read_to_string(&corpus_path).unwrap_or_default();
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();
    if corpus.is_empty() {
        eprintln!("empty corpus at {}", corpus_path);
        std::process::exit(1);
    }

    let split = Harness::split(corpus, 42);
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    (split, facet)
}

/// The downward pass, on the same training sentences the facet was fitted on.
pub(super) fn run_descent(facet: &Facet, split: &Split) -> Facet {
    let train_toks: Vec<Vec<String>> = split
        .train
        .iter()
        .map(|s| Tokenizer::tokenize(s))
        .collect();
    let mut descended = facet.clone();
    let dr = Descent::cycle(&mut descended, &train_toks, Up::Bag, 4, 0.25, true);
    println!(
        "descent: {} of {} words moved, dispersion {:.3} -> {:.3}, band {:.3} -> {:.3}{}",
        dr.words_moved,
        dr.words_seen,
        dr.dispersion_before,
        dr.dispersion_after,
        dr.band_before,
        dr.band_after,
        match dr.rejected {
            true => "  REJECTED BY THE GUARD — this arm is the control",
            false => "",
        }
    );
    descended
}
