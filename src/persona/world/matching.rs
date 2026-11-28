#[allow(unused_imports)]
use super::*;

impl Styles {
    /// Adds a persona without printing (the library path; `add_persona`
    /// announces itself, which a loader should not).
    pub fn add_persona_quiet(&mut self, name: &str, examples: &[String], facet: &mut Facet, trainer: &Trainer) {
        let persona = Style::from_examples(name, examples, facet, trainer);
        self.personas.insert(name.to_string(), persona);
    }

    /// Adds a persona by reading the lexicon, without training it — for a
    /// lexicon that already carries this persona's ingestion.
    pub fn add_persona_read(&mut self, name: &str, examples: &[String], facet: &Facet) {
        let fingerprint = Fingerprint::extract(facet, examples);
        self.personas.insert(name.to_string(), Style { name: name.to_string(), fingerprint, examples: examples.to_vec() });
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

    /// [`Styles::match_text`] against fingerprints already extracted.
    pub fn match_text_with(&self, facet: &Facet, text: &str, fps: &[(String, Fingerprint)]) -> Option<MatchResult> {
        if fps.is_empty() {
            return None;
        }
        let text_fp = Fingerprint::extract(facet, &[text.to_string()]);
        let mut ranked: Vec<(String, f64)> = fps.iter().map(|(name, fp)| (name.clone(), fp.likelihood(&text_fp))).collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let (best_name, best_score) = (ranked[0].0.clone(), ranked[0].1);
        Some(MatchResult { text: text.to_string(), confidence: confidence(&ranked), ranked, best_name, best_score })
    }
}

/// How far the best score stands above the next, as a share of itself.
///
/// One persona has no competitor, so it is matched with full confidence; a
/// best score at or under zero matched nothing, and claiming confidence in
/// a non-match is what the two spellings of the matcher used to disagree
/// about - `match_text` answered 0.0 here where `match_text_with` answered
/// 1.0, and 0.0 is the right one.
fn confidence(ranked: &[(String, f64)]) -> f64 {
    let best = match ranked.first() {
        Some((_, score)) => *score,
        None => return 0.0,
    };
    match (ranked.len() > 1, best > 0.0) {
        (false, _) => 1.0,
        (true, false) => 0.0,
        (true, true) => (best - ranked[1].1) / best,
    }
}

#[cfg(test)]
mod test {
    use super::confidence;

    fn ranked(scores: &[f64]) -> Vec<(String, f64)> {
        scores.iter().enumerate().map(|(i, score)| (format!("persona{i}"), *score)).collect()
    }

    #[test]
    fn one_persona_has_no_competitor() {
        assert_eq!(confidence(&ranked(&[0.4])), 1.0);
    }

    #[test]
    fn nothing_matched_is_no_confidence() {
        assert_eq!(confidence(&ranked(&[0.0, -0.2])), 0.0);
        assert_eq!(confidence(&ranked(&[-0.1, -0.5])), 0.0);
    }

    #[test]
    fn no_personas_is_no_confidence() {
        assert_eq!(confidence(&ranked(&[])), 0.0);
    }

    #[test]
    fn the_best_above_the_next_as_a_share_of_itself() {
        assert_eq!(confidence(&ranked(&[1.0, 0.25])), 0.75);
    }
}
