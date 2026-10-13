#[allow(unused_imports)]
use super::*;

/// Everything the experiment arms are built from: the trained manifold, the
/// tokenised training sentences the bands are discovered over, and the vocabulary
/// above the count floor.
pub(super) struct Setup {
    pub(super) learned: Facet,
    pub(super) train_toks: Vec<Vec<String>>,
    pub(super) vocab: Vec<(String, u32)>,
}

/// Loads the corpus named on the command line, trains the manifold every arm is
/// quoted against, and reports the corpus size.
pub(super) fn setup() -> Setup {
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
    let mut learned = base.clone();
    Descent::cycle(&mut learned, &train_toks, Up::Bag, 4, 0.25, true);

    let vocab: Vec<(String, u32)> = learned
        .lexicon
        .iter()
        .filter(|(_, p)| p.count >= COUNT_FLOOR)
        .map(|(w, p)| (w.clone(), p.count))
        .collect();
    let mut vocab = vocab;
    vocab.sort();
    println!(
        "prose: {} sentences, {} words above count {}",
        split.train.len() + split.valid.len() + split.test.len(),
        vocab.len(),
        COUNT_FLOOR
    );

    Setup { learned, train_toks, vocab }
}

/// The learned manifold, expressed as points so every arm is scored by the
/// same code. Magnitude is the phasor's own amplitude.
pub(super) fn learned_points(learned: &Facet, vocab: &[(String, u32)]) -> HashMap<String, Point> {
    vocab
        .iter()
        .map(|(w, _)| {
            let p = &learned.lexicon[w];
            let pt = (0..PHASE_CHANNELS)
                .map(|k| {
                    let t = p.theta(k);
                    (p.amplitude * t.cos(), p.amplitude * t.sin())
                })
                .collect();
            (w.clone(), pt)
        })
        .collect()
}
