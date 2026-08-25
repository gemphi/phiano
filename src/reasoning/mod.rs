/// Reasoning module: phase-space pathfinding, analogy, hybrid reasoning,
/// program synthesis, planning, and abstraction.
///
/// Module structure:
/// - pathfinding.rs: Phase-space pathfinding (existing ReasoningEngine)
/// - analogy.rs: Value-centric and program-centric analogy
/// - program_analogy.rs: Structural/program-centric analogy
/// - hybrid.rs: Hybrid reasoner combining geometric + structural
/// - sorting.rs: Sorting as a reasoning test
/// - planning.rs: Multi-step planning
/// - abstraction.rs: Abstraction extraction from examples
/// - counterfactual.rs: Hypothetical/counterfactual reasoning

pub mod pathfinding;
pub mod analogy;
pub mod program_analogy;
pub mod hybrid;
pub mod sorting;
pub mod planning;
pub mod abstraction;
pub mod counterfactual;

pub use pathfinding::{ReasoningEngine, ReasoningChain, ReasoningStep};
pub use hybrid::{HybridReasoner, HybridResult};
pub use analogy::{AnalogyResult, value_centric_analogy, find_analogies};
pub use planning::{Plan, PlanStep, plan};
