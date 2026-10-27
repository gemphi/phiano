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

mod arms;
mod documents;
mod report;
mod setup;
mod types;
#[allow(unused_imports)]
use arms::*;
#[allow(unused_imports)]
use documents::*;
#[allow(unused_imports)]
use report::*;
#[allow(unused_imports)]
use setup::*;
#[allow(unused_imports)]
use types::*;

use phiano::config::LEARNING_RATE;
use phiecs::phasor::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::{CountFloor, RelationBenchmark};
use phiano::metrics::sentence::SentenceBenchmark;
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, DescentReport, Up};
use phiano::trainer::Trainer;

fn main() {
    let Setup { seed, corpus } = setup();

    let cut = corpus.len() * 80 / 100;
    let (train, held) = corpus.split_at(cut);

    let base = train_base(train, seed);
    println!(
        "corpus {} sentences ({} train / {} held out), vocabulary {}",
        corpus.len(),
        train.len(),
        held.len(),
        base.vocabulary_size()
    );

    let arms = build_arms(&base, train, seed);

    report_descent(&arms);
    report_word_level(&arms, &base);
    report_sentence_level(&arms, held, seed);
    report_footer();
}
