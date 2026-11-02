//! Tests for the run-start record, the per-corpus shares, and the side-by-side
//! comparison, including Property 54 over the shares summing to the whole.

use super::super::{Candidate, CandidateResult};
use super::*;
use phistd::gen::{for_all, Gen, Seeded, CASES};

/// A measured result with the given numbers.
fn measured_result(top1: f64, perplexity: f64) -> CandidateResult {
    CandidateResult::Measured { top1, perplexity, scored: 4 }
}

#[test]
fn provenance_reports_each_corpus_and_its_share() {
    let corpora = vec![Corpus::wider("simplewiki", 75), Corpus::guide("murphy", 25)];
    let rows = provenance(&corpora);
    assert_eq!(rows, vec![
        ("simplewiki".to_string(), 75, 75.0),
        ("murphy".to_string(), 25, 25.0),
    ]);
    assert_eq!(guide_share(&corpora), 25.0);
}

#[test]
fn provenance_of_all_zero_words_is_zero_not_a_divide_by_zero() {
    let corpora = vec![Corpus::wider("empty", 0), Corpus::guide("murphy", 0)];
    let rows = provenance(&corpora);
    assert_eq!(rows, vec![("empty".to_string(), 0, 0.0), ("murphy".to_string(), 0, 0.0)]);
    assert_eq!(guide_share(&corpora), 0.0);
}

#[test]
fn guide_share_rounds_to_one_decimal() {
    // 1 of 3 words -> 33.333...% -> 33.3.
    let corpora = vec![Corpus::wider("w", 2), Corpus::guide("g", 1)];
    assert_eq!(guide_share(&corpora), 33.3);
}

#[test]
fn run_start_reads_the_seed_epochs_channels_and_vocabulary() {
    let c = Candidate::External {
        corpus: CorpusRef { name: "w".into(), bytes: 10, licence: "CC".into(), on_disk: false },
        epochs: 5,
        seed: 0xABCD,
        channels: 8,
    };
    let start = RunStart::of(&c, 20_000);
    assert_eq!(start, RunStart { seed: 0xABCD, epochs: 5, channels: 8, vocabulary: 20_000 });
    // 8 / 20000 = 0.0004.
    assert_eq!(start.channel_ratio(), Some(0.0004));
}

#[test]
fn run_start_ratio_is_none_on_an_empty_vocabulary() {
    let c = Candidate::Staged { stages: vec![], epochs: 4, seed: 1, channels: 8 };
    let start = RunStart::of(&c, 0);
    assert_eq!(start.channel_ratio(), None);
}

#[test]
fn present_puts_both_numbers_side_by_side_and_registers_nothing() {
    let external = measured_result(26.0, 80.0);
    let staged = measured_result(30.0, 70.0);
    let cmp = present(&external, &staged);
    // Both numbers are carried, unchanged, as a record for the user.
    assert_eq!(cmp.numbers(), (Some((26.0, 80.0)), Some((30.0, 70.0))));
    // present returns a value; it holds no default curriculum and names no
    // winner — the caller waits for the user. The Comparison is pure data.
    assert_eq!(cmp.external, external);
    assert_eq!(cmp.staged, staged);
}

#[test]
fn present_carries_a_non_measured_result_as_no_number() {
    let external = measured_result(30.0, 70.0);
    let staged = CandidateResult::Incomplete { epochs_finished: 2, reason: "cap".into() };
    let cmp = present(&external, &staged);
    assert_eq!(cmp.numbers(), (Some((30.0, 70.0)), None));
}

#[test]
fn a_staged_candidate_honouring_the_order_is_valid() {
    // 4 epochs: wider, wider, guide, guide — wider early, guide-only final 2.
    let stages = vec![
        Stage { epoch: 1, fed: Fed::Wider },
        Stage { epoch: 2, fed: Fed::Wider },
        Stage { epoch: 3, fed: Fed::GuideTrain },
        Stage { epoch: 4, fed: Fed::GuideTrain },
    ];
    let c = Candidate::Staged { stages, epochs: 4, seed: 1, channels: 8 };
    assert!(c.staging_valid());
}

#[test]
fn a_staged_candidate_with_wider_text_in_the_final_two_is_invalid() {
    let stages = vec![
        Stage { epoch: 1, fed: Fed::Wider },
        Stage { epoch: 2, fed: Fed::GuideTrain },
        Stage { epoch: 3, fed: Fed::Wider },
        Stage { epoch: 4, fed: Fed::GuideTrain },
    ];
    let c = Candidate::Staged { stages, epochs: 4, seed: 1, channels: 8 };
    assert!(!c.staging_valid());
}

#[test]
fn a_staged_candidate_under_four_epochs_is_invalid() {
    let stages = vec![
        Stage { epoch: 1, fed: Fed::Wider },
        Stage { epoch: 2, fed: Fed::GuideTrain },
        Stage { epoch: 3, fed: Fed::GuideTrain },
    ];
    let c = Candidate::Staged { stages, epochs: 3, seed: 1, channels: 8 };
    assert!(!c.staging_valid());
}

#[test]
fn an_external_candidate_is_vacuously_staging_valid() {
    let c = Candidate::External {
        corpus: CorpusRef { name: "w".into(), bytes: 1, licence: "CC".into(), on_disk: true },
        epochs: 4,
        seed: 1,
        channels: 8,
    };
    assert!(c.staging_valid());
}

/// A set of per-corpus word counts over 1 to 8 corpora, including the single
/// corpus and the zero-word corpus the design's generator names.
#[derive(Debug, Clone)]
struct Counts {
    words: Vec<usize>,
    guide: usize,
}

impl Gen for Counts {
    fn draw(g: &mut Seeded) -> Self {
        let n = 1 + g.below(8) as usize; // 1 to 8 corpora
        let words: Vec<usize> = (0..n)
            .map(|i| {
                // Deliberately include a zero-word corpus now and then.
                if i == 0 && g.below(4) == 0 { 0 } else { g.below(5000) as usize }
            })
            .collect();
        let guide = g.below(n as u64) as usize; // which corpus is the guide
        Counts { words, guide }
    }
}

/// Property 54: Corpus shares sum to the whole.
///
/// For any set of per-corpus word counts, the recorded shares sum to 100.0
/// within rounding (whenever any words were taken) and the guide's recorded
/// share equals its word count over the total, both to one decimal place.
///
/// **Validates: Requirements 4.9**
#[test]
fn property_54_corpus_shares_sum_to_the_whole() {
    let result = for_all::<Counts>(0x5A4E_0054, CASES, |c| {
        let corpora: Vec<Corpus> = c
            .words
            .iter()
            .enumerate()
            .map(|(i, &w)| {
                if i == c.guide {
                    Corpus::guide(&format!("corpus{i}"), w)
                } else {
                    Corpus::wider(&format!("corpus{i}"), w)
                }
            })
            .collect();

        let total: usize = c.words.iter().sum();
        let rows = provenance(&corpora);

        // Every row's word count is carried through unchanged.
        for (row, &w) in rows.iter().zip(c.words.iter()) {
            if row.1 != w {
                return false;
            }
        }

        if total == 0 {
            // Every share is 0.0, and so is the guide's.
            return rows.iter().all(|r| r.2 == 0.0) && guide_share(&corpora) == 0.0;
        }

        // The shares sum to 100.0 within one-decimal rounding slack. Each of
        // up to 8 shares is rounded independently, so the slack is 8 * 0.05.
        let sum: f64 = rows.iter().map(|r| r.2).sum();
        if (sum - 100.0).abs() > 0.05 * c.words.len() as f64 + 1e-9 {
            return false;
        }

        // The guide's share equals its words over the total, to one decimal.
        let guide_words = c.words[c.guide];
        let want = (100.0 * guide_words as f64 / total as f64 * 10.0).round() / 10.0;
        (guide_share(&corpora) - want).abs() < 1e-9
    });
    assert!(result.is_ok(), "{}", result.unwrap_err());
}
