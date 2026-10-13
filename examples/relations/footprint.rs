#[allow(unused_imports)]
use super::*;

/// Footprint of the interned on-disk format, on a dictionary-scale model.
pub(super) fn print_footprint(facet: &Facet) {
    let path = std::env::temp_dir().join("phiano_relations_model.chroma");
    let path = path.to_str().unwrap_or("model.chroma");
    if phiano::storage::Storage::save(facet, path).is_ok() {
        if let Ok(meta) = std::fs::metadata(path) {
            let bytes = meta.len();
            // What the previous string-keyed layout would have cost: every
            // n-gram follower stored its word as an owned String, trigram keys
            // stored two, and phase_lags duplicated the bigram key set again.
            let avg_word: f64 = facet
                .lexicon
                .keys()
                .map(|w| w.len() as f64)
                .sum::<f64>()
                / facet.vocabulary_size().max(1) as f64;
            let entries = facet.ngram_entries() as f64;
            // 8 bytes of length prefix + the word, per stored key, per table.
            let v2_ngrams = entries * (8.0 + avg_word) * 2.0 + entries * 4.0;
            let v2_est = bytes as f64 + v2_ngrams;

            println!("\n=== footprint (interned v3 format) ===");
            println!("  vocabulary      : {}", facet.vocabulary_size());
            println!("  n-gram entries  : {}", facet.ngram_entries());
            println!("  mean word length: {:.1} chars", avg_word);
            println!("  on disk (v3)    : {:.1} MB", bytes as f64 / 1_048_576.0);
            println!("  string-keyed est: {:.1} MB", v2_est / 1_048_576.0);
            println!("  reduction       : {:.0}%", 100.0 * (1.0 - bytes as f64 / v2_est));
        }
    }
}
