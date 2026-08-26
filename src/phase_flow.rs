use crate::config::{LEARNING_RATE, SYNTACTIC_MOMENTUM_DEFAULT, TWO_PI};
use crate::facet::Facet;
use crate::phical::PhicalOps;
use crate::phasor::{SpectralPhasor, TorusPhasor};
use crate::tokenizer::Tokenizer;
use crate::wave::c64;
use serde::Serialize;

/// Coupling type between two flow nodes.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CouplingKind {
    Bigram,
    SyntaxLag,
    Semantic,
    AntiPhase,
}

/// A node in the dynamic phase computation graph.
#[derive(Debug, Clone, Serialize)]
pub struct FlowNode {
    pub word: String,
    pub phase: f64,
    pub amplitude: f64,
    pub band_n: u32,
    pub activation: f64,
    pub novelty: f64,
}

/// An edge in the dynamic phase computation graph.
#[derive(Debug, Clone, Serialize)]
pub struct FlowEdge {
    pub from: usize,
    pub to: usize,
    pub coupling: CouplingKind,
    pub lag: f64,
    pub weight: f64,
}

/// A snapshot of the phase flow at one generation step.
#[derive(Debug, Clone, Serialize)]
pub struct FlowStep {
    pub step: usize,
    pub collective_phase: f64,
    pub momentum: f64,
    pub selected_word: Option<String>,
    pub resonance_score: f64,
    pub novelty: f64,
}

/// PhaseFlow — a dynamic phase computation graph built per input.
///
/// Like PyTorch's dynamic graph, the topology emerges from execution.
/// Parameters persist in Facet; the PhaseFlow is ephemeral per forward pass.
#[derive(Debug, Clone, Serialize)]
pub struct PhaseFlow {
    pub nodes: Vec<FlowNode>,
    pub edges: Vec<FlowEdge>,
    pub trajectory: Vec<FlowStep>,
    pub collective_phase: f64,
    pub momentum: f64,
    pub order_parameter: f64,
}

#[allow(dead_code)]
impl PhaseFlow {
    /// Builds a PhaseFlow from a prompt and facet.
    /// Creates nodes for each token, edges for bigrams and semantic neighbors.
    pub fn build(facet: &Facet, prompt: &str) -> Self {
        let tokens = Tokenizer::tokenize(prompt);
        let mut nodes = Vec::with_capacity(tokens.len());
        let mut edges = Vec::new();
        let mut word_indices: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        for token in &tokens {
            let idx = match word_indices.get(token) {
                Some(&i) => i,
                None => {
                    let i = nodes.len();
                    word_indices.insert(token.clone(), i);
                    let phasor = facet.lexicon.get(token).copied().unwrap_or_else(|| {
                        let seed = (token.len() as f64 * crate::config::PHI).rem_euclid(TWO_PI);
                        SpectralPhasor::new(seed, crate::config::AMPLITUDE_INITIAL, crate::config::BAND_N_INITIAL)
                    });
                    nodes.push(FlowNode {
                        word: token.clone(),
                        phase: phasor.phase,
                        amplitude: phasor.amplitude,
                        band_n: phasor.band_n,
                        activation: phasor.amplitude,
                        novelty: 0.0,
                    });
                    i
                }
            };
            if nodes.len() > 1 && idx != nodes.len() - 1 {
                let prev_idx = nodes.len() - 2;
                let lag = facet.phase_lag(&nodes[prev_idx].word, token);
                edges.push(FlowEdge {
                    from: prev_idx,
                    to: idx,
                    coupling: CouplingKind::SyntaxLag,
                    lag,
                    weight: 1.0,
                });
            }
        }

        for i in 0..nodes.len().saturating_sub(1) {
            let lag = facet.phase_lag(&nodes[i].word, &nodes[i + 1].word);
            let weight = facet.bigram_probability(&nodes[i].word, &nodes[i + 1].word);
            edges.push(FlowEdge {
                from: i,
                to: i + 1,
                coupling: CouplingKind::Bigram,
                lag,
                weight: weight.max(0.1),
            });
        }

        let collective_phase = Self::compute_collective_phase(&nodes);
        let order_parameter = Self::compute_order_parameter(&nodes);

        Self {
            nodes,
            edges,
            trajectory: Vec::new(),
            collective_phase,
            momentum: SYNTACTIC_MOMENTUM_DEFAULT,
            order_parameter,
        }
    }

    /// Propagates phase waves through the graph for `steps` iterations.
    /// Each step: update activations via Kuramoto coupling at each edge.
    pub fn propagate(&mut self, steps: usize) {
        for _ in 0..steps {
            let mut new_activations = self.nodes.iter().map(|n| n.activation).collect::<Vec<_>>();
            for edge in &self.edges {
                let from_phase = self.nodes[edge.from].phase;
                let to_phase = self.nodes[edge.to].phase;
                let coupling_strength = edge.weight * (to_phase - from_phase + edge.lag).sin();
                new_activations[edge.to] += 0.1 * coupling_strength * self.nodes[edge.from].amplitude;
            }
            for (i, act) in new_activations.into_iter().enumerate() {
                self.nodes[i].activation = act.max(0.0);
            }
            self.collective_phase = Self::compute_collective_phase(&self.nodes);
            self.order_parameter = Self::compute_order_parameter(&self.nodes);
        }
    }

    /// Records a generation step in the trajectory.
    pub fn record_step(&mut self, step: usize, word: Option<&str>, resonance: f64, novelty: f64) {
        self.trajectory.push(FlowStep {
            step,
            collective_phase: self.collective_phase,
            momentum: self.momentum,
            selected_word: word.map(|w| w.to_string()),
            resonance_score: resonance,
            novelty,
        });
    }

    /// Updates momentum based on phase difference.
    pub fn update_momentum(&mut self, phase_diff: f64) {
        self.momentum = (0.85 * self.momentum + 0.15 * phase_diff.abs().max(0.05)).min(0.5);
    }

    /// Applies Hebbian plasticity — shifts node phases toward the collective.
    pub fn hebbian_update(&self, facet: &mut Facet) {
        for node in &self.nodes {
            match facet.lexicon.get_mut(&node.word) {
                Some(phasor) => {
                    let diff = (self.collective_phase - phasor.phase).sin();
                    phasor.phase = (phasor.phase + LEARNING_RATE * diff).rem_euclid(TWO_PI);
                }
                None => {}
            }
        }
    }

    /// Physics-aware Hebbian update using the color-space-time manifold gradient.
    ///
    /// Instead of raw sin(Δφ), this uses [`PhicalOps::relax_phase`] which
    /// computes the geodesic gradient on the T² manifold — accounting for
    /// the fine-structure sub-band spacing and golden-ratio weighting.
    /// This produces more natural phase convergence than the raw Kuramoto rule.
    pub fn hebbian_update_phical(&self, facet: &mut Facet) {
        for node in &self.nodes {
            match facet.lexicon.get_mut(&node.word) {
                Some(phasor) => {
                    phasor.phase = PhicalOps::relax_phase(
                        phasor.phase,
                        phasor.band_n,
                        self.collective_phase,
                        0,
                    );
                }
                None => {}
            }
        }
    }

    /// Applies an anti-phase pulse between two words (self-correction).
    pub fn apply_antiphase(facet: &mut Facet, wrong: &str, correct: &str) {
        match (facet.lexicon.get(wrong), facet.lexicon.get(correct)) {
            (Some(w), Some(c)) => {
                let repulsion = c.phase + crate::config::PHASE_REPULSION;
                let target = (repulsion - w.phase).sin();
                if let Some(w_phasor) = facet.lexicon.get_mut(wrong) {
                    w_phasor.phase = (w_phasor.phase + 0.5 * target).rem_euclid(TWO_PI);
                }
            }
            _ => {}
        }
    }

    /// Computes the collective phase from all nodes.
    fn compute_collective_phase(nodes: &[FlowNode]) -> f64 {
        let sum_x: f64 = nodes.iter().map(|n| n.amplitude * n.phase.cos()).sum();
        let sum_y: f64 = nodes.iter().map(|n| n.amplitude * n.phase.sin()).sum();
        let angle = sum_y.atan2(sum_x);
        match angle < 0.0 {
            true => angle + TWO_PI,
            false => angle,
        }
    }

    /// Computes the Kuramoto order parameter R ∈ [0, 1].
    fn compute_order_parameter(nodes: &[FlowNode]) -> f64 {
        match nodes.is_empty() {
            true => 0.0,
            false => {
                let sum_x: f64 = nodes.iter().map(|n| n.phase.cos()).sum();
                let sum_y: f64 = nodes.iter().map(|n| n.phase.sin()).sum();
                (sum_x * sum_x + sum_y * sum_y).sqrt() / nodes.len() as f64
            }
        }
    }

    /// Computes resonance of a candidate word with the current collective phase.
    pub fn resonance_with(&self, facet: &Facet, word: &str) -> f64 {
        match facet.lexicon.get(word) {
            Some(phasor) => {
                let target_torus = TorusPhasor::from_spectral(&SpectralPhasor::new(self.collective_phase, 1.0, 0));
                let word_torus = TorusPhasor::from_spectral(phasor);
                target_torus.resonance(&word_torus)
            }
            None => 0.0,
        }
    }

    /// Returns the complex wave representation of the collective phase.
    pub fn collective_wave(&self) -> c64 {
        c64::new(self.collective_phase.cos(), self.collective_phase.sin())
    }

    /// Computes novelty as phase distance from the starting collective phase.
    pub fn novelty(&self) -> f64 {
        match self.trajectory.first() {
            Some(first) => {
                let diff = (self.collective_phase - first.collective_phase).abs();
                let wrapped = diff.min(TWO_PI - diff);
                wrapped / std::f64::consts::PI
            }
            None => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_flow_build() {
        let mut facet = Facet::new();
        facet.get_or_init("hello");
        facet.get_or_init("world");

        let flow = PhaseFlow::build(&facet, "hello world");

        assert_eq!(flow.nodes.len(), 2);
        assert!(!flow.edges.is_empty());
        assert!(flow.collective_phase >= 0.0 && flow.collective_phase < TWO_PI);
        assert!(flow.order_parameter > 0.0);
    }

    #[test]
    fn test_phase_flow_propagate() {
        let mut facet = Facet::new();
        facet.get_or_init("alpha");
        facet.get_or_init("beta");
        facet.get_or_init("gamma");

        let mut flow = PhaseFlow::build(&facet, "alpha beta gamma");
        let initial_phase = flow.collective_phase;
        flow.propagate(5);

        assert!(flow.trajectory.is_empty());
        assert!(flow.order_parameter > 0.0);
        let _ = initial_phase;
    }

    #[test]
    fn test_antiphase_correction() {
        let mut facet = Facet::new();
        facet.get_or_init("wrong");
        facet.get_or_init("correct");

        let before = facet.lexicon["wrong"].phase;
        PhaseFlow::apply_antiphase(&mut facet, "wrong", "correct");
        let after = facet.lexicon["wrong"].phase;

        assert!(before != after, "anti-phase pulse should shift the phase");
    }

    #[test]
    fn test_novelty_empty_trajectory() {
        let facet = Facet::new();
        let flow = PhaseFlow::build(&facet, "test");
        assert_eq!(flow.novelty(), 0.0);
    }

    #[test]
    fn test_resonance_with_known_word() {
        let mut facet = Facet::new();
        facet.get_or_init("hello");

        let flow = PhaseFlow::build(&facet, "hello");
        let res = flow.resonance_with(&facet, "hello");
        assert!(res > 0.0, "self-resonance should be positive");
    }
}
