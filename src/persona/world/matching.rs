#[allow(unused_imports)]
use super::*;

impl World {
    /// Adds a persona without printing (the library path; `add_persona`
    /// announces itself, which a loader should not).
    pub fn add_persona_quiet(&mut self, name: &str, examples: &[String], facet: &mut Facet, trainer: &Trainer) {
        let persona = Persona::from_examples(name, examples, facet, trainer);
        self.personas.insert(name.to_string(), persona);
    }

    /// Adds a persona by reading the lexicon, without training it — for a
    /// lexicon that already carries this persona's ingestion.
    pub fn add_persona_read(&mut self, name: &str, examples: &[String], facet: &Facet) {
        let fingerprint = Fingerprint::extract(facet, examples);
        self.personas.insert(name.to_string(), Persona { name: name.to_string(), fingerprint, examples: examples.to_vec() });
    }

    /// Every persona's fingerprint against the current lexicon, computed
    /// once. `match_text` re-extracts these on every call, which is fair but
    /// costs one pass over every persona's whole corpus per question — too
    /// much for a dialogue or a long evaluation.
    pub fn fingerprints(&self, facet: &Facet) -> Vec<(String, Fingerprint)> {
        let mut v: Vec<(String, Fingerprint)> = self.personas.iter().map(|(name, p)| (name.clone(), Fingerprint::extract(facet, &p.examples))).collect();
        v.sort_by(|a, b| a.0.cmp(&b.0));
        v
    }

    /// [`World::match_text`] against fingerprints already extracted.
    pub fn match_text_with(&self, facet: &Facet, text: &str, fps: &[(String, Fingerprint)]) -> Option<MatchResult> {
        if fps.is_empty() {
            return None;
        }
        let text_fp = Fingerprint::extract(facet, &[text.to_string()]);
        let mut ranked: Vec<(String, f64)> = fps.iter().map(|(name, fp)| (name.clone(), fp.likelihood(&text_fp))).collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let (best_name, best_score) = (ranked[0].0.clone(), ranked[0].1);
        let confidence = match ranked.len() > 1 && best_score > 0.0 {
            true => (best_score - ranked[1].1) / best_score,
            false => 1.0,
        };
        Some(MatchResult { text: text.to_string(), ranked, best_name, best_score, confidence })
    }
}
