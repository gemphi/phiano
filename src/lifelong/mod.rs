/// Lifelong learning module: coordinates reuse, meta-learning, and monitoring.
/// Implements Ch 14.5's lifelong learning and modular reuse.

pub mod meta;
pub mod history;
pub mod reuse;
pub mod monitor;

pub use meta::{MetaModel, meta_learn};
pub use history::{BenchmarkHistory, BenchmarkEntry};
pub use reuse::{FeatureSet, extract_features, apply_features};
pub use monitor::{ModelMonitor, Alert};
