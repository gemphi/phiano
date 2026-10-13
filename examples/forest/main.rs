//! A story read as a forest of trees (docs/155): everything the system
//! holds about one told text, measured and written as one JSON file for the
//! documents and the app to draw from. Nothing is generated except where it
//! says so (`go`, the story continued).
//!
//! - **scopes**: the text tree it was told into, and where in *Emma* each
//!   stretch of it would be cast — the scope tree read as a bounding-volume
//!   hierarchy (`relate::tree`).
//! - **words**: per word, how often the language has heard it, what it goes
//!   on to there and here, its kin, its post (D2), and — for a word never
//!   heard — what the forest would put in its place.
//! - **world**: the story told to a phiecs world, each word's phasors on
//!   the relation circle (`before`, `after`, `near`).
//! - **waves**: the trained readers (FNO, mirror) read forward and backwards.
//! - **trees**: the sixteen trees the system holds, and where a BVH sits
//!   among them (`trees.rs`), each with what this story measured for it.
//!
//! `cargo run --release --bin forest -- data/text --story text/story/ch2 [--out data/forest/forest.json]`
//! (the strand of the same story: `strand data/text/story/ch2.txt --prior data/text/emma --out data/forest/strand`)

mod casts;
mod trees;
mod waves;
mod words;
mod world;

use phiano::prose::{tokens, Knowing};
use serde_json::{json, Value};
use std::path::PathBuf;

use phiano::lab::flag;

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let story = flag(&mut a, "--story").unwrap_or_else(|| "text/story/ch2".into());
    let out = PathBuf::from(flag(&mut a, "--out").unwrap_or_else(|| "data/forest/forest.json".into()));
    let root = PathBuf::from(a.first().map(String::as_str).unwrap_or("data/text"));
    let text = std::fs::read_to_string(root.parent().unwrap_or(&root).join(format!("{story}.txt"))).expect("the story");
    let toks = tokens(&text);
    // The language with the story told, and the language without it.
    let (with, _) = Knowing::learn(&root);
    let (without, _) = Knowing::learn_except(&root, &|p: &str| p.starts_with(&story));
    let t = std::time::Instant::now();
    let scopes = casts::scopes(&with, &without, &story, &toks);
    let words = words::words(&with, &without, &story, &toks);
    let world = world::world(&toks);
    let strand = std::fs::read_to_string("data/forest/strand/levels.tsv").unwrap_or_default();
    // What the story measured for the trees that define a count.
    let measured = vec![
        (1usize, json!({ "casts": scopes["casts"].as_array().map_or(0, Vec::len), "best_leaf": scopes["casts"][0]["found"][0]["path"].clone() })),
        (2, json!({ "paragraphs": with.paragraphs.len(), "own_scope": scopes["own"]["scope"] })),
        (3, json!({ "tokens": toks.len() })),
        (4, json!({ "levels": strand.lines().count().saturating_sub(1) })),
        (6, json!({ "posts": with.next(&toks).2.len() })),
        (7, json!({ "under": world["under"] })),
        (10, json!({ "never_heard": words.iter().filter(|x| !x["never_heard"].is_null()).count() })),
        (11, json!({ "words": world["words"], "edges": world["edges"] })),
        (16, json!({ "roots": with.tree.roots().len(), "emma_paragraphs": scopes["emma_paragraphs"] })),
    ];
    let doc: Value = json!({
        "story": { "path": story, "text": text.trim(), "tokens": toks },
        "language": { "paragraphs": with.paragraphs.len(), "rows": with.rows, "words": with.counts.values("says").len() },
        "scopes": scopes,
        "words": words,
        "go": casts::go(&with, &without, &toks),
        "world": world,
        "waves": waves::waves(&toks),
        "trees": trees::trees(&measured),
    });
    if let Some(dir) = out.parent() {
        std::fs::create_dir_all(dir).expect("out dir");
    }
    std::fs::write(&out, serde_json::to_string_pretty(&doc).expect("json")).expect("write");
    println!("{} tokens · forest written to {} in {:.1} s", toks.len(), out.display(), t.elapsed().as_secs_f64());
}
