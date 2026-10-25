//! Write a `.chroma` as a helix tree, or read one back.
//!
//! In Phient's world, a word (w) is nothing but what it relates to, under what
//! conditions (c). The helix is the lexicon as a Linux directory tree coded
//! like DNA — 64 codons per word, two strands, a position for every code —
//! so that `ls`, `grep` and `cat` are the query language (`phiano::helix`).
//!
//! `cargo run --release --bin helix -- write <facet.chroma> <dir>`
//! `cargo run --release --bin helix -- read  <dir> <facet.chroma>`
use phiano::helix;
use phiano::storage::Storage;
use std::path::Path;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let usage = || {
        eprintln!("helix write <facet.chroma> <dir> | helix read <dir> <facet.chroma>");
        std::process::exit(2)
    };
    match (a.first().map(String::as_str), a.get(1), a.get(2)) {
        (Some("write"), Some(chroma), Some(dir)) => {
            let facet = Storage::load(chroma).unwrap_or_else(|e| {
                eprintln!("cannot open {chroma}: {e}");
                std::process::exit(1)
            });
            let t = std::time::Instant::now();
            match helix::write(&facet, Path::new(dir)) {
                Ok(n) => println!("{dir}: {n} strands · 64 codons · {} marker relations · {:.1}s", facet.relations.markers(), t.elapsed().as_secs_f64()),
                Err(e) => eprintln!("cannot write {dir}: {e}"),
            }
        }
        (Some("read"), Some(dir), Some(chroma)) => match helix::read(Path::new(dir)) {
            Ok(facet) => match Storage::save(&facet, chroma) {
                Ok(()) => println!("{chroma}: {} words from {dir}", facet.lexicon.len()),
                Err(e) => eprintln!("cannot save {chroma}: {e}"),
            },
            Err(e) => eprintln!("cannot read {dir}: {e}"),
        },
        _ => usage(),
    }
}
