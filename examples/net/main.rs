//! `net` — the phinet experiment's sides in one place: `net more|next|
//! residual …` (docs/172: no two-word files).
mod more;
mod next;
mod residual;

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let cmd = if a.is_empty() { String::new() } else { a.remove(0) };
    match cmd.as_str() {
        "more" => more::run(a),
        "next" => next::run(a),
        "residual" => residual::run(a),
        _ => eprintln!("usage: net <more|next|residual> [args…]"),
    }
}
