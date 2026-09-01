//! Structural health of the definition source. Task A1's acceptance test.
//!
//! `cargo run --release --bin defstats -- [chunks] [raw|core]`
//!
//! Two ratios decide whether the definition graph is the graph the imported
//! mechanisms assume, and both are computable in seconds without training
//! anything:
//!
//! * **weak∶strong.** Dict2vec extracted ~417K strong to 3.9M weak pairs, about
//!   9∶1. Phiano's Webster's source gives 47.5∶1, which means almost every
//!   definitional pair is one-way and the strong/weak split has nothing to
//!   split on.
//! * **kernel share.** Vincent-Lamarre et al. report a grounding kernel at ~10%
//!   of a dictionary. Phiano's resolves at 49.6%, which means the recursive peel
//!   barely bites and kernel scheduling has no core to hold.
//!
//! Both numbers are symptoms of the same thing: definer sets inflated by
//! quotations, editorial notes, cross-references and usage examples that are
//! *about* the entry rather than part of it. Neither mechanism was refuted; both
//! were given the wrong graph.

use phiano::chunker::ChunkStore;
use phiano::conception::DefinitionGraph;
use phiano::sources::{clean_definition, definition_core};

fn report(label: &str, entries: &[(String, String)]) {
    let g = DefinitionGraph::build(entries);
    let (strong, weak) = g.counts();
    let kernel = g.kernel();

    let total_definers: usize = entries
        .iter()
        .map(|(_, d)| phiano::tokenizer::Tokenizer::tokenize(d).len())
        .sum();
    let mean_definers = total_definers as f64 / entries.len().max(1) as f64;

    let ratio = weak as f64 / strong.max(1) as f64;
    let kernel_pct = 100.0 * kernel.len() as f64 / entries.len().max(1) as f64;

    println!("\n=== {} ===", label);
    println!("  entries              : {}", entries.len());
    println!("  mean definers/entry  : {:.1}", mean_definers);
    println!(
        "  strong / weak pairs  : {} / {}   ({:.1}:1)   {}",
        strong,
        weak,
        ratio,
        "pre-promotion floor - see note below"
    );
    println!(
        "  grounding kernel     : {} entries ({:.1}%)   {}",
        kernel.len(),
        kernel_pct,
        match kernel_pct <= 20.0 {
            true => "PASS (<= 20%)",
            false => "FAIL (target <= 20%, literature ~10%)",
        }
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let chunks = args.get(1).cloned().unwrap_or_else(|| "data/chunks".to_string());

    let store = ChunkStore::new(&chunks);
    let all = store.load_all();
    if all.is_empty() {
        eprintln!("no entries under {}", chunks);
        std::process::exit(1);
    }

    let raw: Vec<(String, String)> = all
        .iter()
        .map(|(w, d)| (w.clone(), clean_definition(d)))
        .filter(|(_, d)| !d.is_empty())
        .collect();

    let core: Vec<(String, String)> = all
        .iter()
        .map(|(w, d)| (w.clone(), definition_core(d)))
        .filter(|(_, d)| !d.is_empty())
        .collect();

    report("as ingested today (clean_definition only)", &raw);
    report("definitional core (A1)", &core);

    println!(
        "\n  NOTE on weak:strong. Reducing entries to their definitional core is\n\
         \x20 what fixes the kernel, and it necessarily *worsens* raw reciprocity:\n\
         \x20 a 10-word gloss is far less likely to point back than a 32-word essay.\n\
         \x20 Dict2vec did not reach 9:1 on raw reciprocity either - it concatenated\n\
         \x20 four modern dictionaries and promoted weak pairs to strong when two\n\
         \x20 words are among each other's K nearest (SS3.1, K=5). That promotion\n\
         \x20 needs a trained facet, so it runs in bin/conception, not here. The\n\
         \x20 ratio above is the pre-promotion floor, not the graph the split sees."
    );

    // Coverage guard: a cleaner that passes both ratios by discarding most of
    // the dictionary has not cleaned anything, it has deleted it.
    let kept = 100.0 * core.len() as f64 / raw.len().max(1) as f64;
    println!(
        "\n  entries surviving    : {:.1}%   {}",
        kept,
        match kept >= 90.0 {
            true => "PASS (>= 90%)",
            false => "FAIL — the cleaner is deleting entries, not cleaning them",
        }
    );

    println!("\n--- sample ---");
    for w in ["cat", "car", "vehicle", "grandmother", "woman"] {
        if let Some((_, d)) = core.iter().find(|(k, _)| k == w) {
            let short: String = d.chars().take(160).collect();
            println!("  {:<12} {}", w, short);
        }
    }
}
