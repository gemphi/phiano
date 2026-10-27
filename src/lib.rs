pub mod arena;
pub mod seam;
pub mod scribe;
pub mod attention;
#[path = "bpe/mod.rs"]
pub mod bpe;
pub mod chunker;
pub use phient::code;
pub mod helix;
pub mod cognitive;
pub mod command;
pub mod compose;
pub mod conception;
pub mod correction;
pub mod config;
pub mod curriculum;
pub mod data;
pub mod drivers;
pub mod envision;
pub mod eval;
pub mod frames;
#[path = "generate/mod.rs"]
pub mod generate;
pub mod instruction;
pub mod lab;
pub mod layers;
pub mod learn;
pub mod lifelong;
pub mod memory;
pub mod metrics;
pub mod nonlinear;
pub mod model;
pub mod oscillator;
pub mod flow;
pub mod persona;
/// The spectral phasor lives in phiecs, beside the world's `Phasor`;
/// re-exported so every `crate::phasor::…` path keeps working.
pub use phiecs::phasor::spectral as phasor;
/// The lexicon lives in phiecs (`phiecs::phasor::facet`) and phiano reads it
/// from there. Two read-only examples (`flow`, `meaning`) still name
/// `phiano::facet::Facet`; this keeps them building until they can be edited.
pub use phiecs::phasor::facet;
#[allow(dead_code)]
pub mod phinet;
#[allow(dead_code)]
pub use phient::prose;
#[allow(dead_code)]
/// PhiTon lives in its own crate (`gemphi/phiton`); re-exported so every
/// `crate::phiton::…` path keeps working.
pub use ::phiton;
pub use phient::operator;
pub use phient::relate;
pub mod relational;
pub mod roles;
pub mod sys;
pub mod reasoning;
pub mod kinds;
pub mod server;
pub mod sources;
pub mod storage;
pub mod synthesis;
pub mod synthetic;
pub mod tokenizer;
pub mod topdown;
pub mod trainer;
pub mod wiki;
