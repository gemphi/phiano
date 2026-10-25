//! Phi-4 asked to write Rust (docs/117), so the code reader can learn to go
//! on as Phi-4's code goes on. Each answer is saved as
//! `<out>/<family>/<task>.rs` as it arrives; a stopped run resumes. Then
//! `code reader <out> --by-dir` reads Phi-4's code with whole families held
//! out.
//!
//! In Phient's world, a word (w) is nothing but what it relates to, under what
//! conditions (c): a token, under "d tokens before", in Phi-4's own code.
//!
//! `cargo run --release -p phiano --example code -- quest -- <out_dir> [--parallel 3] ollama run phi4-builder`

mod tasks;

use phiecs::quest::{builders, Oracle, Spawned};
use std::sync::Mutex;

/// The code in a reply: the first fenced block, else the whole reply.
fn code_of(reply: &str) -> String {
    let r = builders::strip_escapes(reply);
    let Some(i) = r.find("```") else { return r.trim().to_string() };
    let body = &r[i + 3..];
    let body = body.split_once('\n').map_or(body, |(_, b)| b);
    body.split("```").next().unwrap_or(body).trim().to_string()
}

pub fn run(a: Vec<String>) {
    let mut a = a;
    let parallel = match a.iter().position(|x| x == "--parallel") {
        Some(i) => {
            let p = a.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1);
            a.drain(i..(i + 2).min(a.len()));
            p
        }
        None => 1usize,
    };
    if a.len() < 2 {
        eprintln!("usage: code quest <out_dir> [--parallel n] <oracle…>");
        std::process::exit(2);
    }
    let out = std::path::PathBuf::from(&a[0]);
    let oracle = &a[1..];
    // `--glosses` as the oracle's first word: ask what code token says each word of the names.
    if oracle[0] == "--glosses" {
        return glosses(&out, &oracle[1..], false);
    }
    // `--exprs`: the same, as an expression over x (and y) rather than one token (docs/121).
    if oracle[0] == "--exprs" {
        return glosses(&out, &oracle[1..], true);
    }
    // `--uses <words.tsv>`: the words the student does not know yet (docs/124):
    // Phi-4 shows each in three functions, saved one per file under uses/<word>/.
    if oracle[0] == "--uses" {
        return uses(&out, std::path::Path::new(&oracle[1]), &oracle[2..]);
    }
    let todo: Vec<tasks::Task> = tasks::all().into_iter().filter(|t| !out.join(&t.family).join(format!("{}.rs", t.name)).exists()).collect();
    println!("{} tasks · {} to ask · {parallel} at a time · {}", tasks::all().len(), todo.len(), oracle.join(" "));
    let next = Mutex::new(0usize);
    std::thread::scope(|s| {
        for _ in 0..parallel.max(1) {
            s.spawn(|| {
                let args: Vec<&str> = oracle[1..].iter().map(String::as_str).collect();
                let mut o = Spawned::new(&oracle[0], &args);
                loop {
                    let i = {
                        let mut g = next.lock().unwrap();
                        *g += 1;
                        *g - 1
                    };
                    let Some(t) = todo.get(i) else { break };
                    let t0 = std::time::Instant::now();
                    let reply = o.ask(&tasks::prompt(t)).unwrap_or_default();
                    let code = code_of(&reply);
                    let dir = out.join(&t.family);
                    std::fs::create_dir_all(&dir).expect("create family dir");
                    std::fs::write(dir.join(format!("{}.rs", t.name)), format!("{code}\n")).expect("write answer");
                    println!("{:>4}. {:<16} {:>4} lines {:.1}s", i + 1, t.name, code.lines().count(), t0.elapsed().as_secs_f64());
                }
            });
        }
    });
}

/// `code quest <out_dir> --glosses <oracle…>`: one question per word of the
/// task names; `<out_dir>/glosses.tsv` gets `word \t token` (docs/120).
fn glosses(out: &std::path::Path, oracle: &[String], exprs: bool) {
    let path = out.join(if exprs { "exprs.tsv" } else { "glosses.tsv" });
    let done = std::fs::read_to_string(&path).unwrap_or_default();
    let args: Vec<&str> = oracle[1..].iter().map(String::as_str).collect();
    let mut o = Spawned::new(&oracle[0], &args);
    let _ = std::fs::create_dir_all(out);
    for (w, example) in tasks::words().into_iter().filter(|(w, _)| !done.lines().any(|l| l.split('\t').next() == Some(w))) {
        let prompt = if exprs { tasks::expr_prompt(&w, &example) } else { tasks::gloss_prompt(&w, &example) };
        let reply = builders::strip_escapes(&o.ask(&prompt).unwrap_or_default());
        let tok = if exprs {
            // The first line that is code: backticks and a trailing full stop dropped.
            reply.lines().map(|l| l.trim().trim_matches('`').trim_end_matches('.').trim()).find(|l| !l.is_empty() && l.contains('x') && !l.contains("```")).unwrap_or("none").to_string()
        } else {
            reply.split_whitespace().map(|x| x.trim_matches(|c: char| c == '`' || c == '"' || c == '.' || c == '\'')).find(|x| tasks::GLOSSES.contains(x)).unwrap_or("none").to_string()
        };
        println!("{w:<12} ⇒ {tok}");
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&path).expect("glosses.tsv");
        let _ = writeln!(f, "{w}\t{tok}");
    }
}

/// `code_quest <out_dir> --uses <words.tsv> <oracle…>`: for each `word \t
/// example` line, three short functions whose names say the word, each
/// saved as `uses/<word>/<fn name>.rs`. The student asked for them
/// (`code_student --unknown`); this is its chain of requests.
fn uses(out: &std::path::Path, words: &std::path::Path, oracle: &[String]) {
    let args: Vec<&str> = oracle[1..].iter().map(String::as_str).collect();
    let mut o = Spawned::new(&oracle[0], &args);
    for line in std::fs::read_to_string(words).unwrap_or_default().lines() {
        let (w, example) = line.split_once('\t').unwrap_or((line, line));
        let dir = out.join("uses").join(w);
        if dir.exists() {
            continue;
        }
        let reply = o.ask(&tasks::uses_prompt(w, example)).unwrap_or_default();
        let code = code_of(&reply);
        if syn::parse_file(&code).is_err() {
            println!("{w:<12} ⇒ no Rust");
            continue;
        }
        std::fs::create_dir_all(&dir).expect("uses dir");
        let mut n = 0;
        for text in items(&code) {
            let name = text.split(|c: char| !c.is_alphanumeric() && c != '_').skip_while(|t| *t != "fn").nth(1).unwrap_or("item");
            std::fs::write(dir.join(format!("{name}.rs")), format!("{text}\n")).expect("write use");
            n += 1;
        }
        println!("{w:<12} ⇒ {n} functions");
    }
}

/// The top-level items of `code`, each from its first line to the line
/// where its braces close.
fn items(code: &str) -> Vec<String> {
    let (mut out, mut cur, mut depth) = (Vec::new(), String::new(), 0i32);
    for line in code.lines() {
        cur += line;
        cur.push('\n');
        depth += line.matches('{').count() as i32 - line.matches('}').count() as i32;
        if depth == 0 && cur.contains('{') {
            out.push(std::mem::take(&mut cur).trim().to_string());
        }
    }
    out
}
