#[allow(unused_imports)]
use super::*;

/// The dictionary entries to train on, and the store they came from.
pub(super) struct Setup {
    pub(super) epochs: usize,
    pub(super) store: ChunkStore,
    pub(super) selected: Vec<(String, String)>,
}

/// Parse the command line, load the dictionary, and choose the training subset.
pub(super) fn setup(families: &[RelationFamily]) -> Setup {
    let args: Vec<String> = std::env::args().collect();
    let max_entries: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(20_000);
    let epochs: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);

    let store = ChunkStore::new("data/chunks");
    let entries = store.load_all();
    if entries.is_empty() {
        eprintln!("no dictionary entries found under data/chunks");
        std::process::exit(1);
    }

    let probe_words: Vec<String> = families
        .iter()
        .flat_map(|f| f.pairs.iter().flat_map(|p| [p.a.clone(), p.b.clone()]))
        .collect();

    // Keep every probe word, then fill up to the budget with the rest, so the
    // benchmark is never scored on a vocabulary that happens to exclude it.
    let mut selected: Vec<(String, String)> = entries
        .iter()
        .filter(|(w, _)| probe_words.contains(w))
        .cloned()
        .collect();
    let probe_count = selected.len();
    for (w, d) in &entries {
        if selected.len() >= max_entries {
            break;
        }
        if !probe_words.contains(w) {
            selected.push((w.clone(), d.clone()));
        }
    }

    println!(
        "dictionary: {} entries available, training on {} ({} probe words present), {} epoch(s)",
        entries.len(),
        selected.len(),
        probe_count,
        epochs
    );

    Setup { epochs, store, selected }
}
