//! Do the levels of language separate by rate?
//! `cargo run --release --bin bands [corpus] [seed]`
//!
//! Bentov, *Stalking the Wild Pendulum*, ch. 3, "Hierarchies of motion": each
//! level up the ladder of structures moves much more slowly than the one below.
//! Electrons orbit at enormous rates; the molecules built from them vibrate more
//! slowly; the cells built from those respond more slowly still — "a process of
//! gradually stepping down the high vibratory rates of the atoms, to the
//! 'reasonable' vibratory rates of the molecules, to the 'acceptable' frequency
//! response of the cells." Nothing in that hierarchy is a sum of the level below.
//! Each level has its own rate, and that rate is what makes it a level.
//!
//! Phiano already has the ingredient and has never checked it. `channel_kernel`
//! gives channel `k` a decay of `CONTEXT_LAMBDA^(1 + 3k/64)` and a frequency of
//! `2*pi*p_k/PRIME_MODULUS` with `p_k` the k-th prime. Low channels are slow with
//! long memory; high channels are fast with short memory. That is a spread of
//! rates designed in and then used as though all 64 channels were
//! interchangeable — every score in this project averages over all of them.
//!
//! # The question, and what would answer it
//!
//! Score two tasks band by band:
//!
//! * **word level** — do related words resonate more than unrelated ones, using
//!   only this band's channels? (`pair_vs_random`, chance 50%.)
//! * **sentence level** — does the accumulated state of three sentences pick the
//!   true continuation out of 50 candidates, using only this band? (chance
//!   1/50.)
//!
//! If the levels are frequency-separated, the two curves peak in *different*
//! bands — sentences low and slow, words high and fast. If they peak together,
//! the channel index carries no hierarchy and the per-channel kernel is
//! decoration.
//!
//! Bentov's ch. 5 sharpens the prediction into something falsifiable: a system
//! "interacts strongly within a band of frequencies," with the energy-exchange
//! curve peaked in its own band and tailing into the neighbours. So the
//! prediction is *overlapping but distinctly peaked* profiles, not a clean
//! partition — and a flat profile refutes it.
//!
//! # What it measured
//!
//! ```text
//! after descent   band  channels  kernel |z|  word pair>rnd  sentence MRR
//!                    0       0-7      0.9075          59.8%        0.0903
//!                    1      8-15      0.8796          63.8%        0.0881
//!                    2     16-23      0.8525          62.1%        0.0892
//!                    3     24-31      0.8263          64.4%        0.0874
//!                    4     32-39      0.8008          63.3%        0.0850
//!                    5     40-47      0.7762          63.0%        0.0861
//!                    6     48-55      0.7523          62.4%        0.0887
//!                    7     56-63      0.7291          63.3%        0.0808
//!                  all      0-63           -          68.6%        0.0946
//!             chance                                   50.0%        0.0900
//! ```
//!
//! **The hierarchy-of-rates prediction is refuted.** No band peaks beyond the
//! spread of the others — the best word band is +1.16 sd, the best sentence band
//! +1.12 sd, and on the bottom-up manifold +1.60 and +2.05 on profiles that are
//! entirely at chance. The channel index carries no linguistic hierarchy. The
//! kernel's spread of decay rates from 0.91 down to 0.73 is, for these two tasks,
//! decoration. That the prime spacing cut perplexity 11% is a separate fact and
//! is not level separation.
//!
//! **The hologram property holds instead, and it is strong.** One band of eight
//! channels keeps 69% of the word-level effect that all sixty-four carry — 12.8
//! of 18.6 points over chance. The relational signal is distributed across
//! channels rather than localised in any of them, so losing seven-eighths of the
//! representation costs under a third of the effect. That is Bentov ch. 1 on the
//! hologram — information stored as an interference pattern is recoverable from a
//! piece of the film at lower fidelity — and it is the robustness claim this
//! architecture can actually support, as opposed to the one it was designed for.
//!
//! The sentence row is the same null it has been all along: every band at or
//! below chance, 0.0946 against 0.0900 using everything.
//!
//! Re-measured on fixed seeds after the [`phiano::phasor::mix64`] fix, both
//! conclusions stand. The profile is still flat — best word band +1.30 sd, best
//! sentence band +1.36 sd, neither near the 2 sd bar — and holographic retention
//! is unchanged at 69% (13.1 of 18.9 points). The bottom-up arm now reads 50.7%
//! using all 64 channels, so its retention is reported as undefined rather than
//! as a ratio to an effect that is not there.

mod profile;
mod closing;
mod pair;
mod peak;
mod setup;
mod types;
#[allow(unused_imports)]
use profile::*;
#[allow(unused_imports)]
use closing::*;
#[allow(unused_imports)]
use pair::*;
#[allow(unused_imports)]
use peak::*;
#[allow(unused_imports)]
use setup::*;
#[allow(unused_imports)]
use types::*;

use phiano::config::{LEARNING_RATE, PHASE_CHANNELS};
use phiano::facet::Facet;
use phiano::metrics::harness::Harness;
use phiano::metrics::relation::RelationBenchmark;
use phiano::phasor::fnv1a;
use phiano::tokenizer::Tokenizer;
use phiano::topdown::{Descent, Up};
use phiano::trainer::Trainer;

fn main() {
    let s = setup();

    for (label, facet) in [("bottom-up", &s.base), ("after descent", &s.descended)] {
        band_profile(label, facet, &s.items);
    }

    closing();
}
