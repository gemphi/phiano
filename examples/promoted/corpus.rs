#[allow(unused_imports)]
use super::*;

/// Reads the corpus named on the command line (or the default prose corpus),
/// keeps sentences long enough to train on, and splits them with the fixed seed.
pub(super) fn load_split() -> Split {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| {
        match std::path::Path::new(&phiano::data::path("prose_corpus.txt")).exists() {
            true => phiano::data::path("prose_corpus.txt"),
            false => phiano::data::path("rust_book_corpus.txt"),
        }
    });
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = raw
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| Tokenizer::tokenize(l).len() >= 5)
        .collect();
    if corpus.len() < 500 {
        eprintln!("corpus too small at {path}");
        std::process::exit(1);
    }
    println!("corpus: {path} ({} sentences)", corpus.len());

    Harness::split(corpus, 42)
}
