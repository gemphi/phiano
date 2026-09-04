use crate::config::{LEARNING_RATE, SYNTACTIC_MOMENTUM_DEFAULT, TWO_PI};
use crate::facet::Facet;
use crate::phical::PhicalOps;
use crate::phasor::SpectralPhasor;
use crate::relational::{RelationVocab, RelationalFacet, WINDOW};
use crate::tokenizer::Tokenizer;
use crate::wave::c64;
use serde::Serialize;

/// What kind of coupling holds two flow nodes together.
///
/// This enum used to carry two more variants, `Semantic` and `AntiPhase`, which
/// nothing constructed — held alive by an `#[allow(dead_code)]`. Deleting them
/// was right and it was not enough, because the idea they gestured at is real
/// and was missing: when two words are associated, the association *has a kind*,
/// and the kind is nameable. In `a dog eats a donut` the coupling between `dog`
/// and `donut` is `eats`. The graph did not have that edge in any form. It built
/// `dog→eats`, `eats→a`, `a→donut` and stopped, so the one relation in the
/// sentence a reader would name was the one relation the graph could not.
///
/// [`CouplingKind::Relation`] is that edge, and it carries the thing an enum
/// variant could never carry: the **dimension** the coupling acts in.
/// [`crate::relational::RelationVocab`] assigns every observable relation its
/// own channel out of the sixty-four, so `eats` is not just a label on the edge
/// — it names which of the word's sixty-four complex components the coupling
/// reads. Two words coupled by `eats` are compared on the `eats` channel and no
/// other. That is why the variant is a struct and not a bare tag, and why it is
/// discovered from the corpus rather than declared here: the kinds are whatever
/// the text supplies, up to the channel count.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CouplingKind {
    /// Adjacency, weighted by bigram probability.
    Bigram,
    /// Adjacency, carrying the learned phase lag.
    SyntaxLag,
    /// A named relation between two words that need not be adjacent, and the
    /// channel it occupies. `marker` is the word standing between them, or the
    /// offset's name when the relation is bare distance.
    Relation { marker: String, channel: usize },
}

impl CouplingKind {
    /// The channel this coupling reads, when it has one.
    ///
    /// Adjacency has no channel: a bigram edge is a claim about position, and
    /// position is not one of the sixty-four relations.
    pub fn channel(&self) -> Option<usize> {
        match self {
            CouplingKind::Relation { channel, .. } => Some(*channel),
            _ => None,
        }
    }
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

    /// Builds the graph with named relation edges as well as adjacency.
    ///
    /// [`Self::build`] connects each word to the next one and stops, so the only
    /// couplings it can represent are positional. This adds the edge a reader
    /// would draw: for every pair of words inside the relation window that
    /// [`RelationVocab`] can name, an edge carrying that name and the channel it
    /// owns. `a dog eats a donut` gains `dog —eats→ donut`.
    ///
    /// The lag on a relation edge is read from the channel of its own kind —
    /// `theta_to[k] - theta_from[k]` — not from the word's single global phase.
    /// This is where the coupling kind stops being a label: `eats` and `of`
    /// between the same two words would pull them toward different angles,
    /// because they are angles in different dimensions. The weight is the
    /// geometric mean of the evidence both words carry on that channel, so a
    /// relation asserted once between two words the corpus barely places there
    /// pulls proportionately less than one it has seen a thousand times.
    pub fn build_relational(
        facet: &Facet,
        rel: &RelationalFacet,
        vocab: &RelationVocab,
        prompt: &str,
    ) -> Self {
        let mut flow = Self::build(facet, prompt);
        let toks = Tokenizer::tokenize(prompt);

        // Node indices are deduplicated by word, so a repeated word is one node.
        let index_of = |w: &str| flow.nodes.iter().position(|n| n.word == w);

        for i in 0..toks.len() {
            for j in (i + 1)..(i + 1 + WINDOW).min(toks.len()) {
                let k = match vocab.channel(&toks, i, j) {
                    Some(k) => k,
                    None => continue,
                };
                // Adjacency already has two edges; a relation edge on top of them
                // would be a third vote for the same fact.
                if j == i + 1 {
                    continue;
                }
                let (from, to) = match (index_of(&toks[i]), index_of(&toks[j])) {
                    (Some(a), Some(b)) if a != b => (a, b),
                    _ => continue,
                };
                let (pa, pb) = match (rel.words.get(&toks[i]), rel.words.get(&toks[j])) {
                    (Some(a), Some(b)) => (a, b),
                    _ => continue,
                };
                let weight = (pa.mag[k] * pb.mag[k]).sqrt();
                if weight <= 0.0 {
                    continue;
                }
                flow.edges.push(FlowEdge {
                    from,
                    to,
                    coupling: CouplingKind::Relation {
                        marker: vocab.name(k),
                        channel: k,
                    },
                    lag: (pb.theta[k] - pa.theta[k]).rem_euclid(TWO_PI),
                    weight,
                });
            }
        }
        flow
    }

    /// The distinct relation kinds this graph actually used.
    ///
    /// Reported rather than assumed: the architecture allows sixty-four, and how
    /// many a given sentence or corpus supplies is a fact about the text.
    pub fn coupling_kinds(&self) -> Vec<(String, usize)> {
        let mut seen: Vec<(String, usize)> = Vec::new();
        for e in &self.edges {
            if let CouplingKind::Relation { marker, channel } = &e.coupling {
                if !seen.iter().any(|(_, c)| c == channel) {
                    seen.push((marker.clone(), *channel));
                }
            }
        }
        seen.sort_by_key(|(_, c)| *c);
        seen
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
    /// Alignment between the flow's collective phase and a word.
    ///
    /// Compared on channel 0 alone, because that is the only channel a single
    /// collective angle determines. The previous version built a whole
    /// `SpectralPhasor` from that one angle and compared all its harmonics — but
    /// those harmonics came from `SpectralPhasor::new`, which spreads by the
    /// golden angle, while a real word's come from its hash. So 63 of the 64
    /// channels being compared were fabricated on one side and unrelated on the
    /// other, and the result was noise with a sign.
    ///
    /// It read as working only because seeded phases used to cluster by spelling:
    /// with that fixed, `test_resonance_with_known_word` began failing on a word
    /// compared against *itself*, which is what a fabricated comparison looks
    /// like once the accidental structure it rode on is removed.
    pub fn resonance_with(&self, facet: &Facet, word: &str) -> f64 {
        match facet.lexicon.get(word) {
            Some(phasor) => (self.collective_phase - phasor.phase).cos(),
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
        assert!(
            res > 0.99,
            "a flow built from one word sits on that word's phase, so its \
             self-resonance is 1: got {res:.4}"
        );
    }

    /// The edge the graph could not previously draw.
    ///
    /// `a dog eats a donut` has one relation in it that a reader would name, and
    /// it holds between two words that are not adjacent. `build` produces only
    /// adjacency, so `dog` and `donut` are three hops apart and connected by
    /// nothing that mentions `eats`. This asserts the relation edge exists, that
    /// it runs between the right two nodes, and that its channel is the one the
    /// vocabulary gave `eats` — not merely that *some* extra edge appeared.
    #[test]
    fn test_a_relation_edge_names_the_word_between_its_endpoints() {
        use crate::relational::{RelationVocab, RelationalFacet};

        // A corpus, not four sentences. The rule that picks the marker compares
        // how often each candidate links, so it needs a text where the
        // determiner actually behaves like one — common everywhere, not merely
        // present. On a four-sentence fixture `a` occurs twice and `eats` three
        // times, so `a` is the rarer word and correctly wins a rule that is
        // looking for the rarer word. The fixture was wrong, not the rule.
        let mut sentences: Vec<String> = Vec::new();
        let eaters = ["dog", "cat", "child", "bird", "fox", "otter"];
        let foods = ["donut", "fish", "apple", "seed", "berry", "clam"];
        for (a, f) in eaters.iter().zip(foods.iter()) {
            sentences.push(format!("a {a} eats a {f}"));
        }
        // The determiner earns its frequency the way it does in English: by
        // appearing in every other sentence, most of which are about nothing in
        // particular.
        for a in &eaters {
            for verb in ["sleeps", "waits", "runs", "watches", "rests"] {
                sentences.push(format!("a {a} {verb} near a river"));
                sentences.push(format!("the {a} {verb} by the shore"));
            }
        }
        for f in &foods {
            for adj in ["fresh", "small", "ripe", "plain"] {
                sentences.push(format!("the {f} was {adj} today"));
                sentences.push(format!("a {f} sits on a table"));
            }
        }
        let sentences: Vec<&str> = sentences.iter().map(|s| s.as_str()).collect();
        let docs: Vec<Vec<String>> = sentences
            .iter()
            .map(|s| Tokenizer::tokenize(s))
            .collect();
        let vocab = RelationVocab::learn(&docs);
        let rel = RelationalFacet::train(&docs, &vocab, 4);
        let facet = Facet::new();

        let flow = PhaseFlow::build_relational(&facet, &rel, &vocab, "a dog eats a donut");
        let plain = PhaseFlow::build(&facet, "a dog eats a donut");

        assert!(
            plain.coupling_kinds().is_empty(),
            "adjacency-only build must produce no named relation at all — that \
             absence is what this edge exists to fix"
        );

        let dog = flow.nodes.iter().position(|n| n.word == "dog").unwrap();
        let donut = flow.nodes.iter().position(|n| n.word == "donut").unwrap();
        let eats_channel = vocab.channel(&Tokenizer::tokenize("a dog eats a donut"), 1, 3);

        let edge = flow
            .edges
            .iter()
            .find(|e| e.from == dog && e.to == donut)
            .expect("dog and donut must be coupled directly, not only through the words between them");

        match &edge.coupling {
            CouplingKind::Relation { marker, channel } => {
                assert_eq!(
                    marker, "eats",
                    "the coupling must be named by the word standing between the pair"
                );
                assert_eq!(
                    Some(*channel),
                    eats_channel,
                    "the edge must read the channel the vocabulary assigned to `eats`, \
                     or the kind is a label and not a dimension"
                );
            }
            other => panic!("expected a named relation, got {other:?}"),
        }
    }
}
