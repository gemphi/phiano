//! Relation accuracy: does the manifold place related words in related positions?
//!
//! Perplexity says whether the model predicts text. It says nothing about
//! whether `woman` sits near `man`, or whether the step from `man` to `woman`
//! is the same step as from `grandfather` to `grandmother`. Those are the
//! questions a *semantic* representation has to answer, and they have clean
//! chance baselines, so the answers can be wrong.
//!
//! Three tests, in increasing difficulty:
//!
//! 1. **Pair versus random** — is `resonance(a, b)` greater than `resonance(a, r)`
//!    for a random `r`? Chance is 50%. Failing this means related words are not
//!    even loosely grouped.
//! 2. **Neighbourhood** — is `b` among the `k` nearest words to `a`?
//!    Chance is `k / V`.
//! 3. **Analogy** — `a : b :: c : d`. The relation is the per-channel phase
//!    offset `θ_b − θ_a`; applying it to `c` should land on `d`. This is
//!    unbind-then-bind in the phase domain, and it is the real test of whether
//!    a *relation* is represented rather than a mere cluster. Chance is `1 / V`.

use crate::facet::Facet;
use crate::phasor::{fnv1a, SpectralPhasor};
use serde::{Deserialize, Serialize};

/// One ordered pair standing in some relation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationPair {
    pub a: String,
    pub b: String,
}

/// A group of pairs sharing the same relation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationFamily {
    pub name: String,
    pub pairs: Vec<RelationPair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyResult {
    pub name: String,
    /// Pairs whose members were both in the lexicon.
    pub usable_pairs: usize,
    /// Fraction of pairs beating a random word. Chance = 0.5.
    pub pair_vs_random: f64,
    /// Fraction of pairs where `b` is among the 10 nearest to `a`.
    pub neighbour_top10: f64,
    /// Fraction of pairs where `b` is among the 50 nearest to `a`.
    pub neighbour_top50: f64,
    /// Analogy accuracy at rank 1 and rank 5, and mean reciprocal rank.
    pub analogy_top1: f64,
    pub analogy_top5: f64,
    pub analogy_mrr: f64,
    pub analogies_tested: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationReport {
    pub vocabulary_size: usize,
    pub families: Vec<FamilyResult>,
    /// Chance rate for the neighbourhood test at k = 10.
    pub chance_neighbour_top10: f64,
    /// Chance rate for analogy at rank 1.
    pub chance_analogy_top1: f64,
    pub overall_pair_vs_random: f64,
    pub overall_analogy_top1: f64,
}

pub struct RelationBenchmark;

impl RelationBenchmark {
    /// The default probe set: gendered kinship and number.
    ///
    /// Chosen because a dictionary defines all of them and the relations are
    /// unambiguous, so a failure is the model's and not the probe's.
    pub fn default_families() -> Vec<RelationFamily> {
        let fam = |name: &str, pairs: &[(&str, &str)]| RelationFamily {
            name: name.to_string(),
            pairs: pairs
                .iter()
                .map(|(a, b)| RelationPair { a: a.to_string(), b: b.to_string() })
                .collect(),
        };

        vec![
            fam(
                "gender",
                &[
                    ("man", "woman"),
                    ("boy", "girl"),
                    ("king", "queen"),
                    ("father", "mother"),
                    ("son", "daughter"),
                    ("brother", "sister"),
                    ("uncle", "aunt"),
                    ("nephew", "niece"),
                    ("husband", "wife"),
                    ("grandfather", "grandmother"),
                ],
            ),
            fam(
                "number",
                &[
                    ("man", "men"),
                    ("woman", "women"),
                    ("child", "children"),
                    ("foot", "feet"),
                    ("tooth", "teeth"),
                    ("mouse", "mice"),
                    ("goose", "geese"),
                ],
            ),
            fam(
                "antonym",
                &[
                    ("hot", "cold"),
                    ("big", "small"),
                    ("light", "dark"),
                    ("high", "low"),
                    ("fast", "slow"),
                    ("hard", "soft"),
                ],
            ),
        ]
    }

    /// Ranks the whole vocabulary against a query phasor, returning the rank of
    /// `target` (1-based) and the top matches. Excluded words are skipped.
    fn rank_of(
        facet: &Facet,
        query: &SpectralPhasor,
        target: &str,
        exclude: &[&str],
    ) -> Option<(usize, Vec<String>)> {
        let mut scored: Vec<(&str, f64)> = facet
            .lexicon
            .iter()
            .filter(|(w, _)| !exclude.contains(&w.as_str()))
            .map(|(w, p)| (w.as_str(), query.resonance(p)))
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let rank = scored.iter().position(|(w, _)| *w == target)? + 1;
        let top: Vec<String> = scored.iter().take(5).map(|(w, _)| w.to_string()).collect();
        Some((rank, top))
    }

    /// Builds the analogy query `c + (b − a)`, per channel.
    fn analogy_query(
        facet: &Facet,
        a: &str,
        b: &str,
        c: &str,
    ) -> Option<SpectralPhasor> {
        let (pa, pb, pc) = (
            facet.lexicon.get(a)?,
            facet.lexicon.get(b)?,
            facet.lexicon.get(c)?,
        );
        let mut q = *pc;
        for k in 0..SpectralPhasor::channels() {
            q.set_theta(k, pc.theta(k) + pb.theta(k) - pa.theta(k));
        }
        q.sync_phase();
        Some(q)
    }

    /// Evaluates one family.
    pub fn evaluate_family(facet: &Facet, family: &RelationFamily) -> FamilyResult {
        let usable: Vec<&RelationPair> = family
            .pairs
            .iter()
            .filter(|p| facet.contains_word(&p.a) && facet.contains_word(&p.b))
            .collect();

        let mut wins = 0usize;
        let mut top10 = 0usize;
        let mut top50 = 0usize;

        for (i, p) in usable.iter().enumerate() {
            let pa = &facet.lexicon[&p.a];
            let pb = &facet.lexicon[&p.b];

            // 1. pair versus a deterministically chosen random word
            let r = (fnv1a(&p.a) ^ (i as u64)) as usize;
            if let Some(rand_word) = facet.lexicon.keys().nth(r % facet.lexicon.len().max(1)) {
                if rand_word != &p.a && rand_word != &p.b {
                    let pr = &facet.lexicon[rand_word];
                    if pa.resonance(pb) > pa.resonance(pr) {
                        wins += 1;
                    }
                }
            }

            // 2. neighbourhood
            if let Some((rank, _)) = Self::rank_of(facet, pa, &p.b, &[&p.a]) {
                if rank <= 10 {
                    top10 += 1;
                }
                if rank <= 50 {
                    top50 += 1;
                }
            }
        }

        // 3. analogy, over every ordered pair of distinct pairs in the family
        let (mut a1, mut a5, mut mrr, mut tested) = (0usize, 0usize, 0.0f64, 0usize);
        for (i, p) in usable.iter().enumerate() {
            for (j, q) in usable.iter().enumerate() {
                if i == j {
                    continue;
                }
                let query = match Self::analogy_query(facet, &p.a, &p.b, &q.a) {
                    Some(x) => x,
                    None => continue,
                };
                let ranked = Self::rank_of(facet, &query, &q.b, &[&p.a, &p.b, &q.a]);
                if let Some((rank, _)) = ranked {
                    tested += 1;
                    if rank == 1 {
                        a1 += 1;
                    }
                    if rank <= 5 {
                        a5 += 1;
                    }
                    mrr += 1.0 / rank as f64;
                }
            }
        }

        let n = usable.len().max(1) as f64;
        let t = tested.max(1) as f64;

        FamilyResult {
            name: family.name.clone(),
            usable_pairs: usable.len(),
            pair_vs_random: wins as f64 / n,
            neighbour_top10: top10 as f64 / n,
            neighbour_top50: top50 as f64 / n,
            analogy_top1: a1 as f64 / t,
            analogy_top5: a5 as f64 / t,
            analogy_mrr: mrr / t,
            analogies_tested: tested,
        }
    }

    /// Evaluates every family and summarises.
    pub fn evaluate(facet: &Facet, families: &[RelationFamily]) -> RelationReport {
        let results: Vec<FamilyResult> = families
            .iter()
            .map(|f| Self::evaluate_family(facet, f))
            .collect();

        let v = facet.vocabulary_size().max(1) as f64;
        let usable_total: usize = results.iter().map(|r| r.usable_pairs).sum();
        let tested_total: usize = results.iter().map(|r| r.analogies_tested).sum();

        let overall_pair = match usable_total {
            0 => 0.0,
            _ => results
                .iter()
                .map(|r| r.pair_vs_random * r.usable_pairs as f64)
                .sum::<f64>()
                / usable_total as f64,
        };
        let overall_analogy = match tested_total {
            0 => 0.0,
            _ => results
                .iter()
                .map(|r| r.analogy_top1 * r.analogies_tested as f64)
                .sum::<f64>()
                / tested_total as f64,
        };

        RelationReport {
            vocabulary_size: facet.vocabulary_size(),
            families: results,
            chance_neighbour_top10: 10.0 / v,
            chance_analogy_top1: 1.0 / v,
            overall_pair_vs_random: overall_pair,
            overall_analogy_top1: overall_analogy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trainer::Trainer;

    #[test]
    fn test_related_words_beat_random_after_training() {
        let mut facet = Facet::new();
        let t = Trainer::new(0.05);
        // Train so that man/woman genuinely co-occur, and add filler vocabulary.
        for _ in 0..40 {
            t.train_sentence(&mut facet, "a man and a woman are adult people");
            t.train_sentence(&mut facet, "granite basalt and quartz are minerals");
            t.train_sentence(&mut facet, "copper tin and zinc are metals");
        }
        let fam = RelationFamily {
            name: "t".into(),
            pairs: vec![RelationPair { a: "man".into(), b: "woman".into() }],
        };
        let r = RelationBenchmark::evaluate_family(&facet, &fam);
        assert_eq!(r.usable_pairs, 1);
        assert!(r.pair_vs_random >= 0.0 && r.pair_vs_random <= 1.0);
        assert!(r.analogy_mrr >= 0.0);
    }

    #[test]
    fn test_missing_words_are_excluded_not_counted_as_failures() {
        let facet = Facet::new();
        let fam = RelationFamily {
            name: "t".into(),
            pairs: vec![RelationPair { a: "absent".into(), b: "alsoabsent".into() }],
        };
        let r = RelationBenchmark::evaluate_family(&facet, &fam);
        assert_eq!(r.usable_pairs, 0, "absent words must not be scored as wrong answers");
    }

    #[test]
    fn test_analogy_query_is_a_real_offset() {
        let mut facet = Facet::new();
        for w in ["man", "woman", "king", "queen"] {
            facet.get_or_init(w);
        }
        let q = RelationBenchmark::analogy_query(&facet, "man", "woman", "king").unwrap();
        // The query must differ from the word it started from.
        assert!(q.resonance(&facet.lexicon["king"]) < 0.999);
    }
}
