//! Phinum — the 16/32/64 variation engine for language classification.
//!
//! Phinum is the numerical aspect of language understanding. It provides
//! three granularity levels — `Phinum16`, `Phinum32`, `Phinum64` — that
//! classify words, sentences, and paragraphs into structural keys without
//! storing examples. The variations and their links form a "spider net"
//! that captures language topology.
//!
//! # Architecture
//!
//! ```text
//!   Phinum16  ──▶  16 core classifications  (fast, coarse)
//!       │
//!   Phinum32  ──▶  32 core classifications  (balanced)
//!       │
//!   Phinum64  ──▶  64 core classifications  (fine, complete)
//! ```
//!
//! Powered by phidoc ◂ puijs.

pub mod config;
pub mod iching;
pub mod lexicon;
pub mod searle;
pub mod syntax;
pub mod topology;
pub mod variants;

#[allow(unused_imports)]
pub use config::PhinumConfig;
#[allow(unused_imports)]
pub use iching::{Hexagram, Trigram};
#[allow(unused_imports)]
pub use lexicon::{PhinumLexicon, WordClass};
#[allow(unused_imports)]
pub use searle::{SearleClassifier, SpeechAct};
#[allow(unused_imports)]
pub use syntax::{PartOfSpeech, PosDictionary, SyntaxKey, SyntaxParser};
#[allow(unused_imports)]
pub use topology::{SentenceType, ParagraphType, SpiderNet};
#[allow(unused_imports)]
pub use variants::{Phinum16, Phinum32, Phinum64, PhinumLevel, Variation};
