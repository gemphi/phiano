//! Does the promoted lexicon keep what the shipped one had, and gain what
//! `relational.rs` found?
//! `cargo run --release --bin promoted [corpus]`
//!
//! `bin/relational` measured a side type, `RelationalPhasor`, against the
//! shipped manifold and found a split verdict: relation-specific channels lose
//! on **similarity** (sparse comparison against dense averaging) and win on
//! **composition** (traverse one named channel instead of blending all of
//! them). Both findings were about a representation the engine did not store.
//!
//! This measures the promotion: the lexicon's own `SpectralPhasor` now carries
//! a magnitude per channel, and the shipped trainer couples each pair on its
//! relation's channel and deposits evidence there. So the questions are:
//!
//! 1. Does adding the relation coupling to the ranking objective *cost* the
//!    dense similarity the engine's retrieval and generation depend on? The
//!    control is the identical trainer with `relations` off.
//! 2. Does the evidence-weighted comparison, now available on the same
//!    phasor, recover what `bin/relational` saw?
//! 3. Does composition through the pair's own most-evidenced channel beat the
//!    dense analogy, against a decoy null?
//!
//! Every number is quoted beside the arm that differs from it in exactly one
//! thing, on identical probes and identical candidate pools.

mod composition;
mod corpus;
mod pair;
mod profiles;
mod similarity;
mod train;
mod types;
#[allow(unused_imports)]
use composition::*;
#[allow(unused_imports)]
use corpus::*;
#[allow(unused_imports)]
use pair::*;
#[allow(unused_imports)]
use profiles::*;
#[allow(unused_imports)]
use similarity::*;
#[allow(unused_imports)]
use train::*;
#[allow(unused_imports)]
use types::*;

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::facet::Facet;
use phiano::metrics::harness::{Harness, Split};
use phiano::metrics::relation::{RelationBenchmark, RelationFamily};
use phiano::phasor::fnv1a;
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;

fn main() {
    let split = load_split();
    let (control, promoted) = train_arms(&split);

    let balance = promoted.channel_balance();
    let families = RelationBenchmark::default_families();

    let mut pool: Vec<String> = promoted
        .lexicon
        .iter()
        .filter(|(_, p)| p.count >= COUNT_FLOOR)
        .map(|(w, _)| w.clone())
        .collect();
    pool.sort();
    let known = |w: &str| promoted.lexicon.contains_key(w) && control.lexicon.contains_key(w);

    // ---- 1 and 2: similarity ----
    experiment_similarity(&control, &promoted, &pool, &families, &known, &balance);

    // ---- 3: composition ----
    experiment_composition(&control, &promoted, &pool, &families, &known, &balance);

    // ---- what a word's polygon looks like ----
    report_profiles(&promoted, &pool);
}
