//! The code reader writing (docs/117): given a function's signature, it goes
//! on token by token, the likeliest token each time, until the body's braces
//! close. Read on our own Rust; asked the signatures of `code_quest`'s tasks
//! (the ones Phi-4 is asked). Scored by whether the result parses as Rust
//! (`syn`), for the 4-gram and for 4-gram ⊕ movement with learned weights
//! (docs/110). Literals come out as their class: ⟨num⟩ as `1`, ⟨str⟩ as `""`.
//!
//! In Phient's world, a word (w) is nothing but what it relates to, under what
//! conditions (c): here the conditions are the tokens before, and writing is
//! going on from them.
//!
//! With `--phi4 <dir>` (Phi-4's answers from `code_quest`), each family is
//! held out in turn: the reader learns our Rust and Phi-4's answers to the
//! other families, and writes the held-out one, scored also against Phi-4's
//! own answer and by the family's asserts (`checks.rs`).
//!
//! `cargo run --release --bin code_write -- <dir>… [--show 4] [--phi4 data/phi4_code] [--answered] [--by-task]`

#[allow(dead_code)]
#[path = "../code_reader/calibrate.rs"]
mod calibrate;
#[allow(dead_code)]
#[path = "../code_quest/tasks.rs"]
mod tasks;
#[path = "../code_reader/lex.rs"]
mod lex;
#[allow(dead_code)]
#[path = "../code_reader/models.rs"]
mod models;
#[allow(dead_code)]
#[path = "../code_reader/score.rs"]
mod score;

mod analog;
mod ask;
mod audit;
mod checks;
mod exprs;
mod gloss;
mod model;
mod proportion;
mod routes;
mod sig;
mod tally;
mod types;
mod write;

use model::Model;
use tally::Tally;
use write::{beam, compiles, render};

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let show: usize = score::flag(&mut a, "--show").and_then(|s| s.parse().ok()).unwrap_or(4);
    let phi4_dir = score::flag(&mut a, "--phi4");
    // `--answered`: only the tasks Phi-4 has answered (else every task, the
    // unanswered ones written from the answered families alone).
    let answered = a.iter().position(|x| x == "--answered").map(|i| a.remove(i)).is_some();

    lex::KEEP_SMALL.store(true, std::sync::atomic::Ordering::Relaxed);
    let mut ours = Vec::new();
    a.iter().for_each(|d| lex::files(std::path::Path::new(d), &mut ours));
    let mut phi4 = Vec::new();
    if let Some(d) = &phi4_dir {
        lex::files(std::path::Path::new(d), &mut phi4);
    }
    let family = |p: &str| std::path::Path::new(p).parent().and_then(|d| d.file_name()).map_or(String::new(), |f| f.to_string_lossy().into_owned());
    let answer = |t: &tasks::Task| phi4.iter().find(|(p, _)| p.ends_with(&format!("{}.rs", t.name)) && family(p) == t.family).map(|(_, toks)| toks.clone());
    let fns: Vec<tasks::Task> = tasks::all().into_iter().filter(|t| t.signature.starts_with("fn ")).filter(|t| !answered || answer(t).is_some()).collect();
    if a.iter().any(|x| x == "--audit") {
        let refs: Vec<(String, String, Vec<String>)> = phi4.iter().map(|(p, t)| (family(p), p.clone(), t.clone())).collect();
        return println!("{}", audit::run(&fns, &refs));
    }
    let taught = |f: &str| phi4_dir.as_ref().map(|d| std::path::Path::new(d).join(f)).unwrap_or_default();
    let (glosses, exprs) = (gloss::load(&taught("glosses.tsv")), exprs::load(&taught("exprs.tsv")));
    if let Some(signature) = score::flag(&mut a, "--ask") {
        let test = score::flag(&mut a, "--test");
        let all: Vec<Vec<String>> = phi4.iter().map(|(_, t)| t.clone()).collect();
        return ask::ask(&signature, test.as_deref(), &all, &exprs, &glosses);
    }
    // `--by-task`: each task held out alone, its type siblings seen (the test
    // of a student who has watched Phi-4: a new type of a known task).
    let by_task = a.iter().position(|x| x == "--by-task").map(|i| a.remove(i)).is_some();
    let folds: Vec<String> = match (phi4_dir.is_some(), by_task) {
        (false, _) => vec![String::new()],
        (true, false) => fns.iter().map(|t| t.family.clone()).collect::<std::collections::BTreeSet<_>>().into_iter().collect(),
        (true, true) => fns.iter().map(|t| t.name.clone()).collect(),
    };
    let held = |p: &str, h: &str| if by_task { p.ends_with(&format!("/{h}.rs")) || p.ends_with(&format!("\\{h}.rs")) } else { family(p) == h };
    let names = ["4-gram, greedy", "4-gram ⊕ movement, greedy", "the same + signature as condition (relations, copy), greedy", "the same, beam 8, first that compiles", "family resemblance (a learned answer, words exchanged; first of 5 that compiles), else the beam", "proportion (as sum_even : sum_odd, so count_even : count_odd), else family resemblance, else the beam", "the same, with Phi-4's glosses of the words (above ⇒ >, below ⇒ <)", "the same, with Phi-4's expressions for the words (even ⇒ x % 2 == 0)"];
    let mut tally = vec![Tally::default(); names.len()];
    let mut examples = String::new();
    let mut size = String::new();
    for h in &folds {
        let train: Vec<&(String, Vec<String>)> = ours.iter().chain(phi4.iter().filter(|(p, _)| !held(p, h))).collect();
        let m = Model::fit(&train);
        let learned_toks: Vec<Vec<String>> = phi4.iter().filter(|(p, _)| !held(p, h)).map(|(_, t)| t.clone()).collect();
        let learned: Vec<analog::Answer> = learned_toks.iter().map(|t| analog::answer(t)).collect();
        let taught: Vec<Vec<u32>> = learned_toks.iter().map(|t| m.encode(t)).collect();
        let sig = sig::Sig::new(&taught, m.vocab.len(), m.id.get("{").copied().unwrap_or(0), &|t: u32| score::is_identifier(&m.vocab[t as usize]));
        size = format!("{} files, {} tokens, vocabulary {}", train.len(), m.c.total, m.vocab.len());
        for t in fns.iter().filter(|t| h.is_empty() || if by_task { t.name == *h } else { t.family == *h }) {
            let toks = lex::tokens(&format!("{} {{}}", t.signature)).unwrap_or_default();
            // Words of the signature the reader never saw get ids of their
            // own past the vocabulary, so they can be said again (copied).
            let mut vocab = m.vocab.clone();
            let prompt: Vec<u32> = toks[..toks.len() - 1].iter().map(|x| m.id.get(x).copied().unwrap_or_else(|| match vocab.iter().position(|v| v == x) {
                Some(i) => i as u32,
                None => { vocab.push(x.clone()); (vocab.len() - 1) as u32 }
            })).collect();
            let copyable: Vec<usize> = prompt.iter().map(|&t| t as usize).filter(|&t| score::is_identifier(&vocab[t])).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
            let known: Vec<u32> = prompt.iter().map(|&t| if (t as usize) < m.vocab.len() { t } else { 0 }).collect();
            let mut boost = sig.boost(&known);
            boost.resize(vocab.len(), 0.0);
            let widen = |p: Vec<f64>| { let mut p = p; p.resize(vocab.len(), 0.0); p };
            let truth = answer(t);
            let with_sig = |x: &[u32]| {
                let p = widen(m.moved(x));
                let s: Vec<f64> = p.iter().zip(&boost).map(|(a, b)| a * b.exp()).collect();
                let z: f64 = s.iter().sum();
                let k = sig.kappa(*x.last().unwrap_or(&0));
                let mut q: Vec<f64> = s.into_iter().map(|v| (1.0 - k) * v / z).collect();
                copyable.iter().for_each(|&c| q[c] += k / copyable.len() as f64);
                q
            };
            for (r, name) in names.iter().enumerate() {
                let next = |x: &[u32]| match r {
                    0 => widen(m.four(x)),
                    1 => widen(m.moved(x)),
                    _ => with_sig(x),
                };
                if r >= 4 {
                    let sig_toks = toks[..toks.len() - 2].to_vec();
                    let cands = routes::vote(r, &sig_toks, &learned, &learned_toks, &exprs, &glosses);
                    let texts: Vec<String> = cands.iter().map(|b| render(&[sig_toks.clone(), b.clone()].concat())).collect();
                    let found = texts.iter().find(|x| syn::parse_file(x).is_ok() && compiles(x)).cloned();
                    // No learned answer of the same shape that compiles: the beam writes it.
                    let text = found.unwrap_or_else(|| {
                        let cands = beam(&prompt, &vocab, &with_sig, 8);
                        let r = |c: &Vec<u32>| render(&c.iter().map(|&i| vocab[i as usize].clone()).collect::<Vec<_>>());
                        cands.iter().map(r).find(|x| syn::parse_file(x).is_ok() && compiles(x)).unwrap_or_default()
                    });
                    let parses = !text.is_empty() && syn::parse_file(&text).is_ok();
                    let builds = parses && compiles(&text);
                    let works = if text.is_empty() { checks::works(t, "").map(|_| false) } else { checks::works(t, &text) };
                    if std::env::var("CODE_WRITE_DEBUG").is_ok() {
                        eprintln!("== {} works={:?} compiles={builds}\n{text}", t.name, works);
                    }
                    let body_len = cands.first().map_or(0, |b| b.len());
                    let said = truth.as_ref().zip(cands.first()).map(|(p, b)| tally::agree(&b[1..], &p[prompt.len().min(p.len())..]));
                    tally[r].add(parses, builds, body_len, works, said);
                    if works == Some(true) && examples.matches(&format!("**{name}**")).count() < show {
                        examples += &format!("\n**{name}**, `{}` (works)\n\n```rust\n{text}```\n", t.name);
                    }
                    continue;
                }
                let out = if r < 3 {
                    write::write(&prompt, &vocab, &next)
                } else {
                    let cands = beam(&prompt, &vocab, &next, 8);
                    let text = |c: &Vec<u32>| render(&c.iter().map(|&i| vocab[i as usize].clone()).collect::<Vec<_>>());
                    cands.iter().find(|c| syn::parse_file(&text(c)).is_ok() && compiles(&text(c))).or(cands.first()).cloned().unwrap_or_else(|| prompt.clone())
                };
                let words: Vec<String> = out.iter().map(|&i| vocab[i as usize].clone()).collect();
                let text = render(&words);
                let parses = syn::parse_file(&text).is_ok();
                let builds = parses && compiles(&text);
                let body = &words[prompt.len()..];
                let works = checks::works(t, &text);
                let said = truth.as_ref().map(|p| tally::agree(body, &p[prompt.len().min(p.len())..]));
                tally[r].add(parses, builds, body.len(), works, said);
                if r >= 2 && builds && examples.matches("```rust").count() < 2 * show && !examples.contains(&format!("`{}`", t.name)) {
                    examples += &format!("\n**{name}**, `{}` (compiles)\n\n```rust\n{text}```\n", t.name);
                }
            }
        }
    }
    let mut md = format!("# The code reader writing\n\nTrained on {size} (last fold). Asked to write the body of {} function signatures from `code_quest`{}.\n\n| writer | parses as Rust | compiles (rustc) | works (family's asserts pass) | mean tokens written | Phi-4's body reproduced from its start |\n|---|---|---|---|---|---|\n", fns.len(), match (phi4_dir.is_some(), by_task) { (false, _) => "", (true, false) => ", each family held out in turn (Phi-4's answers to the other families in training)", (true, true) => ", each task held out alone (its type siblings and the other families in training)" });
    for (r, name) in names.iter().enumerate() {
        md += &tally[r].row(name);
    }
    if phi4_dir.is_some() {
        md += &checks::teacher(&fns, &phi4.iter().map(|(p, _)| (family(p), p.clone())).collect::<Vec<_>>());
    }
    println!("{md}\n## Examples\n{examples}");
}
