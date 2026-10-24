//! The AI race (docs/160): every persona under a folder, told from its
//! quests, grouped by team, ranked by what it was told.
//!
//! ```text
//! cargo run --release -p phiano --example race -- <quests root> <personas dir> \
//!     [--engage <team> <rows.tsv>]… [--ask <name> <word> <kind>]
//! ```
//!
//! `--engage` tells every member of a team the rows of a `relations.tsv`
//! (what working with a team is, in rows); `--ask` reads one being's answer.

use phiano::sys::{race::Race, Agent, Being};
use std::path::PathBuf;

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let mut engage = Vec::new();
    while let Some(i) = a.iter().position(|x| x == "--engage") {
        let team = a.get(i + 1).cloned().unwrap_or_default();
        let file = a.get(i + 2).cloned().unwrap_or_default();
        a.drain(i..(i + 3).min(a.len()));
        engage.push((team, file));
    }
    let mut ask = None;
    if let Some(i) = a.iter().position(|x| x == "--ask") {
        ask = Some((a.get(i + 1).cloned().unwrap_or_default(), a.get(i + 2).cloned().unwrap_or_default(), a.get(i + 3).cloned().unwrap_or_default()));
        a.drain(i..(i + 4).min(a.len()));
    }
    let (Some(root), Some(dir)) = (a.first().map(PathBuf::from), a.get(1).map(PathBuf::from)) else {
        eprintln!("usage: race <quests root> <personas dir> [--engage team rows.tsv]… [--ask name word kind]");
        std::process::exit(2);
    };
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "md") && p.file_name().is_some_and(|n| n != "README.md"))
        .collect();
    files.sort();
    let beings: Vec<Being> = files.iter().map(|f| Being::from_file(&root, f).unwrap_or_else(|e| panic!("{e}"))).collect();
    println!("{} personas from {}, told from {}\n", beings.len(), dir.display(), root.display());
    let mut race = Race::new(beings);
    for (team, file) in &engage {
        let text = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("{file}: {e}"));
        let rows = phiecs::parse_tsv(&text);
        let n = race.engage(team, &rows);
        println!("engaged {team}: {} rows told to {n} members", rows.len());
    }
    if !engage.is_empty() {
        println!();
    }
    print!("{}", race.table());
    if let Some((name, word, kind)) = ask {
        let Some(b) = race.beings.iter().find(|b| b.name().eq_ignore_ascii_case(&name)) else {
            eprintln!("\nno persona called {name}");
            return;
        };
        let answers = b.ask(&word, &kind);
        println!("\n{} · {word} {kind} → {}", b.name(), match answers.is_empty() {
            true => "nothing told".to_string(),
            false => answers.iter().map(|(o, w)| format!("{o} ({w:.2})")).collect::<Vec<_>>().join(", "),
        });
    }
}
