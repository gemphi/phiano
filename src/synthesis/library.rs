/// Reusable component library: stores synthesized programs for reuse.
/// Implements Ch 14.5's modular reuse and lifelong learning.

use super::program::Program;
use crate::config::TWO_PI;
use crate::facet::Facet;
use crate::tokenizer::Tokenizer;
use serde::Serialize;
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize)]
pub struct Component {
    pub name: String,
    pub program: Program,
    pub phase_signature: Vec<f64>,
    pub reuse_count: usize,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ComponentLibrary {
    pub components: Vec<Component>,
}

impl ComponentLibrary {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new reusable component.
    pub fn register(&mut self, name: &str, program: Program, facet: &Facet) {
        let phase_sig = Self::compute_phase_signature(facet, &name);
        self.components.push(Component {
            name: name.to_string(),
            program,
            phase_signature: phase_sig,
            reuse_count: 0,
        });
    }

    /// Finds a component with matching phase signature.
    pub fn find_reusable(&self, facet: &Facet, task: &str) -> Option<&Component> {
        let task_sig = Self::compute_phase_signature(facet, task);
        let mut best: Option<(&Component, f64)> = None;

        for comp in &self.components {
            let similarity = Self::signature_similarity(&comp.phase_signature, &task_sig);
            match &best {
                Some((_, best_sim)) if similarity <= *best_sim => {}
                _ => best = Some((comp, similarity)),
            }
        }

        best.and_then(|(c, sim)| if sim > 0.6 { Some(c) } else { None })
    }

    /// Increments reuse count for a component.
    pub fn mark_used(&mut self, name: &str) {
        for comp in &mut self.components {
            if comp.name == name {
                comp.reuse_count += 1;
                break;
            }
        }
    }

    fn compute_phase_signature(facet: &Facet, text: &str) -> Vec<f64> {
        let tokens = Tokenizer::tokenize(text);
        tokens
            .iter()
            .filter_map(|t| facet.lexicon.get(t).map(|p| p.phase))
            .collect()
    }

    fn signature_similarity(a: &[f64], b: &[f64]) -> f64 {
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }
        let min_len = a.len().min(b.len());
        let mut total = 0.0;
        for i in 0..min_len {
            let mut diff = (a[i] - b[i]).abs();
            if diff > PI {
                diff = TWO_PI - diff;
            }
            total += 1.0 - diff / PI;
        }
        total / min_len as f64
    }
}
