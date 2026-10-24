//! An answer as the student hears it: the grammar's rows (`phiano::code`),
//! the name's words and signature, and the lexer's tokens with Rust's
//! two-character operators joined as the grammar counts them.

use phiecs::quest::Row;
use syn::visit::Visit;

/// Rust's two-character operators, split by the lexer, as one token each,
/// so a body's `>=` is the `>=` the grammar counted under `uses-op`.
pub fn join_ops(toks: &[String]) -> Vec<String> {
    const PAIRS: [&str; 17] = ["==", "!=", "<=", ">=", "&&", "||", "+=", "-=", "*=", "/=", "%=", "<<", ">>", "->", "=>", "::", ".."];
    let mut out: Vec<String> = Vec::with_capacity(toks.len());
    for t in toks {
        let joined = out.last().map(|p| format!("{p}{t}"));
        match joined.filter(|j| PAIRS.contains(&j.as_str())) {
            Some(j) => *out.last_mut().unwrap() = j,
            None => out.push(t.clone()),
        }
    }
    out
}

/// The rows the grammar reads off `text`, and the name of its last
/// top-level function (the answer; anything before it is a helper).
pub fn rows_of(text: &str) -> Option<(Vec<Row>, String)> {
    let file = syn::parse_file(text).ok()?;
    let name = file.items.iter().rev().find_map(|i| match i {
        syn::Item::Fn(f) => Some(f.sig.ident.to_string()),
        _ => None,
    })?;
    let mut r = phiano::code::Reader::default();
    r.visit_file(&file);
    Some((r.rows, name))
}

pub fn given(rows: &[Row], name: &str) -> (Vec<String>, Vec<String>, Option<String>) {
    let of = |k: &str| rows.iter().filter(|r| r.about == name && r.kind == k).map(|r| r.other.clone()).collect::<Vec<_>>();
    (of("has-word"), of("takes"), of("returns").into_iter().next())
}

