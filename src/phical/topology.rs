/// Topology of the color-space-time manifold — abstract shapes for
/// linguistic building blocks.
///
/// Words, phrases, sentences, stories, and concepts are not just points
/// on the manifold — they have **shape**. This module defines the
/// topological structures that emerge when linguistic units are mapped
/// onto the color-space-time manifold as graphs.
///
/// # Building Blocks
///
/// ```text
///   Word       → Vertex (0-simplex)     — a single manifold point
///   Phrase     → Edge (1-simplex)       — two words + their coupling
///   Sentence   → Path (1-chain)         — ordered sequence of edges
///   Story      → Surface (2-complex)    — network of connected sentences
///   Concept    → Manifold region        — attractor basin of related words
/// ```
///
/// Each shape is evaluated without data concreteness — we understand
/// the building blocks through their geometric relationships on the
/// manifold, not through specific word content.

use super::ColorSpaceTimePoint;
use super::ColorSpaceTimeManifold;
use crate::config::{PHI, TWO_PI};
use crate::phiton::chiton::Chiton;
use crate::phiton::types::LightQuantum;
use serde::Serialize;
use std::f64::consts::PI;

/// A vertex — the simplest topological shape: a single word on the manifold.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Vertex {
    pub point: ColorSpaceTimePoint,
    pub word_hash: u64,
}

impl Vertex {
    pub fn new(point: ColorSpaceTimePoint, word_hash: u64) -> Self {
        Self { point, word_hash }
    }
}

/// An edge — connects two vertices. Represents a phrase (two-word relation).
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Edge {
    pub from: Vertex,
    pub to: Vertex,
    /// Geodesic distance between the two vertices.
    pub length: f64,
    /// Coupling strength (interference × resonance).
    pub coupling: f64,
}

impl Edge {
    pub fn new(from: Vertex, to: Vertex) -> Self {
        let length = ColorSpaceTimeManifold::geodesic(&from.point, &to.point);
        let qa = LightQuantum::from_phase(from.point.effective_phase, 1.0, from.point.band_n);
        let qb = LightQuantum::from_phase(to.point.effective_phase, 1.0, to.point.band_n);
        let ca = Chiton::from_quantum(&qa);
        let cb = Chiton::from_quantum(&qb);
        let coupling = ca.resonance(&cb);
        Self { from, to, length, coupling }
    }
}

/// A path — ordered sequence of edges. Represents a sentence.
#[derive(Debug, Clone, Serialize)]
pub struct Path {
    pub edges: Vec<Edge>,
    /// Total path length (sum of edge geodesics).
    pub total_length: f64,
    /// Average coupling across all edges.
    pub coherence: f64,
    /// Phase winding number — how many times the path wraps around the manifold.
    pub winding: f64,
}

impl Path {
    pub fn from_edges(edges: Vec<Edge>) -> Self {
        let total_length: f64 = edges.iter().map(|e| e.length).sum();
        let coherence: f64 = if edges.is_empty() {
            0.0
        } else {
            edges.iter().map(|e| e.coupling).sum::<f64>() / edges.len() as f64
        };
        let winding = Self::compute_winding(&edges);

        Self { edges, total_length, coherence, winding }
    }

    fn compute_winding(edges: &[Edge]) -> f64 {
        let mut total_phase: f64 = 0.0;
        for e in edges {
            let mut delta = e.to.point.effective_phase - e.from.point.effective_phase;
            if delta > PI { delta -= TWO_PI; }
            if delta < -PI { delta += TWO_PI; }
            total_phase += delta;
        }
        total_phase / TWO_PI
    }

    /// Returns the number of edges (sentence length in word transitions).
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Returns true if the path has no edges.
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}

/// A surface — network of connected paths. Represents a story.
#[derive(Debug, Clone, Serialize)]
pub struct Surface {
    pub paths: Vec<Path>,
    /// Adjacency: which paths share vertices.
    pub adjacency: Vec<(usize, usize)>,
    /// Surface area — total semantic content.
    pub area: f64,
    /// Genus — topological complexity (number of "holes" in the story).
    pub genus: u32,
}

impl Surface {
    pub fn from_paths(paths: Vec<Path>) -> Self {
        let area: f64 = paths.iter().map(|p| p.total_length).sum();
        let adjacency = Self::find_adjacencies(&paths);
        let genus = Self::estimate_genus(&paths, &adjacency);
        Self { paths, adjacency, area, genus }
    }

    fn find_adjacencies(paths: &[Path]) -> Vec<(usize, usize)> {
        let mut adj = Vec::new();
        for i in 0..paths.len() {
            for j in (i + 1)..paths.len() {
                if Self::paths_share_vertex(&paths[i], &paths[j]) {
                    adj.push((i, j));
                }
            }
        }
        adj
    }

    fn paths_share_vertex(a: &Path, b: &Path) -> bool {
        for ea in &a.edges {
            for eb in &b.edges {
                if ea.from.word_hash == eb.from.word_hash
                    || ea.from.word_hash == eb.to.word_hash
                    || ea.to.word_hash == eb.from.word_hash
                    || ea.to.word_hash == eb.to.word_hash
                {
                    return true;
                }
            }
        }
        false
    }

    fn estimate_genus(paths: &[Path], adj: &[(usize, usize)]) -> u32 {
        let v = paths.len();
        let e = adj.len();
        let f = 1;
        let euler = v as i32 - e as i32 - f;
        let g = (2 - euler).max(0) as u32;
        g
    }
}

/// A region — an attractor basin of related vertices. Represents a concept.
#[derive(Debug, Clone, Serialize)]
pub struct Region {
    pub vertices: Vec<Vertex>,
    /// Centroid of the region in manifold space.
    pub centroid_phase: f64,
    /// Spread — how dispersed the concept is.
    pub spread: f64,
    /// Density — how tightly packed the vertices are.
    pub density: f64,
}

impl Region {
    pub fn from_vertices(vertices: Vec<Vertex>) -> Self {
        if vertices.is_empty() {
            return Self { vertices, centroid_phase: 0.0, spread: 0.0, density: 0.0 };
        }

        let n = vertices.len() as f64;
        let sum_x: f64 = vertices.iter().map(|v| v.point.effective_phase.cos()).sum();
        let sum_y: f64 = vertices.iter().map(|v| v.point.effective_phase.sin()).sum();
        let centroid_phase = sum_y.atan2(sum_x).rem_euclid(TWO_PI);

        let mut spread = 0.0;
        for v in &vertices {
            let mut delta = (v.point.effective_phase - centroid_phase).abs();
            if delta > PI { delta = TWO_PI - delta; }
            spread += delta;
        }
        spread /= n;

        let density = if spread > 0.0 { 1.0 / (1.0 + spread * PHI) } else { 1.0 };

        Self { vertices, centroid_phase, spread, density }
    }

    /// Returns the number of vertices in this concept region.
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    /// Returns true if the region has no vertices.
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }
}
