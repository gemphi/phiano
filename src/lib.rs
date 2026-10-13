pub mod attention;
#[path = "bpe/mod.rs"]
pub mod bpe;
pub mod chunker;
pub use phiplm::code;
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
pub mod facet;
pub mod frames;
#[allow(dead_code)]
pub mod gemgum;
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
pub mod phasor;
#[allow(dead_code)]
pub mod physics;
pub mod phinet;
#[allow(dead_code)]
#[cfg(feature = "phinum")]
pub mod phinum;
pub use phiplm::prose;
#[allow(dead_code)]
/// PhiTon lives in its own crate (`gemphi/phiton`); re-exported so every
/// `crate::phiton::…` path keeps working.
pub use ::phiton;
pub use phiplm::operator;
pub use phiplm::relate;
pub mod relational;
pub mod roles;
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
pub mod wave;
pub mod wiki;
