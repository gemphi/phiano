//! Is `X as Y` the right unit of representation?
//! `cargo run --release --bin senses [corpus]`
//!
//! "X counts as Y" is not only about `cow counts as animal`. Its commonest
//! instance is the one every sentence performs: *greet* as verb, *cool* as
//! adjective, *mum* as noun — and the same surface word taking a different Y in
//! a different place. `cool` is an adjective in "cool water" and a verb in "cool
//! the engine", and those are two different things wearing one spelling.
//!
//! `bin/frames` showed this project cannot represent that. Two occurrences of the
//! same word pair land on the same harmonic mode **100.0% of the time**, because
//! `offset(a, b) = theta(b) - theta(a)` and each word has exactly one phasor. The
//! relation between two words is a constant of those two words, so no context can
//! vary it and Searle's third term has nowhere to live.
//!
//! The fix the argument implies: make `X as Y` the unit rather than `X`. A word
//! gets one position per role it plays instead of one position.
//!
//! # Where Y comes from
//!
//! Not from a tag set. `phiton::p64::PosDictionary` maps each word to a single
//! `PartOfSpeech`, which is the same collapse one level up — it cannot say `cool`
//! is an adjective here and a verb there. Y is discovered from use instead, which
//! is what the CLU argument asks for: each occurrence of a word is described by
//! the phase state of its local window, those descriptions are clustered, and the
//! clusters are the word's roles. Nothing is named, and the number of roles per
//! word is bounded.
//!
//! # The measurement, and the control that decides it
//!
//! Splitting a word into `k` variants divides its evidence `k` ways and multiplies
//! its parameters by `k`. Both effects change the score on their own, so "split
//! beats unsplit" is not a result.
//!
//! * **unsplit** — one phasor per word. What the project ships.
//! * **sense-split** — variants discovered from context.
//! * **RANDOM-split** — the same number of variants per word, occurrences
//!   assigned by a hash instead of by context. Identical vocabulary growth,
//!   identical count dilution, no linguistic information. **This is the control.**
//!
//! Scored as held-out log-loss of the *next surface word*. The target stays the
//! surface word in every arm, so the label space is identical and the three
//! numbers are directly comparable — which they would not be if the split changed
//! what was being predicted.
//!
//! # What it measured
//!
//! WikiText-2 prose, 76,765 training and 9,596 held-out sentences, 3,291 words
//! split (seen 60+ times). Log-loss in nats per token, lower better:
//!
//! ```text
//!   k   unsplit    sense    random   sense cost   random cost   recovered
//!   2    7.4776   7.4808    7.7485       0.0032        0.2709       98.8%
//!   3    7.4776   7.4846    7.9032       0.0070        0.4256       98.4%
//!   4    7.4776   7.4893    8.0180       0.0117        0.5404       97.8%
//! ```
//!
//! **The roles are real, and nearly free.** Dividing a word's evidence `k` ways
//! costs 0.27 to 0.54 nats when the division carries no information. Dividing it
//! by discovered role costs 0.003 to 0.012 — between 1.2% and 2.2% of that. A
//! third of the occurrences predicting almost as well as all of them is only
//! possible if the variants are internally coherent, and a hash assignment with
//! identical vocabulary growth and identical count dilution cannot do it.
//!
//! **And they do not yet pay.** Sense splitting is slightly *worse* than not
//! splitting, at every `k`. Affordable is not the same as profitable.
//!
//! That is not surprising in this particular test, and it is worth saying why
//! rather than treating it as a verdict on the idea. Next-word prediction is
//! close to the worst place to look for a payoff: the immediate successor of a
//! token is largely fixed by its surface form, so knowing that `cool` is the
//! adjective rather than the verb buys little about what follows it. Where the
//! split should pay is exactly where this project has been failing — relations,
//! where `bin/frames` found two occurrences of a pair on the same mode 100% of
//! the time because the words have one position each. That number cannot be 100%
//! once a word has several, and whether it drops for a *reason* is the next
//! measurement, with the random split as its control again.

mod types;
mod variants;
#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use variants::*;

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiecs::phasor::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::phasor::{fnv1a, mix64};
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "data/prose_corpus.txt".to_string());
    let _ = VARIANTS_CELL.set(args.get(2).and_then(|s| s.parse().ok()).unwrap_or(3));

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

    let split = Harness::split(corpus, 42);
    let facet = Harness::train_ranking_only(&split, &Trainer::new(LEARNING_RATE).with_seed(0), 4);

    let train: Vec<Vec<String>> = split.train.iter().map(|s| Tokenizer::tokenize(s)).collect();
    let held: Vec<Vec<String>> = split.valid.iter().map(|s| Tokenizer::tokenize(s)).collect();

    // RoleTable are learned on the training half only. Learning them on everything
    // would let the held-out sentences choose their own conditioning.
    let roles = RoleTable::learn(&facet, &train);
    let vocab = facet.vocabulary_size();

    println!(
        "prose: {} train / {} held-out sentences, {} words\n\
         \x20 {} words split into {} roles each (seen {}+ times)",
        train.len(),
        held.len(),
        vocab,
        roles.split_words(),
        variants(),
        SPLIT_FLOOR
    );

    println!(
        "\n  {:<16} {:>14} {:>14}",
        "conditioning", "log-loss", "vs unsplit"
    );
    let base = next_word_loss(Scheme::Unsplit, &roles, &facet, &train, &held, vocab);
    println!("  {:<16} {:>14.4} {:>14}", "unsplit (X)", base, "-");

    for (label, scheme) in [
        ("sense (X as Y)", Scheme::Sense),
        ("RANDOM split", Scheme::Random),
    ] {
        let ll = next_word_loss(scheme, &roles, &facet, &train, &held, vocab);
        println!("  {:<16} {:>14.4} {:>+14.4}", label, ll, base - ll);
    }

    println!(
        "\n  Log-loss in nats per token, predicting the next surface word; lower is\n\
         \x20 better and 'vs unsplit' is positive when splitting helped. The target is\n\
         \x20 the surface word in every arm, so the label space is identical and the\n\
         \x20 three numbers compare directly.\n\n\
         \x20 The RANDOM row is the one that decides it. Splitting a word into {} and\n\
         \x20 dividing its evidence three ways changes the score by itself, so sense\n\
         \x20 splitting has to beat a split that carries no information — not merely\n\
         \x20 beat no split at all.",
        variants()
    );
}
