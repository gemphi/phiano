//! Personality trait derivation from fingerprint sector distribution.
//!
//! Maps the phase-space color distribution to temperament descriptors.
//! This is NOT hardcoded persona content - it is derived purely from
//! the geometry of the fingerprint in phase space. All methods are encapsulated
//! in [`PersonalityMapper`], following the Diem convention that all public symbols
//! belong to named types.

use super::Fingerprint;

/// Personality trait mapper for translating phase-space fingerprints into temperaments.
pub struct PersonalityMapper;

impl PersonalityMapper {
    /// Derives temperament descriptors from the dominant sectors of a [`Fingerprint`].
    pub fn personality_traits(fp: &Fingerprint) -> Vec<String> {
        let dominant = fp.dominant_sectors(5);
        let mut traits = Vec::new();

        let mut warm = 0.0f64;
        let mut cool = 0.0f64;
        let mut green = 0.0f64;
        let mut total = 0.0f64;

        for &(sector, weight) in &dominant {
            let color = crate::compose::SectorPalette::color(sector);
            match color.as_str() {
                "crimson" | "red" | "scarlet" | "orange" | "amber" | "gold" | "rose" => {
                    warm += weight;
                }
                "yellow" | "lime" => {
                    green += weight;
                }
                "green" | "emerald" | "teal" => {
                    green += weight * 0.7;
                }
                "blue" | "indigo" | "violet" | "magenta" => {
                    cool += weight;
                }
                _ => {}
            }
            total += weight;
        }

        if total > 0.0 {
            let warm_frac = warm / total;
            let cool_frac = cool / total;
            let green_frac = green / total;

            if warm_frac > 0.4 {
                traits.push("passionate".to_string());
                traits.push("direct".to_string());
            }
            if cool_frac > 0.4 {
                traits.push("analytical".to_string());
                traits.push("contemplative".to_string());
            }
            if green_frac > 0.3 {
                traits.push("balanced".to_string());
                traits.push("adaptive".to_string());
            }
            if dominant.len() >= 4 {
                traits.push("eclectic".to_string());
            }
        }

        if traits.is_empty() {
            traits.push("neutral".to_string());
        }

        traits
    }
}
