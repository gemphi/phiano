//! Phinet applied to Phiano: the promoted lexicon as a function space.
//!
//! The engine itself — complex tensors, the tape, the FFT, the Fourier
//! neural operator, joint training, holographic bind/unbind — is the
//! standalone [`phinet`] crate (`../phinet`), which knows nothing about words.
//! This module is what Phiano adds on top of it:
//!
//! * [`space`]   — a `SpectralPhasor` read as a function on the relation circle,
//!   contexts as functions, and `(context → next word)` examples.
//! * [`reader`]  — what is in the existing model files.
//! * [`lm`]      — the operator as a scorer inside the perplexity harness.
//! * [`joint`]   — examples for joint training, and writing learned functions
//!   back into a facet.
//! * [`compose`] — nested role binding from corpus offsets, with the operator
//!   as the learned cleanup that makes recursive unbinding recoverable.

#[path = "compose/mod.rs"]
pub mod compose;
pub mod joint;
pub mod lm;
pub mod reader;
pub mod space;

pub use phistd::{Adam, Function, Tape, Tensor, Var};
pub use phical::{Example, Fno, FnoConfig};
pub use space::{FunctionSpace, WordFunction};
