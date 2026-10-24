//! Two personas think (docs/162): language as a set game. What each holds
//! under a kind is a relation — a set of pairs — and the two are joined by
//! union, meet and composition. Nothing is inferred that no one was told.
//!
//! ```text
//! cargo run --release -p phiano --example think -- <quests root> <personas dir> russell whitehead
//! ```

use phiano::sys::{set, Agent};
use std::path::PathBuf;

fn find(root: &PathBuf, dir: &PathBuf, who: &str) -> phiano::sys::Being {
    phiano::sys::find(root, dir, who).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(2)
    })
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (Some(root), Some(dir), Some(x), Some(y)) = (a.first().map(PathBuf::from), a.get(1).map(PathBuf::from), a.get(2), a.get(3)) else {
        eprintln!("usage: think <quests root> <personas dir> <A> <B>");
        std::process::exit(2);
    };
    let (p, q) = (find(&root, &dir, x), find(&root, &dir, y));
    println!("{} ({} told) and {} ({} told)\n", p.name(), p.told(), q.name(), q.told());
    print!("{}", set::render(p.name(), q.name(), &set::think(&p, &q)));
}
