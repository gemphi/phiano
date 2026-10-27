//! Constructed positions against learned ones — and complex against phase-only.
//! `cargo run --release --bin constructed [corpus]`
//!
//! Two criticisms, both correct, both testable here.
//!
//! # One: the complex numbers are not being used
//!
//! `SpectralPhasor::resonance` is the innermost operation in every retrieval,
//! every relation probe and every analogy in this project, and it reads only the
//! packed phase bytes. Amplitude is computed, stored, updated on every
//! observation — and never enters the comparison. What every result in this repo
//! measured is a torus of 64 angles, not a field of complex numbers. The
//! magnitude has been carried and ignored.
//!
//! So each arm below is scored twice: phase-only, as shipped, and with the full
//! complex inner product where magnitude counts. The gap is what ignoring half
//! the representation costs.
//!
//! # Two: the positions could be constructed rather than learned
//!
//! The proposal: put a word in a **band** by its category, then place it *within*
//! the band by its characters — alphabetically, so `female` sits just below
//! `male` because f precedes m. There are 26 letters and a bounded set of
//! categories; the claim is that this is enough, and that training a position is
//! solving a problem that ordering already solves.
//!
//! Made concrete: a word gets one scalar position `p = (band + rank/size) / B`,
//! and that scalar is spread across the 64 channels as its harmonics,
//! `theta_k = 2*pi * p * m_k` with `m_k` the k-th prime. A scalar encoded as a
//! standing wave — the same geometry `kinds::harmonic_rotation` uses,
//! now for the position rather than the relation. Magnitude is log frequency, so
//! the complex number carries something on both axes.
//!
//! # Where the bands come from, and the two controls
//!
//! Not from `phiton::p64::PosDictionary`: it holds 238 words, covering 0.7% of the
//! corpus's types. It has no entry for `female`, `male` or `cow`, so it cannot
//! band content words at all.
//!
//! * **suffix** — the last three characters. `-ing`, `-ed`, `-ly`, `-tion` are
//!   real categories and they come straight out of the alphabet, which is the
//!   spirit of the proposal.
//! * **co-occurrence** — bands discovered by clustering words on the company they
//!   keep. Category from use.
//!
//! And two controls, because a scheme with this many moving parts can look good
//! for the wrong reason:
//!
//! * **band, shuffled within** — same bands, random order inside them. Isolates
//!   whether *alphabetical* ordering adds anything beyond the band.
//! * **shuffled band, alphabetical within** — random bands, real ordering.
//!   Isolates whether the *band* adds anything beyond the ordering.
//!
//! Scored on pair-versus-random over the relation families, chance 50%, with the
//! gender family called out separately since `female`/`male` is the worked
//! example the proposal rests on.
//!
//! # What it measured
//!
//! ```text
//! positions                            phase-only    complex   gender
//! learned (trained manifold)                68.3%      68.3%    75.2%
//! constructed: suffix + alphabetical        52.0%      52.0%    66.9%
//!   control: suffix + shuffled order        50.7%      50.7%    61.6%
//!   control: shuffled band + alpha          45.4%      45.4%    47.0%
//! constructed: co-occur + alphabetical      80.0%      80.0%    55.8%
//!   control: co-occur + shuffled order      50.3%      50.3%    38.9%
//! ```
//!
//! **Constructed beats learned.** Co-occurrence bands with alphabetical order
//! inside them score 80.0% against the trained manifold's 68.3%, and against
//! their own shuffled-order control's 50.3%. Coarsening positions into a band
//! plus a rank does not lose information relative to the continuous trained
//! positions — it *denoises* them, by nearly twelve points.
//!
//! The caveat that keeps this honest: those bands are computed from the trained
//! manifold, which describes each word's neighbourhood. So this is learn-then-
//! coarsen, not training-free. A genuinely constructed version needs bands from
//! raw co-occurrence counts, and until that runs the claim is "banding improves a
//! learned manifold", not "banding replaces learning".
//!
//! Character bands are weakly real. Suffix plus alphabetical is 52.0% against
//! 50.7% for the same bands in shuffled order and 45.4% for shuffled bands in
//! alphabetical order — so the band carries most of the little there is, and
//! alphabetical order on random bands is *worse than chance*. On the gender
//! family specifically the suffix band does well (66.9%), which is the worked
//! example behaving: `female` and `male` share their last three characters and
//! land in the same band by construction.
//!
//! # And the magnitude cannot help, for a reason worth stating
//!
//! Phase-only and complex are **identical to the decimal place in every row**.
//! Not close — the same. The reason is structural rather than an oversight:
//! amplitude is one number per word, shared across all 64 channels, so it factors
//! out of a normalised inner product exactly. A per-word scalar magnitude cannot
//! affect any normalised similarity, however the comparison is written.
//!
//! So "use the complex numbers" is right, and the fix is not to change the
//! similarity function — it is that magnitude has to vary *per channel* before it
//! can carry anything. As stored, half of every phasor in this project is
//! mathematically incapable of influencing a single result.

mod arms;
mod bands;
mod place;
mod report;
mod setup;
mod types;
#[allow(unused_imports)]
use arms::*;
#[allow(unused_imports)]
use bands::*;
#[allow(unused_imports)]
use place::*;
#[allow(unused_imports)]
use report::*;
#[allow(unused_imports)]
use setup::*;
#[allow(unused_imports)]
use types::*;

use phiano::config::{CHANNEL_PRIMES, LEARNING_RATE, PHASE_CHANNELS, TWO_PI};
use phiecs::phasor::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::RelationBenchmark;
use phiano::phasor::{fnv1a, mix64};
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;
use std::collections::HashMap;

fn main() {
    let s = setup();
    let rows = build_rows(&s);
    score_rows(&rows, &s.vocab);
}
