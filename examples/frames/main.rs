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

mod types;
mod train;
mod gate;
mod shift;
mod prior;
#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use train::*;
#[allow(unused_imports)]
use gate::*;
#[allow(unused_imports)]
use shift::*;
#[allow(unused_imports)]
use prior::*;

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiecs::phasor::facet::Facet;
use phiano::frames::{collapse_small, discover_classes, predictive_gain, shuffle_classes};
use phiano::metrics::harness::Harness;
use phiano::kinds::{OffsetEstimator, OffsetTable};
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;
use std::collections::HashMap;

fn main() {
    let trained = train_facet();
    let rows = gate(&trained);
    shift_section(&trained.facet, &rows);
    mode_prior_section(&trained.facet, &rows);
}
