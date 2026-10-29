pub mod arena;
pub mod seam;
pub use phient::code;
pub mod core;
pub mod command;
/// The generation engine is `phient::generate`; the read-only example
/// `flow` still names `phiano::compose::flow::RiverFlow`.
pub use phient::generate::compose;
pub mod config;
pub mod drivers;
pub mod frames;
pub mod instruction;
pub mod lab;
pub mod model;
pub mod persona;
/// The spectral phasor lives in phiecs, beside the world's `Phasor`;
/// re-exported so every `crate::phasor::…` path keeps working.
pub use phiecs::phasor::spectral as phasor;
/// The lexicon lives in phiecs (`phiecs::phasor::facet`) and phiano reads it
/// from there. Two read-only examples (`flow`, `meaning`) still name
/// `phiano::facet::Facet`; this keeps them building until they can be edited.
pub use phiecs::phasor::facet;
/// The facet's files are `phiecs::phasor::facet::storage`; the read-only
/// example `flow` still names `phiano::storage::Storage`.
pub use phiecs::phasor::facet::storage;
/// The trainer is `phient::train::trainer`; the read-only example `meaning`
/// still names `phient::train::trainer::Trainer`.
pub use phient::train::trainer;
#[allow(dead_code)]
pub use phient::prose;
#[allow(dead_code)]
/// PhiTon lives in its own crate (`gemphi/phiton`); re-exported so every
/// `crate::phiton::…` path keeps working.
pub use ::phiton;
pub use phient::operator;
pub use phient::relate;
pub mod sys;
pub mod server;
pub mod tokenizer;
pub mod wiki;
