pub mod dialog;
pub mod fingerprint;
pub mod impersonate;
pub mod store;
pub mod world;

pub use dialog::{Dialogue, Reply};
pub use fingerprint::Fingerprint;
pub use store::PersonaStore;
pub use impersonate::Impersonator;
pub use world::Styles;

use phiecs::phasor::facet::Facet;
use crate::trainer::Trainer;
use std::fmt;

/// Style - a persona's learned style fingerprint in the phase manifold.
/// (phiecs's `Persona` is a different thing: a persona file, a quest's plan.)
///
/// A persona is NOT a hardcoded character. It is a phase-space
/// fingerprint extracted from text examples. The fingerprint captures
/// which sectors the persona's words cluster in, their amplitude
/// distribution, and their characteristic flow patterns.
///
/// To create a persona:
/// 1. Feed it examples of the persona's writing/speech
/// 2. The trainer learns the examples (Kuramoto re-tuning)
/// 3. The fingerprint is extracted from the resulting phase distribution
/// 4. The impersonator can then compose in that persona's style
///
/// This is generic - it works for anyone whose text you can feed it.
/// Elon Musk, Taylor Swift, Shakespeare - all are just text distributions
/// in the phase manifold. The system finds what makes each unique.
pub struct Style {
    /// The name of the persona (user-provided label).
    pub name: String,
    /// The style fingerprint extracted from examples.
    pub fingerprint: Fingerprint,
    /// The examples used to create this persona.
    pub examples: Vec<String>,
}

impl Style {
    /// Creates a new persona from a set of text examples.
    ///
    /// Trains the facet on the examples, then extracts the fingerprint
    /// from the resulting phase distribution.
    pub fn from_examples(
        name: &str,
        examples: &[String],
        facet: &mut Facet,
        trainer: &Trainer,
    ) -> Self {
        for example in examples {
            trainer.train_sentence(facet, example);
        }

        let fingerprint = Fingerprint::extract(facet, examples);

        Style {
            name: name.to_string(),
            fingerprint,
            examples: examples.to_vec(),
        }
    }

    /// Compares this persona's fingerprint with another.
    ///
    /// Returns a similarity score (0.0 = completely different, 1.0 = identical).
    /// This is how the system finds style differences between personas.
    pub fn similarity_to(&self, other: &Style) -> f64 {
        self.fingerprint.similarity(&other.fingerprint)
    }

}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  ── persona: {} ──", self.name)?;
        writeln!(f, "  learned from {} examples", self.examples.len())?;
        writeln!(f, "{}", self.fingerprint)?;
        Ok(())
    }
}
