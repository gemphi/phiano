//! Ask an ingested quest what a word calls for under a kind.
//!
//! In Phient's world, a word (w) is nothing but what it relates to, under what
//! conditions (c). `bin/quest_ingest` wrote a quest into a facet and saved it
//! as `.chroma`; this opens that facet and puts the quest's own question back
//! to it — `word · kind · ?` — reading the kind's marker channel
//! (`Facet::under`). Nothing is looked up: the answer is where the word's
//! source side points on that channel, ranked over every word the facet has.
//!
//! `cargo run --release --bin quest_ask <quest.chroma> <word> <kind> [n]`
//! `cargo run --release --bin quest_ask <quest.chroma>` reads `word kind`
//! lines from stdin — `kinds` lists the kinds, `words` the words.
use phiano::facet::Facet;
use phiano::storage::Storage;
use std::io::BufRead;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let path = a.first().cloned().unwrap_or_else(|| "data/quest/synthetic.chroma".to_string());
    let facet = match Storage::load(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("cannot open {path}: {e}");
            std::process::exit(1);
        }
    };
    let mut pool: Vec<String> = facet.lexicon.keys().cloned().collect();
    pool.sort();
    let kinds: Vec<String> = facet.relations.entries().into_iter().map(|(w, _)| w).collect();
    let n: usize = a.get(3).and_then(|s| s.parse().ok()).unwrap_or(5);
    eprintln!("{path}: {} words · kinds: {}", pool.len(), kinds.join(", "));

    if let (Some(word), Some(kind)) = (a.get(1), a.get(2)) {
        answer(&facet, &pool, word, kind, n);
        return;
    }
    for line in std::io::stdin().lock().lines().map_while(Result::ok) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            ["kinds"] => println!("{}", kinds.join(", ")),
            ["words"] => println!("{}", pool.join(" ")),
            [word, kind] => answer(&facet, &pool, word, kind, n),
            [word, kind, k] => answer(&facet, &pool, word, kind, k.parse().unwrap_or(n)),
            _ => eprintln!("word kind [n]"),
        }
    }
}

fn answer(facet: &Facet, pool: &[String], word: &str, kind: &str, n: usize) {
    if facet.marker_channel(kind).is_none() {
        println!("{word} · {kind}: not a kind this quest learned");
        return;
    }
    if !facet.lexicon.contains_key(word) {
        println!("{word} · {kind}: not a word this quest met");
        return;
    }
    let read = facet.under(word, kind, pool);
    let shown: Vec<String> = read.iter().take(n).map(|(w, s)| format!("{w} ({s:.2})")).collect();
    println!("{word} · {kind} ⇒ {}", if shown.is_empty() { "nothing".to_string() } else { shown.join(", ") });
}
