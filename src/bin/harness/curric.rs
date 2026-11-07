//! The curriculum measurement folded into the walk: train a facet on the
//! Guide's Split_Rule training sentences and measure held-out perplexity for
//! the learner and the Kneser-Ney trigram, on the same test set the
//! `curriculum.perplexity` Bar is declared over.
//!
//! Honesty note. Only perplexity is measured here, because phient exposes a
//! faithful held-out perplexity for both the learner (`Harness::perplexity`)
//! and the trigram (`KneserNey`). `curriculum.top1` has no matching next-word
//! accuracy entry point yet, so it is left unmeasured - the Recorder reports
//! it unjudged rather than substituting the 50-distractor ranking metric,
//! which measures a different thing. The training is bounded to a few epochs
//! so the walk stays within its cap; the number is a real floor, not the
//! learner's best possible.

mod bands;

use super::bars::{Bars, Unit};
use super::record::Measured;
use phient::metrics::baseline::kn::KneserNey;
use phient::metrics::harness::{Harness, Split};
use phient::train::sources::wiktionary::WiktionarySource;
use phient::train::trainer::Trainer;
use phiecs::phasor::facet::Facet;
use phisor::guide::holdout::{role_of, Role};
use phisor::guide::{self};
use std::path::Path;

/// The Guide and its Key, relative to the workspace root.
const GUIDE: &str = "crates/phiano/data/person/murphy.txt";
const KEY: &str = "crates/phiano/data/person/murphy_key.txt";

/// An optional Wiktionary (Kaikki/Wiktextract) JSONL dump. When present, its
/// relation triples - synonyms, hypernyms, antonyms - are folded into training
/// as short `[about, kind, other]` sentences, so a word resonates with its kin
/// and not only with the words it co-occurs with in the Guide. Absent, the
/// facet trains on the Guide alone, exactly as before.
const WIKTIONARY: &str = "crates/phiano/data/wiktionary.jsonl";

/// Bounded training so the walk stays under its cap. A larger budget is the
/// curriculum's own job; this is the floor the walk can afford to measure.
const EPOCHS: usize = 3;
const PREDICTIVE_PASSES: usize = 2;
const LEARNING_RATE: f64 = 0.05;

/// Measure the curriculum numbers and return them as Recorder `Measured`.
/// An unreadable corpus yields none, so the walk records the curriculum Bars
/// unjudged rather than failing.
pub fn run(bars: &Bars) -> Vec<Measured> {
    let (Ok(text), Ok(key)) = (
        std::fs::read_to_string(Path::new(GUIDE)),
        std::fs::read_to_string(Path::new(KEY)),
    ) else {
        eprintln!("harness: the Guide at {GUIDE} is unreadable; curriculum numbers are unjudged");
        return Vec::new();
    };

    // The Guide as plain sentences, in index order, so the Split_Rule role of
    // each is the one the Bar is declared over.
    let sentences: Vec<String> = guide::read(&text, &key)
        .iter()
        .map(|s| s.words.join(" "))
        .collect();
    let mut train = Vec::new();
    let mut test = Vec::new();
    for (i, s) in sentences.iter().enumerate() {
        match role_of(i) {
            Role::Test => test.push(s.clone()),
            Role::Train => train.push(s.clone()),
            Role::Dev => {}
        }
    }
    if train.is_empty() || test.is_empty() {
        return Vec::new();
    }

    // Fold in Wiktionary relation sentences when a dump is on disk. Each is a
    // three-word `[about, kind, other]` string the trainer couples through the
    // marker channel; absent, this is empty and training is the Guide alone.
    let relations: Vec<String> = match Path::new(WIKTIONARY).exists() {
        true => {
            let source = WiktionarySource::new(WIKTIONARY);
            let sentences = source.relation_sentences();
            eprintln!("harness: folding {} Wiktionary relations into training", sentences.len());
            sentences.into_iter().map(|s| s.join(" ")).collect()
        }
        false => Vec::new(),
    };

    // Build the facet by hand so the relation sentences join the Guide's. The
    // relation inventory is learned over both, so the synonym/hypernym markers
    // get their own channels before any coupling runs.
    let mut facet = Facet::new();
    let tokenised: Vec<Vec<String>> = train
        .iter()
        .chain(relations.iter())
        .map(|s| s.split_whitespace().map(str::to_string).collect())
        .collect();
    facet.learn_relations(&tokenised);
    let trainer = Trainer::new(LEARNING_RATE);
    for _ in 0..EPOCHS {
        for s in train.iter().chain(relations.iter()) {
            trainer.train_sentence(&mut facet, s);
            for _ in 0..PREDICTIVE_PASSES {
                trainer.train_predictive(&mut facet, s);
            }
        }
    }
    let _ = Split { train: train.clone(), valid: test.clone(), test: test.clone() };

    let learner_ppl = Harness::perplexity(&facet, &test);
    let trigram_ppl = KneserNey::train(&train).perplexity(&test);
    eprintln!(
        "harness: curriculum perplexity learner {learner_ppl:.1} vs trigram {trigram_ppl:.1} \
         over {} test sentences",
        test.len()
    );

    bands::print(&facet, &train, &test);

    // Only the learner's perplexity is judged against the declared Bar
    // (AtMost 84.7). The trigram's is recorded beside it with no Bar, so it
    // is unjudged; top-1 stays unmeasured and therefore unjudged too.
    vec![
        Measured::judged("curriculum.perplexity", one_decimal(learner_ppl), Unit::Perplexity, bars),
        Measured::judged(
            "curriculum.trigram_perplexity",
            one_decimal(trigram_ppl),
            Unit::Perplexity,
            bars,
        ),
    ]
}

fn one_decimal(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

#[cfg(test)]
mod tests {
    use phient::train::sources::wiktionary::WiktionarySource;

    /// One bounded curriculum measurement without the ten-level walk or a
    /// record page: prints the perplexities and the band diagnostic.
    /// `cargo test -p phiano --bin harness measure_curriculum -- --ignored --nocapture`
    #[test]
    #[ignore = "trains a facet; run explicitly"]
    fn measure_curriculum() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        std::env::set_current_dir(root).unwrap();
        let bars = super::super::bars::Bars::read(std::path::Path::new("docs/bars.md"));
        for m in super::run(&bars) {
            eprintln!("{m:?}");
        }
    }

    /// The fold path reads real relation sentences from a Kaikki-shaped dump:
    /// the same `relation_sentences` the walk folds in, proven here so the
    /// wire is checked without a full training walk. The absent-dump default
    /// (train on the Guide alone) needs no test - it is the prior behaviour.
    #[test]
    fn the_fold_path_reads_relation_sentences_from_a_dump() {
        let jsonl = concat!(
            "{\"word\":\"cat\",\"pos\":\"noun\",\"synonyms\":[{\"word\":\"feline\"}],",
            "\"senses\":[{\"glosses\":[\"a small carnivore\"],\"hypernyms\":[{\"word\":\"animal\"}]}]}\n"
        );
        let dir = std::env::temp_dir().join("phiano_curric_fold");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("wiktionary.jsonl");
        std::fs::write(&path, jsonl).unwrap();

        let source = WiktionarySource::new(path.to_str().unwrap());
        let sentences: Vec<String> = source
            .relation_sentences()
            .into_iter()
            .map(|s| s.join(" "))
            .collect();
        assert!(sentences.contains(&"cat synonym feline".to_string()));
        assert!(sentences.contains(&"cat hypernym animal".to_string()));
        // Every folded sentence is a three-word relation the trainer can read.
        assert!(sentences.iter().all(|s| s.split_whitespace().count() == 3));
        let _ = std::fs::remove_dir_all(&dir);
    }
}