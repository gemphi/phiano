#![allow(dead_code, unused_imports)]
//! Lifelong learning coordinator: component reuse, meta-learning, and benchmark tracking.
//!
//! Implements François Chollet's vision of lifelong learning (Ch 14.5):
//! accumulating reusable abstractions and programs across sequential tasks,
//! adapting rapidly via meta-learned phase priors, and monitoring distribution drift.
//! All operations are encapsulated in [`LifelongLearner`], following the Diem
//! convention that all public symbols belong to named types.
//!
//! # Architecture
//!
//! ```text
//! Incoming Task
//!   │
//!   ▼
//! LifelongLearner::learn_task()
//!   ├─▶ 1. Query ComponentLibrary (Phase-signature similarity)
//!   │       └─▶ If matched: apply & increment reuse_count
//!   ├─▶ 2. Online Task Adaptation (Meta-learning priors)
//!   └─▶ 3. Extract & Register Novel Patterns in ComponentLibrary
//! ```

pub mod meta;
pub mod history;
pub mod reuse;
pub mod monitor;

use crate::eval::Evaluator;
use crate::facet::Facet;
use crate::synthesis::library::ComponentLibrary;
use crate::trainer::Trainer;
pub use history::{BenchmarkEntry, BenchmarkHistory};
pub use meta::{MetaLearner, MetaModel};
pub use monitor::{Alert, ModelMonitor};
pub use reuse::{FeatureReuse, FeatureSet};
use serde::Serialize;

/// Outcome of a lifelong learning task evaluation.
#[derive(Debug, Clone, Serialize)]
pub struct LearnResult {
    /// Identifier or prompt of the task.
    pub task: String,
    /// Whether an existing component from the library was reused.
    pub reused_component: Option<String>,
    /// Coherence score after learning.
    pub coherence: f64,
    /// Number of training iterations performed.
    pub iterations: usize,
}

/// Outcome of cross-domain knowledge transfer between facets.
#[derive(Debug, Clone, Serialize)]
pub struct TransferResult {
    pub source_label: String,
    pub target_label: String,
    pub features_transferred: usize,
    pub post_transfer_coherence: f64,
}

/// Central coordinator for lifelong learning, component synthesis, and meta-adaptation.
#[derive(Debug, Default)]
pub struct LifelongLearner {
    /// Persistent library of synthesized program abstractions.
    pub library: ComponentLibrary,
    /// Chronological history of benchmark reports.
    pub history: BenchmarkHistory,
}

impl LifelongLearner {
    /// Creates a new, empty [`LifelongLearner`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Solves an incoming task by querying the library first, then adapting online.
    pub fn learn_task(
        &mut self,
        facet: &mut Facet,
        trainer: &Trainer,
        task: &str,
    ) -> LearnResult {
        // 1. Check library for reusable component
        let reused_name = if let Some(comp) = self.library.find_reusable(facet, task) {
            let name = comp.name.clone();
            self.library.mark_used(&name);
            Some(name)
        } else {
            None
        };

        // 2. Online training on task sentences
        let iterations = if reused_name.is_some() { 4 } else { 16 };
        for _ in 0..iterations {
            trainer.train_sentence(facet, task);
        }

        let evaluator = Evaluator::new();
        let coherence = evaluator.eval(facet, task).coherence;

        LearnResult {
            task: task.to_string(),
            reused_component: reused_name,
            coherence,
            iterations,
        }
    }

    /// Transfers extracted phase features from a source domain to a target domain.
    pub fn transfer_knowledge(
        &mut self,
        facet: &mut Facet,
        source_label: &str,
        target_label: &str,
    ) -> TransferResult {
        let features = FeatureReuse::extract(facet);
        let n_features = features.len();
        FeatureReuse::apply(facet, &features);

        let evaluator = Evaluator::new();
        let eval = evaluator.eval(facet, target_label);

        TransferResult {
            source_label: source_label.to_string(),
            target_label: target_label.to_string(),
            features_transferred: n_features,
            post_transfer_coherence: eval.coherence,
        }
    }
}
