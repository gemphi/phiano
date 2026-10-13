#[allow(unused_imports)]
use super::*;

/// Arguments, corpus, the baseline facet, and the definition entries every
/// condition below is run against.
pub(super) struct Setup {
    pub(super) runs: usize,
    pub(super) base_seed: u64,
    pub(super) rounds: usize,
    pub(super) split: Split,
    pub(super) store: ChunkStore,
    pub(super) base: Facet,
    pub(super) entries: Vec<(String, String)>,
}

/// Parse arguments, load and split the corpus, train the baseline facet, and
/// collect the definitions that overlap its vocabulary.
pub(super) fn setup() -> Setup {
    let args: Vec<String> = std::env::args().collect();
    // A3: --runs N repeats the whole experiment at N seeds and reports mean and
    // spread. One deterministic number has no error bar, and the effects left
    // after A2 are small enough that the interval is what decides them.
    let runs: usize = std::env::var("RUNS").ok().and_then(|v| v.parse().ok()).unwrap_or(1);
    let base_seed: u64 = std::env::var("SEED").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
    let corpus_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let chunks_path = args.get(2).cloned().unwrap_or_else(|| "data/chunks".to_string());
    let rounds: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(3);

    let raw = match std::fs::read_to_string(&corpus_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("could not read {}: {}", corpus_path, e);
            std::process::exit(1);
        }
    };
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();
    let split = Harness::split(corpus, 42);

    // Ranking-only training, because it is the regime the objective experiment
    // measured as 27x better at putting relational structure in the manifold.
    // Testing a composition rule on top of the weaker objective would confound
    // the two.
    let trainer = Trainer::new(LEARNING_RATE).with_seed(base_seed);
    let base = Harness::train_ranking_only(&split, &trainer, 4);
    println!(
        "trained: {} sentences, vocabulary {}",
        split.train.len(),
        base.vocabulary_size()
    );

    let store = ChunkStore::new(&chunks_path);
    let all = store.load_all();
    // Only entries whose headword the corpus actually contains can move
    // anything, and only those are counted, so "entries" is not inflated by the
    // dictionary's size.
    // A1: entries reduced to their definitional core. Webster's stacks the
    // gloss together with elaboration, editorial notes, usage examples and
    // quoted lines; composing all of it put ~32 words into every definer set,
    // against ~10 that actually define. The kernel share this produces (9.9%)
    // lands on the literature's ~10%, which the 30.5% it replaced did not.
    let entries: Vec<(String, String)> = all
        .into_iter()
        .filter(|(w, _)| base.lexicon.contains_key(w))
        .map(|(w, d)| (w, definition_core(&d)))
        .filter(|(_, d)| d.split_whitespace().count() >= 2)
        .collect();
    println!("definitions covering the vocabulary: {}", entries.len());
    if entries.is_empty() {
        eprintln!("no definitions overlap the trained vocabulary — nothing to compose");
        std::process::exit(1);
    }

    Setup { runs, base_seed, rounds, split, store, base, entries }
}
