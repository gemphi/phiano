//! Does the operator predict the next hexagram? And what does I Ching
//! novelty do to generation?
//! `cargo run --release --bin hexagram_eval [corpus]`
//!
//! A word's target side has a hexagram (its centroid phase in 64 sectors);
//! so does the operator's prediction. On held-out positions: how often does
//! the predicted hexagram equal the true next word's, against chance (1/64),
//! "same as the previous word", and "the next in the King Wen sequence"?

use phiano::config::{LEARNING_RATE, OPERATOR_FILE};
use phiano::generate::OperatorGuide;
use phient::metrics::harness::Harness;
use phient::net::space::{FunctionSpace, Sides};
use phistd::hexagram::KING_WEN_MAP;
use phiano::tokenizer::Tokenizer;
use phient::train::trainer::Trainer;
use phistd::hexagram::{hexagram_of, lines, novelty};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).cloned().unwrap_or_else(|| "data/rust_book_corpus.txt".to_string());
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 3)
        .collect();
    let split = Harness::split(corpus, 42);
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let space = FunctionSpace::from_facet_sides(&facet, Sides::Directed);
    let mut guide = OperatorGuide::load(OPERATOR_FILE, &facet, Sides::Directed, 6).expect("operator");
    let table = space.table().to_vec();
    let hex_of_word: Vec<u8> = table.iter().map(|f| hexagram_of(f)).collect();
    // King Wen successor: number n → n+1 (64 → 1), mapped back to a 6-bit id.
    let mut id_of_number = [0u8; 65];
    for (id, &n) in KING_WEN_MAP.iter().enumerate() {
        id_of_number[n as usize] = id as u8;
    }
    let king_wen_next = |id: u8| id_of_number[(KING_WEN_MAP[id as usize] as usize % 64) + 1];

    let (mut n, mut hit_op, mut hit_prev, mut hit_kw, mut hit_id) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let mut hist = [0usize; 64];
    for s in split.valid.iter().take(2000) {
        let toks = Tokenizer::tokenize(s);
        for i in 2..toks.len() {
            let (prev, next) = match (space.index.get(&toks[i - 1]), space.index.get(&toks[i])) {
                (Some(&p), Some(&t)) => (p, t),
                _ => continue,
            };
            guide.focus(&toks[..i]);
            let y = match guide.predict_public() {
                Some(y) => y,
                None => continue,
            };
            let ids: Vec<usize> = toks[..i].iter().filter_map(|w| space.index.get(w).copied()).collect();
            let ctx = match space.context(&ids, ids.len(), 6) {
                Some(c) => c,
                None => continue,
            };
            let truth = hex_of_word[next];
            hist[truth as usize] += 1;
            n += 1;
            hit_op += (hexagram_of(&y) == truth) as usize;
            hit_id += (hexagram_of(&ctx) == truth) as usize;
            hit_prev += (hex_of_word[prev] == truth) as usize;
            hit_kw += (king_wen_next(hex_of_word[prev]) == truth) as usize;
        }
    }
    let most = *hist.iter().max().unwrap_or(&0);
    println!("next-hexagram prediction on {n} held-out positions (chance 1/64 = {:.3}):", 1.0 / 64.0);
    println!("  {:<40} {:>7.3}", "operator's predicted function", hit_op as f64 / n as f64);
    println!("  {:<40} {:>7.3}", "context function (identity)", hit_id as f64 / n as f64);
    println!("  {:<40} {:>7.3}", "same hexagram as the previous word", hit_prev as f64 / n as f64);
    println!("  {:<40} {:>7.3}", "next in the King Wen sequence", hit_kw as f64 / n as f64);
    println!("  {:<40} {:>7.3}", "most frequent hexagram, always", most as f64 / n as f64);

    // The lines of a few words, and the novelty of each single-line gate.
    println!("\nhexagram readings (octave energy, bottom line first):");
    for w in ["the", "borrow", "function", "compiler", "value"] {
        if let Some(&i) = space.index.get(w) {
            let e = lines(&table[i]);
            let id = hex_of_word[i];
            println!("  {:<10} ䷀ id {id:>2} (King Wen {:>2})  lines {}", w, KING_WEN_MAP[id as usize], e.iter().map(|v| format!("{v:.2}")).collect::<Vec<_>>().join(" "));
        }
    }
    if let Some(&i) = space.index.get("borrow") {
        let f = &table[i];
        println!("\nnovelty of gating `borrow` by each single-line hexagram (1 − cos):");
        for j in 0..6 {
            println!("  line {j} only (id {:>2}): {:.3}", 1u8 << j, novelty(f, 1 << j));
        }
    }
}
