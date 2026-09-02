//! Relation types as an abstract data type, discovered from use.
//!
//! [`crate::roles::Role`] is an enum I wrote: six variants, named in advance,
//! and every caller of `bind` has to know which variant it wants. That is the
//! thing the CLU argument objects to. An abstract data type is defined by its
//! operations, not by its representation, and its inhabitants come from the data
//! rather than from the author — so a relation type should be something the
//! corpus produces, that callers can only obtain by asking, and whose innards
//! they cannot see.
//!
//! [`RelationType`] is that: an opaque handle with no public constructor and no
//! visible representation. The only way to get one is [`RelationTypes::discover`]
//! over a corpus, and the only things you can do with one are the operations
//! below. Nothing here is named `genus` or `function`, and nothing needs to be.
//!
//! # The bound
//!
//! The claim under test is that a language holds at most 64 relation types —
//! that the set is *bounded*, so a fixed-width type can hold it. A cap alone
//! proves nothing: `.take(64)` on anything yields at most 64 of it. What would
//! make the bound real is saturation — held-out structure stops improving before
//! the cap is reached, so the 65th type would have nothing left to explain.
//!
//! [`RelationTypes::saturation`] measures exactly that, and it can come out
//! against the claim. Centroids are fitted on one half of the pairs and scored
//! on the other, because in-sample cluster tightness rises with `k` for free and
//! would "confirm" any bound you asked it about. A shuffled-pair null runs
//! beside it, because a curve without a floor under it is not a measurement.
//!
//! # What it measured
//!
//! 19,797 head-filler pairs from the dictionary, restricted to words seen 25+
//! times, clustered on their phase offsets. Half fitted, half held out. Run on
//! two manifolds: the bottom-up one this project ships, and the same one after
//! the downward pass in [`crate::topdown`].
//!
//! ```text
//!                    k   held-out   shuffled      gain     noise   gain/noise
//! bottom-up          4     0.2019     0.2006    0.0013    0.0040         0.3x
//!                   16     0.2644     0.2585    0.0060    0.0032         1.9x
//!                   64     0.3600     0.3484    0.0116    0.0044         2.6x
//!                   96     0.3878     0.3803    0.0075    0.0028         2.7x
//!                  128     0.4107     0.4075    0.0032    0.0042         0.8x
//!
//! after descent      4     0.2981     0.2859    0.0122    0.0014         8.7x
//!                   16     0.3387     0.3292    0.0094    0.0028         3.4x
//!                   64     0.3740     0.3619    0.0122    0.0008        15.3x
//! ```
//!
//! **Bottom-up: no relation types at any k.** Held-out agreement climbs steadily
//! with the type count and the shuffled null climbs with it point for point.
//! Every gain sits within a few multiples of the distance between two
//! independent runs of the null itself, and it is not monotonic. Adding
//! centroids fits random offsets as well as real ones, which is what k-means
//! does to any cloud of points.
//!
//! **After the descent: structure.** The same procedure on the same pairs clears
//! its null by 8.7x at four types and 15x at sixty-four. So the downward pass
//! builds the shared offsets that the bottom-up manifold does not have, and the
//! threefold analogy gain it produced *is* relational structure rather than
//! something else wearing the shape of one.
//!
//! On the bound: the gain is flat across the whole range — 0.0122 at k=4 and
//! 0.0122 at k=64. Sixty more types buy nothing measurable. So a 64-wide type is
//! not too small; if anything the evidence supports a handful. But read the
//! absolute number before celebrating: 0.012 on a 0-to-1 agreement scale is a
//! real effect and a tiny one.
//!
//! And the mechanism the clusters use is still mostly word identity. **71.3% of
//! pairs share their cluster with another pair of the same head, against 9.1%
//! expected if the head were ignored; the descent moves that to 59.8%.**
//! `adversary->against` and `adversary->another` land together because they share
//! a word, not because they are the same relation. Offsets are dominated by
//! where the step started rather than by the step. The descent improves this by
//! eleven points and does not fix it, which is the next thing to attack.
//!
//! That is a finding about the representation, not about the discovery
//! procedure: the planted-relation tests below pass, so the clustering finds
//! real shared offsets when there are any to find.

use crate::facet::Facet;
use crate::phasor::SpectralPhasor;
use crate::roles::RoleDiscovery;
use crate::config::PHASE_CHANNELS;

/// The most relation types this can represent.
///
/// Chosen to match the channel count, so a type index and a channel index are
/// the same width — the bounded-type claim, made structural. Whether the *data*
/// needs this many is a separate question, and [`RelationTypes::saturation`] is
/// how it gets asked rather than assumed.
pub const MAX_RELATION_TYPES: usize = 64;

/// A relation type.
///
/// Opaque by construction: the field is private, there is no public constructor,
/// and the rotation it stands for lives in the [`RelationTypes`] that issued it.
/// A caller can hold one, compare it, and pass it to an operation. It cannot
/// build one, take it apart, or mean anything by it that the data did not put
/// there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RelationType(u8);

impl RelationType {
    /// A stable label for logging — `t07`, not `genus`.
    ///
    /// Deliberately uninformative. A discovered type has no name until someone
    /// looks at its members and gives it one, and putting a familiar word here
    /// would smuggle the enum back in through the display path.
    pub fn label(self) -> String {
        format!("t{:02}", self.0)
    }
}

/// One discovered type's evidence.
#[derive(Debug, Clone)]
pub struct TypeEvidence {
    pub relation: RelationType,
    /// Pairs assigned to it.
    pub members: Vec<(String, String)>,
    /// Mean agreement between a member's offset and the type's rotation.
    pub coherence: f64,
}

/// A vocabulary's relation types, and the operations over them.
#[derive(Debug, Clone)]
pub struct RelationTypes {
    rotations: Vec<Vec<f64>>,
    evidence: Vec<TypeEvidence>,
}

/// One point on the saturation curve.
#[derive(Debug, Clone, Copy)]
pub struct SaturationPoint {
    pub k: usize,
    /// Mean agreement of *held-out* offsets to their nearest fitted centroid.
    pub held_out: f64,
    /// The same, on pairs whose fillers have been permuted. The floor.
    pub shuffled: f64,
    /// A *second*, independently permuted null, so the distance between the two
    /// nulls gives the scale of variation this whole procedure has at this `k`.
    ///
    /// Without it a gain has no unit. The first run of this sweep reported a
    /// best gain of 0.0116 and a verdict of "the bound holds" — on a curve whose
    /// two arms differed by less than the arms differ from each other.
    pub shuffled_b: f64,
}

impl SaturationPoint {
    /// What this `k` buys over the null.
    pub fn gain(self) -> f64 {
        self.held_out - self.shuffled
    }

    /// How far apart two runs of the null itself are — the noise floor under
    /// [`SaturationPoint::gain`]. A gain smaller than this measures nothing.
    pub fn noise(self) -> f64 {
        (self.shuffled - self.shuffled_b).abs()
    }
}

impl RelationTypes {
    pub fn len(&self) -> usize {
        self.rotations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rotations.is_empty()
    }

    /// Every type this corpus produced.
    pub fn all(&self) -> Vec<RelationType> {
        (0..self.rotations.len()).map(|i| RelationType(i as u8)).collect()
    }

    pub fn evidence(&self, t: RelationType) -> Option<&TypeEvidence> {
        self.evidence.get(t.0 as usize)
    }

    /// Discovers the types a corpus's pairs exhibit, at a given count.
    ///
    /// `k` is clamped to [`MAX_RELATION_TYPES`]. Use [`RelationTypes::saturation`]
    /// to choose it from evidence rather than by taste.
    pub fn discover(facet: &Facet, pairs: &[(String, String)], k: usize, rounds: usize) -> Self {
        let k = k.min(MAX_RELATION_TYPES);
        let found = RoleDiscovery::discover(facet, pairs, k, rounds);
        let rotations: Vec<Vec<f64>> = found.iter().map(|d| d.rotation.clone()).collect();
        let evidence = found
            .into_iter()
            .enumerate()
            .map(|(i, d)| TypeEvidence {
                relation: RelationType(i as u8),
                members: d.members,
                coherence: d.coherence,
            })
            .collect();
        Self { rotations, evidence }
    }

    /// Binds a head to a relation type: the head's phases rotated by the type's.
    ///
    /// The caller supplies a handle it obtained from discovery. It never sees
    /// the rotation, which is the point — the representation is free to change
    /// without touching a single call site.
    pub fn bind(&self, facet: &Facet, head: &str, t: RelationType) -> Option<SpectralPhasor> {
        let rot = self.rotations.get(t.0 as usize)?;
        let p = facet.lexicon.get(head)?;
        let mut out = *p;
        for k in 0..PHASE_CHANNELS {
            out.set_theta(k, p.theta(k) + rot[k]);
        }
        out.sync_phase();
        Some(out)
    }

    /// The inverse: recovers what a bound phasor was bound *from*.
    pub fn unbind(&self, bound: &SpectralPhasor, t: RelationType) -> Option<SpectralPhasor> {
        let rot = self.rotations.get(t.0 as usize)?;
        let mut out = *bound;
        for k in 0..PHASE_CHANNELS {
            out.set_theta(k, bound.theta(k) - rot[k]);
        }
        out.sync_phase();
        Some(out)
    }

    /// Which type — if any — the step from `head` to `filler` belongs to.
    ///
    /// Returns the best type and its agreement. A pair standing in no discovered
    /// relation has a low agreement with all of them, so the caller can decide
    /// what to do about it; this does not invent a type to hold it.
    pub fn classify(
        &self,
        facet: &Facet,
        head: &str,
        filler: &str,
    ) -> Option<(RelationType, f64)> {
        let off = RoleDiscovery::offset(facet, head, filler)?;
        self.rotations
            .iter()
            .enumerate()
            .map(|(i, r)| (RelationType(i as u8), RoleDiscovery::agreement(&off, r)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Does held-out structure keep improving as types are added?
    ///
    /// For each `k`: fit centroids on half the pairs, then measure how well the
    /// *other* half's offsets agree with their nearest fitted centroid. In-sample
    /// tightness rises with `k` mechanically and would confirm any bound; this
    /// does not.
    ///
    /// The `shuffled` column repeats everything on pairs whose fillers have been
    /// permuted, which destroys the relations while preserving the vocabulary,
    /// the offset distribution's marginals and the cluster count. A held-out
    /// curve that the null tracks is measuring k-means, not language.
    ///
    /// A bounded type system shows up as [`SaturationPoint::gain`] flattening
    /// below the cap. A gain still climbing at the cap says the bound is wrong,
    /// and that is a result this is allowed to return.
    pub fn saturation(
        facet: &Facet,
        pairs: &[(String, String)],
        ks: &[usize],
        rounds: usize,
    ) -> Vec<SaturationPoint> {
        ks.iter()
            .map(|&k| Self::saturation_at(facet, pairs, k, rounds))
            .collect()
    }

    /// One point of [`RelationTypes::saturation`].
    ///
    /// Split out so a caller sweeping a long list of `k` can print each row as
    /// it lands. A sweep that speaks only once every value has finished gives no
    /// way to tell a slow run from a wedged one, and the large `k` values are
    /// both the slowest and the ones the claim turns on.
    pub fn saturation_at(
        facet: &Facet,
        pairs: &[(String, String)],
        k: usize,
        rounds: usize,
    ) -> SaturationPoint {
        let (fit, held, null_fit, null_held) = Self::split(pairs);
        let (null_fit_b, null_held_b) = (
            permute_fillers(&fit, 0x1234_5678_9ABC_DEF1),
            permute_fillers(&held, 0x0FED_CBA9_8765_4321),
        );
        SaturationPoint {
            k,
            held_out: Self::held_out_agreement(facet, &fit, &held, k, rounds),
            shuffled: Self::held_out_agreement(facet, &null_fit, &null_held, k, rounds),
            shuffled_b: Self::held_out_agreement(facet, &null_fit_b, &null_held_b, k, rounds),
        }
    }

    /// The fit/held halves and their shuffled nulls.
    #[allow(clippy::type_complexity)]
    fn split(
        pairs: &[(String, String)],
    ) -> (
        Vec<(String, String)>,
        Vec<(String, String)>,
        Vec<(String, String)>,
        Vec<(String, String)>,
    ) {
        // A deterministic split, keyed on the pair's *content* rather than its
        // position. Splitting on index parity looks equally deterministic and is
        // a trap: any input where pairs alternate by relation — which is exactly
        // how a per-head extractor emits them — puts every instance of one
        // relation in the fit half and every instance of another in the held
        // half, so the centroids are fitted on relations the evaluation never
        // contains. That produces strongly *negative* agreement and reads as
        // "more types are worse", which is a fact about the split.
        //
        // Hashing the pair is not by itself enough either. FNV-1a's final step
        // is a multiply by an odd prime, which preserves the low bit, so that
        // bit is a parity of the input bytes — and `_x` and `_y` differ by one
        // in their last byte, which put every pair of one relation on one side
        // and every pair of the other on the other, exactly as index parity had.
        // See [`split_side`].
        let mut fit = Vec::new();
        let mut held = Vec::new();
        for p in pairs {
            match split_side(&p.0, &p.1) {
                true => fit.push(p.clone()),
                false => held.push(p.clone()),
            }
        }
        let null_fit = permute_fillers(&fit, 0x9E37_79B9_7F4A_7C15);
        let null_held = permute_fillers(&held, 0xC2B2_AE3D_27D4_EB4F);
        (fit, held, null_fit, null_held)
    }

    /// Deliberately clusters *uncapped*.
    ///
    /// [`RelationTypes::discover`] clamps to [`MAX_RELATION_TYPES`], which is
    /// right for the type — its handles have to fit — and fatal here. A sweep
    /// that silently repeats k=64 for every k above it cannot find the cap too
    /// small; it would report a flat curve past the cap and call that
    /// saturation, confirming the bound by construction. The measurement
    /// therefore goes straight to the clustering.
    fn held_out_agreement(
        facet: &Facet,
        fit: &[(String, String)],
        held: &[(String, String)],
        k: usize,
        rounds: usize,
    ) -> f64 {
        let found = RoleDiscovery::discover(facet, fit, k, rounds);
        if found.is_empty() {
            return 0.0;
        }
        let mut sum = 0.0;
        let mut n = 0usize;
        for (h, f) in held {
            let off = match RoleDiscovery::offset(facet, h, f) {
                Some(o) => o,
                None => continue,
            };
            let best = found
                .iter()
                .map(|d| RoleDiscovery::agreement(&off, &d.rotation))
                .fold(f64::NEG_INFINITY, f64::max);
            if best.is_finite() {
                sum += best;
                n += 1;
            }
        }
        match n {
            0 => 0.0,
            _ => sum / n as f64,
        }
    }
}

/// Which half of the fit/held split a pair falls in.
///
/// The hash is finalised with a bit-mixing step before a bit is taken from it.
/// FNV-1a alone will not do: its last operation is a multiply by an odd
/// constant, so the low bit of the digest is the parity of the input bytes, and
/// two words differing by one in their final character land on opposite sides
/// every time. That is not a hypothetical — it stratified this very test, fitting
/// centroids on one planted relation and scoring them on the other, and the
/// resulting curve said confidently that more types are worse.
fn split_side(head: &str, filler: &str) -> bool {
    let h = crate::phasor::fnv1a(&format!("{head}\u{1}{filler}"));
    // splitmix64's finaliser: avalanches every input bit into every output bit.
    let mut z = h.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    z >> 63 == 0
}

/// Permutes fillers among pairs, destroying the relations and keeping everything
/// else.
///
/// A derangement is attempted — a pair mapped to its own filler would leave a
/// real relation intact inside the null — but a single-pair input has no
/// derangement, and that case is returned unchanged rather than pretended about.
fn permute_fillers(pairs: &[(String, String)], seed: u64) -> Vec<(String, String)> {
    let mut fillers: Vec<String> = pairs.iter().map(|p| p.1.clone()).collect();
    let mut r = seed | 1;
    for i in (1..fillers.len()).rev() {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        let j = (r % (i as u64 + 1)) as usize;
        fillers.swap(i, j);
    }
    pairs
        .iter()
        .zip(fillers)
        .map(|(p, f)| (p.0.clone(), f))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facet_with(words: &[&str]) -> Facet {
        let mut f = Facet::new();
        for w in words {
            f.get_or_init(w);
        }
        f
    }

    /// Two planted relations, built by construction, must come back as two
    /// types — without either being named.
    #[test]
    fn test_two_planted_relations_are_recovered_as_two_types() {
        let heads = ["a", "b", "c", "d", "e", "f"];
        let mut words: Vec<String> = Vec::new();
        for h in heads {
            words.push(h.to_string());
            words.push(format!("{h}_x"));
            words.push(format!("{h}_y"));
        }
        let refs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        let mut f = facet_with(&refs);

        // Relation one: rotate by 0.7 on every channel. Relation two: by -1.9.
        for h in heads {
            let base: Vec<f64> = (0..PHASE_CHANNELS).map(|k| f.lexicon[h].theta(k)).collect();
            for (suffix, delta) in [("_x", 0.7), ("_y", -1.9)] {
                let p = f.lexicon.get_mut(&format!("{h}{suffix}")).unwrap();
                for k in 0..PHASE_CHANNELS {
                    p.set_theta(k, base[k] + delta);
                }
                p.sync_phase();
            }
        }

        let pairs: Vec<(String, String)> = heads
            .iter()
            .flat_map(|h| {
                [
                    (h.to_string(), format!("{h}_x")),
                    (h.to_string(), format!("{h}_y")),
                ]
            })
            .collect();

        let types = RelationTypes::discover(&f, &pairs, 2, 12);
        assert_eq!(types.len(), 2);

        // Every `_x` pair lands in one type and every `_y` pair in the other,
        // and neither type is called anything.
        let of = |suffix: &str| -> RelationType {
            let (h, fl) = ("a".to_string(), format!("a{suffix}"));
            types.classify(&f, &h, &fl).unwrap().0
        };
        assert_ne!(of("_x"), of("_y"));
        for h in heads {
            assert_eq!(types.classify(&f, h, &format!("{h}_x")).unwrap().0, of("_x"));
            assert_eq!(types.classify(&f, h, &format!("{h}_y")).unwrap().0, of("_y"));
        }
    }

    /// Bind and unbind are inverses, through the handle only.
    #[test]
    fn test_bind_unbind_round_trips_through_the_handle() {
        let f = facet_with(&["alpha", "beta", "gamma", "delta"]);
        let pairs = vec![
            ("alpha".to_string(), "beta".to_string()),
            ("gamma".to_string(), "delta".to_string()),
        ];
        let types = RelationTypes::discover(&f, &pairs, 2, 8);
        let t = types.all()[0];

        let bound = types.bind(&f, "alpha", t).unwrap();
        let back = types.unbind(&bound, t).unwrap();
        for k in 0..PHASE_CHANNELS {
            let d = (back.theta(k) - f.lexicon["alpha"].theta(k)).abs();
            assert!(d < 1e-9 || (d - crate::config::TWO_PI).abs() < 1e-9, "channel {k}");
        }
    }

    /// The null has to be a real null: permuting fillers must actually change
    /// the pairs, or every saturation curve sits on top of its own floor and
    /// the comparison is vacuous.
    #[test]
    fn test_the_shuffled_null_actually_permutes() {
        let pairs: Vec<(String, String)> = (0..50)
            .map(|i| (format!("h{i}"), format!("f{i}")))
            .collect();
        let null = permute_fillers(&pairs, 12345);
        let unchanged = pairs.iter().zip(&null).filter(|(a, b)| a.1 == b.1).count();
        assert!(
            unchanged < pairs.len() / 10,
            "{unchanged} of {} fillers stayed put",
            pairs.len()
        );
    }

    /// The split must not correlate with anything about the pairs.
    ///
    /// This is the bug that produced a confident, entirely false saturation
    /// curve. FNV-1a's low bit is the parity of the input bytes, so `h_x` and
    /// `h_y` — differing by one in the last character — landed in opposite
    /// halves without exception. Every planted relation of one kind went into
    /// the fit set and every one of the other kind into the held set, and the
    /// measurement dutifully reported that adding types makes things worse.
    #[test]
    fn test_the_fit_held_split_does_not_stratify_by_a_one_character_difference() {
        let n = 400;
        let mut same = 0usize;
        for i in 0..n {
            let h = format!("h{i}");
            if split_side(&h, &format!("{h}_x")) == split_side(&h, &format!("{h}_y")) {
                same += 1;
            }
        }
        // Independent sides land together about half the time. A hash whose low
        // bit tracks the last byte gives zero, which is what this caught.
        let frac = same as f64 / n as f64;
        assert!(
            (frac - 0.5).abs() < 0.1,
            "`_x` and `_y` should fall on the same side about half the time, got {frac:.3}"
        );

        // And the halves themselves have to be roughly balanced.
        let fit = (0..n).filter(|i| split_side(&format!("h{i}"), "f")).count();
        assert!(
            (fit as f64 / n as f64 - 0.5).abs() < 0.1,
            "unbalanced split: {fit} of {n}"
        );
    }

    /// Saturation must be measured out of sample, or it confirms any bound.
    ///
    /// On planted data with exactly two relations, held-out agreement should not
    /// keep climbing once k passes two. The in-sample equivalent would, which is
    /// why it is not what this reports.
    #[test]
    fn test_saturation_flattens_once_the_planted_types_are_covered() {
        let heads: Vec<String> = (0..40).map(|i| format!("h{i}")).collect();
        let mut words: Vec<String> = Vec::new();
        for h in &heads {
            words.push(h.clone());
            words.push(format!("{h}_x"));
            words.push(format!("{h}_y"));
        }
        let refs: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        let mut f = facet_with(&refs);
        for h in &heads {
            let base: Vec<f64> = (0..PHASE_CHANNELS).map(|k| f.lexicon[h].theta(k)).collect();
            for (suffix, delta) in [("_x", 0.7), ("_y", -1.9)] {
                let p = f.lexicon.get_mut(&format!("{h}{suffix}")).unwrap();
                for k in 0..PHASE_CHANNELS {
                    p.set_theta(k, base[k] + delta);
                }
                p.sync_phase();
            }
        }
        let pairs: Vec<(String, String)> = heads
            .iter()
            .flat_map(|h| {
                [
                    (h.clone(), format!("{h}_x")),
                    (h.clone(), format!("{h}_y")),
                ]
            })
            .collect();

        let curve = RelationTypes::saturation(&f, &pairs, &[1, 2, 4, 8], 12);
        let at = |k: usize| curve.iter().find(|p| p.k == k).unwrap();

        assert!(
            at(2).gain() > at(1).gain(),
            "the second planted type should be worth having: {:.3} -> {:.3}",
            at(1).gain(),
            at(2).gain()
        );
        assert!(
            at(8).gain() <= at(2).gain() + 0.02,
            "there are only two relations here; k=8 should buy nothing: \
             k2 {:.3}, k8 {:.3}",
            at(2).gain(),
            at(8).gain()
        );
    }
}

