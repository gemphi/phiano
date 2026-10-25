//! `persona` — the persona verbs in one place: `persona ingest|chat|quest …`
//! (docs/172: no two-word binaries; the verb is a subcommand on the noun).
mod chat;
mod ingest;
mod quest;

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let cmd = if a.is_empty() { String::new() } else { a.remove(0) };
    match cmd.as_str() {
        "chat" => chat::run(a),
        "ingest" => ingest::run(a),
        "quest" => quest::run(a),
        _ => eprintln!("usage: persona <chat|ingest|quest> [args…]"),
    }
}
