pub mod arena;
pub mod seam;
pub use phient::code;
pub mod core;
pub mod command;
/// The generation engine is `phient::generate`; the read-only example
/// `flow` still names `phiano::compose::flow::RiverFlow`.
pub use phient::generate::compose;
pub mod config;
pub mod data;
pub mod drivers;
pub mod frames;
pub mod instruction;
pub mod lab;
pub mod model;
pub mod persona;
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
