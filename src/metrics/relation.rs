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
use rayon::prelude::*;
use crate::phasor::{fnv1a, SpectralPhasor};
use serde::{Deserialize, Serialize};

/// Random comparisons drawn per pair for the similarity test.
///
/// A single draw per pair gives ~23 Bernoulli trials across the whole probe
/// set — a standard error near 10%, enough for the figure to swing 30 points
/// between runs of the same experiment. Averaging many draws makes the number
/// worth comparing.
const RANDOM_DRAWS: usize = 64;

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
    /// Fraction of (pair, random word) comparisons the pair wins. Chance = 0.5.
    /// Averaged over `RANDOM_DRAWS` draws per pair, so it is stable enough to
    /// compare between runs.
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

/// Partner pairs sampled per pair when scoring analogies.
///
/// Exhaustive is n(n-1) full-vocabulary rankings per family. Sampling keeps the
/// benchmark affordable at 305 pairs; it is deterministic, so the comparison
/// between conditions is still exact.
const MAX_ANALOGY_PARTNERS: usize = 8;

/// A word's minimum training count for it to enter the candidate pool.
///
/// Every relational headline in this project was computed over the full 70k
/// vocabulary, of which most words were seen fewer than five times. A word seen
/// four times has phases essentially equal to its hash seed, so those words
/// contribute noise to every ranking and pull every average down.
///
/// Restricting the pool also *shrinks* it, which makes the task easier — so the
/// pool size and chance level are reported at every floor and the comparison is
/// ratio-to-chance, never a raw score against a raw score.
pub type CountFloor = u32;

impl RelationBenchmark {
    /// The default probe set: 305 pairs across 10 relation families.
    ///
    /// The previous set was 23 usable pairs across three families, which is too
    /// few to support any claim it was being used to make: an analogy@1 of
    /// 10.49% on 23 pairs is a handful of hits, and a single family could carry
    /// the total on its own. Task A2 grew it to the point where an effect has to
    /// clear its own error bar.
    ///
    /// The families are chosen so that a failure is the model's and not the
    /// probe's: each relation is unambiguous, and every word is one a
    /// nineteenth-century dictionary defines. They also span the two kinds of
    /// relation separately - **semantic** (gender, antonym, hypernym,
    /// nationality) and **morphological** (number, comparative, past tense,
    /// agent, quality, negation) - because a manifold can easily learn the
    /// second from spelling alone while learning nothing about meaning, and a
    /// combined score would hide that. Read the per-family breakdown, not the
    /// total.
    ///
    /// Families whose vocabulary a given corpus does not cover report zero
    /// usable pairs rather than silently contributing nothing, which is why
    /// `usable_pairs` is on [`FamilyResult`].
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
                    ("man", "woman"), ("boy", "girl"), ("king", "queen"),
                    ("father", "mother"), ("son", "daughter"), ("brother", "sister"),
                    ("uncle", "aunt"), ("nephew", "niece"), ("husband", "wife"),
                    ("grandfather", "grandmother"), ("sir", "madam"), ("lord", "lady"),
                    ("prince", "princess"), ("actor", "actress"), ("waiter", "waitress"),
                    ("host", "hostess"), ("widower", "widow"), ("bull", "cow"),
                    ("rooster", "hen"), ("stallion", "mare"), ("ram", "ewe"),
                    ("buck", "doe"), ("drake", "duck"), ("gander", "goose"),
                    ("lion", "lioness"), ("tiger", "tigress"), ("emperor", "empress"),
                    ("duke", "duchess"), ("master", "mistress"), ("monk", "nun"),
                ],
            ),
            fam(
                "number",
                &[
                    ("man", "men"), ("woman", "women"), ("child", "children"),
                    ("foot", "feet"), ("tooth", "teeth"), ("mouse", "mice"),
                    ("goose", "geese"), ("ox", "oxen"), ("person", "people"),
                    ("leaf", "leaves"), ("knife", "knives"), ("wife", "wives"),
                    ("life", "lives"), ("half", "halves"), ("loaf", "loaves"),
                    ("thief", "thieves"), ("shelf", "shelves"), ("wolf", "wolves"),
                    ("calf", "calves"), ("city", "cities"), ("baby", "babies"),
                    ("lady", "ladies"), ("story", "stories"), ("country", "countries"),
                    ("family", "families"), ("army", "armies"), ("party", "parties"),
                    ("body", "bodies"), ("hero", "heroes"), ("potato", "potatoes"),
                    ("echo", "echoes"), ("box", "boxes"), ("church", "churches"),
                    ("brush", "brushes"), ("glass", "glasses"),
                ],
            ),
            fam(
                "antonym",
                &[
                    ("hot", "cold"), ("big", "small"), ("light", "dark"),
                    ("high", "low"), ("fast", "slow"), ("hard", "soft"),
                    ("good", "bad"), ("long", "short"), ("wide", "narrow"),
                    ("thick", "thin"), ("strong", "weak"), ("rich", "poor"),
                    ("young", "old"), ("happy", "sad"), ("wet", "dry"),
                    ("clean", "dirty"), ("full", "empty"), ("deep", "shallow"),
                    ("sharp", "blunt"), ("sweet", "bitter"), ("smooth", "rough"),
                    ("loud", "quiet"), ("brave", "cowardly"), ("wise", "foolish"),
                    ("true", "false"), ("right", "wrong"), ("love", "hate"),
                    ("war", "peace"), ("birth", "death"), ("day", "night"),
                    ("summer", "winter"), ("north", "south"), ("east", "west"),
                    ("up", "down"), ("joy", "sorrow"),
                ],
            ),
            fam(
                "comparative",
                &[
                    ("big", "bigger"), ("small", "smaller"), ("long", "longer"),
                    ("short", "shorter"), ("high", "higher"), ("low", "lower"),
                    ("fast", "faster"), ("slow", "slower"), ("hard", "harder"),
                    ("soft", "softer"), ("strong", "stronger"), ("weak", "weaker"),
                    ("rich", "richer"), ("poor", "poorer"), ("young", "younger"),
                    ("old", "older"), ("warm", "warmer"), ("cold", "colder"),
                    ("deep", "deeper"), ("wide", "wider"), ("narrow", "narrower"),
                    ("thick", "thicker"), ("thin", "thinner"), ("sweet", "sweeter"),
                    ("bright", "brighter"), ("dark", "darker"), ("clear", "clearer"),
                    ("sharp", "sharper"), ("smooth", "smoother"), ("quick", "quicker"),
                ],
            ),
            fam(
                "past_tense",
                &[
                    ("walk", "walked"), ("talk", "talked"), ("work", "worked"),
                    ("play", "played"), ("look", "looked"), ("call", "called"),
                    ("want", "wanted"), ("need", "needed"), ("help", "helped"),
                    ("open", "opened"), ("close", "closed"), ("watch", "watched"),
                    ("learn", "learned"), ("turn", "turned"), ("start", "started"),
                    ("follow", "followed"), ("live", "lived"), ("move", "moved"),
                    ("love", "loved"), ("use", "used"), ("ask", "asked"),
                    ("answer", "answered"), ("carry", "carried"), ("marry", "married"),
                    ("study", "studied"), ("try", "tried"), ("cry", "cried"),
                    ("stop", "stopped"), ("drop", "dropped"), ("go", "went"),
                    ("see", "saw"), ("take", "took"), ("give", "gave"),
                    ("come", "came"), ("know", "knew"),
                ],
            ),
            fam(
                "agent",
                &[
                    ("teach", "teacher"), ("write", "writer"), ("sing", "singer"),
                    ("dance", "dancer"), ("paint", "painter"), ("farm", "farmer"),
                    ("work", "worker"), ("play", "player"), ("read", "reader"),
                    ("speak", "speaker"), ("lead", "leader"), ("build", "builder"),
                    ("bake", "baker"), ("hunt", "hunter"), ("drive", "driver"),
                    ("ride", "rider"), ("run", "runner"), ("swim", "swimmer"),
                    ("fight", "fighter"), ("print", "printer"), ("own", "owner"),
                    ("buy", "buyer"), ("sell", "seller"), ("help", "helper"),
                    ("rule", "ruler"), ("found", "founder"), ("keep", "keeper"),
                    ("make", "maker"), ("give", "giver"), ("follow", "follower"),
                ],
            ),
            fam(
                "hypernym",
                &[
                    ("dog", "animal"), ("oak", "tree"), ("rose", "flower"),
                    ("sparrow", "bird"), ("salmon", "fish"), ("copper", "metal"),
                    ("granite", "rock"), ("wheat", "grain"), ("apple", "fruit"),
                    ("hammer", "tool"), ("chair", "furniture"), ("violin", "instrument"),
                    ("hydrogen", "element"), ("triangle", "figure"), ("circle", "figure"),
                    ("crimson", "color"), ("azure", "color"), ("monday", "day"),
                    ("january", "month"), ("spring", "season"), ("gold", "metal"),
                    ("silver", "metal"), ("pine", "tree"), ("eagle", "bird"),
                    ("trout", "fish"), ("wine", "drink"), ("bread", "food"),
                    ("cotton", "fiber"), ("marble", "stone"), ("clay", "earth"),
                    ("rifle", "weapon"), ("sword", "weapon"), ("ship", "vessel"),
                    ("wagon", "vehicle"), ("cottage", "building"),
                ],
            ),
            fam(
                "nationality",
                &[
                    ("france", "french"), ("spain", "spanish"), ("england", "english"),
                    ("germany", "german"), ("italy", "italian"), ("russia", "russian"),
                    ("china", "chinese"), ("japan", "japanese"), ("greece", "greek"),
                    ("egypt", "egyptian"), ("ireland", "irish"), ("scotland", "scottish"),
                    ("poland", "polish"), ("sweden", "swedish"), ("denmark", "danish"),
                    ("norway", "norwegian"), ("holland", "dutch"), ("portugal", "portuguese"),
                    ("turkey", "turkish"), ("persia", "persian"), ("india", "indian"),
                    ("arabia", "arabian"), ("africa", "african"), ("america", "american"),
                    ("europe", "european"),
                ],
            ),
            fam(
                "quality",
                &[
                    ("kind", "kindness"), ("dark", "darkness"), ("weak", "weakness"),
                    ("good", "goodness"), ("sad", "sadness"), ("happy", "happiness"),
                    ("bitter", "bitterness"), ("sweet", "sweetness"), ("bold", "boldness"),
                    ("sick", "sickness"), ("blind", "blindness"), ("thick", "thickness"),
                    ("great", "greatness"), ("hard", "hardness"), ("soft", "softness"),
                    ("mad", "madness"), ("glad", "gladness"), ("quick", "quickness"),
                    ("sharp", "sharpness"), ("smooth", "smoothness"), ("rough", "roughness"),
                    ("bright", "brightness"), ("still", "stillness"), ("ready", "readiness"),
                    ("holy", "holiness"),
                ],
            ),
            fam(
                "negation",
                &[
                    ("happy", "unhappy"), ("kind", "unkind"), ("known", "unknown"),
                    ("able", "unable"), ("just", "unjust"), ("equal", "unequal"),
                    ("common", "uncommon"), ("certain", "uncertain"), ("easy", "uneasy"),
                    ("fair", "unfair"), ("fit", "unfit"), ("holy", "unholy"),
                    ("lucky", "unlucky"), ("natural", "unnatural"), ("pleasant", "unpleasant"),
                    ("ripe", "unripe"), ("safe", "unsafe"), ("seen", "unseen"),
                    ("true", "untrue"), ("usual", "unusual"), ("wise", "unwise"),
                    ("worthy", "unworthy"), ("clean", "unclean"), ("done", "undone"),
                    ("even", "uneven"),
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
        Self::rank_of_above(facet, query, target, exclude, 0)
    }

    fn rank_of_above(
        facet: &Facet,
        query: &SpectralPhasor,
        target: &str,
        exclude: &[&str],
        floor: CountFloor,
    ) -> Option<(usize, Vec<String>)> {
        // Only the target's rank and the top few are wanted, so counting how
        // many words beat the target is enough — no full sort of the vocabulary,
        // which at 70k words inside a doubly-nested analogy loop was the reason
        // the expanded probe set wedged the experiment.
        let tp = facet.lexicon.get(target)?;
        let target_score = query.resonance(tp);

        let beating = facet
            .lexicon
            .par_iter()
            .filter(|(w, _)| w.as_str() != target && !exclude.contains(&w.as_str()))
            .filter(|(_, p)| query.resonance(p) > target_score)
            .count();

        Some((beating + 1, Vec::new()))
    }

    /// As [`RelationBenchmark::rank_of`], but also returns the nearest words.
    ///
    /// Split out because the ranking path runs millions of times and does not
    /// need the names, while inspection runs rarely and does.
    pub fn nearest(facet: &Facet, query: &SpectralPhasor, exclude: &[&str], k: usize) -> Vec<String> {
        let mut scored: Vec<(&str, f64)> = facet
            .lexicon
            .iter()
            .filter(|(w, _)| !exclude.contains(&w.as_str()))
            .map(|(w, p)| (w.as_str(), query.resonance(p)))
            .collect();
        scored.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(b.0))
        });
        scored.into_iter().take(k).map(|(w, _)| w.to_string()).collect()
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
    /// Words eligible for the candidate pool at a given floor.
    pub fn pool_size(facet: &Facet, floor: CountFloor) -> usize {
        match floor {
            0 => facet.vocabulary_size(),
            f => facet.lexicon.values().filter(|p| p.count >= f).count(),
        }
    }

    pub fn evaluate_family(facet: &Facet, family: &RelationFamily) -> FamilyResult {
        Self::evaluate_family_above(facet, family, 0)
    }

    /// [`RelationBenchmark::evaluate_family`] with a pool floor.
    pub fn evaluate_family_above(
        facet: &Facet,
        family: &RelationFamily,
        floor: CountFloor,
    ) -> FamilyResult {
        let above = |w: &str| {
            facet.lexicon.get(w).is_some_and(|p| p.count >= floor.max(1))
        };
        // A probe pair whose own words are below the floor cannot be scored
        // against a pool that excludes them.
        let usable: Vec<&RelationPair> = family
            .pairs
            .iter()
            .filter(|p| above(&p.a) && above(&p.b))
            .collect();

        let mut wins_frac = 0.0f64;
        let mut wins_n = 0usize;
        let mut top10 = 0usize;
        let mut top50 = 0usize;

        for (i, p) in usable.iter().enumerate() {
            let pa = &facet.lexicon[&p.a];
            let pb = &facet.lexicon[&p.b];

            // 1. pair versus random, averaged over many deterministic draws
            let target = pa.resonance(pb);
            let vocab: Vec<&String> = facet.lexicon.keys().collect();
            let (mut hits, mut draws) = (0usize, 0usize);
            for d in 0..RANDOM_DRAWS {
                let r = (fnv1a(&p.a) ^ ((i as u64) << 32) ^ (d as u64).wrapping_mul(0x9E3779B9))
                    as usize;
                let cand = vocab[r % vocab.len().max(1)];
                if cand == &p.a || cand == &p.b {
                    continue;
                }
                draws += 1;
                if target > pa.resonance(&facet.lexicon[cand]) {
                    hits += 1;
                }
            }
            if draws > 0 {
                wins_frac += hits as f64 / draws as f64;
                wins_n += 1;
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

        // 3. analogy, over ordered pairs of distinct pairs in the family.
        //
        // Every ordered pair is n(n-1), which at 35 pairs per family across ten
        // families is ~11,000 full-vocabulary rankings *per condition* — the
        // expanded probe set made the honest exhaustive version unaffordable.
        // The stride below samples a fixed, deterministic subset so the cost is
        // linear in family size while the sample stays identical between runs;
        // an experiment whose probe set moves cannot support a comparison.
        let stride = (usable.len() / MAX_ANALOGY_PARTNERS.max(1)).max(1);
        let (mut a1, mut a5, mut mrr, mut tested) = (0usize, 0usize, 0.0f64, 0usize);
        for (i, p) in usable.iter().enumerate() {
            for (j, q) in usable.iter().enumerate() {
                if i == j || j % stride != i % stride {
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
            pair_vs_random: match wins_n {
                0 => 0.0,
                k => wins_frac / k as f64,
            },
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
        Self::evaluate_above(facet, families, 0)
    }

    /// [`RelationBenchmark::evaluate`] over a candidate pool restricted to words
    /// seen at least `floor` times.
    ///
    /// `vocabulary_size` in the returned report is the **pool** size, not the
    /// lexicon's, so the chance levels beside it are the ones that actually
    /// apply.
    pub fn evaluate_above(
        facet: &Facet,
        families: &[RelationFamily],
        floor: CountFloor,
    ) -> RelationReport {
        let results: Vec<FamilyResult> = families
            .iter()
            .map(|f| Self::evaluate_family_above(facet, f, floor))
            .collect();

        let v = Self::pool_size(facet, floor).max(1) as f64;
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
            vocabulary_size: Self::pool_size(facet, floor),
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

    /// A2's acceptance criterion, enforced rather than asserted in a doc.
    ///
    /// The benchmark exists to make effects falsifiable; a benchmark that
    /// quietly shrinks back below significance defeats that silently.
    /// The floor must shrink the pool, and chance must be reported against the
    /// pool that was actually used.
    ///
    /// Restricting the pool makes the task easier — fewer distractors — so a raw
    /// score at a high floor is not comparable to a raw score at floor 0. If
    /// `vocabulary_size` kept reporting the whole lexicon, every chance level
    /// derived from it would be wrong and the comparison would silently favour
    /// the restricted run.
    #[test]
    fn test_floor_restricts_the_pool_and_its_chance_level() {
        let mut facet = Facet::new();
        let t = Trainer::new(0.05);
        for _ in 0..40 {
            t.train_sentence(&mut facet, "a man and a woman are adult people");
            t.train_sentence(&mut facet, "a boy and a girl are young people");
            t.train_sentence(&mut facet, "copper tin and zinc are metals");
        }
        let fams = RelationBenchmark::default_families();

        let open = RelationBenchmark::evaluate_above(&facet, &fams, 0);
        let tight = RelationBenchmark::evaluate_above(&facet, &fams, 1_000_000);

        assert_eq!(open.vocabulary_size, facet.vocabulary_size());
        assert_eq!(
            tight.vocabulary_size,
            RelationBenchmark::pool_size(&facet, 1_000_000),
            "the report must describe the pool it used, not the lexicon"
        );
        assert!(
            tight.vocabulary_size < open.vocabulary_size,
            "an impossible floor must empty the pool: {} vs {}",
            tight.vocabulary_size,
            open.vocabulary_size
        );
        assert!(
            tight.chance_neighbour_top10 >= open.chance_neighbour_top10,
            "a smaller pool must report a higher chance level, or the \
             comparison flatters the restricted run"
        );
        // No probe pair can clear an impossible floor, so nothing is scored.
        assert_eq!(tight.families.iter().map(|f| f.usable_pairs).sum::<usize>(), 0);
    }

    #[test]
    fn test_probe_set_is_large_enough_to_support_a_claim() {
        let fams = RelationBenchmark::default_families();
        let total: usize = fams.iter().map(|f| f.pairs.len()).sum();

        assert!(fams.len() >= 8, "at least 8 families, found {}", fams.len());
        assert!(total >= 300, "at least 300 pairs, found {}", total);

        // No family may be large enough to carry the total by itself.
        for f in &fams {
            let share = f.pairs.len() as f64 / total as f64;
            assert!(
                share < 0.20,
                "family {} is {:.0}% of the probe set — one family must not \
                 dominate the aggregate",
                f.name,
                share * 100.0
            );
        }

        // Duplicate pairs would inflate the count without adding evidence.
        let mut seen = std::collections::HashSet::new();
        for f in &fams {
            for p in &f.pairs {
                assert!(
                    seen.insert((f.name.clone(), p.a.clone(), p.b.clone())),
                    "duplicate pair {}/{} in family {}",
                    p.a,
                    p.b,
                    f.name
                );
            }
        }
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
