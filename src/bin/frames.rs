//! Does the same pair mean different things in different frames?
//! `cargo run --release --bin frames [corpus]`
//!
//! Searle's "X counts as Y **in C**" was measured twice on the dictionary and
//! came out at nothing both times — as a mean shift and as a prior over the
//! harmonic modes. But a dictionary cannot answer the question. Each entry states
//! its relation once, so a pair almost never recurs in a second frame, and a C
//! with nothing to disambiguate is not a test of C.
//!
//! This runs the same two tests on prose, where `cow` appears in a paragraph
//! about farming and again in one about mythology, and the pair really does
//! recur. `bin/prepare_prose` builds the corpus from WikiText-2 — contiguous
//! Wikipedia articles, 96,000 sentences, 21 tokens each, document boundaries
//! preserved.
//!
//! # The gate
//!
//! Before either test is worth running, the corpus has to deliver what the
//! dictionary could not: pairs that occur in several *different* sentences. That
//! count is printed first, next to the dictionary's, and if it is small the rest
//! of the page means nothing.
//!
//! # The two readings, unchanged
//!
//! * **C as a shift** — subtract the frame class's mean offset, and see whether
//!   what remains agrees better with the fixed harmonic basis.
//! * **C as a mode prior** — predict which of the 64 standing waves a pair sits
//!   on from its frame class, held out, against the context-free marginal.
//!
//! Both against a shuffled-C null with identical class sizes, because any
//! partition removes variance and any partition costs estimation noise. And the
//! frame is always computed with the pair's own two words removed, or the
//! sentence is quoting the answer back.
//!
//! # What it measured
//!
//! The gate opens. 1,485,878 distinct pairs at 3.78 frames each; **202,190 pairs
//! (13.6%) occur in four or more different sentences**, against exactly one frame
//! per pair in the dictionary. The corpus delivers what was asked of it.
//!
//! ```text
//! C as a shift          classes   agreement   head-dom   vs shuffled
//! none                        1      0.6090     100.0%             -
//! discovered, K=8             8      0.6090     100.0%       -0.0001
//! discovered, K=64           15      0.6090      99.9%       -0.0001
//!
//! C as a mode prior     classes    log-loss  vs marginal   vs shuffled
//! discovered, K=8             8      0.2686      +0.0044       +0.0091
//! discovered, K=64           15      0.2753      +0.0047       +0.0158
//! ```
//!
//! And the line that makes the rest of the page redundant:
//!
//! **Two occurrences of the same pair land on the same harmonic mode 100.0% of
//! the time, over 140,000 comparisons.**
//!
//! Not 99%. Not "usually". Always — because `offset(h, f) = theta(f) - theta(h)`
//! and each word has exactly one phasor, so the relation between two words is a
//! *constant of those two words*. `cow` in a paragraph about farming and `cow` in
//! one about mythology are the same point on the manifold, so the step from it to
//! anything is the same step.
//!
//! That is the answer to the whole context question, and it is not a fact about
//! dictionaries or about Wikipedia. **"X counts as Y in C" is unrepresentable
//! here.** C cannot select, shift, or condition anything, on any corpus, because
//! there is nothing for it to vary. The two null results before this one were
//! measuring the absence correctly and attributing it to the wrong cause — mine,
//! twice: I proposed the corpus as the limitation and built this to test it, and
//! it says the limitation is the architecture.
//!
//! The residual +0.0044 in the mode-prior rows is not context changing a pair's
//! mode, which cannot happen. It is the context class correlating with *which
//! pairs* fall in it — a different and much weaker fact.
//!
//! What would make C representable: a word needs more than one position, or the
//! relation needs to be a function of `(h, f, C)` rather than `(h, f)`. The data
//! supports it — `defstats` found 1.51 dictionary senses per word, 12,394 words
//! with more than one, up to 45 — and `MAX_WORD_SENSES = 64` exists in
//! `sources` as a `.take(64)` on a list the representation then collapses to a
//! single phasor. That collapse is where Searle's third term is being lost.

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::frames::{collapse_small, discover_classes, predictive_gain, shuffle_classes};
use phiano::metrics::harness::Harness;
use phiano::relation_type::{OffsetEstimator, OffsetTable};
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;
use std::collections::HashMap;

/// Words seen fewer times than this sit near their seed and carry no relation.
const COUNT_FLOOR: u32 = 25;

/// A pair must occur in at least this many *different* sentences to be scored.
///
/// The entire point. A pair seen once has one frame, and one frame cannot show
/// that the frame matters — which is the dictionary's situation for nearly every
/// pair it produces.
const MIN_FRAMES: usize = 4;

/// Distance within a sentence for two words to count as a pair.
///
/// Adjacent-ish rather than any-two-words: a 21-token sentence has 210 unordered
/// pairs, most of which stand in no relation at all, and they would swamp the
/// ones that do.
const WINDOW: usize = 4;

/// Rows scored, most-recurring pairs first. Keeps the run affordable and spends
/// the budget on pairs that actually have several frames.
const MAX_ROWS: usize = 40_000;

/// Frames scored per pair.
///
/// Without this the sample is eaten by a handful of pairs: sorting by recurrence
/// puts `of the` at the top with thousands of occurrences, and the first run
/// spent 40,000 rows on a few hundred distinct pairs — 590 million same-pair
/// comparisons, when 40,000 rows over 10,000 pairs should give a few hundred
/// thousand. The headline finding is true either way, but every other number on
/// the page was describing `of the`.
const MAX_ROWS_PER_PAIR: usize = 8;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/prose_corpus.txt".to_string());

    let raw = match std::fs::read_to_string(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("could not read {path}: {e}\nRun `bin/prepare_prose` first.");
            std::process::exit(1);
        }
    };
    let corpus: Vec<String> = raw
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| Tokenizer::tokenize(l).len() >= 5)
        .collect();
    if corpus.len() < 1000 {
        eprintln!("corpus too small: {} sentences", corpus.len());
        std::process::exit(1);
    }

    let split = Harness::split(corpus.clone(), 42);
    let base = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);
    let train_toks: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let mut facet = base.clone();
    let dr = Descent::cycle(&mut facet, &train_toks, Up::Bag, 4, 0.25, true);

    println!(
        "prose: {} sentences, {} words, descent moved {}{}",
        corpus.len(),
        facet.vocabulary_size(),
        dr.words_moved,
        match dr.rejected {
            true => "  (REJECTED — bottom-up control)",
            false => "",
        }
    );

    // ---- pairs, and the frames they occur in ----
    let above = |w: &str| facet.lexicon.get(w).is_some_and(|p| p.count >= COUNT_FLOOR);
    let mut frames: HashMap<(String, String), Vec<usize>> = HashMap::new();
    let toks: Vec<Vec<String>> = corpus.iter().map(|s| Tokenizer::tokenize(s)).collect();

    for (si, t) in toks.iter().enumerate() {
        for i in 0..t.len() {
            if !above(&t[i]) {
                continue;
            }
            for j in (i + 1)..(i + 1 + WINDOW).min(t.len()) {
                if t[j] == t[i] || !above(&t[j]) {
                    continue;
                }
                frames
                    .entry((t[i].clone(), t[j].clone()))
                    .or_default()
                    .push(si);
            }
        }
    }

    let total_pairs = frames.len();
    let recurring: usize = frames.values().filter(|v| v.len() >= MIN_FRAMES).count();
    let mean_frames: f64 =
        frames.values().map(|v| v.len() as f64).sum::<f64>() / total_pairs.max(1) as f64;

    println!(
        "\n  === the gate ===\n\
         \x20 {} distinct pairs, {:.2} frames each on average\n\
         \x20 {} pairs ({:.1}%) occur in {} or more different sentences\n\
         \x20 the dictionary, for comparison, gives one gloss per head — a pair\n\
         \x20 there has exactly one frame by construction",
        total_pairs,
        mean_frames,
        recurring,
        recurring as f64 / total_pairs.max(1) as f64 * 100.0,
        MIN_FRAMES
    );
    if recurring < 2000 {
        eprintln!("\n  too few recurring pairs to test C. Stopping rather than reporting noise.");
        std::process::exit(1);
    }

    // One row per (pair, frame): the same pair appears several times, each with a
    // different C. That repetition is the experiment.
    let mut ranked: Vec<((String, String), Vec<usize>)> = frames
        .into_iter()
        .filter(|(_, v)| v.len() >= MIN_FRAMES)
        .collect();
    ranked.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(&b.0)));

    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut context: Vec<String> = Vec::new();
    let mut distinct = 0usize;
    'outer: for (p, sents) in &ranked {
        let before = pairs.len();
        for si in sents.iter().take(MAX_ROWS_PER_PAIR) {
            // The frame with the pair's own words taken out.
            let c: Vec<String> = toks[*si]
                .iter()
                .filter(|w| **w != p.0 && **w != p.1)
                .cloned()
                .collect();
            if c.len() < 3 {
                continue;
            }
            pairs.push(p.clone());
            context.push(c.join(" "));
        }
        if pairs.len() > before {
            distinct += 1;
        }
        if pairs.len() >= MAX_ROWS {
            break 'outer;
        }
    }
    println!(
        "  scoring {} (pair, frame) rows over {} distinct pairs, at most {} frames each",
        pairs.len(),
        distinct,
        MAX_ROWS_PER_PAIR
    );

    // ---- reading one: C as a shift ----
    println!("\n  === C as a shift ===");
    println!(
        "  {:<22} {:>8} {:>12} {:>11} {:>12}",
        "context C", "classes", "agreement", "head-dom", "vs shuffled"
    );
    let none: Vec<u16> = vec![0; pairs.len()];
    let base_table = OffsetTable::build_contextual(&facet, &pairs, &none);
    println!(
        "  {:<22} {:>8} {:>12.4} {:>10.1}% {:>12}",
        "none",
        1,
        base_table.harmonic_agreement(),
        base_table.head_domination() * 100.0,
        "-"
    );
    for k in [8usize, 64] {
        let (classes, used) = collapse_small(&discover_classes(&facet, &context, k, 8));
        let t = OffsetTable::build_contextual(&facet, &pairs, &classes);
        let st = OffsetTable::build_contextual(
            &facet,
            &pairs,
            &shuffle_classes(&classes, 0x9E37_79B9_7F4A_7C15),
        );
        println!(
            "  {:<22} {:>8} {:>12.4} {:>10.1}% {:>+12.4}",
            format!("discovered, K={k}"),
            used,
            t.harmonic_agreement(),
            t.head_domination() * 100.0,
            t.harmonic_agreement() - st.harmonic_agreement()
        );
    }

    // ---- reading two: C as a mode prior ----
    println!("\n  === C as a mode prior ===");
    let table = OffsetTable::build(&facet, &pairs, OffsetEstimator::Raw);
    let modes = table.modes();
    let kept = table.pairs().to_vec();
    let index: HashMap<(&str, &str), Vec<usize>> = {
        let mut m: HashMap<(&str, &str), Vec<usize>> = HashMap::new();
        for (i, p) in pairs.iter().enumerate() {
            m.entry((p.0.as_str(), p.1.as_str())).or_default().push(i);
        }
        m
    };
    // `build` keeps rows in order and drops none here, but realign anyway rather
    // than assume: a silent misalignment would look like a result.
    let mut used_at: HashMap<(&str, &str), usize> = HashMap::new();
    let ctx_kept: Vec<String> = kept
        .iter()
        .map(|p| {
            let key = (p.0.as_str(), p.1.as_str());
            let n = used_at.entry(key).or_insert(0);
            let out = index
                .get(&key)
                .and_then(|v| v.get(*n))
                .map(|i| context[*i].clone())
                .unwrap_or_default();
            *n += 1;
            out
        })
        .collect();

    println!(
        "  {:<22} {:>8} {:>14} {:>14} {:>12}",
        "context C", "classes", "log-loss", "vs marginal", "vs shuffled"
    );
    for k in [8usize, 64] {
        let (classes, used) = collapse_small(&discover_classes(&facet, &ctx_kept, k, 8));
        let real = predictive_gain(&classes, &modes);
        let sh = predictive_gain(&shuffle_classes(&classes, 0xC2B2_AE3D_27D4_EB4F), &modes);
        println!(
            "  {:<22} {:>8} {:>14.4} {:>+14.4} {:>+12.4}",
            format!("discovered, K={k}"),
            used,
            real.0,
            real.1,
            real.1 - sh.1
        );
    }

    // The control that matters most on this corpus: the *same pair* in different
    // frames. If C does anything, two occurrences of `cow` with different
    // neighbours should sit on different modes more often than two occurrences
    // with similar neighbours.
    let mut by_pair: HashMap<(&str, &str), Vec<usize>> = HashMap::new();
    for (i, p) in kept.iter().enumerate() {
        by_pair.entry((p.0.as_str(), p.1.as_str())).or_default().push(i);
    }
    let (mut same, mut n) = (0usize, 0usize);
    for occ in by_pair.values().filter(|v| v.len() >= 2) {
        for a in 0..occ.len() {
            for b in (a + 1)..occ.len() {
                if modes[occ[a]] == modes[occ[b]] {
                    same += 1;
                }
                n += 1;
            }
        }
    }
    println!(
        "\n  Two occurrences of the same pair land on the same mode {:.1}% of the\n\
         \x20 time ({} comparisons). The offset is a function of the two words alone,\n\
         \x20 so this is 100% by construction unless something varies it — and\n\
         \x20 nothing in the representation does. That number is the ceiling on what\n\
         \x20 any C can achieve here, and it is the finding, not the noise floor.",
        same as f64 / n.max(1) as f64 * 100.0,
        n
    );
    println!(
        "\n  Uniform chance over modes is ln(64) = {:.4} nats.",
        (PHASE_CHANNELS as f64).ln()
    );
}
