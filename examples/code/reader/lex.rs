//! Rust source as a sequence of tokens, by Rust's own lexer (proc-macro2):
//! identifiers and keywords as written, punctuation one character a token,
//! brackets as open and close tokens, literals by class (`⟨num⟩`, `⟨str⟩`,
//! `⟨char⟩`) — a literal's value is not the language, its place is.

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::str::FromStr;

/// Literals kept as themselves (`code_write`, docs/118–119): in a written
/// body `0`, `2`, `"aeiou"` and `' '` are the language, not a value.
pub static KEEP_SMALL: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn walk(ts: TokenStream, out: &mut Vec<String>) {
    for t in ts {
        match t {
            TokenTree::Ident(i) => out.push(i.to_string()),
            TokenTree::Punct(p) => out.push(p.as_char().to_string()),
            TokenTree::Literal(l) => {
                let s = l.to_string();
                if KEEP_SMALL.load(std::sync::atomic::Ordering::Relaxed) {
                    out.push(s);
                    continue;
                }
                out.push(
                    match s.chars().next() {
                        Some('"') | Some('r') | Some('b') if s.contains('"') => "⟨str⟩",
                        Some('\'') => "⟨char⟩",
                        _ => "⟨num⟩",
                    }
                    .to_string(),
                );
            }
            TokenTree::Group(g) => {
                let (open, close) = match g.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Brace => ("{", "}"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::None => ("", ""),
                };
                if !open.is_empty() {
                    out.push(open.to_string());
                }
                walk(g.stream(), out);
                if !close.is_empty() {
                    out.push(close.to_string());
                }
            }
        }
    }
}

/// A file's tokens, doc comments dropped (the lexer keeps them as
/// attributes; they are prose, not code). `None` if it does not lex.
pub fn tokens(source: &str) -> Option<Vec<String>> {
    let without_docs: String = source.lines().filter(|l| !l.trim_start().starts_with("//")).collect::<Vec<_>>().join("\n");
    let ts = TokenStream::from_str(&without_docs).ok()?;
    let mut out = Vec::new();
    walk(ts, &mut out);
    Some(out)
}

/// Every `.rs` file under `dir` (target skipped), with its path.
pub fn files(dir: &std::path::Path, out: &mut Vec<(String, Vec<String>)>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut paths: Vec<_> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
    paths.sort();
    for p in paths {
        if p.is_dir() && p.file_name().is_some_and(|n| n != "target" && n != ".git") {
            files(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            if let Some(t) = std::fs::read_to_string(&p).ok().and_then(|s| tokens(&s)) {
                out.push((p.to_string_lossy().into_owned(), t));
            }
        }
    }
}
