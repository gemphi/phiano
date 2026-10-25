#[allow(unused_imports)]
use super::*;

/// The corpus split and the facet everything below is measured on.
pub(super) struct Setup {
    pub(super) epochs: usize,
    pub(super) split: Split,
    pub(super) facet: Facet,
}

/// Parse the command line, load the corpus, train the facet, and print the
/// Kneser-Ney baseline on the same split.
pub(super) fn setup() -> Setup {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let epochs: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(4);

    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 3)
        .collect();
    if corpus.len() < 500 {
        eprintln!("corpus too small at {path}");
        std::process::exit(1);
    }
    let split = Harness::split(corpus, 42);
    println!(
        "corpus: {path}: {} train / {} valid sentences, {} epochs",
        split.train.len(),
        split.valid.len(),
        epochs
    );

    let t0 = std::time::Instant::now();
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    println!(
        "facet: {} words, {} evidenced, trained in {:.0}s",
        facet.vocabulary_size(),
        facet.evidenced_words(),
        t0.elapsed().as_secs_f64()
    );
    let kn = KneserNey::train(&split.train);
    println!("Kneser-Ney trigram on the same split: {:.2}\n", kn.perplexity(&split.valid));

    Setup { epochs, split, facet }
}
