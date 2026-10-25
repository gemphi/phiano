//! The student of code on the base, measured (docs/124). Phi-4's answers
//! to `code_quest` are told to `student::Student`; each held-out task is
//! asked by its signature and the answer checked by its family's asserts
//! (`code_write/checks.rs`, audited in docs/119). Three tests:
//!
//! - `--by-task`: a task held out alone, its type siblings told (a new type
//!   of a known task);
//! - by family (default): the whole family held out (a task never seen);
//! - `--one-pointing`: the family held out but one of its answers told
//!   (the word taught once, as `sepia` in the builders' game).
//!
//! `--unknown words.tsv` writes the words of the task names the student does
//! not know yet (`student::Student::unknown`), for `code_quest --uses` to ask
//! Phi-4 about: the student's own chain of requests. Answers land in
//! `uses/<word>/` and are told with the rest; one whose name is a held-out
//! task's is held out with it.
//!
//! `cargo run --release --bin code_student -- --phi4 data/phi4_code [--by-task | --one-pointing] [--flat | --breadth | --weights path] [--show 4] [--ask "fn …"] [--unknown words.tsv]`

#[allow(dead_code)]
#[path = "../reader/calibrate.rs"]
mod calibrate;
#[allow(dead_code)]
#[path = "../quest/tasks.rs"]
mod tasks;
#[path = "../reader/lex.rs"]
mod lex;
#[allow(dead_code)]
#[path = "../reader/models.rs"]
mod models;
#[allow(dead_code)]
#[path = "../reader/score.rs"]
mod score;
#[allow(dead_code)]
#[path = "../write/analog.rs"]
mod analog;
#[allow(dead_code)]
#[path = "../write/checks.rs"]
mod checks;
#[allow(dead_code)]
#[path = "../write/write.rs"]
mod write;
mod answer;
mod rows;
mod student;
mod writing;

use answer::{answer, family_of};
use student::Student;

pub fn run(a: Vec<String>) {
    let mut a = a;
    let show: usize = score::flag(&mut a, "--show").and_then(|s| s.parse().ok()).unwrap_or(4);
    let phi4_dir = score::flag(&mut a, "--phi4").unwrap_or_else(|| "data/phi4_code".into());
    let by_task = a.iter().position(|x| x == "--by-task").map(|i| a.remove(i)).is_some();
    let one_pointing = a.iter().position(|x| x == "--one-pointing").map(|i| a.remove(i)).is_some();
    // `--flat`: no scope tree (docs/124's reading); the default reads under
    // the scopes a cast finds (docs/126).
    let levels = a.iter().position(|x| x == "--flat").map(|i| a.remove(i)).is_none();
    // `--breadth` or `--weights breadth|path`: the levels under another rule (docs/147, docs/149).
    let weights = match (a.iter().position(|x| x == "--breadth").map(|i| a.remove(i)), score::flag(&mut a, "--weights").as_deref()) {
        (Some(_), _) | (_, Some("breadth")) => Some(phiano::relate::Weighting::Breadth),
        (_, Some("path")) => Some(phiano::relate::Weighting::Path),
        _ => None,
    };
    let student = || {
        let mut s = Student::new(levels);
        if let Some(w) = weights {
            s.counts.weights = w;
        }
        s
    };
    lex::KEEP_SMALL.store(true, std::sync::atomic::Ordering::Relaxed);
    let mut phi4 = Vec::new();
    lex::files(std::path::Path::new(&phi4_dir), &mut phi4);
    let phi4: Vec<(String, String, Vec<String>)> = phi4.into_iter().filter_map(|(p, t)| std::fs::read_to_string(&p).ok().map(|text| (p, text, t))).collect();
    let rel = |p: &str| std::path::Path::new(p).strip_prefix(&phi4_dir).map_or(p.to_string(), |r| r.to_string_lossy().into_owned());
    let fns: Vec<tasks::Task> = tasks::all().into_iter().filter(|t| t.signature.starts_with("fn ")).filter(|t| phi4.iter().any(|(p, _, _)| p.ends_with(&format!("{}.rs", t.name)) && family_of(p) == t.family)).collect();

    if let Some(path) = score::flag(&mut a, "--unknown") {
        let mut s = student();
        phi4.iter().for_each(|(p, text, toks)| { s.learn(text, toks, &rel(p)); });
        let mut words: Vec<(String, String)> = fns.iter().flat_map(|t| s.unknown(t.name.rsplit_once('_').map_or(t.name.as_str(), |x| x.0))).collect();
        words.sort();
        words.dedup_by(|a, b| a.0 == b.0);
        let tsv: String = words.iter().map(|(w, e)| format!("{w}\t{e}\n")).collect();
        std::fs::write(&path, &tsv).expect("write words");
        println!("{} words not yet known (told in fewer than {} functions) → {path}\n{}", words.len(), student::KNOWN, words.iter().map(|w| w.0.as_str()).collect::<Vec<_>>().join(" "));
        return;
    }
    if let Some(signature) = score::flag(&mut a, "--ask") {
        let mut s = student();
        phi4.iter().for_each(|(p, text, toks)| { s.learn(text, toks, &rel(p)); });
        let name = signature.split([' ', '(', '<']).nth(1).unwrap_or("");
        for (w, _) in s.unknown(name) {
            println!("not yet known: `{w}` (ask: code_quest --uses)");
        }
        let conds = Student::conditions(&signature);
        let (path, moved) = s.drawn(&conds, &s.counts.values("said-again"));
        println!("drawn into {} (moves {} answers)", if path.is_empty() { "nothing".to_string() } else { path.join("/") }, moved.iter().filter(|m| m.abs() > 0.01).count());
        let ranked = s.write(&signature, 8);
        for (i, (body, src, why)) in ranked.iter().enumerate() {
            println!("{}. says again `{src}`{}", i + 1, if why.is_empty() { String::new() } else { format!(": {}", why.join("; ")) });
            let _ = body;
        }
        match answer(&s, &signature) {
            Some((text, src, _, n)) => println!("\n{text}(the {n}. candidate, from `{src}`, compiles)"),
            None => println!("\nno candidate compiles"),
        }
        return;
    }

    // A use Phi-4 showed for a word (`uses/<word>/<fn>.rs`) is held out when
    // its function is the held-out task's own (the same name is the answer).
    let stems: Vec<(String, String)> = fns.iter().map(|t| (t.family.clone(), t.name.rsplit_once('_').map_or(t.name.clone(), |x| x.0.to_string()))).collect();
    let held = |p: &str, t: &tasks::Task| {
        let stem = std::path::Path::new(p).file_stem().map_or(String::new(), |x| x.to_string_lossy().into_owned());
        let same_fn = p.contains("/uses/") && stems.iter().any(|(f, s)| *f == t.family && *s == stem);
        same_fn || if by_task { p.ends_with(&format!("/{}.rs", t.name)) } else { family_of(p) == t.family }
    };
    let test_name = match (by_task, one_pointing) {
        (true, _) => "a new type of a known task (the task held out alone, its type siblings told)",
        (false, true) => "one pointing (the family held out, one of its answers told)",
        (false, false) => "a task never seen (the whole family held out)",
    };
    let (mut n, mut built, mut works, mut checked, mut said) = (0usize, 0usize, 0usize, 0usize, [0usize; 5]);
    let mut examples = String::new();
    let mut fails = String::new();
    for t in &fns {
        let mut s = student();
        let mut shown: Option<&str> = None;
        for (p, text, toks) in &phi4 {
            let mine = held(p, t);
            // One pointing: the first sibling of the family that is not the task.
            let point = one_pointing && !by_task && mine && !p.ends_with(&format!("/{}.rs", t.name)) && shown.is_none();
            if !mine || point {
                s.learn(text, toks, &rel(p));
                if point {
                    shown = Some(p);
                }
            }
        }
        n += 1;
        let got = answer(&s, &t.signature);
        let text = got.as_ref().map_or(String::new(), |g| g.0.clone());
        built += got.is_some() as usize;
        if let Some(g) = &got {
            said[(g.3 - 1).min(4)] += 1;
        }
        let w = if text.is_empty() { checks::works(t, "").map(|_| false) } else { checks::works(t, &text) };
        checked += w.is_some() as usize;
        works += (w == Some(true)) as usize;
        if std::env::var("CODE_STUDENT_DEBUG").is_ok() {
            eprintln!("== {} works={w:?} {}\n{text}", t.name, got.as_ref().map_or(String::new(), |g| format!("from `{}` {:?}", g.1, g.2)));
        }
        let note = |g: &(String, String, Vec<String>, usize)| format!("says again `{}`{}", g.1, if g.2.is_empty() { String::new() } else { format!("; {}", g.2.join("; ")) });
        if w == Some(true) && examples.matches("```rust").count() < show {
            examples += &format!("\n`{}` (works): {}\n\n```rust\n{text}```\n", t.name, got.as_ref().map(note).unwrap_or_default());
        } else if w == Some(false) && fails.matches("```rust").count() < show {
            fails += &format!("\n`{}` (fails): {}\n\n```rust\n{text}```\n", t.name, got.as_ref().map(note).unwrap_or_else(|| "no candidate compiles".into()));
        }
    }
    let pct = |x: usize, d: usize| format!("{x} of {d} ({:.0}%)", 100.0 * x as f64 / d.max(1) as f64);
    let mut md = format!("## The student of code on the base: {test_name}\n\n| student | compiles | works (family's asserts pass) | said again: 1st / 2nd / 3rd / 4th / 5th–8th candidate |\n|---|---|---|---|\n");
    let how = match (levels, weights) {
        (false, _) => "flat (docs/124)".to_string(),
        (true, Some(w)) => format!("under the scopes a cast finds, {w:?} weights"),
        (true, None) => "under the scopes a cast finds, weights counted (docs/147)".to_string(),
    };
    md += &format!("| counts, D4 movement over the grammar's rows, {how} | {} | {} | {} |\n", pct(built, n), pct(works, checked), said.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" / "));
    let teacher = checks::teacher(&fns, &phi4.iter().map(|(p, _, _)| (family_of(p), p.clone())).collect::<Vec<_>>());
    md += &teacher.replace("| – | – | ", "| – | ").replace(" | – | 100% |", " | – |");
    println!("{md}\n### Works\n{examples}\n### Fails\n{fails}");
}
