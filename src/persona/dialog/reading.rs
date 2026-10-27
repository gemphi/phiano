//! Reading the question: which condition it names, and what that condition
//! calls for in this persona's manifold.
#[allow(unused_imports)]
use super::*;

impl Dialogue {
    /// The persona's most-used content words that the lexicon knows.
    pub(super) fn answer_pool(facet: &Facet, examples: &[String]) -> Vec<String> {
        let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for e in examples {
            for w in Tokenizer::tokenize(e) {
                if !Tokenizer::is_function_word(&w) && w.len() > 2 && facet.lexicon.contains_key(&w) {
                    *counts.entry(w).or_insert(0) += 1;
                }
            }
        }
        let mut v: Vec<(String, usize)> = counts.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        v.into_iter().take(DIALOG_POOL).map(|(w, _)| w).collect()
    }

    /// The word of the user's turn the persona answers under: the most
    /// specific content word it knows — the rarest, because a word the corpus
    /// uses everywhere names no condition in particular.
    pub(super) fn topic(&self, facet: &Facet, turn: &str) -> Option<String> {
        let mut best: Option<(String, f64)> = None;
        for w in Tokenizer::tokenize(turn) {
            if Tokenizer::is_function_word(&w) || w.len() <= 2 {
                continue;
            }
            let Some(p) = facet.lexicon.get(&w) else { continue };
            let count = p.evidence_total().max(1.0);
            if best.as_ref().is_none_or(|(_, c)| count < *c) {
                best = Some((w, count));
            }
        }
        best.map(|(w, _)| w)
    }

    /// What the topic calls for in this persona's manifold: the pool word
    /// with the strongest directed reading from the topic (X ⇒ Y).
    pub(super) fn calls_for(&self, facet: &Facet, topic: &str, said: &[String]) -> Option<String> {
        let mut best: Option<(String, f64)> = None;
        for w in &self.pool {
            if w == topic || said.iter().any(|s| s == w) {
                continue;
            }
            let score = facet.calls_for_with(w, topic, phiecs::phasor::facet::Reading::from_env());
            if best.as_ref().is_none_or(|(_, s)| score > *s) {
                best = Some((w.clone(), score));
            }
        }
        best.filter(|(_, s)| *s > 0.0).map(|(w, _)| w)
    }

}
