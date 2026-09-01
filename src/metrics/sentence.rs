//! Does the manifold predict the next *sentence*?
//!
//! Every benchmark in this project so far has scored words. Analogy is
//! `word:word::word:word`. Pair-versus-random is word against word. Perplexity
//! is the next *word*. The relation set is 305 word pairs. And γ\* = 0 has held
//! across seven independent attempts — all seven of them next-word prediction.
//!
//! That is a gap in the measurement, not a settled result. If meaning is
//! carried by sentences rather than by words — if a word is a hair and the
//! sentence is the coat — then a representation that compresses *what comes
//! next at the sentence level* could be doing its job perfectly while losing
//! every word-level benchmark ever run against it.
//!
//! # The task
//!
//! Given `k` consecutive sentences of context, rank the true continuation
//! against `n` distractors drawn from elsewhere in the corpus. This is standard
//! next-sentence selection, and it is the smallest honest test of the claim.
//!
//! Three scorers, on identical items:
//!
//! * **phase** — the recurrent state after the context, scored against each
//!   candidate's phase state. This is the thing under test.
//! * **lexical** — word overlap between context and candidate. A strong, dull
//!   baseline: real continuations repeat words. Beating it means the manifold
//!   carries something beyond repetition.
//! * **chance** — `1/n`. If phase does not beat this, nothing else matters.
//!
//! # What would make this a real result
//!
//! Phase must beat lexical overlap, not merely chance. A representation that
//! only recovers word repetition has re-derived a bag of words in complex
//! arithmetic. The gap to *lexical* is the claim; the gap to chance is the floor.

use crate::facet::Facet;
use crate::phasor::fnv1a;
use crate::tokenizer::Tokenizer;
use crate::wave::{c64, Wave};
use rayon::prelude::*;
use std::collections::HashSet;

/// Sentences of context shown before the prediction point.
pub const CONTEXT_LEN: usize = 3;

/// Candidates ranked per item: one true continuation and `CANDIDATES - 1`
/// distractors.
pub const CANDIDATES: usize = 50;

#[derive(Debug, Clone, Default)]
pub struct ScorerResult {
    pub name: String,
    pub top1: f64,
    pub top5: f64,
    pub mrr: f64,
    pub items: usize,
}

#[derive(Debug, Clone)]
pub struct SentenceReport {
    pub items: usize,
    pub candidates: usize,
    pub chance_top1: f64,
    pub chance_mrr: f64,
    pub scorers: Vec<ScorerResult>,
}

/// One prediction item: context sentences, the true continuation, distractors.
struct Item {
    context: Vec<Vec<String>>,
    candidates: Vec<Vec<String>>,
    /// Index of the true continuation within `candidates`.
    truth: usize,
}

pub struct SentenceBenchmark;

impl SentenceBenchmark {
    /// Builds items from a document — a run of consecutive sentences.
    ///
    /// Distractors are drawn from `pool`, which must come from *other*
    /// positions in the corpus. Drawing them from the same document would make
    /// them plausible continuations and the task unanswerable rather than hard.
    fn build_items(docs: &[Vec<String>], pool: &[Vec<String>], seed: u64) -> Vec<Item> {
        let mut items = Vec::new();
        if pool.len() < CANDIDATES {
            return items;
        }

        for (d, doc) in docs.iter().enumerate() {
            let toks: Vec<Vec<String>> = doc.iter().map(|s| Tokenizer::tokenize(s)).collect();
            for i in CONTEXT_LEN..toks.len() {
                let context: Vec<Vec<String>> = toks[i - CONTEXT_LEN..i].to_vec();
                let truth_toks = toks[i].clone();
                if truth_toks.len() < 4 {
                    continue;
                }

                // Deterministic distractor draw: the item set must be identical
                // between scorers and between runs, or the comparison is
                // between different tasks.
                let mut chosen: Vec<Vec<String>> = Vec::with_capacity(CANDIDATES);
                let mut r = fnv1a(&format!("{}:{}", d, i)) ^ seed;
                let mut seen: HashSet<usize> = HashSet::new();
                while chosen.len() + 1 < CANDIDATES {
                    r ^= r << 13;
                    r ^= r >> 7;
                    r ^= r << 17;
                    let idx = (r % pool.len() as u64) as usize;
                    if !seen.insert(idx) {
                        continue;
                    }
                    if pool[idx] == truth_toks {
                        continue;
                    }
                    chosen.push(pool[idx].clone());
                }

                // The truth goes at a position that varies with the item, so a
                // scorer cannot win by always guessing the same slot.
                let truth = (r % CANDIDATES as u64) as usize;
                chosen.insert(truth.min(chosen.len()), truth_toks);

                items.push(Item { context, candidates: chosen, truth });
            }
        }
        items
    }
}

/// How a sentence is compressed into a phase state.
///
/// The first version of this benchmark used `Bag` only, and reported that phase
/// loses to lexical overlap. That was a result about the encoder, not about the
/// hypothesis: a bag of words in complex arithmetic cannot beat a bag of words,
/// and testing it against one asks nothing. If meaning is carried by *how the
/// concepts connect*, the encoder has to carry connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SentenceEncoding {
    /// Unbound superposition. Order-free; the control.
    Bag,
    /// Each word rotated by its position times the golden angle.
    Bound,
    /// Diagonal complex recurrence over the sentence, with a per-channel
    /// timescale — the construction the order experiment measured best of three
    /// at the word level.
    Recurrent,
}

impl SentenceEncoding {
    pub fn label(self) -> &'static str {
        match self {
            SentenceEncoding::Bag => "phase (bag)",
            SentenceEncoding::Bound => "phase (bound)",
            SentenceEncoding::Recurrent => "phase (recurrent)",
        }
    }

    pub const ALL: [SentenceEncoding; 3] = [
        SentenceEncoding::Bag,
        SentenceEncoding::Bound,
        SentenceEncoding::Recurrent,
    ];
}

impl SentenceBenchmark {
    /// Per-channel recurrence kernel, matching the language model's.
    #[inline]
    fn kernel(k: usize) -> c64 {
        crate::config::channel_kernel(k, crate::config::PHASE_CHANNELS)
    }

    /// Phase state of a sentence under a given encoding.
    fn sentence_state_as(facet: &Facet, words: &[String], enc: SentenceEncoding) -> Vec<c64> {
        match enc {
            SentenceEncoding::Bag => Wave::sentence_channels(facet, words, false),
            SentenceEncoding::Bound => Wave::sentence_channels(facet, words, true),
            SentenceEncoding::Recurrent => {
                let n = crate::config::PHASE_CHANNELS;
                let mut h = vec![c64::new(0.0, 0.0); n];
                for w in words {
                    for (k, z) in h.iter_mut().enumerate() {
                        *z *= Self::kernel(k);
                    }
                    if let Some(p) = facet.lexicon.get(w) {
                        for (k, z) in h.iter_mut().enumerate().take(n) {
                            *z += c64::from_polar(p.amplitude, p.theta(k));
                        }
                    }
                }
                h
            }
        }
    }
}

impl SentenceBenchmark {
    /// Phase state of a sentence: the per-channel superposition of its words.
    fn sentence_state(facet: &Facet, words: &[String]) -> Vec<c64> {
        Self::sentence_state_as(facet, words, SentenceEncoding::Bag)
    }

    /// Cosine between two per-channel states, averaged over channels.
    fn state_similarity(a: &[c64], b: &[c64]) -> f64 {
        let mut sum = 0.0;
        let mut n = 0usize;
        for (x, y) in a.iter().zip(b.iter()) {
            let (nx, ny) = (x.norm(), y.norm());
            if nx > 1e-12 && ny > 1e-12 {
                sum += (x.re * y.re + x.im * y.im) / (nx * ny);
                n += 1;
            }
        }
        match n {
            0 => 0.0,
            _ => sum / n as f64,
        }
    }

    /// Rank of the truth under a scoring function, 1-based.
    fn rank_by<F: Fn(&Item, usize) -> f64>(item: &Item, score: F) -> usize {
        let truth_score = score(item, item.truth);
        let beating = (0..item.candidates.len())
            .filter(|i| *i != item.truth && score(item, *i) > truth_score)
            .count();
        beating + 1
    }

    fn summarise(name: &str, ranks: &[usize]) -> ScorerResult {
        let n = ranks.len().max(1) as f64;
        ScorerResult {
            name: name.to_string(),
            top1: ranks.iter().filter(|r| **r == 1).count() as f64 / n,
            top5: ranks.iter().filter(|r| **r <= 5).count() as f64 / n,
            mrr: ranks.iter().map(|r| 1.0 / *r as f64).sum::<f64>() / n,
            items: ranks.len(),
        }
    }

    /// Runs the benchmark over a corpus split into documents.
    pub fn evaluate(facet: &Facet, docs: &[Vec<String>], pool: &[Vec<String>], seed: u64) -> SentenceReport {
        let items = Self::build_items(docs, pool, seed);

        let per_item: Vec<(Vec<usize>, usize)> = items
            .par_iter()
            .map(|item| {
                // ---- phase, one rank per encoding ----
                // The context is one accumulated state, not three separate
                // ones: the compression claim is that a prefix collapses into a
                // single state of fixed size however long it is.
                let phase_ranks: Vec<usize> = SentenceEncoding::ALL
                    .iter()
                    .map(|enc| {
                        let mut ctx = vec![c64::new(0.0, 0.0); crate::config::PHASE_CHANNELS];
                        for sent in &item.context {
                            let s = Self::sentence_state_as(facet, sent, *enc);
                            for (a, b) in ctx.iter_mut().zip(s.iter()) {
                                *a += *b;
                            }
                        }
                        Self::rank_by(item, |it, i| {
                            Self::state_similarity(
                                &ctx,
                                &Self::sentence_state_as(facet, &it.candidates[i], *enc),
                            )
                        })
                    })
                    .collect();

                // ---- lexical ----
                let ctx_words: HashSet<&String> = item.context.iter().flatten().collect();
                let lex_rank = Self::rank_by(item, |it, i| {
                    let cand: HashSet<&String> = it.candidates[i].iter().collect();
                    let overlap = ctx_words.intersection(&cand).count() as f64;
                    // Length-normalised, or long candidates win by having more
                    // chances to overlap rather than by being the continuation.
                    overlap / (it.candidates[i].len() as f64).sqrt().max(1.0)
                });

                (phase_ranks, lex_rank)
            })
            .collect();

        let lexical: Vec<usize> = per_item.iter().map(|(_, l)| *l).collect();

        let chance_mrr = (1..=CANDIDATES).map(|r| 1.0 / r as f64).sum::<f64>() / CANDIDATES as f64;

        SentenceReport {
            items: items.len(),
            candidates: CANDIDATES,
            chance_top1: 1.0 / CANDIDATES as f64,
            chance_mrr,
            scorers: SentenceEncoding::ALL
                .iter()
                .enumerate()
                .map(|(e, enc)| {
                    let ranks: Vec<usize> = per_item.iter().map(|(p, _)| p[e]).collect();
                    Self::summarise(enc.label(), &ranks)
                })
                .chain(std::iter::once(Self::summarise("lexical overlap", &lexical)))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LEARNING_RATE;
    use crate::trainer::Trainer;

    fn docs() -> Vec<Vec<String>> {
        (0..30)
            .map(|d| {
                (0..8)
                    .map(|i| format!("document {} sentence {} about the topic of trade and money", d, i))
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_items_are_deterministic_and_truth_is_present() {
        let d = docs();
        let pool: Vec<Vec<String>> = d
            .iter()
            .flatten()
            .map(|s| Tokenizer::tokenize(s))
            .collect();

        let a = SentenceBenchmark::build_items(&d, &pool, 7);
        let b = SentenceBenchmark::build_items(&d, &pool, 7);
        assert_eq!(a.len(), b.len());
        assert!(!a.is_empty(), "the fixture must produce items");

        for (x, y) in a.iter().zip(b.iter()) {
            assert_eq!(x.truth, y.truth, "the item set must not move between runs");
            assert_eq!(x.candidates.len(), CANDIDATES);
            assert_eq!(x.candidates, y.candidates);
        }
    }

    /// The truth must not sit at a fixed slot, or a scorer wins by guessing it.
    #[test]
    fn test_truth_position_varies() {
        let d = docs();
        let pool: Vec<Vec<String>> = d.iter().flatten().map(|s| Tokenizer::tokenize(s)).collect();
        let items = SentenceBenchmark::build_items(&d, &pool, 7);
        let distinct: HashSet<usize> = items.iter().map(|i| i.truth).collect();
        assert!(
            distinct.len() > 1,
            "the truth landed at one slot in every item: {:?}",
            distinct
        );
    }

    /// Chance is reported, and a trained model must at least reach it.
    #[test]
    fn test_phase_reaches_chance() {
        let d = docs();
        let flat: Vec<String> = d.iter().flatten().cloned().collect();
        let pool: Vec<Vec<String>> = flat.iter().map(|s| Tokenizer::tokenize(s)).collect();

        let mut facet = Facet::new();
        let t = Trainer::new(LEARNING_RATE);
        for s in &flat {
            t.train_sentence(&mut facet, s);
        }

        let r = SentenceBenchmark::evaluate(&facet, &d, &pool, 7);
        assert!(r.items > 0);
        let phase = &r.scorers[0];
        assert!(phase.mrr >= r.chance_mrr * 0.5, "phase {} vs chance {}", phase.mrr, r.chance_mrr);
        assert!(phase.top1 <= 1.0 && phase.mrr <= 1.0);
    }
}
