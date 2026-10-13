#[allow(unused_imports)]
use super::*;

/// Everything the run needs that comes from outside: the seed and the corpus.
pub(super) struct Setup {
    pub(super) seed: u64,
    pub(super) corpus: Vec<String>,
}

/// Parse the command line and load the corpus, exiting on anything unusable.
pub(super) fn setup() -> Setup {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/dictionary_corpus.txt".to_string());
    let seed: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(42);

    let raw = match std::fs::read_to_string(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("could not read {}: {}", path, e);
            std::process::exit(1);
        }
    };

    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();
    if corpus.is_empty() {
        eprintln!("empty corpus at {}", path);
        std::process::exit(1);
    }

    Setup { seed, corpus }
}
