//! The circus (docs/161): two personas talk, each line a row one was told,
//! heard by the other.
//!
//! ```text
//! cargo run --release -p phiano --example circus -- <quests root> <personas dir> <A> <B> [--turns 12] [--word w]
//! ```
//!
//! `A` and `B` are persona names (case-insensitive: `gates`, `zuckerberg`,
//! `"alexandr wang"`, `musk`). The script is printed, then what each heard.

use phiano::sys::{circus, Agent};
use std::path::PathBuf;

fn flag(a: &mut Vec<String>, name: &str) -> Option<String> {
    let i = a.iter().position(|x| x == name)?;
    let v = a.get(i + 1).cloned();
    a.drain(i..(i + 2).min(a.len()));
    v
}

fn find(root: &PathBuf, dir: &PathBuf, who: &str) -> phiano::sys::Being {
    phiano::sys::find(root, dir, who).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(2)
    })
}

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let turns: usize = flag(&mut a, "--turns").and_then(|s| s.parse().ok()).unwrap_or(12);
    let word = flag(&mut a, "--word");
    let (Some(root), Some(dir), Some(x), Some(y)) = (a.first().map(PathBuf::from), a.get(1).map(PathBuf::from), a.get(2), a.get(3)) else {
        eprintln!("usage: circus <quests root> <personas dir> <A> <B> [--turns n] [--word w]");
        std::process::exit(2);
    };
    let (mut p, mut q) = (find(&root, &dir, x), find(&root, &dir, y));
    let (tp, tq) = (p.told(), q.told());
    println!("{} ({} told) and {} ({} told), {turns} turns\n", p.name(), tp, q.name(), tq);
    let script = circus::talk(&mut p, &mut q, word.as_deref(), turns);
    print!("{}", circus::render(&script));
    println!("\n{} heard {} new edges; {} heard {}.", p.name(), p.told() - tp, q.name(), q.told() - tq);
}
