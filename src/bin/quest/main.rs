//! `quest` — the quest verbs in one place: `quest ingest|ask …` (docs/172).
mod ask;
mod ingest;

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let cmd = if a.is_empty() { String::new() } else { a.remove(0) };
    match cmd.as_str() {
        "ask" => ask::run(a),
        "ingest" => ingest::run(a),
        _ => eprintln!("usage: quest <ask|ingest> [args…]"),
    }
}
