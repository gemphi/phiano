//! A story as a tree of knowing, and going on from it (docs/127). The
//! story lies on disk as `story/<chapter>/<paragraph>/<sentence>.txt`;
//! every word is told under its path (`phiano::prose`), counted
//! (`phiano::relate::Tally`), and the tree of scopes is built from the
//! `within` rows (`ScopeTree`). Nothing is trained: a new paragraph on
//! disk is new rows the moment it is read.
//!
//! Going on: the words said so far are cast into the tree (the scopes
//! whose words they are drawn to), and the next word is read under that
//! path — story, chapter, paragraph, each a level moving the one above —
//! and then under the last word (D4 nested, `read_levels`). A word that
//! would repeat three words already said is passed over (no loops).
//!
//! `cargo run --release --bin story -- data/text [--in text/story] [--rows out.tsv] [--go "She suddenly"] [--words 40] [--beyond]`
//! `… story -- data/text --in text/story --unknown words.tsv` · `… story -- data/text --uses words.tsv ollama run phi4-builder`
//! `… story -- data/text --in text/story --teach 20 ollama run phi4-builder` (the teacher goes on; teach.log)
//! `--phasor R`: cast by the field of phasors (docs/136) instead of by counts; `--held-out` then scores both casts,
//! and `--operator R,epochs [--model file.relate]` adds the Joint operator (docs/137): the held-out positions by what the
//! counts know, the levels moving from the operator's distribution, and the field cast with its word functions
//! (`--dev 0.10`: its mixture fitted on the paragraphs between the two shares, which an operator trained so never saw);
//! `--kin` reads the unheard word through its kin (docs/138) and adds those rows and that column;
//! levels weighted by leaving out (docs/146–147); `--weights breadth|path` reads them again another way (docs/149).

mod operator;
mod strata;

use phiano::prose::Knowing;
use phiano::relate::{Tally, ScopeTree};

use phiano::lab::flag;

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    let rows_out = flag(&mut a, "--rows");
    let go = flag(&mut a, "--go");
    let words: usize = flag(&mut a, "--words").and_then(|s| s.parse().ok()).unwrap_or(40);
    let within = flag(&mut a, "--in");
    let beyond = a.iter().position(|x| x == "--beyond").map(|i| a.remove(i)).is_some();
    let quiet_flag = a.iter().position(|x| x == "--quiet").map(|i| a.remove(i)).is_some(); // only what was asked for
    let unknown = flag(&mut a, "--unknown");
    let uses = flag(&mut a, "--uses");
    // `--teach <n> <oracle…>`: n cycles of the teacher going on with the
    // story (docs/132), each logged to `<root>/<story>/teach.log`.
    let teach: Option<usize> = flag(&mut a, "--teach").and_then(|s| s.parse().ok());
    // `--held-out 0.1 [--seed 7]`: that share of the paragraphs held out
    // and the next word read at every position (docs/135).
    let held_out: Option<f64> = flag(&mut a, "--held-out").and_then(|s| s.parse().ok());
    let seed: u64 = flag(&mut a, "--seed").and_then(|s| s.parse().ok()).unwrap_or(7);
    let phasor: Option<usize> = flag(&mut a, "--phasor").and_then(|s| s.parse().ok());
    let operator: Option<(usize, u64)> = flag(&mut a, "--operator").and_then(|s| s.split_once(',').and_then(|(r, e)| Some((r.parse().ok()?, e.parse().ok()?))));
    let (model, dev) = (flag(&mut a, "--model"), flag(&mut a, "--dev").and_then(|s| s.parse::<f64>().ok()));
    let kin = a.iter().position(|x| x == "--kin").map(|i| a.remove(i)).is_some();
    // `--weights breadth|path` (`--breadth`): the held-out readings again under another rule (docs/147, 149).
    let other = match (a.iter().position(|x| x == "--breadth").map(|i| a.remove(i)), flag(&mut a, "--weights").as_deref()) {
        (Some(_), _) | (_, Some("breadth")) => Some(phiano::relate::Weighting::Breadth),
        (_, Some("path")) => Some(phiano::relate::Weighting::Path),
        _ => None,
    };
    let order = match flag(&mut a, "--order").as_deref() {
        Some("attraction") => phiano::relate::Order::Attraction,
        _ => phiano::relate::Order::Told,
    };
    let root = std::path::PathBuf::from(a.first().map(String::as_str).unwrap_or("data/story"));
    let (mut story, told) = match held_out {
        Some(share) => Knowing::learn_except(&root, &phiano::prose::heldout::held(share, seed)),
        None => Knowing::learn(&root),
    };
    if let Some(w) = &within {
        story.within = phiano::relate::scopes(w, 0);
    }
    story.order = order;
    if let (Some(r), None) = (phasor, held_out) {
        story = story.with_field(r);
    }
    if !quiet_flag {
        println!("{} paragraphs · {} rows counted · {} words in the tree", told.len(), story.rows, story.counts.values("says").len());
        let few = told.len() <= 12;
        for (path, n) in told.iter().filter(|_| few) {
            println!("  {path}: {n} words");
        }
        for root in story.tree.roots() {
            print_tree(&story.tree, &story.counts, root, 1, if few { 9 } else { 2 });
        }
    }
    if let Some(p) = rows_out {
        let tsv: String = phiano::prose::read_tree(&root).iter().flat_map(|(_, rows)| rows.iter().map(|r| format!("{}\t{}\t{}\t{:.0}\n", r.about, r.kind, r.other, r.weight))).collect();
        std::fs::write(&p, tsv).expect("write rows");
        println!("rows → {p}");
    }
    // `--unknown words.tsv`: the words of the scope (`--in`) said in fewer
    // than KNOWN paragraphs, with where they were said, for the quest.
    if let Some(p) = unknown {
        let words = story.unknown();
        let tsv: String = words.iter().map(|(w, around)| format!("{w}\t{around}\n")).collect();
        std::fs::write(&p, &tsv).expect("write words");
        println!("{} words said in fewer than {} paragraphs → {p}\n{}", words.len(), phiano::prose::KNOWN, words.iter().map(|w| w.0.as_str()).collect::<Vec<_>>().join(" "));
    }
    // `--uses words.tsv <oracle…>`: the teacher shows each word in three
    // sentences, saved as `<root>/uses/<word>.txt` (one paragraph each).
    if let Some(p) = uses {
        let args: Vec<&str> = a[1..].iter().map(String::as_str).collect();
        let mut o = phiecs::quest::Spawned::new(args[0], &args[1..]);
        let dir = root.join("uses");
        std::fs::create_dir_all(&dir).expect("uses dir");
        for line in std::fs::read_to_string(&p).unwrap_or_default().lines() {
            let (w, around) = line.split_once('\t').unwrap_or((line, line));
            let file = dir.join(format!("{w}.txt"));
            if file.exists() {
                continue;
            }
            let reply = phiecs::quest::Oracle::ask(&mut o, &phiano::prose::uses_prompt(w, around)).unwrap_or_default();
            let text = phiecs::quest::builders::strip_escapes(&reply);
            let sentences: Vec<&str> = text.lines().map(str::trim).filter(|l| l.len() > 8).collect();
            std::fs::write(&file, format!("{}\n", sentences.join(" "))).expect("write use");
            println!("{w:<16} ⇒ {} sentences", sentences.len());
        }
    }
    if held_out.is_some() {
        let n: usize = story.held.iter().map(|x| x.1.len()).sum();
        println!("{} paragraphs held out ({n} words) · seed {seed}\n\n| reading | asked | top-1 | MRR | perplexity |\n|---|---|---|---|---|", story.held.len());
        story.flat = true;
        println!("{}", phiano::prose::heldout::score(&story).row("flat: the last word, then the last two"));
        story.flat = false;
        println!("{}", phiano::prose::heldout::score(&story).row("tree: the scopes a cast finds, then the words under them"));
        if let Some(w) = other {
            story.counts.weights = w;
            story.flat = true;
            println!("{}", phiano::prose::heldout::score(&story).row(&format!("flat, {w:?} weights")));
            story.flat = false;
            println!("{}", phiano::prose::heldout::score(&story).row(&format!("tree, {w:?} weights")));
            story.counts.weights = phiano::relate::Weighting::Counted;
        }
        if let Some(r) = phasor {
            story = story.with_field(r);
            println!("{}", phiano::prose::heldout::score(&story).row(&format!("tree, cast by phasor (R = {r})")));
        }
        if kin {
            println!("{}", strata::with_kin(&mut story));
        }
        match operator {
            Some((r, epochs)) => {
                let (told, _) = operator::rows(&root, &phiano::prose::heldout::held(held_out.unwrap_or(0.0), seed));
                let (mut m, note) = operator::model(&told, r, epochs, model.as_deref());
                let vp = story.counts.values_and_prior("next");
                let fit = dev.map(|d| operator::rows(&root, &|p: &str| !phiano::prose::heldout::held(d, seed)(p) || phiano::prose::heldout::held(held_out.unwrap_or(0.0), seed)(p)).0);
                let reader = operator::Reader::fit(&mut m, fit.as_deref().unwrap_or(&told), &vp.0, &vp.1, 2000);
                println!("\n{note}; as a base, fitted on 2,000 {} rows: {}\n", if dev.is_some() { "dev" } else { "told" }, reader.note());
                println!("{}\n{}", strata::strata(&mut story, Some(&mut m)), strata::together(&mut story, std::rc::Rc::new(std::cell::RefCell::new(m)), reader));
            }
            None if kin => println!("{}", strata::strata(&mut story, None)),
            None => {}
        }
        return;
    }
    if let Some(n) = teach {
        let args: Vec<&str> = a[1..].iter().map(String::as_str).collect();
        let mut o = phiecs::quest::Spawned::new(args[0], &args[1..]);
        let mut ask = |prompt: &str| phiecs::quest::builders::strip_escapes(&phiecs::quest::Oracle::ask(&mut o, prompt).unwrap_or_default());
        let story_dir = root.join(within.as_deref().and_then(|w| w.split('/').nth(1)).unwrap_or("story"));
        let log = story_dir.join("teach.log");
        for i in 0..n {
            // The paragraph told last cycle is read into the tree; nothing
            // told before is read again (`learn_new`, docs/137).
            story.learn_new(&root);
            let k = &story;
            let Some(line) = phiano::prose::teach::cycle(k, &story_dir, &mut ask, 12) else { println!("cycle {}: the teacher said nothing usable", i + 1); continue };
            use std::io::Write;
            let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&log).expect("teach.log");
            let _ = writeln!(f, "{line}");
            println!("cycle {}: {line}", i + 1);
        }
        return;
    }
    if let Some(from) = go {
        println!("{}{}", if quiet_flag { "" } else { "\n" }, story.go(&from, words, beyond));
    }
}

/// The tree with what each scope says most (its words by count).
fn print_tree(t: &ScopeTree, c: &Tally, scope: &str, depth: usize, max_depth: usize) {
    if depth > max_depth {
        return;
    }
    let values = c.values("says");
    let base = vec![1.0 / values.len().max(1) as f64; values.len()];
    let mut said: Vec<(String, f64)> = c.dist(scope, "says", &values, &base).map(|d| values.iter().cloned().zip(d).collect()).unwrap_or_default();
    said.retain(|(w, _)| w.chars().any(char::is_alphanumeric));
    said.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    let top: Vec<&str> = said.iter().take(8).map(|x| x.0.as_str()).collect();
    println!("{}{scope} — {} words told; says: {}", "  ".repeat(depth), c.told(scope, "next") as usize, top.join(" "));
    for child in t.children(scope) {
        print_tree(t, c, child, depth + 1, max_depth);
    }
}
