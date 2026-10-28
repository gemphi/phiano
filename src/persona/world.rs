mod matching;
mod personas;
#[allow(unused_imports)]
use matching::*;
#[allow(unused_imports)]
use personas::*;

use phiecs::phasor::facet::Facet;
use crate::persona::{Fingerprint, Style};
use phient::train::trainer::Trainer;
use std::collections::HashMap;
use std::fmt;

/// Styles - a collection of personas' styles that can interact, compare,
/// and be impersonated. (phiecs's `World` is a different thing: the world
/// of told rows.)
///
/// The world is the context personas live in. It stores multiple
/// personas, computes differences between them, and can generate
/// compositions in any persona's style.
///
/// Usage:
///   1. Create a world
///   2. Add personas from examples (feed text samples)
///   3. Compare personas to find style differences
///   4. Impersonate any persona on any prompt
///   5. Chat as a persona - the world routes input through the
///      impersonator with the active persona
///
/// The world is generic - it works with any text examples.
/// No hardcoded names. Feed it anyone's writing and it learns.
pub struct Styles {
    /// All personas in this world, keyed by name.
    pub personas: HashMap<String, Style>,
}

/// PersonaComparison - the result of comparing two personas.
pub struct PersonaComparison {
    /// Name of the first persona.
    pub name_a: String,
    /// Name of the second persona.
    pub name_b: String,
    /// Overall similarity (0.0 = different, 1.0 = identical).
    pub similarity: f64,
    /// Sectors where they differ most, sorted by absolute difference.
    pub differences: Vec<(u16, f64)>,
}

/// MatchResult - the result of matching unknown text to personas.
pub struct MatchResult {
    /// The text that was matched.
    pub text: String,
    /// All personas ranked by similarity (name, score).
    pub ranked: Vec<(String, f64)>,
    /// The best-matching persona name.
    pub best_name: String,
    /// The best similarity score.
    pub best_score: f64,
    /// Confidence: how much the winner stands out from the runner-up.
    pub confidence: f64,
}
