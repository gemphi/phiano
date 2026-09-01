#![allow(dead_code)]
//! Metrics module: baselines, validation-aware evaluation, capacity tuning,
//! regularization, generalization, adversarial robustness, ARC benchmarks.

pub mod baseline;
pub mod kn_baseline;
pub mod harness;
pub mod relation;
pub mod eval_split;
pub mod capacity;
pub mod regularization;
pub mod generalization;
pub mod adversarial;
pub mod ood_detection;
pub mod distribution_shift;
pub mod arc;
pub mod shortcut_detection;
pub mod adaptation;
pub mod novelty_benchmark;
pub mod benchmark_runner;
pub mod report;
