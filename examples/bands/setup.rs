#[allow(unused_imports)]
use super::*;

/// The two arms to profile and the sentence items both are scored on.
pub(super) struct Setup {
    pub(super) base: Facet,
    pub(super) descended: Facet,
    pub(super) items: Vec<Item>,
}

/// Loads the corpus, trains the bottom-up arm and its descended counterpart,
/// builds the next-sentence items, and reports what there is to score.
pub(super) fn setup() -> Setup {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/dictionary_corpus.txt".to_string());
    let seed: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(42);

    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();
    if corpus.is_empty() {
        eprintln!("empty corpus at {}", path);
        std::process::exit(1);
    }

    // Sentence order preserved in the held-out half: a next-sentence task needs
    // contiguity, and a shuffled corpus has none.
    let cut = corpus.len() * 80 / 100;
    let (train, held) = corpus.split_at(cut);

    let split = Harness::split(train.to_vec(), seed);
    let trainer = Trainer::new(LEARNING_RATE).with_seed(seed);
    let base = Harness::train_ranking_only(&split, &trainer, 4);

    let train_toks: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let mut descended = base.clone();
    let dr = Descent::cycle(&mut descended, &train_toks, Up::Bag, 4, 0.25, true);

    let items = build_items(held, seed);
    println!(
        "vocabulary {}, {} items of {} candidates, descent moved {} words{}",
        base.vocabulary_size(),
        items.len(),
        CANDIDATES,
        dr.words_moved,
        match dr.rejected {
            true => " (REJECTED — arm equals the control)",
            false => "",
        }
    );

    Setup { base, descended, items }
}
