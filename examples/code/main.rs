//! `code` — the code experiments in one place: `code quest|reader|relations|
//! student|write …` (docs/172: no two-word files; the verb is a subcommand).
mod quest;
mod reader;
mod relations;
mod student;
mod write;

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let cmd = if a.is_empty() { String::new() } else { a.remove(0) };
    match cmd.as_str() {
        "quest" => quest::run(a),
        "reader" => reader::run(a),
        "relations" => relations::run(a),
        "student" => student::run(a),
        "write" => write::run(a),
        _ => eprintln!("usage: code <quest|reader|relations|student|write> [args…]"),
    }
}
