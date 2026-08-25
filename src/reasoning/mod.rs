#![allow(dead_code)]
//! Reasoning module: phase-space pathfinding, analogy, hybrid reasoning,
//! program synthesis, planning, and abstraction.
//!
//! Module structure:
//! - pathfinding.rs: Phase-space pathfinding (existing ReasoningEngine)
//! - analogy.rs: Value-centric and program-centric analogy
//! - program_analogy.rs: Structural/program-centric analogy
//! - hybrid.rs: Hybrid reasoner combining geometric + structural
//! - sorting.rs: Sorting as a reasoning test
//! - planning.rs: Multi-step planning
//! - abstraction.rs: Abstraction extraction from examples
//! - counterfactual.rs: Hypothetical/counterfactual reasoning
//! - diagnostics.rs: Convergence diagnostics
//! - multi_path.rs: Multi-path and depth-controlled reasoning
//! - comparison.rs: Reasoning comparison and step templating

pub mod pathfinding;
pub mod analogy;
pub mod program_analogy;
pub mod hybrid;
pub mod sorting;
pub mod planning;
pub mod abstraction;
pub mod counterfactual;
pub mod diagnostics;
pub mod multi_path;
pub mod comparison;

pub use pathfinding::{ReasoningEngine, ReasoningChain};
pub use hybrid::HybridReasoner;
pub use multi_path::MultiPath;
pub use comparison::ReasoningComparison;
pub use diagnostics::Diagnostics;
