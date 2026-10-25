//! `code_write --ask "<signature>" [--test "<asserts>"] --phi4 <dir>`: the
//! student writes one function from everything it has learned, and says how
//! it got there — which routes arrived at the body, how many votes, whether
//! it compiles, and whether the asserts pass (docs/123, the demo).

use super::{analog, routes, write};
use std::collections::HashMap;

pub fn ask(signature: &str, test: Option<&str>, learned_toks: &[Vec<String>], ex: &HashMap<String, Vec<String>>, gl: &HashMap<String, Vec<String>>) {
    let Some(toks) = super::lex::tokens(&format!("{signature} {{}}")) else { return println!("not a signature: {signature}") };
    let sig = toks[..toks.len() - 2].to_vec();
    let learned: Vec<analog::Answer> = learned_toks.iter().map(|t| analog::answer(t)).collect();
    let cands = routes::explain(7, &sig, &learned, learned_toks, ex, gl);
    println!("asked: `{signature}` · {} answers learned · {} bodies proposed\n", learned.len(), cands.len());
    let mut chosen = false;
    for (i, (body, votes, why)) in cands.iter().take(4).enumerate() {
        let text = write::render(&[sig.clone(), body.clone()].concat());
        let builds = syn::parse_file(&text).is_ok() && write::compiles(&text);
        let passes = match (builds, test) {
            (true, Some(t)) => Some(run(&text, t)),
            _ => None,
        };
        let verdict = match (builds, passes) {
            (false, _) => "does not compile",
            (true, None) => "compiles",
            (true, Some(true)) => "compiles, test passes",
            (true, Some(false)) => "compiles, test fails",
        };
        // The student's own choice is the first that compiles; a test, when
        // given, is reported beside it and never moves the choice.
        let mark = if builds && !chosen { chosen = true; "✓ chosen" } else { "        " };
        println!("{mark} #{} · {votes} votes · {} · {verdict}\n{}", i + 1, why.join(" + "), text.lines().map(|l| format!("    {l}")).collect::<Vec<_>>().join("\n"));
    }
    if cands.is_empty() {
        println!("no route: no learned answer has this shape or shares a word with this name");
    }
}

fn run(code: &str, asserts: &str) -> bool {
    let dir = std::env::temp_dir().join(format!("code_ask_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let (src, exe) = (dir.join("a.rs"), dir.join("a.bin"));
    if std::fs::write(&src, format!("#![allow(warnings)]\n{code}\nfn main() {{ {asserts} }}\n")).is_err() {
        return false;
    }
    let built = std::process::Command::new("rustc").args(["--edition", "2021", "-o"]).arg(&exe).arg(&src).stderr(std::process::Stdio::null()).status().is_ok_and(|s| s.success());
    built && std::process::Command::new(&exe).stderr(std::process::Stdio::null()).status().is_ok_and(|s| s.success())
}
