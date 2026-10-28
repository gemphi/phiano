#[allow(unused_imports)]
use super::*;

use phient::metrics::baseline::kn::KneserNey;

/// A panel of judges, one per persona: a Kneser-Ney trigram model trained on
/// that writer's own sentences. The reply is read by all of them and the one
/// least surprised by it wins.
///
/// The sector-histogram reader (`Styles::match_text`) is the engine's own
/// attribution and it leans toward the writer with the most text (docs/79
/// §3); a per-writer language model is the standard outside check, and the
/// two disagreeing is worth seeing.
pub(crate) struct Panel {
    models: Vec<(String, KneserNey)>,
}

impl Panel {
    pub fn train(world: &Styles) -> Self {
        let mut names: Vec<&String> = world.personas.keys().collect();
        names.sort();
        let models = names.into_iter().map(|n| (n.clone(), KneserNey::train(&world.personas[n].examples))).collect();
        Self { models }
    }

    /// The persona least surprised by `text`, and by how much: the ratio of
    /// the runner-up's perplexity to the winner's (1.0 = a tie).
    pub fn read(&self, text: &str) -> Option<(String, f64, f64)> {
        let mut scored: Vec<(String, f64)> = self.models.iter().map(|(n, kn)| (n.clone(), kn.perplexity(&[text.to_string()]))).collect();
        scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        let (best, ppl) = scored.first().cloned()?;
        let margin = scored.get(1).map_or(1.0, |(_, p)| p / ppl.max(1e-9));
        Some((best, ppl, margin))
    }
}
