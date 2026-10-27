//! Does a relation channel work on the relation it is named after?
//! `cargo run --release --bin marked [corpus]`
//!
//! `bin/relational` established that traversing one channel composes better than
//! blending sixty-four, and then established that the channels doing the work
//! were all *offset* channels. Every one of its ten relation families peaks on
//! position. The named marker channels — `of`, `in`, `to`, `was` — are built,
//! correct, and idle.
//!
//! The reason is the benchmark and not the architecture. `king` and `queen` do
//! not appear either side of `of` in WikiText-2, so the gender channel has no
//! marker evidence about that pair and falls back to the one relation always
//! available, which is adjacency. Asking a marker channel about pairs it has
//! never seen under its marker measures nothing.
//!
//! So this benchmark takes its pairs from the corpus instead of from a word list.
//! For each marker `m`, the pairs are the words the text actually put either side
//! of `m`. Then the question that matters can be asked properly:
//!
//! 1. **Does the channel compose?** `a : b :: c : ?` on pairs joined by `m`,
//!    traversing `m`'s own channel.
//! 2. **Is the channel specific to its relation?** The same probes scored on
//!    *other* markers' channels. This is the load-bearing one. If channel `k`
//!    beats the others on relation `k`'s pairs, the dimension belongs to the
//!    relation, which is the whole claim. If every channel does equally well,
//!    then all that was learned is which words go together, and the assignment of
//!    relations to dimensions is decoration.
//! 3. **Does it beat the alternatives?** All sixty-four blended, and the shipped
//!    manifold, on identical probes.
//!
//! Every row carries a decoy null — the same scorer, same pool, same code path,
//! asked for a word that is not the answer — because a ratio against `1/N` was
//! wrong by a factor of nine the last time this was measured.
//!
//! # What it found
//!
//! Two things, and they point in opposite directions.
//!
//! **The dimension does belong to the relation.** Own-channel beats rival
//! channels on 10 of 12 markers, sign test p = 0.039, on identical probes with
//! identical pools differing only in which dimension the step is taken in. This
//! is the first direct evidence in the project that channel `k` holds something
//! specific to relation `k` — `bin/dimensions` looked for it in the shipped
//! manifold and found nothing, and `bin/relational` built the mechanism but
//! measured it on pairs that never co-occur under a marker, so its channels had
//! nothing to be specific about. The effect is small: 0.0224 against 0.0166.
//!
//! **And it does not pay.** On these pairs the ordering is the reverse of
//! `bin/relational`'s:
//!
//! ```text
//!                    MRR    vs null
//! own channel     0.0224       4.7x
//! rival channels  0.0166       2.8x
//! all 64 blended  0.1370      36.5x
//! shipped         0.5490     112.7x
//! ```
//!
//! Blending every dimension beats traversing the right one by six times, and the
//! shipped manifold beats it by twenty-four. `bin/relational` reported the
//! opposite — one channel at 5.2x its null while both dense readings sat at or
//! below theirs — and the difference is entirely in which pairs were asked
//! about. There, curated pairs that never co-occur, where sparse evidence is all
//! there is. Here, pairs the corpus actually placed either side of a marker,
//! where there is a great deal of evidence and using all of it wins.
//!
//! So the honest statement is narrower than either benchmark alone. Assigning a
//! relation its own dimension puts something real in that dimension. It is not a
//! better way to answer an analogy than reading every dimension at once, on any
//! pair where reading every dimension is possible.
//!
//! # What it does not show
//!
//! Pairs are mined from the same text the model trained on, so a pair's two words
//! have been coupled directly on that channel. That is why the interesting number
//! here is the *contrast between channels* and not the absolute MRR: a model that
//! merely memorised each pair would place them consistently on every channel it
//! touched, and would show no advantage for the right one.
//!
//! It also flatters the two dense scorers, and the shipped column especially.
//! With all four words of `a : b :: c : ?` drawn from pairs that co-occur, the
//! analogy is close to "name a word that keeps company with `c`", which is the
//! one question a context-trained manifold is built to answer. 0.549 means it
//! returns the exact word first about half the time, and that number should be
//! read as an upper bound on a easier question rather than as analogy accuracy.

mod types;
mod train;
mod mine;
mod marker;
mod benchmark;
mod summary;
#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use train::*;
#[allow(unused_imports)]
use mine::*;
#[allow(unused_imports)]
use marker::*;
#[allow(unused_imports)]
use benchmark::*;
#[allow(unused_imports)]
use summary::*;

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiecs::phasor::facet::Facet;
use phiano::metrics::harness::Harness;
use phiecs::relation::{RelationVocab, MARKER_MAX_GAP, OFFSET_CHANNELS};
use phiano::relational::RelationalFacet;
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;
use std::collections::HashMap;

fn main() {
    let trained = train();
    let mined = mine_pairs(&trained);
    run_benchmark(&trained, &mined);
}
