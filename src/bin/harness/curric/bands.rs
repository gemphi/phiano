//! The frequency-band diagnostic (spec phinet-system-completion, 6.1). Each
//! held-out token is grouped by how often it occurred in the training
//! sentences, and the learner's and the trigram's mean log-loss is reported
//! per group, with the share of held-out tokens the group holds. It says which
//! band carries the perplexity gap before any change is tried.

use phient::metrics::baseline::kn::KneserNey;
use phient::metrics::harness::Harness;
use phiecs::phasor::facet::Facet;
use phiton::p64::words::Tokenizer;
use std::collections::HashMap;

/// Band labels, in order: unseen in training, then by training count.
pub const LABELS: [&str; 5] = ["unseen", "1", "2-4", "5-19", "20+"];

/// The band of a token seen `count` times in training.
pub fn band_of(count: usize) -> usize {
    match count {
        0 => 0,
        1 => 1,
        2..=4 => 2,
        5..=19 => 3,
        _ => 4,
    }
}

/// One band's totals over the held-out tokens.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Band {
    pub tokens: usize,
    pub learner_loss: f64,
    pub trigram_loss: f64,
}

/// Group `(token, learner ln p, trigram ln p)` rows by the token's training
/// count. Losses are summed negative log-probabilities (nats).
pub fn group(counts: &HashMap<String, usize>, rows: &[(String, f64, f64)]) -> [Band; 5] {
    let mut bands = [Band::default(); 5];
    for (token, learner, trigram) in rows {
        let b = &mut bands[band_of(counts.get(token).copied().unwrap_or(0))];
        b.tokens += 1;
        b.learner_loss -= learner;
        b.trigram_loss -= trigram;
    }
    bands
}

/// One printable line per band: tokens, share of all held-out tokens, mean
/// log-loss for each model, and the band's share of the learner-minus-trigram
/// loss gap (negative where the learner is ahead).
pub fn report(bands: &[Band; 5]) -> Vec<String> {
    let total: usize = bands.iter().map(|b| b.tokens).sum();
    let gap: f64 = bands.iter().map(|b| b.learner_loss - b.trigram_loss).sum();
    bands
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let n = b.tokens.max(1) as f64;
            format!(
                "harness: curriculum band {:>6}: {:>6} tokens ({:>5.1}% of test) learner {:.3} trigram {:.3} nats; {:>6.1}% of the gap",
                LABELS[i],
                b.tokens,
                100.0 * b.tokens as f64 / total.max(1) as f64,
                b.learner_loss / n,
                b.trigram_loss / n,
                if gap == 0.0 { 0.0 } else { 100.0 * (b.learner_loss - b.trigram_loss) / gap },
            )
        })
        .collect()
}

/// Score the test tokens with both models and print the per-band diagnostic.
/// Both are scored on the same positions: token 2 onward of each sentence,
/// the ones the learner's perplexity averages.
pub fn print(facet: &Facet, train: &[String], test: &[String]) {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for s in train {
        for t in Tokenizer::tokenize(s) {
            *counts.entry(t).or_default() += 1;
        }
    }
    let kn = KneserNey::train(train);
    let lm = Harness::language_model(facet, true);
    let learner = lm.scored_tokens(test);
    // `scored_tokens` walks the sentences in order, skipping the first two
    // tokens of each; walk the trigram over the same positions.
    let mut rows = Vec::with_capacity(learner.len());
    let mut scored = learner.into_iter();
    for s in test {
        let toks = Tokenizer::tokenize(s);
        for w in toks.windows(3) {
            if let Some((token, learner)) = scored.next() {
                rows.push((token, learner, kn.probability(&w[0], &w[1], &w[2]).ln()));
            }
        }
    }
    for line in report(&group(&counts, &rows)) {
        eprintln!("{line}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_fall_in_the_declared_bands() {
        let bands: Vec<usize> = [0, 1, 2, 4, 5, 19, 20, 500].iter().map(|c| band_of(*c)).collect();
        assert_eq!(bands, vec![0, 1, 2, 2, 3, 3, 4, 4]);
    }

    #[test]
    fn grouping_sums_losses_by_training_count() {
        let counts: HashMap<String, usize> =
            [("the".to_string(), 50), ("cat".to_string(), 1)].into_iter().collect();
        let rows = vec![
            ("the".to_string(), -1.0, -0.5),
            ("cat".to_string(), -4.0, -3.0),
            ("zebra".to_string(), -8.0, -6.0),
            ("the".to_string(), -3.0, -1.5),
        ];
        let bands = group(&counts, &rows);
        assert_eq!(bands[4], Band { tokens: 2, learner_loss: 4.0, trigram_loss: 2.0 });
        assert_eq!(bands[1], Band { tokens: 1, learner_loss: 4.0, trigram_loss: 3.0 });
        assert_eq!(bands[0], Band { tokens: 1, learner_loss: 8.0, trigram_loss: 6.0 });
        assert_eq!(report(&bands).len(), 5);
    }
}
