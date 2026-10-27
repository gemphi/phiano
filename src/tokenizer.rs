//! The words phiano reads: lowercase alphanumeric tokens, function words
//! and sentences. The one implementation is phiton's (`phiton::p64::words`),
//! where the P64 engine reads with it; every `crate::tokenizer` path stays.

pub use phiton::p64::words::Tokenizer;
