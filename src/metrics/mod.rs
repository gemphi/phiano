#![allow(dead_code)]
//! Metrics module: baselines, validation-aware evaluation, capacity tuning,
//! regularization, generalization, adversarial robustness, ARC benchmarks.

pub mod baseline;
pub mod forgetting;
pub mod harness;
pub mod relation;
pub mod report;
pub mod sentence;
pub mod generalization;
pub mod adversarial;
pub mod detection;
pub use detection::ood as ood_detection;
pub mod arc;
pub mod adaptation;
pub mod benchmark;
