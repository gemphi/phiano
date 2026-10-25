//! How many relation types does the language actually hold?
//! `cargo run --release --bin types [chunks] [corpus]`
//!
//! The claim: a language holds a bounded number of relation types — at most 64 —
//! so they fit in a fixed-width type and the whole system can be built on
//! discovered relations rather than hardcoded ones.
//!
//! A cap does not test itself. `.take(64)` yields at most 64 of anything, and
//! `MAX_WORD_SENSES = 64` in this codebase is exactly that: a truncation wearing
//! a claim's clothes. What would make the bound real is *saturation* — held-out
//! structure ceasing to improve before the cap, so a 65th type would have
//! nothing left to explain.
//!
//! So: fit relation types on half the extracted pairs, score the other half, and
//! sweep the type count past the cap. Two columns decide it.
//!
//! * **held-out** — agreement between a held-out pair's phase offset and its
//!   nearest fitted type. In-sample tightness rises with `k` mechanically and
//!   would confirm any bound put to it, so it is not what is reported.
//! * **shuffled** — the same, on pairs whose fillers have been permuted. This
//!   preserves the vocabulary, the offset marginals and the type count, and
//!   destroys only which head goes with which filler. A held-out curve its null
//!   tracks is measuring k-means, not language.
//!
//! The bound is supported if the gain flattens below 64 and the flattening is
//! not the null flattening too. It is refuted if the gain is still climbing at
//! 128 — and that is an outcome this is set up to be able to report.
//!
//! # Two manifolds
//!
//! The sweep runs on both the bottom-up manifold and the one the downward pass
//! in [`phiano::topdown`] produced, because those two disagree about whether
//! this project has relational structure at all. The descent tripled analogy
//! MRR at the same frequency floor — 0.00247 to 0.00814, four times its own
//! null — while the first run of this sweep found no relation types in the
//! bottom-up manifold whatsoever.
//!
//! Both cannot be the whole story, and the disagreement is informative either
//! way. If the descended manifold's types clear their null, then the analogy
//! gain *is* shared offsets and the descent built the structure the bounded-type
//! claim needs. If it does not, the analogy gain is something other than a
//! shared step between word pairs, and the bounded-type claim has no
//! representation to stand on in either manifold.

mod floor;
mod harmonic;
mod pairs;
mod setup;
mod sweep;
#[allow(unused_imports)]
use floor::*;
#[allow(unused_imports)]
use harmonic::*;
#[allow(unused_imports)]
use pairs::*;
#[allow(unused_imports)]
use setup::*;
#[allow(unused_imports)]
use sweep::*;

use phiano::chunker::ChunkStore;
use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::metrics::harness::{Harness, Split};
use phiano::kinds::{OffsetEstimator, KindTable};
use phiano::sources::definition_core;
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;
use std::collections::HashMap;

fn main() {
    let (split, facet) = train_facet();
    let pairs = gloss_pairs(&facet);
    let descended = run_descent(&facet, &split);

    sweep(&facet, &descended, &pairs);

    // ---- the fixed basis ----
    harmonic_basis(&facet, &descended, &pairs);

    head_dom_chance(&pairs);
}
