use std::fs;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use phiano::config::{CHROMA_FILE, LEARNING_RATE};
use phiano::curriculum::ChildCurriculum;
use phiano::eval::Evaluator;
use phiano::facet::Facet;
use phiano::storage::Storage;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;

fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  PHIANO — bootstrap a shippable default facet        ║");
    println!("╚══════════════════════════════════════════════════════╝\n");

    let mut facet = Facet::new();
    let trainer = Trainer::new(LEARNING_RATE);
    let start = Instant::now();

    let curriculum = ChildCurriculum::new();
    if !curriculum.stages.is_empty() {
        let chunk_store = phiano::chunker::ChunkStore::new("data/chunks");
        let result = curriculum.run(&mut facet, &trainer, &chunk_store);
        println!(
            "  [curriculum] {} stages, {} words, {} sentences ({} ms)",
            result.stages_completed,
            result.words_learned,
            result.sentences_trained,
            result.elapsed_ms
        );
    }

    let corpus_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let max_lines: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(4000);

    match load_sentences(&corpus_path, max_lines) {
        Ok(sentences) => {
            println!(
                "  [corpus] {} sentences from {} (cap {})",
                sentences.len(),
                corpus_path,
                max_lines
            );
            let tokens = trainer.train_corpus(&mut facet, &sentences);
            println!("  [corpus] {} token updates", tokens);
        }
        Err(e) => println!("  [corpus] skipped ({}): {}", corpus_path, e),
    }

    let out = std::env::args()
        .nth(3)
        .unwrap_or_else(|| CHROMA_FILE.to_string());
    if let Some(parent) = std::path::Path::new(&out).parent() {
        let _ = fs::create_dir_all(parent);
    }
    Storage::save(&facet, &out).expect("failed to save facet");

    let eval = Evaluator::new();
    let probe = "the child learns language through conversation";
    let scores = eval.eval(&facet, probe);
    println!("\n  vocabulary : {}", facet.vocabulary_size());
    println!("  saved      : {}", out);
    println!(
        "  probe      : coherence={:.3} novelty={:.3} resonance={:.3}",
        scores.coherence, scores.novelty, scores.resonance
    );
    println!("  elapsed    : {:?}", start.elapsed());
}

fn load_sentences(path: &str, max_lines: usize) -> std::io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut out = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        for sentence in Tokenizer::split_sentences(&line) {
            if Tokenizer::tokenize(&sentence).len() >= 3 {
                out.push(sentence);
            }
        }
        if out.len() >= max_lines {
            break;
        }
    }
    Ok(out)
}
