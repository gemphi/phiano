/// Personality trait derivation from fingerprint sector distribution.
///
/// Maps the phase-space color distribution to temperament descriptors.
/// This is NOT hardcoded persona content - it's derived purely from
/// the geometry of the fingerprint in phase space.

use super::Fingerprint;

pub fn personality_traits(fp: &Fingerprint) -> Vec<String> {
    let dominant = fp.dominant_sectors(5);
    let mut traits = Vec::new();

    let mut warm = 0.0f64;
    let mut cool = 0.0f64;
    let mut green = 0.0f64;
    let mut total = 0.0f64;

    for &(sector, weight) in &dominant {
        let color = crate::compose::sector_color(sector);
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

        if warm_frac > 0.4 { traits.push("passionate".to_string()); }
        if cool_frac > 0.4 { traits.push("contemplative".to_string()); }
        if green_frac > 0.3 { traits.push("balanced".to_string()); }
        if warm_frac > 0.3 && cool_frac > 0.3 { traits.push("dynamic".to_string()); }
        if fp.diversity > 3.5 {
            traits.push("versatile".to_string());
        } else if fp.diversity < 2.5 {
            traits.push("focused".to_string());
        }
        if fp.avg_length > 12.0 {
            traits.push("elaborate".to_string());
        } else if fp.avg_length < 7.0 {
            traits.push("concise".to_string());
        }
    }

    if traits.is_empty() {
        traits.push("enigmatic".to_string());
    }

    traits
}
