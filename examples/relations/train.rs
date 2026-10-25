#[allow(unused_imports)]
use super::*;

/// Train the same data under different objectives.
///
/// The mixing sweep in RESULTS §3 found that training the manifold on
/// next-word ranking rather than centroid attraction moved its recovered
/// predictive signal from 0.9% to 24.3% — 27x, from the objective alone. If
/// relational structure is going to appear anywhere, it should appear under
/// the objective that already demonstrated it carries more information.
pub(super) fn train_regimes(selected: &[(String, String)], epochs: usize) -> Vec<(String, Facet)> {
    let trainer = Trainer::new(LEARNING_RATE);
    let mut trained: Vec<(String, Facet)> = Vec::new();

    for regime in ["co-occurrence", "ranking", "both"] {
        let mut facet = Facet::new();
        let start = std::time::Instant::now();

        for _ in 0..epochs {
            for (word, def) in selected {
                let text = format!("{} {}", word, def);
                match regime {
                    "co-occurrence" => {
                        trainer.train_sentence(&mut facet, &text);
                    }
                    "ranking" => {
                        // One structural pass to populate the lexicon and the
                        // n-gram tables, then the ranking objective alone.
                        let seed = Trainer { learning_rate: 0.0, neg_samples: 0, definitions: None, seed: 0, relations: false, rounding: Default::default() };
                        seed.train_sentence(&mut facet, &text);
                        trainer.train_predictive(&mut facet, &text);
                        trainer.train_predictive(&mut facet, &text);
                    }
                    _ => {
                        trainer.train_full(&mut facet, &text);
                    }
                }
            }
        }

        println!(
            "  [{}] {:?}  vocabulary {}  dispersion {:.4}",
            regime,
            start.elapsed(),
            facet.vocabulary_size(),
            facet.phase_dispersion()
        );
        trained.push((regime.to_string(), facet));
    }

    trained
}
