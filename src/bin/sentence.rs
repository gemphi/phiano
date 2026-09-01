//! Is the sentence the unit of meaning? `cargo run --release --bin sentence`
//!
//! Seven independent measurements now say the phase manifold does not beat a
//! unigram at next-word prediction. Every one of those seven scored a *word*.
//! This scores a sentence.
//!
//! The claim under test: a word carries no meaning on its own, meaning lives in
//! sentences and groups of them, and what the manifold is actually offering is a
//! compression of *what comes next* at that level rather than at the token
//! level. If that is right, the word-level null results were measuring the wrong
//! thing all along, and this task should show it.
//!
//! If phase only ties the lexical-overlap baseline, it has re-derived a bag of
//! words in complex arithmetic and the claim is not supported. Beating chance is
//! the floor, not the result.

use phiano::config::LEARNING_RATE;
use phiano::metrics::harness::Harness;
use phiano::metrics::sentence::{SentenceBenchmark, CANDIDATES, CONTEXT_LEN};
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;

/// Groups consecutive sentences into documents of `n`.
///
/// Real paragraph boundaries would be better; a fixed window is the honest
/// approximation available from a flat corpus, and it keeps the context
/// genuinely contiguous, which is all the task needs.
fn documents(sentences: &[String], n: usize) -> Vec<Vec<String>> {
    sentences.chunks(n).map(|c| c.to_vec()).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let seed: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(42);

    let raw = match std::fs::read_to_string(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("could not read {}: {}", path, e);
            std::process::exit(1);
        }
    };

    // Sentence order is preserved here, unlike the shuffled 80/10/10 split the
    // word-level harness uses. A next-sentence task needs contiguity, and a
    // shuffled corpus has none.
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();

    let cut = corpus.len() * 80 / 100;
    let (train, held) = corpus.split_at(cut);
    println!(
        "corpus: {} sentences, {} train / {} held out, {} sentences of context, {} candidates",
        corpus.len(),
        train.len(),
        held.len(),
        CONTEXT_LEN,
        CANDIDATES
    );

    // Trained on the training half only. Distractors come from the held-out
    // half, so a distractor is never a sentence the model was fitted on.
    let split = Harness::split(train.to_vec(), seed);
    let trainer = Trainer::new(LEARNING_RATE).with_seed(seed);

    for (label, facet) in [
        ("co-occurrence + ranking", Harness::train_and_measure(&split, &trainer, 1, true).0),
        ("ranking only", Harness::train_ranking_only(&split, &trainer, 4)),
    ] {
        let docs = documents(held, 8);
        let pool: Vec<Vec<String>> = held.iter().map(|s| Tokenizer::tokenize(s)).collect();
        let r = SentenceBenchmark::evaluate(&facet, &docs, &pool, seed);

        println!("\n=== {} ===", label);
        println!("  vocabulary {}, {} items", facet.vocabulary_size(), r.items);
        println!(
            "  {:<18} {:>9} {:>9} {:>9}",
            "scorer", "top-1", "top-5", "MRR"
        );
        println!(
            "  {:<18} {:>8.1}% {:>9} {:>9.4}",
            "chance",
            r.chance_top1 * 100.0,
            "-",
            r.chance_mrr
        );
        for s in &r.scorers {
            println!(
                "  {:<18} {:>8.1}% {:>8.1}% {:>9.4}",
                s.name,
                s.top1 * 100.0,
                s.top5 * 100.0,
                s.mrr
            );
        }

        let lexical = r.scorers.last().expect("lexical baseline present");
        let best_phase = r.scorers[..r.scorers.len() - 1]
            .iter()
            .max_by(|a, b| a.mrr.partial_cmp(&b.mrr).unwrap_or(std::cmp::Ordering::Equal))
            .expect("at least one phase encoding");

        println!(
            "\n  best phase encoding : {} ({:.4} MRR, {:.1}x chance)",
            best_phase.name,
            best_phase.mrr,
            best_phase.mrr / r.chance_mrr
        );
        println!(
            "  vs lexical          : {} ({:+.4} MRR)",
            match best_phase.mrr > lexical.mrr {
                true => "WINS — carries more than word repetition",
                false => "LOSES — no evidence beyond a bag of words",
            },
            best_phase.mrr - lexical.mrr
        );

        // Ordered against unordered, on identical items. This is the question
        // the first version of the benchmark could not ask, because it only
        // had the unordered encoder.
        let bag = r.scorers.iter().find(|s| s.name.contains("bag"));
        if let (Some(bag), true) = (bag, best_phase.name != "phase (bag)") {
            println!(
                "  order contributes   : {:+.4} MRR over the unordered bag",
                best_phase.mrr - bag.mrr
            );
        }
    }

    println!(
        "\n  Reading this honestly: beating chance is the floor. Real continuations\n\
         \x20 repeat words, so lexical overlap is a strong and dull baseline, and only\n\
         \x20 the gap to *it* is evidence that sentence-level phase composition carries\n\
         \x20 something a bag of words does not."
    );
}
