#[allow(unused_imports)]
use super::*;

/// The trained facet, the glosses it is scored against, and the rules those
/// glosses yield.
pub(super) struct Setup {
    pub(super) chunks: String,
    pub(super) facet: Facet,
    pub(super) glosses: Vec<(String, String)>,
    pub(super) rules: RuleSet,
    pub(super) fillers_of: HashMap<String, Vec<String>>,
}

/// Parse the command line, train on the corpus, and extract the rule set from
/// the dictionary glosses.
pub(super) fn setup() -> Setup {
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
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    println!("facet: {} words", facet.vocabulary_size());

    // Glosses, reduced to their definitional core.
    let glosses: Vec<(String, String)> = ChunkStore::new(&chunks)
        .load_all()
        .into_iter()
        .map(|(w, d)| (w, definition_core(&d)))
        .filter(|(_, d)| d.split_whitespace().count() >= 2)
        .collect();

    // Definitional in-degree: how many entries use this word to define
    // something. General terms score high, which is what makes the genus
    // recoverable without a parser.
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for (_, g) in &glosses {
        for t in Tokenizer::tokenize(g) {
            *in_degree.entry(t).or_insert(0) += 1;
        }
    }

    let mut rules = RuleSet::new();
    let mut fillers_of: HashMap<String, Vec<String>> = HashMap::new();
    for (head, gloss) in &glosses {
        for (role, filler) in extract(gloss, &in_degree, glosses.len(), head) {
            rules.add(head, role, &filler);
            fillers_of.entry(head.clone()).or_default().push(filler);
        }
    }
    println!("extracted {} rules from {} glosses", rules.len(), glosses.len());
    for (r, n) in rules.role_counts() {
        println!("  {:<10} {}", r.name(), n);
    }

    Setup { chunks, facet, glosses, rules, fillers_of }
}
