//! Is channel `k` the *k-th relation*?
//! `cargo run --release --bin dimensions [corpus]`
//!
//! The architecture under test: a word occupies 64 dimensions on the complex
//! plane, and each dimension is one of the 64 possible relations it can stand in.
//! Channel 17 is not "channel 17" — it is whatever relation 17 is, and a word's
//! value there says how that word participates in it. Composition then has
//! somewhere to happen: to compose along a relation, you work in its dimension.
//!
//! Phiano does not do this. Its 64 channels are interchangeable — hash-derived
//! angles differing only in decay and frequency — and **every score in this
//! project averages over all of them**. `SpectralPhasor::resonance` sums 64
//! cosines and divides by 64.
//!
//! That averaging is fatal to the proposed architecture. If channel 17 says
//! "these two words stand in a gender relation" and the other 63 say nothing,
//! the mean reports 1/64th of the signal and calls it noise. `bin/bands` already
//! found the channel profile flat, but it averaged within bands of eight, which
//! would bury a single specialised channel just as effectively.
//!
//! # What would show the architecture is there
//!
//! Three things, in order of strength:
//!
//! 1. **A family has a channel.** Gender pairs agree far more on some channel
//!    than shuffled pairs do on their best channel. Compared against a
//!    permuted-filler null, because the maximum over 64 channels is upward-biased
//!    and a null that has not paid the same bias is not a floor.
//! 2. **Different families have different channels.** If gender, plural and
//!    antonym all peak on the same channel, that channel is not a relation — it
//!    is whichever dimension happens to carry the most variance.
//! 3. **Max beats mean.** If relations live in single dimensions, scoring a pair
//!    by its best channel should beat averaging over all of them. This is the
//!    practical form of the claim, and the cheapest thing to act on if true.
//!
//! A flat result here does not refute the architecture. It says the architecture
//! is not present — which would be unsurprising, since nothing in the training
//! objective ever asks a channel to specialise. That distinction matters: "the
//! design is wrong" and "the design was never built" call for different work.
//!
//! # What it measured
//!
//! ```text
//! family        pairs  best chan  agreement  null best    excess
//! gender           22          1     0.4571     0.3322   +0.1249
//! nationality      25         25     0.5431     0.4262   +0.1169
//! quality          14         13     0.5371     0.4181   +0.1191
//! antonym          33         31     0.4697     0.3647   +0.1050
//! comparative      27          1     0.4020     0.3112   +0.0908
//! hypernym         34         40     0.3618     0.2724   +0.0894
//! number           32         53     0.3838     0.3837   +0.0001
//! agent            26         58     0.3182     0.3172   +0.0010
//! past_tense       35         59     0.4481     0.4542   -0.0061
//! negation         16         33     0.3752     0.5344   -0.1592
//!
//! similarity                   pair>random
//! mean of 64 (as shipped)            61.3%
//! best single dimension              43.9%
//! mean of best 8                     57.1%
//! ```
//!
//! **The architecture is not present in this manifold, and the practical test
//! fails outright.** Scoring a pair by its best dimension gives 43.9% — *below*
//! the 50% chance line, and far below the 61.3% of averaging all 64. Taking the
//! best eight is also worse than taking all of them. If relations lived in single
//! dimensions this ordering would be reversed, and it is not close.
//!
//! The per-family column looks more encouraging and should not be read that way.
//! Six of ten families have a best channel exceeding their null by 0.09 to 0.12,
//! and the ten families peak on nine distinct channels rather than crowding onto
//! one — which is the shape the architecture predicts. But **negation's null beat
//! its signal by 0.1592**, which is larger than any family's excess. That is the
//! noise scale, measured on the same page: with 64 channels and 14 to 35 pairs,
//! a maximum over 64 wanders by more than the effects being reported. The peaks
//! are not established, and a version of this table that omitted the two negative
//! rows would have looked like a discovery.
//!
//! Why the maximum fails at all is worth stating: the best of 64 channels is near
//! 1.0 for *every* pair, related or not, so it saturates and discriminates
//! nothing. Averaging is what makes agreement informative here.
//!
//! # What the architecture would require
//!
//! Not a different similarity function — a different objective. Phiano's training
//! is **symmetric in the channels**: seeds are hashes, the Kuramoto step treats
//! every channel alike, and `channel_kernel` varies only decay and frequency.
//! Under a symmetric objective no channel can become "the gender channel",
//! because nothing distinguishes it from the other 63 and nothing pushes toward
//! the distinction. The dimensions cannot specialise by accident.
//!
//! Making them relation-specific means breaking that symmetry on purpose: bind
//! relation `r` to channel `r` during training, so that observing a gender pair
//! moves channel `r_gender` and leaves the rest alone. That is a real design and
//! this measurement is not evidence against it — it is evidence that it has never
//! been built. The fixed harmonic basis in `kinds` is the nearest thing
//! present, and it works precisely because it *does* impose structure across
//! channels rather than leaving them interchangeable.

mod profile;
mod floor;
mod channels;
mod setup;
mod similarity;
#[allow(unused_imports)]
use profile::*;
#[allow(unused_imports)]
use floor::*;
#[allow(unused_imports)]
use channels::*;
#[allow(unused_imports)]
use setup::*;
#[allow(unused_imports)]
use similarity::*;

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiecs::phasor::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::{RelationBenchmark, RelationFamily};
use phiano::phasor::fnv1a;
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;

fn main() {
    let facet = train_facet();
    let families = RelationBenchmark::default_families();
    println!("facet {} words\n", facet.vocabulary_size());

    report_family_channels(&facet, &families);
    report_similarity(&facet, &families);
}
