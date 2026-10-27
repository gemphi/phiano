//! Does breaking channel symmetry build the dimensions?
//! `cargo run --release --bin relational [corpus]`
//!
//! `bin/dimensions` found no relation-specific dimensions in the shipped
//! manifold and named the reason: training is symmetric in the channels, so
//! nothing can push one toward meaning something. `bin/constructed` found the
//! magnitude inert, and named that reason too: one amplitude per word factors
//! out of a normalised comparison exactly.
//!
//! [`phiano::relational`] fixes both. A word carries a phase *and a magnitude*
//! per channel; channel `r` belongs to relation `r`; an observation moves the
//! channel of the relation it observed and no other. The relations come from the
//! text — relative offset, and the most frequent word standing between a pair.
//!
//! This runs the same three tests `bin/dimensions` ran, on the new
//! representation, so the two pages are directly comparable:
//!
//! 1. Does a relation family have a channel, beyond what a permuted-filler null
//!    reaches on its own best channel?
//! 2. Do different families claim different channels?
//! 3. Does the best single dimension beat the average of all of them — the
//!    practical form of the claim, and the one that failed at 43.9% before?
//!
//! And the comparison that decides whether any of it was worth doing:
//! pair-versus-random against the shipped trained manifold, on identical probes.
//!
//! # What it found
//!
//! On **similarity** the new representation loses, and the module doc for
//! [`phiano::relational`] says why: relation-specific channels make comparison
//! sparse, and sparse comparison loses to dense averaging when the question is
//! "are these two words alike" — 38.8% for the best single dimension against
//! 61.5% for the shipped mean of sixty-four.
//!
//! On **composition** the order reverses, and that is the test the architecture
//! exists for. `a : b :: c : ?` scored as mean reciprocal rank over 1,536
//! probes, each scorer also run against a decoy target so every ratio has an
//! empirical null underneath it:
//!
//! ```text
//! best channel   0.00810   5.6x its null
//! all channels   0.00088   0.6x
//! shipped        0.00207   0.8x
//! ```
//!
//! Traversing one named channel is the only reading of the three that composes
//! at all; both dense readings sit at or below their own null. But the win is
//! confined to families whose step has a surface form — gender, number,
//! comparative, past tense, agent, quality, negation clear the null; antonym,
//! hypernym and nationality do not.
//!
//! The first version of that table quoted 53.2x, because it took chance for
//! mean reciprocal rank to be `1/N`. That is precision@1. A random ranking
//! scores `H_N / N`, which at this pool size is 9.4 times larger — and the
//! decoy null now printed under the table reproduces it to within a percent.
//!
//! # The named relations are not what is winning
//!
//! `flow::CouplingKind::Relation` now exists and fires: the demo line at
//! the end of this run reports `the(ch8), of(ch9), in(ch12), was(ch14)` on a
//! sentence, alongside the offsets. The marker inventory is right — `bin/markers`
//! prints it, and it is `the of and to in a was that as on for with by`.
//!
//! And not one of the ten relation families peaks on a marker channel. Every
//! best channel in the family table is an offset channel, in every run. The
//! composition result is adjacency doing the work; the named couplings are
//! representable, correct, and idle.
//!
//! The marker rule was rewritten four times getting here and the composition
//! total went 5.6x, 8.1x, 5.2x — no direction, which is itself the evidence:
//! a number that does not move when the markers change is not a number the
//! markers produced. What would make them matter is a benchmark whose pairs
//! actually co-occur under a marker in the corpus. `king`/`queen` do not appear
//! either side of `of` in WikiText-2, so the gender channel has no marker
//! evidence to use and falls back to position. That is the next measurement,
//! not a conclusion about the architecture.

mod types;
mod permute;
mod setup;
mod families;
mod similarity;
mod composition;
mod scoring;
mod coupling;
mod closing;
#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use permute::*;
#[allow(unused_imports)]
use setup::*;
#[allow(unused_imports)]
use families::*;
#[allow(unused_imports)]
use similarity::*;
#[allow(unused_imports)]
use composition::*;
#[allow(unused_imports)]
use scoring::*;
#[allow(unused_imports)]
use coupling::*;
#[allow(unused_imports)]
use closing::*;

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiecs::phasor::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::{RelationBenchmark, RelationFamily};
use phiano::phasor::fnv1a;
use phiano::flow::PhaseFlow;
use phiecs::relation::RelationVocab;
use phiano::relational::{RelationalFacet, WINDOW};
use std::collections::HashMap;
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;

fn main() {
    let run = setup();
    let families = RelationBenchmark::default_families();

    // ---- 1 and 2 ----
    family_channels(&run.rel, &run.vocab, &families);

    // ---- 3, and the comparison ----
    let pool = build_pool(&run.rel);
    similarity_section(&run.rel, &run.shipped, &pool, &families);
    composition_section(&run.rel, &run.shipped, &pool, &families);
    coupling_kinds_section(&run.rel, &run.shipped, &run.vocab, &run.toks);
    print_closing();
}
