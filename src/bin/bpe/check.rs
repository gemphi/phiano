//! Phi-4's tokenizer on Phiano's corpora: pieces per word, how many
//! whitespace words are rare (≤ 2 occurrences) versus how many pieces are,
//! and the ids of a few sentences for the parity check against Hugging Face
//! `tokenizers` (`torch/bpe_parity.py`).
//!
//! `cargo run --release --bin bpe_check [vocab_dir] [corpus.txt ...]`
use phiton::p64::bpe::Bpe;
use phiano::tokenizer::Tokenizer;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = args.get(1).cloned().unwrap_or_else(|| "data/tokenizers/phi4".into());
    let bpe = Bpe::from_dir(&dir).expect("vocab.json + merges.txt");
    println!("vocabulary: {} pieces", bpe.vocab_size());
    let corpora: Vec<String> = if args.len() > 2 { args[2..].to_vec() } else { vec!["data/corpora/austen/emma.txt".into(), "data/rust_book_corpus.txt".into()] };
    for path in &corpora {
        let raw = match std::fs::read_to_string(path) { Ok(r) => r, Err(_) => continue };
        let sentences = Tokenizer::split_sentences(&raw);
        let (mut words, mut pieces) = (HashMap::<String, u32>::new(), HashMap::<u32, u32>::new());
        let (mut nw, mut np) = (0usize, 0usize);
        for s in &sentences {
            for w in Tokenizer::tokenize(s) { *words.entry(w).or_default() += 1; nw += 1; }
            for id in bpe.encode(s) { *pieces.entry(id).or_default() += 1; np += 1; }
        }
        fn rare<K>(m: &HashMap<K, u32>) -> usize { m.values().filter(|&&c| c <= 2).count() }
        println!("\n{path}: {} sentences", sentences.len());
        println!("  words : {nw} tokens, {} types, {} rare (≤2) = {:.1}% of types", words.len(), rare(&words), 100.0 * rare(&words) as f64 / words.len() as f64);
        println!("  pieces: {np} tokens, {} types, {} rare (≤2) = {:.1}% of types", pieces.len(), rare(&pieces), 100.0 * rare(&pieces) as f64 / pieces.len() as f64);
        println!("  pieces per word: {:.3}", np as f64 / nw as f64);
        if path.contains("emma") {
            let mut all = String::from("[");
            for (i, s) in sentences.iter().enumerate() {
                if i > 0 { all.push(','); }
                all.push_str(&format!("{{\"text\":{s:?},\"ids\":{:?}}}", bpe.encode(s)));
            }
            all.push(']');
            std::fs::write("data/tokenizers/phi4/parity_rust_emma.json", all).unwrap();
        }
        for s in sentences.iter().take(2) {
            let ids = bpe.encode(s);
            println!("  {:?}\n    -> {:?}", &s[..s.len().min(80)], &ids[..ids.len().min(24)]);
            assert_eq!(bpe.decode(&ids), *s, "round trip");
        }
    }
    // parity dump
    let probe = ["It is a truth universally acknowledged, that a single man in possession of a good fortune, must be in want of a wife.", "The borrow checker ensures that references are always valid — 1984, don't you think?", "  leading spaces\n\nand   gaps"];
    let mut out = String::from("[");
    for (i, s) in probe.iter().enumerate() {
        if i > 0 { out.push(','); }
        out.push_str(&format!("{{\"text\":{s:?},\"ids\":{:?}}}", bpe.encode(s)));
    }
    out.push(']');
    std::fs::write("data/tokenizers/phi4/parity_rust.json", out).unwrap();
}
