use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use phiano::compose::Composition;
use phiano::eval::Evaluator;
use phiano::facet::Facet;
use phiano::trainer::Trainer;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║   PHIANO - RUST BOOK ONLINE INGESTION & TRAINING BENCH   ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    let mut facet = Facet::new();
    let trainer = Trainer::new(0.15); // Learning rate
    let evaluator = Evaluator::new();

    let corpus_path = "data/rust_book_corpus.txt";
    let file = match File::open(corpus_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening corpus: {}", e);
            return;
        }
    };
    let reader = BufReader::new(file);

    let sentences: Vec<String> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.trim().is_empty())
        .collect();

    println!("--> Ingested {} clean sentences from all 104 chapters of the Rust Book.", sentences.len());
    println!("--> Initial Facet Vocabulary Size: {} words\n", facet.vocabulary_size());

    // 1. BASELINE TEST
    let test_prompts = vec![
        "ownership and borrowing rules in Rust",
        "references lifetimes and generic traits",
        "concurrency mutex channels and thread safety",
    ];

    println!("── 1. BEFORE TRAINING (BASELINE EVALUATION) ──");
    for prompt in &test_prompts {
        let eval_res = evaluator.eval(&facet, prompt);
        println!("  Prompt: \"{}\"", prompt);
        println!("  Coherence: {:.4} | Novelty: {:.4} | Resonance: {:.4}\n", 
            eval_res.coherence, eval_res.novelty, eval_res.resonance);
    }

    // 2. KURAMOTO TRAINING ON RUST BOOK CORPUS
    println!("── 2. TRAINING PHIANO ON RUST BOOK (KURAMOTO PHASE ATTRACTION) ──");
    let start_time = Instant::now();
    let epochs = 3;

    for epoch in 1..=epochs {
        let epoch_start = Instant::now();
        let mut epoch_tokens = 0;
        for line in &sentences {
            epoch_tokens += trainer.train_sentence(&mut facet, line);
        }
        println!("  [Epoch {}/{}] Updated {} token phasors in {:.2?}", 
            epoch, epochs, epoch_tokens, epoch_start.elapsed());
    }

    println!("\n--> Kuramoto Phase Attraction Training Complete in {:.2?}!", start_time.elapsed());
    println!("--> Post-Training Facet Vocabulary Size: {} words\n", facet.vocabulary_size());

    // 3. POST-TRAINING TEST & COHERENCE COMPARISON
    println!("── 3. AFTER TRAINING (EVALUATION COMPARISON) ──");
    for prompt in &test_prompts {
        let eval_res = evaluator.eval(&facet, prompt);
        println!("  Prompt: \"{}\"", prompt);
        println!("  Coherence: {:.4} | Novelty: {:.4} | Resonance: {:.4}\n", 
            eval_res.coherence, eval_res.novelty, eval_res.resonance);
    }

    // 4. RIVER FLOW COMPOSITION TEST ON RUST PROMPTS
    println!("── 4. STANDALONE OSCILLATOR RIVER FLOW COMPOSITION ──");
    let compose_prompt = "ownership borrowing and lifetime in Rust code";
    let examples = vec![
        "ownership is Rust's most unique feature enabling memory safety without garbage collection".to_string(),
        "references allow borrowing values without taking ownership".to_string(),
        "the borrow checker ensures references never outlive their data scope".to_string(),
    ];

    let comp = Composition::compose(&mut facet, &trainer, compose_prompt, &examples, 4);
    println!("{}", comp);
}
