//! `student` — one student, every method, one honest protocol (docs/172).
//!
//! Replaces `learn_grammar`, `learn_person`, `learn_bcpl`: the step-by-step
//! method is shared ([`phiplm::study::Study`] — predict first, learn after,
//! a curve over the stream) and the student takes three lessons with it:
//!
//! 1. **grammar** — Murphy's units as a concept tree: [`paragraph`] rows
//!    under each concept path, [`Knowing::tell`] one example at a time,
//!    the next word predicted UNDER its concept before each tell (Q2);
//!    then the practice questions against the book's Key.
//! 2. **person** — a Being told a person's lines one by one, the next
//!    word-choice predicted before each line (Q3, on the fly, no retrain).
//! 3. **code** — BCPL functions read online, the next role predicted from
//!    the construct nesting before each is learned (Q1, by depth).
//!
//! ```text
//! cargo run --release -p phiano --example student -- [bcpl-dir]
//! ```

use phiano::sys::{Agent, Being, Persona};
use phiecs::Row;
use phiplm::prose::{paragraph, tokens, Knowing};
use phiplm::relate::scopes;
use phiplm::study::Study;

/// The curriculum: concept path and example sentences (Murphy, Units 1-7).
const UNITS: &[(&str, &[&str])] = &[
    ("grammar/present-continuous", &[
        "she is driving to work", "i am trying to work", "he is having a shower",
        "it is not raining any more", "are you enjoying it",
        "they are building their own house", "the population is increasing very fast",
    ]),
    ("grammar/present-simple", &[
        "nurses look after patients", "the earth goes round the sun",
        "the cafe opens at seven thirty", "rice does not grow in cold climates",
        "i get up at eight every morning", "julie does not drink tea very often",
    ]),
    ("grammar/past-simple", &[
        "i did a lot of work yesterday", "we went to the cinema last night",
        "she worked in a bank for five years", "it did not rain last week",
    ]),
    ("grammar/present-perfect", &[
        "i have done a lot of work today", "she has gone to the bank",
        "we have known each other for years", "it has not rained for a month",
    ]),
];

/// The person: a few of their lines, in the order they said them.
const PERSON: &[&str] = &[
    "i think the system should learn on the fly",
    "i think we should keep it simple",
    "the system learns a little and keeps learning",
    "i think meaning is a hierarchy not a word",
    "we should keep the operators simple",
];

/// Lesson 1: the concepts. Predict each next word under its concept BEFORE
/// telling the example; the cast does the resolving, nothing bespoke.
fn grammar() -> (Knowing, Study) {
    let vocab: std::collections::HashSet<&str> =
        UNITS.iter().flat_map(|(_, exs)| exs.iter().flat_map(|e| e.split(' '))).collect();
    let mut study = Study::new("grammar under concepts", 5, 100.0 / vocab.len() as f64);
    let mut k = Knowing::fresh();
    let total: usize = UNITS.iter().map(|(_, e)| e.len()).sum();
    let mut i = 0;
    for (concept, exs) in UNITS {
        for ex in *exs {
            let words = tokens(ex);
            k.within = scopes(concept, 0);
            for j in 1..words.len() {
                let (_, ranked, _) = k.next(&words[..j]);
                study.tick(i, total, ranked.first().is_some_and(|(w, _)| *w == words[j]));
            }
            let path = format!("{concept}/e{}", i + 1);
            k.tell(&path, &paragraph(&path, &[ex.to_string()]));
            i += 1;
        }
    }
    (k, study)
}

/// The practice question, against the book's Key: under `concept`, after
/// `said`, what comes next?
fn practice(k: &mut Knowing, concept: &str, said: &[&str], key: &str) {
    k.within = scopes(concept, 0);
    let said: Vec<String> = said.iter().map(|s| s.to_string()).collect();
    let (_, ranked, _) = k.next(&said);
    let got = ranked.first().map(|(w, _)| w.as_str()).unwrap_or("—");
    println!("  under {:<20} \"{}\" → {got}   (key: {key}) {}",
        concept.trim_start_matches("grammar/"), said.join(" "),
        if got == key { "✓" } else { "✗" });
}

/// Lesson 2: the person. Predict their next word-choice BEFORE each line.
fn person() -> Study {
    let vocab: std::collections::HashSet<&str> = PERSON.iter().flat_map(|l| l.split(' ')).collect();
    let mut study = Study::new("a person, line by line", 5, 100.0 / vocab.len() as f64);
    let mut b = Being::new(Persona::from_md("---\nname: Friend\nseeds: think\nkinds: next\n---\n").unwrap());
    for (i, line) in PERSON.iter().enumerate() {
        let w = tokens(line);
        for p in w.windows(2) {
            study.tick(i, PERSON.len(), b.ask(&p[0], "next").first().is_some_and(|(t, _)| *t == p[1]));
        }
        b.tell(&w.windows(2).map(|p| Row::new(&p[0], "next", &p[1])).collect::<Vec<_>>());
    }
    study
}

/// The teacher's answers to the student's quest (docs/130): more sentences
/// of the same four grammars, shown when the student's knowledge is broken
/// — never the unit names, only the uses.
const SHOWN: &[(usize, &str)] = &[
    (0, "the water is boiling can you turn it off"), (0, "my brother is looking for a job"),
    (0, "we are staying at a very nice hotel"), (0, "she is not working this week"),
    (0, "i am getting hungry"), (0, "they are not watching the film"),
    (1, "water boils at a hundred degrees"), (1, "banks lend money to people"),
    (1, "i do not drink coffee at night"), (1, "the museum closes at five"),
    (1, "she speaks four languages"), (1, "we do not watch television very often"),
    (2, "we did not go out last night"), (2, "they arrived at the station an hour ago"),
    (2, "i saw her in town yesterday"), (2, "he did not play very well"),
    (3, "they have lived here for ten years"), (3, "i have lost my keys"),
    (3, "she has not been to the cinema for ages"), (3, "we have just finished dinner"),
];

/// Lesson 0: discovery. The examples WITHOUT their concept labels: the
/// student reads in the order its own interest picks (the most unknown
/// words next), places each where it belongs in what it knows, and opens a
/// new scope when nothing fits — the tree is built up, not given. Scored
/// against Murphy's units it never saw: purity of the discovered groups.
/// `shown`: the teacher's extra uses, round 2 of the quest loop.
fn discovery(shown: bool) {
    let mut pool: Vec<(usize, Vec<String>)> = UNITS.iter().enumerate()
        .flat_map(|(u, (_, exs))| exs.iter().map(move |e| (u, tokens(e)))).collect();
    if shown {
        pool.extend(SHOWN.iter().map(|(u, e)| (*u, tokens(e))));
    }
    // Listen first (in the order its own interest picks — the most unknown
    // words next), so the closed stratum emerges from the counts; THEN
    // organise: group by each sentence's structural skeleton, its known
    // words — two sentences of one grammar share a skeleton (is…ing,
    // does not, did, has) even when they share no content word.
    let mut k = Knowing::fresh();
    let mut heard: Vec<(usize, Vec<String>)> = Vec::new();
    while !pool.is_empty() {
        let words: Vec<Vec<String>> = pool.iter().map(|(_, w)| w.clone()).collect();
        let (unit, w) = pool.remove(k.interest(&words).unwrap_or(0));
        k.tell(&format!("heard/e{}", heard.len()), &paragraph("heard", &[w.join(" ")]));
        heard.push((unit, w));
    }
    // Sentences that share a RECURRING construction — an adjacent pair of
    // known words seen in two or more sentences (does not, did not, has
    // not, we are) — are the same lesson: link them and the groups are the
    // connected families. The pairs were found, not given.
    let pairs: Vec<std::collections::HashSet<(String, String)>> =
        heard.iter().map(|(_, w)| k.construction(w, 2).into_iter().collect()).collect();
    let mut seen: std::collections::HashMap<&(String, String), usize> = std::collections::HashMap::new();
    for ps in &pairs {
        for p in ps { *seen.entry(p).or_default() += 1; }
    }
    let mut link: Vec<usize> = (0..heard.len()).collect();
    fn root(link: &mut Vec<usize>, mut i: usize) -> usize {
        while link[i] != i { link[i] = link[link[i]]; i = link[i]; }
        i
    }
    for i in 0..pairs.len() {
        for j in i + 1..pairs.len() {
            if pairs[i].iter().any(|p| seen[p] >= 2 && pairs[j].contains(p)) {
                let (a, b) = (root(&mut link, i), root(&mut link, j));
                link[a] = b;
            }
        }
    }
    let mut group: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
    for i in 0..heard.len() {
        let r = root(&mut link, i);
        group.entry(r).or_default().push(heard[i].0);
    }
    let group: Vec<Vec<usize>> = group.into_values().collect();
    let (groups, total) = (group.len(), heard.len());
    let pure: usize = group.iter()
        .map(|mine| (0..UNITS.len()).map(|u| mine.iter().filter(|&&x| x == u).count()).max().unwrap_or(0))
        .sum();
    let round = if shown { "round 2, after the teacher showed more uses" } else { "round 1, the book alone" };
    println!("## discovery ({round}) — no labels given:");
    println!("{groups} scopes built from {total} examples (Murphy has {}), purity {:.0}%\n", UNITS.len(), 100.0 * pure as f64 / total.max(1) as f64);
}

fn main() {
    println!("# The student — every method, one protocol: predict first, learn after\n");

    discovery(false);
    discovery(true);

    let (mut k, g) = grammar();
    println!("{}", g.report());
    println!("Practice (the same cue resolved by CONCEPT — Q2, the Key from Murphy):");
    practice(&mut k, "grammar/present-continuous", &["it", "is", "not"], "raining");
    practice(&mut k, "grammar/present-simple", &["it", "does", "not"], "grow");
    practice(&mut k, "grammar/past-simple", &["it", "did", "not"], "rain");
    practice(&mut k, "grammar/present-perfect", &["it", "has", "not"], "rained");
    println!();

    println!("{}", person().report());

    let root = std::env::args().nth(1).unwrap_or_else(|| "../ext/bcpl/cintcode/com".into());
    let bodies = phiplm::code::bcpl::read_bodies(std::path::Path::new(&root));
    if bodies.is_empty() {
        println!("## code — skipped: no BCPL at {root}");
    } else {
        let r = phiplm::code::bcpl::learn::continuous(&bodies, 16, 10);
        println!("{}", r.study.report());
        println!("({} functions; nesting max {} mean {:.1} — buildings, not leaves)", r.functions, r.max_depth, r.mean_depth);
    }
}