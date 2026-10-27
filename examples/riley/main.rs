//! Searle's third term: does "in C" do any work?
//! `cargo run --release --bin in_c [chunks] [corpus]`
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
//! * **syntactic** — `phiton::p64::SentenceType`, the Searle speech-act classifier
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
//! syntactic (p64)                  5      0.2704      65.4%       -0.0011
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
//! # C as a mode prior — the other formalisation, also measured
//!
//! The medium sustains 64 standing waves, so the natural second reading of "in
//! C" is that the context says *which one you are in* rather than translating
//! it: `cow counts as animal` in a taxonomy and `cow counts as livestock` in a
//! market are two modes, not one mode plus an offset. That is a prediction test
//! with no free parameters on either side — the mode labels come from the fixed
//! harmonic basis, which no training produced and no context ever saw.
//!
//! The confound had to be killed first. The gloss contains the head and the
//! filler, so a context built from it trivially knows the pair, and the
//! shuffled-C null does not control for that. The context state is therefore the
//! gloss **with the pair's own two words removed** — the rest of the sentence,
//! not the sentence quoting the answer back.
//!
//! ```text
//! context C              classes   log-loss   vs marginal   vs shuffled
//! discovered, K=8              8     3.1845       +0.0077       +0.0177
//! discovered, K=64            27     3.1747       +0.0020       +0.0320
//! syntactic (p64)              5     3.1589       -0.0014       +0.0089
//! uniform chance = ln(64) = 4.1589
//! ```
//!
//! **The same verdict, from the other direction.** Context beats a random
//! partition of the same shape — the effect is real — and buys **at most +0.008
//! nats out of 3.17**, a quarter of one percent of the log-loss. Knowing the
//! context is very nearly worthless for predicting which relation holds.
//!
//! One column needs reading carefully, because it flatters. "vs shuffled" grows
//! from +0.0177 to +0.0320 as K goes 8 to 64, which looks like finer context
//! helping more. It is mostly the null getting worse: a random partition into 27
//! classes costs more in estimation noise than one into 8, so the shuffled arm
//! degrades faster than the real one improves. The honest column is "vs
//! marginal", which *shrinks* over the same range, +0.0077 to +0.0020.
//!
//! And `p64` is negative against the marginal again (-0.0014), for the second
//! time on a different test. This repo's Searle speech-act classifier carries no
//! information about which relation a pair stands in, under either reading of C.
//!
//! So both formalisations of the third term are now measured and both fail. That
//! is not "C is unimportant in language" — it is that the context available here,
//! from a dictionary gloss with the pair removed, does not determine the relation
//! in this representation. A corpus where the same pair genuinely recurs in
//! different frames would be the fair test, and a dictionary is close to the
//! worst case for it: each entry states its relation once.

mod floor;
mod setup;
mod shift;
mod prior;
mod chance;
#[allow(unused_imports)]
use floor::*;
#[allow(unused_imports)]
use setup::*;
#[allow(unused_imports)]
use shift::*;
#[allow(unused_imports)]
use prior::*;
#[allow(unused_imports)]
use chance::*;

use phiano::chunker::ChunkStore;
use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiecs::phasor::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::frames::{collapse_small, discover_classes, predictive_gain, shuffle_classes};
use phiano::phasor::{fnv1a, mix64};
use phiano::kinds::OffsetTable;
use phiano::sources::definition_core;
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;
use std::collections::HashMap;

fn main() {
    let d = setup();
    mean_shift_table(&d);
    mode_prior_table(&d);
    print_chance(&d.pairs);
}
