use crate::facet::Facet;
use crate::persona::{Fingerprint, Persona};
use crate::trainer::Trainer;
use std::collections::HashMap;
use std::fmt;

/// World — a collection of personas that can interact, compare,
/// and be impersonated.
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
///   5. Chat as a persona — the world routes input through the
///      impersonator with the active persona
///
/// The world is generic — it works with any text examples.
/// No hardcoded names. Feed it anyone's writing and it learns.
pub struct World {
    /// All personas in this world, keyed by name.
    pub personas: HashMap<String, Persona>,
}

impl World {
    /// Creates an empty world.
    pub fn new() -> Self {
        Self {
            personas: HashMap::new(),
        }
    }

    /// Adds a persona to the world from text examples.
    ///
    /// Trains the facet on the examples and extracts a fingerprint.
    pub fn add_persona(
        &mut self,
        name: &str,
        examples: &[String],
        facet: &mut Facet,
        trainer: &Trainer,
    ) {
        let persona = Persona::from_examples(name, examples, facet, trainer);
        println!(
            "  [world] added persona '{}' from {} examples",
            name,
            examples.len(),
        );
        self.personas.insert(name.to_string(), persona);
    }

    /// Returns a persona by name.
    pub fn get(&self, name: &str) -> Option<&Persona> {
        self.personas.get(name)
    }

    /// Lists all personas in the world.
    pub fn list(&self) -> Vec<&Persona> {
        self.personas.values().collect()
    }

    /// Compares two personas and returns their similarity and differences.
    ///
    /// This is how the system finds what makes each persona unique —
    /// the sectors where they differ most are their signature styles.
    pub fn compare(&self, name_a: &str, name_b: &str) -> Option<PersonaComparison> {
        let a = self.personas.get(name_a)?;
        let b = self.personas.get(name_b)?;

        let similarity = a.similarity_to(b);
        let differences = a.fingerprint.difference_vector(&b.fingerprint);

        Some(PersonaComparison {
            name_a: name_a.to_string(),
            name_b: name_b.to_string(),
            similarity,
            differences,
        })
    }

    /// Matches unknown text against all personas to find the author.
    ///
    /// Re-extracts all persona fingerprints from their stored examples
    /// using the current facet state, ensuring fair comparison. Then
    /// extracts a fingerprint from the unknown text and computes
    /// cosine similarity against every persona.
    ///
    /// This is style attribution — "who wrote this?"
    pub fn match_text(&self, facet: &Facet, text: &str) -> Option<MatchResult> {
        if self.personas.is_empty() {
            return None;
        }

        // Re-extract all persona fingerprints from stored examples
        // using the current facet state for fair comparison
        let persona_fps: Vec<(String, Fingerprint)> = self
            .personas
            .iter()
            .map(|(name, persona)| {
                let fp = Fingerprint::extract(facet, &persona.examples);
                (name.clone(), fp)
            })
            .collect();

        let text_fp = Fingerprint::extract(facet, &[text.to_string()]);

        let mut ranked: Vec<(String, f64)> = persona_fps
            .iter()
            .map(|(name, fp)| {
                let score = fp.likelihood(&text_fp);
                (name.clone(), score)
            })
            .collect();

        ranked.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
        });

        let best_name = ranked[0].0.clone();
        let best_score = ranked[0].1;
        let confidence = if ranked.len() > 1 {
            let second = ranked[1].1;
            if best_score > 0.0 {
                (best_score - second) / best_score
            } else {
                0.0
            }
        } else {
            1.0
        };

        Some(MatchResult {
            text: text.to_string(),
            ranked,
            best_name,
            best_score,
            confidence,
        })
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// PersonaComparison — the result of comparing two personas.
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

/// MatchResult — the result of matching unknown text to personas.
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

impl fmt::Display for MatchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  ── style attribution ──")?;
        writeln!(f, "  text: \"{}\"", self.text)?;
        writeln!(f)?;
        writeln!(f, "  ranked matches:")?;
        for (i, (name, score)) in self.ranked.iter().enumerate() {
            let marker = if i == 0 { " ★" } else { "  " };
            writeln!(f, "  {}#{}: {:<16} likelihood {:.4}", marker, i + 1, name, score)?;
        }
        writeln!(f)?;
        writeln!(
            f,
            "  verdict: {} (likelihood {:.4}, confidence {:.1}%)",
            self.best_name,
            self.best_score,
            self.confidence * 100.0,
        )?;
        Ok(())
    }
}

impl fmt::Display for PersonaComparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  ── comparison: {} vs {} ──", self.name_a, self.name_b)?;
        writeln!(f, "  similarity: {:.4}", self.similarity)?;
        writeln!(f, "  top differences:")?;

        for &(sector, diff) in self.differences.iter().take(8) {
            let color = crate::compose::sector_color(sector);
            let direction = if diff > 0.0 { &self.name_a } else { &self.name_b };
            writeln!(
                f,
                "    sector {} ({}): {} is stronger by {:.4}",
                sector, color, direction, diff.abs(),
            )?;
        }

        Ok(())
    }
}
