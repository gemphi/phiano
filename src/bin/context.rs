//! Searle's third term: does "in C" do any work?
//! `cargo run --release --bin context [chunks] [corpus]`
//!
//! The constitutive rule is "X counts as Y **in C**". Every relational
//! measurement in this project has dropped the third term — `roles::Role::Context`
//! is declared and never assigned, and every offset is X to Y with no context
//! attached. That is not a partial version of Searle's rule; it is a different
//! claim, that the step from X to Y means the same thing everywhere. Head
//! domination at 59.8% against a 9.1% floor is what that claim looks like when
//! it fails: `adversary->against` and `adversary->another` land together because
//! they share a word, and nothing in the representation knows they came from
//! different places.
//!
//! # C enters as a frame
//!
//! Each pair is tagged with the class of the gloss it came from, and the class's
//! mean offset is subtracted from its members. What remains is what the pair does
//! *that its context does not already do*. Searle's C as a frame that shifts
//! everything inside it.
//!
//! # Four sources of C, and the one that decides it
//!
//! * **none** — the baseline, what every earlier measurement used.
//! * **syntactic** — `phinum::SentenceType`, the Searle speech-act classifier
//!   already in this repo. Advertised as 16/32/64 variations; on the dictionary
//!   corpus it produces **five** distinct classes, because `v16`, `v32` and `v64`
//!   are pure hashes of the base type and add no information to it. Included at
//!   its real granularity rather than its advertised one.
//! * **discovered** — k-means over the gloss's own phase state, at K = 8 and 64.
//!   Context from use, not from a taxonomy, which is what the CLU argument asks
//!   for.
//! * **SHUFFLED** — random classes with exactly the class-size distribution of
//!   the real ones. **This is the control the experiment turns on.** Subtracting a
//!   per-class mean removes variance whatever the classes are, so a real C must
//!   beat a fake C of the same shape, not merely beat no C at all.
//!
//! Two columns per row: agreement to the fixed harmonic basis (nothing fitted,
//! so every pair is held out), and head domination against its 9.1% floor. They
//! have to move together — a transform that destroys the offsets scores well on
//! the second and nothing on the first.
//!
//! # What it measured
//!
//! ```text
//! context C                  classes   agreement   head-dom   vs shuffled
//! none (every earlier run)         1      0.2717      65.5%       +0.0000
//! syntactic (phinum)               5      0.2704      65.4%       -0.0011
//! discovered, K=8                  7      0.2761      65.6%       +0.0047
//! discovered, K=64                40      0.2784      66.2%       +0.0076
//! one class per head            2418      0.6333      86.5%       +0.2971
//! head-dom chance level                               10.0%
//! ```
//!
//! **C does not do the work.** Three findings, none of them the hoped-for one.
//!
//! The syntactic classifier — this repo's own Searle speech-act module — scores
//! **below a random partition of the same shape** (-0.0011). Whatever it is
//! classifying, it is not a context that changes what a relation means.
//!
//! Discovered context beats its null by +0.0076 at K=64, growing with K. Real,
//! and tiny: 0.008 on a 0-to-1 agreement scale, from the term that was supposed
//! to be the missing third of the rule.
//!
//! And head domination does not move. 65.5% with no context, 66.2% with the best
//! one, against a 10.0% floor — if anything conditioning makes it slightly worse.
//! That was the failure C was meant to repair, and framing C as a mean-shift does
//! not repair it.
//!
//! The last row is the ceiling and the warning. One class per head inflates
//! agreement to 0.6333, nearly +0.30 over its own shuffle — while head
//! domination climbs to 86.5%. Fine partitions make their members agree by
//! construction, and a reading that took the agreement column alone would have
//! called that the best result on the page.
//!
//! What this does not rule out: C as a *shift* is one formalisation of "in C",
//! and the obvious other one is untested — C as a prior over which relation is
//! expected, selecting among the harmonic modes rather than translating them.
//! That is a different experiment, and this result is not evidence against it.

use phiano::chunker::ChunkStore;
use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::phasor::{fnv1a, mix64};
use phiano::relation_type::{OffsetTable, RelationTypes};
use phiano::sources::definition_core;
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;
use phiano::wave::Wave;
use std::collections::HashMap;

const COUNT_FLOOR: u32 = 25;

/// Classes with fewer members than this cannot support a mean, and centering
/// them is subtracting a pair from itself.
const MIN_CLASS: usize = 8;

/// Assigns each pair a class, keeping the class-size distribution of `classes`
/// but destroying which pair belongs to which.
fn shuffle_classes(classes: &[u16], seed: u64) -> Vec<u16> {
    let mut out = classes.to_vec();
    let mut r = seed | 1;
    for i in (1..out.len()).rev() {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        out.swap(i, (r % (i as u64 + 1)) as usize);
    }
    out
}

/// K-means over gloss phase states, returning one class per gloss.
///
/// Farthest-point seeded and run to a fixed round count, so two runs on the same
/// data give the same classes.
fn discover_classes(facet: &Facet, texts: &[String], k: usize, rounds: usize) -> Vec<u16> {
    let states: Vec<Vec<f64>> = texts
        .iter()
        .map(|t| {
            let words = Tokenizer::tokenize(t);
            Wave::sentence_channels(facet, &words, false)
                .iter()
                .map(|z| z.arg())
                .collect()
        })
        .collect();
    if states.len() < k || k == 0 {
        return vec![0; texts.len()];
    }

    let agree = |a: &[f64], b: &[f64]| -> f64 {
        a.iter().zip(b.iter()).map(|(x, y)| (x - y).cos()).sum::<f64>() / PHASE_CHANNELS as f64
    };

    let mut best_sim = vec![f64::NEG_INFINITY; states.len()];
    let mut seeds: Vec<usize> = Vec::with_capacity(k);
    let mut next = 0usize;
    loop {
        seeds.push(next);
        if seeds.len() >= k {
            break;
        }
        let chosen = states[next].clone();
        for (b, s) in best_sim.iter_mut().zip(states.iter()) {
            let a = agree(s, &chosen);
            if a > *b {
                *b = a;
            }
        }
        best_sim[next] = f64::INFINITY;
        match best_sim
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
        {
            Some(i) if best_sim[i].is_finite() => next = i,
            _ => break,
        }
    }

    let mut centroids: Vec<Vec<f64>> = seeds.iter().map(|s| states[*s].clone()).collect();
    let mut assign = vec![0usize; states.len()];
    for _ in 0..rounds {
        let mut moved = false;
        for (i, s) in states.iter().enumerate() {
            let best = centroids
                .iter()
                .enumerate()
                .max_by(|a, b| {
                    agree(s, a.1).partial_cmp(&agree(s, b.1)).unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(c, _)| c)
                .unwrap_or(0);
            if assign[i] != best {
                assign[i] = best;
                moved = true;
            }
        }
        for (c, cen) in centroids.iter_mut().enumerate() {
            for kk in 0..PHASE_CHANNELS {
                let (mut x, mut y) = (0.0f64, 0.0f64);
                for (i, s) in states.iter().enumerate() {
                    if assign[i] == c {
                        x += s[kk].cos();
                        y += s[kk].sin();
                    }
                }
                if x.hypot(y) > 1e-12 {
                    cen[kk] = y.atan2(x);
                }
            }
        }
        if !moved {
            break;
        }
    }
    assign.into_iter().map(|a| a as u16).collect()
}

/// Merges classes too small to have a usable mean into one bucket.
fn collapse_small(classes: &[u16]) -> (Vec<u16>, usize) {
    let mut counts: HashMap<u16, usize> = HashMap::new();
    for c in classes {
        *counts.entry(*c).or_insert(0) += 1;
    }
    let out: Vec<u16> = classes
        .iter()
        .map(|c| match counts[c] >= MIN_CLASS {
            true => *c,
            false => u16::MAX,
        })
        .collect();
    let used = out.iter().collect::<std::collections::HashSet<_>>().len();
    (out, used)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let chunks = args.get(1).cloned().unwrap_or_else(|| "data/chunks".to_string());
    let corpus_path = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "data/dictionary_corpus.txt".to_string());

    let raw = std::fs::read_to_string(&corpus_path).unwrap_or_default();
    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();
    if corpus.is_empty() {
        eprintln!("empty corpus at {}", corpus_path);
        std::process::exit(1);
    }

    let split = Harness::split(corpus, 42);
    let base = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let train_toks: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let mut facet = base.clone();
    let dr = Descent::cycle(&mut facet, &train_toks, Up::Bag, 4, 0.25, true);

    // Pairs, each carrying the gloss it came from — that gloss is C.
    let glosses: Vec<(String, String)> = ChunkStore::new(&chunks)
        .load_all()
        .into_iter()
        .map(|(w, d)| (w, definition_core(&d)))
        .collect();
    let above = |w: &str| facet.lexicon.get(w).is_some_and(|p| p.count >= COUNT_FLOOR);

    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut sources: Vec<String> = Vec::new();
    let mut per_head: HashMap<String, usize> = HashMap::new();
    for (head, gloss) in &glosses {
        if !above(head) {
            continue;
        }
        for t in Tokenizer::tokenize(gloss) {
            if t == *head || !above(&t) {
                continue;
            }
            let n = per_head.entry(head.clone()).or_insert(0);
            if *n >= 8 {
                break;
            }
            *n += 1;
            pairs.push((head.clone(), t));
            sources.push(gloss.clone());
        }
    }

    println!(
        "facet {} words, {} pairs, descent moved {}{}",
        facet.vocabulary_size(),
        pairs.len(),
        dr.words_moved,
        match dr.rejected {
            true => " (REJECTED — this is the bottom-up control)",
            false => "",
        }
    );
    if pairs.len() < 500 {
        eprintln!("too few pairs to say anything");
        std::process::exit(1);
    }

    // ---- the sources of C ----
    let none: Vec<u16> = vec![0; pairs.len()];

    let syntactic: Vec<u16> = sources
        .iter()
        .map(|g| phiano::phinum::topology::SentenceType::classify(g).base as u16)
        .collect();

    let disc8 = discover_classes(&facet, &sources, 8, 8);
    let disc64 = discover_classes(&facet, &sources, 64, 8);

    // A class per head: the strongest possible "context", included as a ceiling
    // rather than a proposal. It removes the head effect by construction, so its
    // head-domination is what perfect conditioning looks like — and its
    // agreement column says what that costs.
    let per_head_class: Vec<u16> = pairs
        .iter()
        .map(|p| (mix64(fnv1a(&p.0)) % 60_000) as u16)
        .collect();

    println!(
        "\n  {:<26} {:>8} {:>12} {:>11} {:>12}",
        "context C", "classes", "agreement", "head-dom", "vs shuffled"
    );

    let rows: Vec<(&str, Vec<u16>)> = vec![
        ("none (every earlier run)", none),
        ("syntactic (phinum)", syntactic),
        ("discovered, K=8", disc8),
        ("discovered, K=64", disc64),
        ("one class per head", per_head_class),
    ];

    for (label, classes) in rows {
        let (classes, used) = collapse_small(&classes);
        let table = OffsetTable::build_contextual(&facet, &pairs, &classes);
        let (agree, dom) = (table.harmonic_agreement(), table.head_domination());

        // The same transform on classes of the same sizes, assigned at random.
        // Subtracting a per-class mean removes variance whatever the classes
        // are; only the gap to this row says C carried information.
        let shuffled = shuffle_classes(&classes, 0x9E37_79B9_7F4A_7C15);
        let st = OffsetTable::build_contextual(&facet, &pairs, &shuffled);
        let (s_agree, s_dom) = (st.harmonic_agreement(), st.head_domination());

        println!(
            "  {:<26} {:>8} {:>12.4} {:>10.1}% {:>+11.4}",
            label,
            used,
            agree,
            dom * 100.0,
            agree - s_agree
        );
        println!(
            "  {:<26} {:>8} {:>12.4} {:>10.1}%   (shuffled C)",
            "", used, s_agree, s_dom * 100.0
        );
    }

    // The chance level for head domination, given each head's fan-out and 64
    // modes to fall into.
    let mut fan: HashMap<&str, usize> = HashMap::new();
    for (h, _) in &pairs {
        *fan.entry(h.as_str()).or_insert(0) += 1;
    }
    let total: usize = fan.values().sum();
    let chance: f64 = fan
        .values()
        .map(|&f| {
            let f = f as f64;
            f * (1.0 - (1.0 - 1.0 / PHASE_CHANNELS as f64).powf(f - 1.0))
        })
        .sum::<f64>()
        / total.max(1) as f64;

    println!(
        "\n  head-dom chance level {:.1}%. Agreement is to the fixed harmonic basis,\n\
         \x20 so nothing is fitted and every pair is held out.\n\n\
         \x20 Read the last column. Centering within any partition removes variance,\n\
         \x20 so 'better than no C' is not evidence — a real C has to beat a fake C\n\
         \x20 of the same shape. And the two columns must move together: a transform\n\
         \x20 that merely destroys the offsets scores well on head-dom and nothing on\n\
         \x20 agreement, which is what the per-head row is there to show.",
        chance * 100.0
    );
}
