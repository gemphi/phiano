//! Builds a prose corpus from WikiText-2.
//! `cargo run --release --bin prepare_prose [in_dir] [out]`
//!
//! Every measurement in this project has run on a dictionary, and a dictionary is
//! close to the worst corpus for the questions being asked of it.
//!
//! * **Context has nothing to disambiguate.** Each entry states its relation
//!   once, so a head-filler pair almost never recurs in a second frame. Both
//!   readings of Searle's "X counts as Y in C" measured near zero here, and that
//!   is what you would expect from a corpus with no varying C to find.
//! * **There is no discourse.** Entries are alphabetical and unrelated, so the
//!   next-sentence benchmark has been asking the model to continue a text that
//!   does not continue. It has sat at 1.01x chance through every change.
//! * **There is no external yardstick.** Nothing published reports perplexity on
//!   a scrape of Webster's, so gamma* = 0 has never been stated next to a number
//!   anyone else can check.
//!
//! WikiText-2 fixes all three. It is contiguous Wikipedia prose, the same word
//! recurs across articles in genuinely different frames, and it is a standard
//! language-modelling benchmark with published perplexities — so the phase
//! manifold can finally be put beside numbers it did not choose.
//!
//! # What this does to the text
//!
//! WikiText ships pre-tokenised with a few conventions that would otherwise
//! become facts about the vocabulary:
//!
//! * `@-@`, `@.@`, `@,@` stand for punctuation inside tokens (`role @-@ playing`).
//!   Rejoined, or `playing` acquires a spurious neighbour.
//! * `<unk>` is one symbol standing for every rare word. Kept, it would become one
//!   of the most frequent tokens in the corpus and sit at the centre of every
//!   context; dropped, the sentence survives without it. Dropped.
//! * `= Title =`, `= = Section = =` are structure, not prose. Removed — but a
//!   single-`=` heading marks a new article, and that boundary is emitted as a
//!   blank line so downstream code can tell where one document ends. The
//!   next-sentence task needs that or it will draw a "continuation" from the
//!   opening of an unrelated article.

use phiano::tokenizer::Tokenizer;
use std::fs;
use std::io::Write;

/// Sentences shorter than this carry no context worth measuring.
const MIN_TOKENS: usize = 5;

fn clean(line: &str) -> String {
    let mut out = line.to_string();
    // Rejoin split punctuation before anything else looks at token boundaries.
    for (from, to) in [(" @-@ ", "-"), (" @.@ ", "."), (" @,@ ", ",")] {
        out = out.replace(from, to);
    }
    out.split_whitespace()
        .filter(|w| *w != "<unk>")
        .collect::<Vec<_>>()
        .join(" ")
}

/// `= Title =` but not `= = Section = =`.
fn is_article_heading(line: &str) -> bool {
    let t = line.trim();
    t.starts_with("= ") && t.ends_with(" =") && !t.starts_with("= =")
}

fn is_heading(line: &str) -> bool {
    let t = line.trim();
    t.starts_with('=') && t.ends_with('=')
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = args.get(1).cloned().unwrap_or_else(|| "data".to_string());
    let out_path = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "data/prose_corpus.txt".to_string());

    let mut out = String::new();
    let (mut articles, mut sentences, mut tokens) = (0usize, 0usize, 0usize);

    for part in ["train", "valid", "test"] {
        let path = format!("{dir}/wt2_{part}.txt");
        let raw = match fs::read_to_string(&path) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  [skip] {path}: {e}");
                continue;
            }
        };
        let (mut a, mut s) = (0usize, 0usize);
        for line in raw.lines() {
            if is_article_heading(line) {
                // Blank line as the document boundary, so a next-sentence task
                // never treats the start of one article as the continuation of
                // another.
                out.push('\n');
                a += 1;
                articles += 1;
                continue;
            }
            if is_heading(line) || line.trim().is_empty() {
                continue;
            }
            let cleaned = clean(line);
            for sentence in Tokenizer::split_sentences(&cleaned) {
                let n = Tokenizer::tokenize(&sentence).len();
                if n < MIN_TOKENS {
                    continue;
                }
                out.push_str(sentence.split_whitespace().collect::<Vec<_>>().join(" ").as_str());
                out.push('\n');
                s += 1;
                sentences += 1;
                tokens += n;
            }
        }
        println!("  [{part:>5}] {a} articles, {s} sentences");
    }

    if sentences == 0 {
        eprintln!(
            "no input found. Fetch WikiText-2 into {dir}/ as wt2_train.txt, \
             wt2_valid.txt, wt2_test.txt first."
        );
        std::process::exit(1);
    }

    let mut f = fs::File::create(&out_path).expect("failed to create corpus");
    f.write_all(out.as_bytes()).expect("failed to write corpus");

    println!(
        "\n  {} articles, {} sentences, {} tokens -> {}",
        articles, sentences, tokens, out_path
    );
    println!(
        "  {:.1} tokens per sentence, {:.0} sentences per article",
        tokens as f64 / sentences as f64,
        sentences as f64 / articles.max(1) as f64
    );
}
