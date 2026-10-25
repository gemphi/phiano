//! `arena` — many models, one run (docs/176).
//!
//! Separate runs cannot be compared. This reads ONE stream and asks every
//! model the same question at the same position before any of them is told
//! it ([`phiplm::study::Arena`]): phiplm's counts, phiplm's concepts (the
//! construction family, backing off), phiecs's rows into a `World`, and
//! phical's conjugate (the relation as a filter on the circle). The table
//! says which model is learning better, on the same positions.
//!
//! A model that must be TRAINED before it can read at all — phifno's Joint
//! operator — enters through the arena's split lane (docs/177): told the
//! stream's first half, trained once, then scored on the second half at the
//! same positions as the online readers, so the table stays one comparison.
//!
//! ```text
//! cargo run --release -p phiano --example arena -- <dir of .txt> [--lines 1500] [--epochs 4]
//! ```

use phiano::arena::{Concept, Count, Ecs, Mirror, Operator};
use phiplm::prose::tokens;
use phiplm::study::{Arena, Reader};

/// Sentences from every `.txt` under `dir`, split on full stops.
fn stream(dir: &str, lines: usize) -> Vec<Vec<String>> {
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(dir)
        .map(|r| r.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "txt")).collect())
        .unwrap_or_default();
    files.sort();
    let mut out: Vec<Vec<String>> = Vec::new();
    for f in files {
        for s in std::fs::read_to_string(&f).unwrap_or_default().split(". ") {
            let w = tokens(s);
            if w.len() >= 5 {
                out.push(w);
            }
            if out.len() >= lines {
                return out;
            }
        }
    }
    out
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let dir = a.first().cloned().unwrap_or_else(|| "data/text".into());
    let flag = |k: &str, d: usize| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(d);
    let lines = flag("--lines", 1500);
    let epochs = flag("--epochs", 4);

    let stream = stream(&dir, lines);
    if stream.is_empty() {
        eprintln!("no sentences under {dir}");
        return;
    }
    let mut vocab: Vec<String> = stream.iter().flatten().cloned().collect();
    vocab.sort();
    vocab.dedup();
    let floor = 100.0 / vocab.len() as f64;

    let readers: Vec<Box<dyn Reader>> = vec![
        Box::new(Count::new()),
        Box::new(Concept::new()),
        Box::new(Ecs::new()),
        Box::new(Mirror::new(vocab.clone(), 64, 7)),
        Box::new(Operator::new(epochs, 7)),
    ];
    let mut arena = Arena::new(readers, 5, floor);
    let t0 = std::time::Instant::now();
    // The split lane: the first half told to everyone and scored for no one,
    // where the operator is trained; the second half scored for all.
    let train = stream.len() / 2;
    let n = arena.read_split(&stream, train);

    println!("# The arena — {} models, one run (docs/176)\n", arena.ranked().len());
    println!("{} sentences from {dir} ({train} told first, unscored), {} words of", stream.len(), vocab.len());
    println!("vocabulary, {n} scored positions, floor {floor:.2}%.");
    println!("Every model guessed at every scored position BEFORE any of them was told");
    println!("the line; phifno was trained once on the told half ({epochs} epochs).\n");
    println!("{}", arena.table());
    println!("Advantages, stated not hidden: phical's filter and phifno's table were both");
    println!("given the vocabulary in advance (a word is a spectrum/row they own a slot");
    println!("for), and phifno trained by epochs over the told half, which the counting");
    println!("readers saw once. Read in {:.0} s.", t0.elapsed().as_secs_f64());
}
