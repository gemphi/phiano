//! Does the level above improve the level below?
//! `cargo run --release --bin topdown [corpus] [seed]`
//!
//! Phiano is bottom-up in its entirety: words train by next-word ranking, a
//! sentence is the sum of its words, a paragraph the sum of its sentences.
//! Nothing above ever constrains what is below. [`Descent`] adds the missing
//! direction — sentences pulling on the words they contain — and this binary
//! measures whether adding it helps.
//!
//! # What counts as an answer
//!
//! The intervention is applied *from* sentences, so reporting a sentence score
//! and calling it a win is close to circular: any rule that pulls a word toward
//! the sentences containing it makes those sentences more self-similar. The
//! sentence benchmark is reported because the task is the level the mechanism
//! claims to serve, but the load-bearing column is the word-level relational
//! score, evaluated at a frequency floor. That is a benchmark the downward pass
//! was never shown and cannot fit, and it is the one that would demonstrate the
//! hierarchy claim: constraint flowing down makes the bottom *better*.
//!
//! Four arms, all from the same bottom-up starting point:
//!
//! * **bottom-up only** — the control, and the state the project ships today.
//! * **descent, bag** — order-free downward pass.
//! * **descent, bound** — order-sensitive.
//! * **descent, shuffled sentences** — the null. Words are pulled toward
//!   *randomly assembled* sentences with the same length distribution and the
//!   same word frequencies. Any improvement this arm also shows is an artefact
//!   of the update rule (attraction concentrates a manifold, and a concentrated
//!   manifold scores differently) rather than of sentence structure. Without
//!   this column a gain over the control means nothing.

use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::{CountFloor, RelationBenchmark};
use phiano::metrics::sentence::SentenceBenchmark;
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, DescentReport, Up};
use phiano::trainer::Trainer;

/// The floor the word-level scores are read at.
///
/// Zero is the whole vocabulary, most of which the model saw fewer than five
/// times; those words sit at their hash seeds and every average over them is
/// dominated by initialisation. The floor sweep in `bin/roles` showed the
/// project's relational headline was carried entirely by that tail.
const FLOOR: CountFloor = 25;

fn documents(sentences: &[String], n: usize) -> Vec<Vec<String>> {
    sentences.chunks(n).map(|c| c.to_vec()).collect()
}

/// Sentences with the same lengths and the same word frequencies, assembled at
/// random.
///
/// This is the null the whole comparison rests on. It preserves everything about
/// the corpus except which words occur *together*, so an effect that survives
/// here is not about sentences.
fn shuffled(sentences: &[Vec<String>], seed: u64) -> Vec<Vec<String>> {
    let mut bag: Vec<&String> = sentences.iter().flatten().collect();
    // Fisher-Yates with a fixed generator: the null must be identical between
    // runs or it is not a control.
    let mut r = seed | 1;
    for i in (1..bag.len()).rev() {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        bag.swap(i, (r % (i as u64 + 1)) as usize);
    }
    let mut out = Vec::with_capacity(sentences.len());
    let mut at = 0usize;
    for s in sentences {
        let take = s.len().min(bag.len() - at);
        out.push(bag[at..at + take].iter().map(|w| (*w).clone()).collect());
        at += take;
    }
    out
}

struct Arm {
    label: String,
    facet: Facet,
    report: Option<DescentReport>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/dictionary_corpus.txt".to_string());
    let seed: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(42);

    let raw = match std::fs::read_to_string(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("could not read {}: {}", path, e);
            std::process::exit(1);
        }
    };

    let corpus: Vec<String> = Tokenizer::split_sentences(&raw)
        .into_iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|s| Tokenizer::tokenize(s).len() >= 4)
        .collect();
    if corpus.is_empty() {
        eprintln!("empty corpus at {}", path);
        std::process::exit(1);
    }

    let cut = corpus.len() * 80 / 100;
    let (train, held) = corpus.split_at(cut);

    // Bottom-up, exactly as the project ships it.
    let split = Harness::split(train.to_vec(), seed);
    let trainer = Trainer::new(LEARNING_RATE).with_seed(seed);
    let base = Harness::train_ranking_only(&split, &trainer, 4);
    println!(
        "corpus {} sentences ({} train / {} held out), vocabulary {}",
        corpus.len(),
        train.len(),
        held.len(),
        base.vocabulary_size()
    );

    // The downward pass sees the training sentences only. Held-out sentences are
    // what everything is scored on, and a rule that had seen them would be
    // fitting the test.
    let train_toks: Vec<Vec<String>> = train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let null_toks = shuffled(&train_toks, seed ^ 0x5DEECE66D);

    let families = RelationBenchmark::default_families();
    let docs = documents(held, 8);
    let pool: Vec<Vec<String>> = held.iter().map(|s| Tokenizer::tokenize(s)).collect();

    let mut arms: Vec<Arm> = vec![Arm {
        label: "bottom-up only".into(),
        facet: base.clone(),
        report: None,
    }];

    for (label, sents, up, contrast) in [
        ("descent bag", &train_toks, Up::Bag, true),
        ("descent bound", &train_toks, Up::Bound, true),
        ("descent bag, no contrast", &train_toks, Up::Bag, false),
        ("descent bag, SHUFFLED", &null_toks, Up::Bag, true),
    ] {
        let mut f = base.clone();
        let r = Descent::cycle(&mut f, sents, up, 4, 0.25, contrast);
        arms.push(Arm { label: label.into(), facet: f, report: Some(r) });
    }

    // ---- what the downward pass did to the manifold ----
    println!("\n=== the descent itself ===");
    println!(
        "  {:<26} {:>9} {:>9} {:>11} {:>11} {:>9}",
        "arm", "moved", "seen", "dispersion", "band", "kept"
    );
    for a in &arms {
        match &a.report {
            None => println!(
                "  {:<26} {:>9} {:>9} {:>11.3} {:>11.3} {:>9}",
                a.label,
                "-",
                "-",
                a.facet.phase_dispersion(),
                a.facet.dispersion_top(2_000),
                "-"
            ),
            Some(r) => println!(
                "  {:<26} {:>9} {:>9} {:>11.3} {:>11.3} {:>9}",
                a.label,
                r.words_moved,
                r.words_seen,
                r.dispersion_after,
                r.band_after,
                match r.rejected {
                    true => "REJECTED",
                    false => "yes",
                }
            ),
        }
    }
    println!(
        "  A rejected arm was discarded by the collapse guard and is identical to\n\
         \x20 the control below — its scores are the control's, not a result."
    );

    // ---- the load-bearing column: does the level below get better? ----
    println!("\n=== word level, at a frequency floor of {} ===", FLOOR);
    let pool_n = RelationBenchmark::pool_size(&base, FLOOR);
    println!(
        "  candidate pool {} words; chance analogy MRR ~{:.5}, chance nbr@10 {:.3}%",
        pool_n,
        1.0 / pool_n.max(1) as f64,
        1000.0 / pool_n.max(1) as f64
    );
    println!(
        "  {:<26} {:>10} {:>10} {:>10} {:>10}",
        "arm", "anlg MRR", "vs chance", "nbr@10", "pair>rnd"
    );
    for a in &arms {
        let r = RelationBenchmark::evaluate_above(&a.facet, &families, FLOOR);
        let nbr: f64 = r.families.iter().map(|f| f.neighbour_top10).sum::<f64>()
            / r.families.len().max(1) as f64;
        let mrr: f64 = r.families.iter().map(|f| f.analogy_mrr).sum::<f64>()
            / r.families.len().max(1) as f64;
        println!(
            "  {:<26} {:>10.5} {:>9.2}x {:>9.2}% {:>9.1}%",
            a.label,
            mrr,
            mrr / (1.0 / pool_n.max(1) as f64),
            nbr * 100.0,
            r.overall_pair_vs_random * 100.0
        );
    }

    // ---- the level the intervention was applied at ----
    println!("\n=== sentence level, held out ===");
    println!(
        "  {:<26} {:>10} {:>10} {:>10}",
        "arm", "phase MRR", "lexical", "vs chance"
    );
    for a in &arms {
        let r = SentenceBenchmark::evaluate(&a.facet, &docs, &pool, seed);
        let lexical = r.scorers.last().map(|s| s.mrr).unwrap_or(0.0);
        let best = r.scorers[..r.scorers.len().saturating_sub(1)]
            .iter()
            .map(|s| s.mrr)
            .fold(0.0f64, f64::max);
        println!(
            "  {:<26} {:>10.4} {:>10.4} {:>9.2}x",
            a.label,
            best,
            lexical,
            best / r.chance_mrr.max(1e-12)
        );
    }

    println!(
        "\n  How to read this. The sentence column is the level the rule was applied\n\
         \x20 at, so a gain there is partly circular by construction. The word column\n\
         \x20 is the claim: the downward pass never saw a relation probe, so an\n\
         \x20 improvement there is constraint from above genuinely improving the level\n\
         \x20 below. And the SHUFFLED row is the floor under both — it applies the same\n\
         \x20 update rule to sentences that are word bags with no co-occurrence\n\
         \x20 structure, so whatever it gains is the rule, not the hierarchy. Only the\n\
         \x20 gap between a real arm and that row is evidence of anything."
    );
}
