//! Do definitions build concepts? Measured, one change at a time.
//!
//! `cargo run --release --bin conception -- [corpus] [chunks] [rounds]`
//!
//! Definition grounding is already in the codebase and already measured: it
//! halves phase dispersion and improves no relation metric. `Conception`
//! changes three things about how a definition reaches the manifold — every
//! channel instead of one, order carried in the phase instead of a centroid,
//! and a mutual pull instead of a one-way one. Three changes need three
//! controls, so this runs five conditions on the same trained facet:
//!
//! | condition           | channels | order | mutual |
//! |---------------------|----------|-------|--------|
//! | baseline            | –        | –     | –      |
//! | grounder (existing) | 1        | no    | no     |
//! | compose, unbound    | 64       | no    | no     |
//! | compose             | 64       | yes   | no     |
//! | compose + reinforce | 64       | yes   | yes    |
//!
//! Each row differs from the one above it in exactly one property, so a gain
//! is attributable. The relation benchmark is the target metric — whether
//! *grandmother* sits near *grandfather* the way *woman* sits near *man* — with
//! phase dispersion and held-out perplexity logged beside it, because a change
//! that improves relations while collapsing the manifold or wrecking prediction
//! has not improved the model.
//!
//! The definition source is any [`Groundable`]. Here it is the local Webster's
//! chunk store; `ApiSource` supplies the same shape from an online dictionary
//! where the network allows it, and nothing below changes.

mod types;
mod stat;
mod setup;
mod conditions;
mod anchors;
mod report;
mod seeds;
#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use stat::*;
#[allow(unused_imports)]
use setup::*;
#[allow(unused_imports)]
use conditions::*;
#[allow(unused_imports)]
use anchors::*;
#[allow(unused_imports)]
use report::*;
#[allow(unused_imports)]
use seeds::*;

use phient::train::chunker::ChunkStore;
use phient::train::grounding::DefinitionGrounder;
use phient::train::conception::{
    Conception, DefinitionGraph, ANCHOR, BETA_STRONG, BETA_WEAK, HEAD_STEP, REINFORCE,
};
use phiano::config::LEARNING_RATE;
use phiecs::phasor::facet::Facet;
use phient::metrics::harness::{Harness, Split};
use phient::metrics::relation::RelationBenchmark;
use phient::train::sources::definition_core;
use phiano::tokenizer::Tokenizer;
use phient::train::trainer::Trainer;

fn main() {
    let s = setup();
    let out = run_conditions(&s);
    print_table(&out.rows);
    print_families(&out.best.1);
    across_seeds(&s);
    print_reading(&out.rows);
}
