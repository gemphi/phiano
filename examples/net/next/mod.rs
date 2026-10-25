//! The three things `docs/65_phinet.md` left open, measured.
//! `cargo run --release -p phiano --example net -- next [corpus] [epochs]`
//!
//! 1. **The operator in the language-model path.** `PhianoLM` scores the
//!    vocabulary by phase coherence with a context vector and mixes the
//!    softmax with a unigram under a trigram affine. Here the same harness
//!    takes its phase term from `Fno(context function)` instead, on the same
//!    facet and held-out sentences, with the identity operator (the context
//!    function itself) as the control. The γ sweep says what each is worth in
//!    perplexity, and γ = 1 is the term alone.
//!
//! 2. **Operator and lexicon trained together.** The word functions become
//!    parameters on the operator's tape; after training they are written back
//!    into a copy of the facet and the whole of (1) is measured again on it —
//!    including the manifold's own scorer, which never saw the operator.
//!
//! 3. **Recursive composition.** Identities built from corpus fillers bound
//!    to offset roles, nested one, two and three levels deep; fillers recovered
//!    by unbinding, with and without the operator as a learned cleanup between
//!    levels.

mod composition;
mod joint;
mod operator;
mod setup;
mod sweep;
mod types;
#[allow(unused_imports)]
use composition::*;
#[allow(unused_imports)]
use joint::*;
#[allow(unused_imports)]
use operator::*;
#[allow(unused_imports)]
use setup::*;
#[allow(unused_imports)]
use sweep::*;
#[allow(unused_imports)]
use types::*;

use phiano::config::LEARNING_RATE;
use phiano::facet::Facet;
use phiano::metrics::harness::{Harness, PhianoLM, Split};
use phiano::metrics::baseline::kn::KneserNey;
use phiano::phinet::compose::Composition;
use phiano::phinet::joint::{index_examples, write_back, Joint};
use phiano::phinet::lm::OperatorScorer;
use phical::rank;
use phiano::phinet::space::{examples, Example, FunctionSpace, WordFunction};
use phiano::phinet::{Adam, Fno, FnoConfig};
use phiano::tokenizer::Tokenizer;
use phiano::trainer::Trainer;

pub fn run(_a: Vec<String>) {
    let Setup { epochs, split, facet } = setup();

    // ── 1. operator in the LM path ──────────────────────────────────────
    let (op, fno) = operator_in_lm_path(&facet, &split, epochs);

    // ── 2. joint training ───────────────────────────────────────────────
    joint_training(&facet, &split, &op, fno, epochs);

    // ── 3. recursive composition ────────────────────────────────────────
    recursive_composition(&op, epochs);
}
