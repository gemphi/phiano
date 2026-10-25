#[allow(unused_imports)]
use super::*;

/// Everything the experiments read: the training corpus in token form, the
/// shipped manifold trained the usual way, and the relational facet with the
/// vocabulary that names its channels.
pub(super) struct Run {
    pub(super) toks: Vec<Vec<String>>,
    pub(super) shipped: Facet,
    pub(super) vocab: RelationVocab,
    pub(super) rel: RelationalFacet,
}

/// Parse the corpus path, load and split it, train both representations, and
/// print the header describing what came out.
pub(super) fn setup() -> Run {
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
    let toks: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();

    // The shipped path, for comparison.
    let base = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let mut shipped = base.clone();
    Descent::cycle(&mut shipped, &toks, Up::Bag, 4, 0.25, true);

    // The new one.
    let vocab = RelationVocab::learn(&toks);
    let rel = RelationalFacet::train(&toks, &vocab, 2);

    println!(
        "shipped {} words; relational {} words, {} marker channels + {} offset",
        shipped.vocabulary_size(),
        rel.len(),
        vocab.markers(),
        phiano::relational::OFFSET_CHANNELS
    );
    println!("  {} of {} channels survived the per-channel collapse guard", rel.live, PHASE_CHANNELS);
    println!(
        "  channels 0..12 are: {}",
        (0..12).map(|k| vocab.name(k)).collect::<Vec<_>>().join(", ")
    );

    Run { toks, shipped, vocab, rel }
}

/// The candidate pool: words frequent enough to be ranked against.
pub(super) fn build_pool(rel: &RelationalFacet) -> Vec<String> {
    let mut pool: Vec<String> = rel
        .counts
        .iter()
        .filter(|(_, c)| **c >= COUNT_FLOOR)
        .map(|(w, _)| w.clone())
        .collect();
    pool.sort();
    pool
}

/// A word both representations can place, so every probe is answerable by both.
pub(super) fn known_word(rel: &RelationalFacet, shipped: &Facet, w: &str) -> bool {
    rel.words.contains_key(w) && shipped.lexicon.contains_key(w)
}
