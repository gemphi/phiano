//! The student's answer to a signature, judged by the compiler as `worse`
//! (docs/97 §2): the least worse body among what the student writes.

use super::student::Student;
use super::{lex, rows};
use super::write::render;

/// An answer: the rendered source, the learned answer it says again, the
/// exchanges made, and its 1-based rank among what the student wrote.
pub type Answered = (String, String, Vec<String>, usize);

/// The compiler as `worse` (docs/97 §2): `None` when the text does not
/// compile as a library; else how many things it leaves unused — a
/// parameter the body never reads is a condition of the order not heard.
pub fn worse(text: &str) -> Option<usize> {
    let dir = std::env::temp_dir().join(format!("code_student_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let f = dir.join("w.rs");
    std::fs::write(&f, text).ok()?;
    let out = std::process::Command::new("rustc")
        .args(["--edition", "2021", "--crate-type", "lib", "--emit=metadata", "-A", "dead_code", "-o"])
        .arg(dir.join("w.rmeta"))
        .arg(&f)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stderr).matches("warning: unused").count())
}

pub fn family_of(p: &str) -> String {
    std::path::Path::new(p).parent().and_then(|d| d.file_name()).map_or(String::new(), |f| f.to_string_lossy().into_owned())
}

/// The least worse body: the first that compiles with nothing unused, else
/// the first that compiles; with its source, exchanges and rank.
pub fn answer(s: &Student, signature: &str) -> Option<Answered> {
    let toks = rows::join_ops(&lex::tokens(&format!("{signature} {{}}")).unwrap_or_default());
    let sig_toks = toks[..toks.len() - 2].to_vec();
    let mut best: Option<(usize, Answered)> = None;
    for (i, (body, src, why)) in s.write(signature, 8).into_iter().enumerate() {
        let text = render(&[sig_toks.clone(), body].concat());
        if syn::parse_file(&text).is_err() {
            continue;
        }
        if let Some(w) = worse(&text) {
            if w == 0 {
                return Some((text, src, why, i + 1));
            }
            if best.as_ref().is_none_or(|b| w < b.0) {
                best = Some((w, (text, src, why, i + 1)));
            }
        }
    }
    best.map(|b| b.1)
}

