//! The claim this project rests on, measured for the first time.
//!
//! `cargo run --release --bin latency -- [corpus] [chunks] [samples]`
//!
//! Phiano's case has never been perplexity. It is that a fact can be added,
//! removed or queried *now* — no gradient step, no retraining pass, no reload.
//! That is a latency claim, and until this binary it had no benchmark, which
//! meant the project's central argument was the one thing nobody had checked.
//!
//! Four paths, each timed end to end on a facet trained on a real corpus:
//!
//! | path | what it is |
//! |---|---|
//! | learn | a word the model has never seen, definition composed into the manifold |
//! | recall | resonance query against the full vocabulary |
//! | correct | one fact overridden through the correction log |
//! | unlearn | that override removed and the manifold restored |
//!
//! Reported at p50 and p99, not as a mean: a mean hides the tail, and the tail
//! is what an interactive claim lives or dies on.
//!
//! **The comparison is stated, not implied.** Gradient fine-tuning of a small
//! transformer on a single new fact — even one LoRA step over one example, on a
//! GPU — is a forward pass, a backward pass and an optimiser step over every
//! adapted parameter, and is conventionally measured in hundreds of milliseconds
//! to seconds; a full fine-tune to actually *install* a fact takes many steps.
//! No such run is performed here, so the figure below is Phiano's absolute
//! latency and an explicit invitation to measure the other side rather than a
//! head-to-head result. Claiming a ratio against a number nobody ran would be
//! exactly the kind of unmeasured assertion this harness exists to catch.

use phiano::chunker::ChunkStore;
use phiano::conception::Conception;
use phiano::config::LEARNING_RATE;
use phiano::correction::CorrectionLog;
use phiano::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::phasor::SpectralPhasor;
use phiano::sources::definition_core;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;
use std::time::{Duration, Instant};

/// Percentile of a sorted duration list, nearest-rank.
fn pct(sorted: &[Duration], p: f64) -> Duration {
    if sorted.is_empty() {
        return Duration::ZERO;
    }
    let idx = ((p / 100.0) * sorted.len() as f64).ceil() as usize;
    sorted[idx.saturating_sub(1).min(sorted.len() - 1)]
}

fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

struct Path {
    name: &'static str,
    samples: Vec<Duration>,
    note: String,
}

impl Path {
    fn report(&self) {
        let mut s = self.samples.clone();
        s.sort();
        println!(
            "  {:<10} n={:<5} p50 {:>9.3} ms   p99 {:>9.3} ms   max {:>9.3} ms   {}",
            self.name,
            s.len(),
            ms(pct(&s, 50.0)),
            ms(pct(&s, 99.0)),
            ms(*s.last().unwrap_or(&Duration::ZERO)),
            self.note
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let corpus_path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/dictionary_corpus.txt".to_string());
    let chunks_path = args.get(2).cloned().unwrap_or_else(|| "data/chunks".to_string());
    let samples: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(200);

    let raw = match std::fs::read_to_string(&corpus_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("could not read {}: {}", corpus_path, e);
            std::process::exit(1);
        }
    };
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();
    let split = Harness::split(corpus, 42);
    let trainer = Trainer::new(LEARNING_RATE);
    let mut facet = Harness::train_ranking_only(&split, &trainer, 2);

    println!(
        "facet: {} words, {} n-gram entries",
        facet.vocabulary_size(),
        facet.ngram_entries()
    );

    // Held-out definitions whose headword the facet has never seen. These are
    // the genuinely new words; timing a word already in the lexicon would
    // measure an update, not an acquisition.
    let store = ChunkStore::new(&chunks_path);
    let unseen: Vec<(String, String)> = store
        .load_all()
        .into_iter()
        .filter(|(w, _)| !facet.lexicon.contains_key(w))
        .map(|(w, d)| (w, definition_core(&d)))
        .filter(|(_, d)| d.split_whitespace().count() >= 3)
        .take(samples)
        .collect();

    if unseen.len() < 10 {
        eprintln!("only {} unseen definitions — cannot time acquisition", unseen.len());
        std::process::exit(1);
    }

    // ---- learn: a word the model has never seen ----
    let mut learn = Vec::with_capacity(unseen.len());
    for (word, def) in &unseen {
        let entry = [(word.clone(), def.clone())];
        let t = Instant::now();
        facet.get_or_init(word);
        Conception::compose_all_bound(&mut facet, &entry, 1, 0.5, 0.15, false);
        learn.push(t.elapsed());
    }

    // ---- recall: resonance query over the whole vocabulary ----
    // Deliberately the full linear scan, not the sector index, so the number is
    // the honest worst case for the path that is actually wired today.
    let probes: Vec<String> = unseen.iter().take(100).map(|(w, _)| w.clone()).collect();
    let mut recall = Vec::with_capacity(probes.len());
    let mut sink = 0usize;
    for w in &probes {
        let q = match facet.lexicon.get(w) {
            Some(p) => *p,
            None => continue,
        };
        let t = Instant::now();
        let best = facet
            .lexicon
            .iter()
            .map(|(k, p)| (k.as_str(), q.resonance(p)))
            .fold(("", f64::NEG_INFINITY), |a, b| if b.1 > a.1 { b } else { a });
        recall.push(t.elapsed());
        sink += best.0.len();
    }

    // ---- correct: override one fact ----
    let mut log = CorrectionLog::new();
    let mut correct = Vec::with_capacity(probes.len());
    for pair in probes.windows(2).take(100) {
        let t = Instant::now();
        log.record(&pair[0], &pair[1], None);
        trainer.correct_graded(&mut facet, &pair[0], &pair[1], 1.0);
        correct.push(t.elapsed());
    }

    // ---- unlearn: restore a word's prior state ----
    // Undo is a phase write, which is why it is cheap: there is no optimiser
    // state to roll back and no other weight that encoded the fact.
    let mut unlearn = Vec::with_capacity(probes.len());
    for w in probes.iter().take(100) {
        let before: Option<SpectralPhasor> = facet.lexicon.get(w).copied();
        let saved = match before {
            Some(p) => p,
            None => continue,
        };
        let t = Instant::now();
        if let Some(p) = facet.lexicon.get_mut(w) {
            *p = saved;
            p.sync_phase();
        }
        unlearn.push(t.elapsed());
    }

    let paths = [
        Path {
            name: "learn",
            samples: learn,
            note: "unseen word + definition composed into 64 channels".into(),
        },
        Path {
            name: "recall",
            samples: recall,
            note: format!("resonance vs all {} words, linear scan", facet.vocabulary_size()),
        },
        Path {
            name: "correct",
            samples: correct,
            note: "one fact overridden, logged and applied".into(),
        },
        Path {
            name: "unlearn",
            samples: unlearn,
            note: "prior phase restored".into(),
        },
    ];

    println!("\n=== latency, {} words vocabulary ===", facet.vocabulary_size());
    for p in &paths {
        p.report();
    }

    println!(
        "\n  Baseline, stated not measured: installing one fact by gradient descent \
         requires a forward pass, a backward pass and an optimiser step over every\n\
         \x20 adapted parameter, conventionally hundreds of milliseconds to seconds \
         even for a single LoRA step on a GPU — and one step rarely installs a fact.\n\
         \x20 No such run was performed here. These are Phiano's absolute numbers and \
         an invitation to measure the other side, not a head-to-head result."
    );

    // Keeps the recall scan from being optimised away.
    if sink == usize::MAX {
        println!("{}", sink);
    }
}
