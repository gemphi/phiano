use phiano::chunker::ChunkStore;
use phiano::config::{CHROMA_FILE, LEARNING_RATE};
use phiano::curriculum::ChildCurriculum;
use phiano::eval::Evaluator;
use phiano::facet::Facet;
use phiano::memory::Memo;
use phiano::sources::dialogue::DialogueSource;
use phiano::sources::phi4::Phi4Source;
use phiano::storage::Storage;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;
use phiano::wiki_bulk;

fn load_or_new(path: &str) -> Facet {
    match Storage::load(path) {
        Ok(f) => {
            println!("  [load] {} words from {}", f.vocabulary_size(), path);
            f
        }
        Err(_) => {
            println!("  [load] no facet at {} — starting empty", path);
            Facet::new()
        }
    }
}

fn train_file_sentences(facet: &mut Facet, trainer: &Trainer, path: &str, max_lines: usize) -> usize {
    use std::io::BufRead;
    let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return 0,
    };
    let mut n = 0usize;
    for line in std::io::BufReader::new(file).lines().flatten() {
        for sentence in Tokenizer::split_sentences(&line) {
            if Tokenizer::tokenize(&sentence).len() >= 4 {
                trainer.train_sentence(facet, &sentence);
                n += 1;
            }
        }
        if n >= max_lines {
            break;
        }
    }
    n
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let wiki_topics: usize = args
        .iter()
        .position(|a| a == "--wiki")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(24);
    let skip_wiki = args.iter().any(|a| a == "--no-wiki");
    let skip_phi4 = args.iter().any(|a| a == "--no-phi4");
    let out = args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| CHROMA_FILE.to_string());

    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  PHIANO — ingest curriculum + wiki + phi-4 + dialog  ║");
    println!("╚══════════════════════════════════════════════════════╝\n");

    let mut facet = load_or_new(&out);
    let trainer = Trainer::new(LEARNING_RATE);
    let mut memo = Memo::new();
    let chunks = ChunkStore::new("data/chunks");

    let curriculum = ChildCurriculum::new();
    if !curriculum.stages.is_empty() {
        let r = curriculum.run(&mut facet, &trainer, &chunks);
        println!(
            "  [curriculum] {} stages, {} sentences, vocab {}",
            r.stages_completed, r.sentences_trained, facet.vocabulary_size()
        );
    }

    let rust_n = train_file_sentences(&mut facet, &trainer, "data/rust_book_corpus.txt", 2000);
    if rust_n > 0 {
        println!("  [rust-book] {} sentences, vocab {}", rust_n, facet.vocabulary_size());
    }

    let dialogues = DialogueSource::default_curriculum();
    let d = dialogues.learn_into_facet(&mut facet, &mut memo, &trainer);
    println!("  [dialogue] {} turns, vocab {}", d, facet.vocabulary_size());

    if !skip_phi4 {
        let phi4 = Phi4Source::discover();
        let s = phi4.learn_into_facet(&mut facet, &trainer);
        println!(
            "  [phi-4] vocab+{} merges+{} docs+{} → {}",
            s.vocab_tokens_loaded, s.merges_trained, s.doc_sentences_trained, s.final_vocabulary_size
        );
    }

    if !skip_wiki && wiki_topics > 0 {
        let rt = tokio::runtime::Runtime::new().expect("tokio");
        let result = rt.block_on(wiki_bulk::WikiBulk::bulk_ingest(
            &mut facet,
            &trainer,
            &chunks,
            Some(wiki_topics),
        ));
        println!(
            "  [wiki] {}/{} topics, {} tokens, vocab {}",
            result.topics_succeeded,
            result.topics_attempted,
            result.total_tokens_trained,
            result.vocabulary_after
        );
    }

    Storage::save(&facet, &out).expect("save facet");
    let eval = Evaluator::new().eval(&facet, "the child learns language through conversation");
    println!("\n  saved {}  vocab {}", out, facet.vocabulary_size());
    println!(
        "  probe coherence={:.3} novelty={:.3} resonance={:.3}",
        eval.coherence, eval.novelty, eval.resonance
    );
}
