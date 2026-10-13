#[allow(unused_imports)]
use super::*;

/// The trained facet and the pairs, each carrying the gloss it came from —
/// that gloss is C.
pub(super) struct Data {
    pub(super) facet: Facet,
    pub(super) pairs: Vec<(String, String)>,
    pub(super) sources: Vec<String>,
}

/// Load the corpus and the chunk store, train the facet, and mine the pairs
/// together with the gloss each came from.
pub(super) fn setup() -> Data {
    let args: Vec<String> = std::env::args().collect();
    let chunks = args.get(1).cloned().unwrap_or_else(|| "data/chunks".to_string());
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
    let base = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let train_toks: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let mut facet = base.clone();
    let dr = Descent::cycle(&mut facet, &train_toks, Up::Bag, 4, 0.25, true);

    // Pairs, each carrying the gloss it came from — that gloss is C.
    let glosses: Vec<(String, String)> = ChunkStore::new(&chunks)
        .load_all()
        .into_iter()
        .map(|(w, d)| (w, definition_core(&d)))
        .collect();
    let above = |w: &str| facet.lexicon.get(w).is_some_and(|p| p.count >= COUNT_FLOOR);

    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut sources: Vec<String> = Vec::new();
    let mut per_head: HashMap<String, usize> = HashMap::new();
    for (head, gloss) in &glosses {
        if !above(head) {
            continue;
        }
        for t in Tokenizer::tokenize(gloss) {
            if t == *head || !above(&t) {
                continue;
            }
            let n = per_head.entry(head.clone()).or_insert(0);
            if *n >= 8 {
                break;
            }
            *n += 1;
            pairs.push((head.clone(), t));
            sources.push(gloss.clone());
        }
    }

    println!(
        "facet {} words, {} pairs, descent moved {}{}",
        facet.vocabulary_size(),
        pairs.len(),
        dr.words_moved,
        match dr.rejected {
            true => " (REJECTED — this is the bottom-up control)",
            false => "",
        }
    );
    if pairs.len() < 500 {
        eprintln!("too few pairs to say anything");
        std::process::exit(1);
    }

    Data { facet, pairs, sources }
}
