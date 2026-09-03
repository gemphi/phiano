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
//! # Removing word identity from the readout
//!
//! The clusters were grouping by word, not by relation: **71.3% of pairs shared
//! a cluster with another pair of the same head, against 9.1% expected if the
//! head were ignored.** `adversary->against` and `adversary->another` land
//! together because both differences contain `-theta(adversary)`.
//!
//! That is a main effect, so [`OffsetEstimator`] centers it out. The full grid,
//! two manifolds by three estimators:
//!
//! ```text
//! manifold        offset             k   held-out   shuffled     gain  g/noise  head-dom
//! bottom-up       raw               16     0.2644     0.2585   0.0060     1.9x     71.3%
//! bottom-up       raw               64     0.3600     0.3484   0.0116     2.6x     71.3%
//! bottom-up       head-centered     16     0.7677     0.7711  -0.0034    -1.8x     70.7%
//! bottom-up       head-centered     64     0.7763     0.7827  -0.0064    -2.4x     70.7%
//! bottom-up       double-centered   16     0.6379     0.6354   0.0025     1.3x     78.2%
//! bottom-up       double-centered   64     0.6514     0.6485   0.0029     3.6x     78.2%
//! after descent   raw               16     0.3387     0.3292   0.0094     3.3x     59.8%
//! after descent   raw               64     0.3740     0.3619   0.0122    14.9x     59.8%
//! after descent   head-centered     16     0.7474     0.7420   0.0054     1.4x     26.5%
//! after descent   head-centered     64     0.7639     0.7587   0.0053    12.1x     26.5%
//! after descent   double-centered   16     0.5649     0.5063   0.0585     5.3x     33.5%
//! after descent   double-centered   64     0.5801     0.5204   0.0597     6.4x     33.5%
//! ```
//!
//! **Descent plus double-centering: gain 0.0597 at 6.4x noise — five times the
//! best raw gain, and the largest relational effect this project has measured.**
//! Head domination falls from 59.8% to 33.5% at the same time, so the clusters
//! are grouping less by word identity *and* the grouping is worth more. Those
//! two columns had to move together for this to mean anything, and they did.
//!
//! The interaction is the real finding. Centering does nothing on the bottom-up
//! manifold — head-centering there is *negative*, fitting worse than centered
//! noise. It only pays on the descended one. So the two halves are not
//! independent improvements to be summed: the descent creates an interaction
//! term, and centering is what lets it be read. Neither works alone.
//!
//! Read the absolute number honestly. 0.0597 on a 0-to-1 agreement scale is
//! still small, head domination at 33.5% is still far from the 9.1% floor, and
//! the sixty-fold increase in types from k=4 to k=64 buys 0.0012. What this
//! establishes is that relations are *present and recoverable*, not that they
//! are strong.
//!
//! # Standing waves: the bound, derived
//!
//! Bentov, *Stalking the Wild Pendulum* (1977), ch. 1: "Standing waves cannot
//! exist unless they divide their medium into an integral number of half waves.
//! A standing wave having a fractional wavelength cannot be sustained... The
//! dimensions of the plate are the factors that govern what is the size or
//! wavelength of the standing wave that can be sustained in the plate."
//!
//! If a relation is a standing wave across the channel index rather than an
//! arbitrary direction in channel space, the medium admits only integer modes,
//! and a 64-channel torus sustains exactly 64 of them — mode 64 aliases onto
//! mode 0. **The bound stops being a claim and becomes the channel count.**
//! `MAX_RELATION_TYPES = PHASE_CHANNELS` is then forced by geometry, and there
//! is no 65th type to look for.
//!
//! The testable half is sharper than the philosophical half. [`harmonic_rotation`]
//! fits *nothing*: no centroids, no k-means, no seed, identical on every corpus.
//! The clustered rows above fit 64 rotations of 64 channels each — 4,096 free
//! numbers — to these very pairs. Same pairs, same null, same scoring:
//!
//! ```text
//! manifold        offset            held-out   shuffled     gain  g/noise
//! bottom-up       raw                 0.2523     0.2519   0.0005     0.9x
//! bottom-up       head-centered       0.7178     0.7187  -0.0009      --
//! bottom-up       double-centered     0.7090     0.7024   0.0066     1.7x
//! after descent   raw                 0.2789     0.2603   0.0187    48.4x
//! after descent   head-centered       0.6672     0.6566   0.0105    29.3x
//! after descent   double-centered     0.5294     0.5366  -0.0072      --
//! ```
//!
//! **On raw offsets the fixed basis beats fitted k-means: 0.0187 against 0.0122,
//! at 48x its noise floor against 15x — with zero fitted parameters against
//! 4,096.** Relations in this manifold are closer to modes of the medium than to
//! arbitrary directions in it, and the discovery apparatus was partly solving a
//! problem the geometry had already answered.
//!
//! Two things not to overstate. The largest *absolute* effect is still fitted
//! k-means on double-centered offsets (0.0597); the harmonic basis wins on
//! significance and on parsimony, not on size. And the two corrections do not
//! compose — double-centering, which was the best estimator under clustering,
//! is worse than its own null under the harmonic basis (-0.0072), because
//! centering destroys the very phase ramp the modes are made of. They are
//! alternative readouts, not stackable improvements.
//!
//! What survives every arm: nothing works on the bottom-up manifold. Fixed basis,
//! fitted basis, centered, uncentered — all at or under their nulls. The descent
//! is the precondition for every relational result in this module.
//!
//! # Re-measured on fixed seeds
//!
//! Every table above predates the seeding bug in [`crate::phasor::mix64`]: FNV-1a
//! had not avalanched after three bytes, so `cow`, `cod` and `cot` started at the
//! same point on 63 of 64 channels and the whole lexicon — nulls included — was
//! pre-clustered by spelling. Re-run:
//!
//! ```text
//! manifold        offset            held-out   shuffled     gain  g/noise
//! bottom-up       raw                 0.2440     0.2436   0.0004     0.7x
//! bottom-up       head-centered       0.7131     0.7144  -0.0013       --
//! bottom-up       double-centered     0.7021     0.6942   0.0080     6.4x
//! after descent   raw                 0.2765     0.2573   0.0192    35.2x
//! after descent   head-centered       0.6667     0.6560   0.0107   138.6x
//! after descent   double-centered     0.5277     0.5415  -0.0138       --
//! ```
//!
//! The fixed harmonic basis holds: 0.0192 on the descended manifold at 35x its
//! noise, against 0.0187 before — unchanged within noise, and still with zero
//! fitted parameters against k-means' 4,096. The bottom-up manifold is still at
//! its null on every estimator.
//!
//! One reversal. Double-centering, which was the largest absolute effect under
//! fitted clustering (0.0597), is now *negative* under the harmonic basis on the
//! descended manifold (-0.0138) — it was -0.0072 before, so the direction was
//! already there and the fixed seeds sharpened it. Centering destroys the phase
//! ramp the modes are made of; the two corrections remain alternative readouts
//! rather than stackable improvements, and this is now unambiguous.
//!
//! The k-means grid at k=16 and 64 has not been re-run on fixed seeds — it is
//! killed for memory at k=64 on this machine — so every number in the two tables
//! above it should be read as provisional.
//!
//! Everything else in that book — subjective time, the model of the universe,
//! the kundalini appendix — is not testable here and was not used.
//!
//! That is a finding about the representation, not about the discovery
//! procedure: the planted-relation tests below pass, so the clustering finds
//! real shared offsets when there are any to find.

use crate::facet::Facet;
use crate::phasor::SpectralPhasor;
use crate::roles::RoleDiscovery;
use crate::config::PHASE_CHANNELS;
use crate::wave::c64;
use std::collections::HashMap;

/// The most relation types this can represent.
///
/// Chosen to match the channel count, so a type index and a channel index are
/// the same width — the bounded-type claim, made structural. Whether the *data*
/// needs this many is a separate question, and [`RelationTypes::saturation`] is
/// how it gets asked rather than assumed.
pub const MAX_RELATION_TYPES: usize = 64;

/// How the step from a head to a filler is turned into a vector.
///
/// The plain difference `theta(filler) - theta(head)` is what every relational
/// measurement in this project has used, and the type sweep showed what is wrong
/// with it: **71.3% of pairs share a cluster with another pair of the same head,
/// against 9.1% expected if the head were ignored.** `adversary->against` and
/// `adversary->another` land together because both differences contain
/// `-theta(adversary)`, not because they are the same relation.
///
/// That is a *main effect* of the head, and main effects have a standard
/// removal: centering. Writing the offset as
///
/// ```text
/// offset(h, f) = grand + head_effect(h) + filler_effect(f) + interaction(h, f)
/// ```
///
/// a relation — a step that means the same thing wherever it is taken — can only
/// live in the interaction term. The estimators below strip the other terms
/// away, one at a time, so the question "is there anything left once word
/// identity is removed" gets an answer instead of an assumption.
///
/// All of it is done in circular space: an offset is a unit complex number per
/// channel, a mean is a circular mean, and centering subtracts angles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OffsetEstimator {
    /// `theta(filler) - theta(head)`. What every earlier measurement used.
    Raw,
    /// Raw, minus the head's own mean offset across all its fillers.
    ///
    /// Removes the head main effect and nothing else. If relations are real,
    /// this should *help*: what remains is how this particular filler differs
    /// from the head's typical filler.
    HeadCentered,
    /// Raw, minus the head's mean and the filler's mean, plus the grand mean.
    ///
    /// The two-way residual — the interaction alone. This is the strictest
    /// version of the question, and the one whose failure would be decisive:
    /// if nothing survives here, the manifold places words with no relational
    /// interaction at all, and no readout can recover what was never encoded.
    DoubleCentered,
}

impl OffsetEstimator {
    pub fn label(self) -> &'static str {
        match self {
            OffsetEstimator::Raw => "raw",
            OffsetEstimator::HeadCentered => "head-centered",
            OffsetEstimator::DoubleCentered => "double-centered",
        }
    }

    pub const ALL: [OffsetEstimator; 3] = [
        OffsetEstimator::Raw,
        OffsetEstimator::HeadCentered,
        OffsetEstimator::DoubleCentered,
    ];
}

/// Offsets for a pair set, under one estimator.
///
/// Built once and clustered many times. Centering needs the whole set — a head's
/// mean is over every filler it takes — so the estimator cannot be applied one
/// pair at a time.
#[derive(Debug, Clone)]
pub struct OffsetTable {
    pairs: Vec<(String, String)>,
    offsets: Vec<Vec<f64>>,
}

impl OffsetTable {
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Computes every pair's offset under `est`.
    pub fn build(
        facet: &Facet,
        pairs: &[(String, String)],
        est: OffsetEstimator,
    ) -> Self {
        let mut kept: Vec<(String, String)> = Vec::new();
        let mut raw: Vec<Vec<f64>> = Vec::new();
        for p in pairs {
            if let Some(o) = RoleDiscovery::offset(facet, &p.0, &p.1) {
                kept.push(p.clone());
                raw.push(o);
            }
        }
        if matches!(est, OffsetEstimator::Raw) || kept.is_empty() {
            return Self { pairs: kept, offsets: raw };
        }

        // Circular means, accumulated as unit vectors. A mean of angles is the
        // argument of the sum of their unit vectors, not the mean of the angles.
        let mut head_acc: HashMap<&str, Vec<c64>> = HashMap::new();
        let mut filler_acc: HashMap<&str, Vec<c64>> = HashMap::new();
        let mut grand = vec![c64::new(0.0, 0.0); PHASE_CHANNELS];
        let zeros = || vec![c64::new(0.0, 0.0); PHASE_CHANNELS];

        for (p, o) in kept.iter().zip(raw.iter()) {
            let h = head_acc.entry(p.0.as_str()).or_insert_with(zeros);
            for k in 0..PHASE_CHANNELS {
                let z = c64::from_polar(1.0, o[k]);
                h[k] += z;
                grand[k] += z;
            }
            let f = filler_acc.entry(p.1.as_str()).or_insert_with(zeros);
            for k in 0..PHASE_CHANNELS {
                f[k] += c64::from_polar(1.0, o[k]);
            }
        }

        let arg_or_zero = |z: c64| match z.norm() > 1e-12 {
            true => z.arg(),
            false => 0.0,
        };

        let offsets: Vec<Vec<f64>> = kept
            .iter()
            .zip(raw.iter())
            .map(|(p, o)| {
                let h = &head_acc[p.0.as_str()];
                let f = &filler_acc[p.1.as_str()];
                (0..PHASE_CHANNELS)
                    .map(|k| {
                        let centered = match est {
                            OffsetEstimator::Raw => o[k],
                            OffsetEstimator::HeadCentered => o[k] - arg_or_zero(h[k]),
                            OffsetEstimator::DoubleCentered => {
                                o[k] - arg_or_zero(h[k]) - arg_or_zero(f[k])
                                    + arg_or_zero(grand[k])
                            }
                        };
                        centered.rem_euclid(crate::config::TWO_PI)
                    })
                    .collect()
            })
            .collect();

        Self { pairs: kept, offsets }
    }

    /// Offsets centered within a *context class* — Searle's C.
    ///
    /// Searle's constitutive rule is "X counts as Y **in C**", and this codebase
    /// has been dropping the third term: every relation measured here is X to Y
    /// with no context, and `roles::Role::Context` is declared and never once
    /// assigned. A context-free offset is not a partial version of the rule, it
    /// is a different claim — that the step from X to Y means the same thing
    /// everywhere — and head domination at 59.8% against a 9.1% floor is what
    /// that claim looks like when it fails.
    ///
    /// Here C enters as a frame that shifts everything inside it: the mean offset
    /// of the class is subtracted from each of its members, so what remains is
    /// what this pair does *that its context does not already do*. A relation
    /// that is only a property of the context vanishes; one that survives is a
    /// relation the context does not explain.
    ///
    /// `classes` is one class per pair, in the same order as `pairs`. Where it
    /// comes from is the caller's problem and the experiment's variable — a
    /// syntactic classifier, a clustering of the sentences, or a shuffle with
    /// matching class sizes, which is the control that decides whether C is doing
    /// anything at all.
    pub fn build_contextual(
        facet: &Facet,
        pairs: &[(String, String)],
        classes: &[u16],
    ) -> Self {
        let mut kept: Vec<(String, String)> = Vec::new();
        let mut raw: Vec<Vec<f64>> = Vec::new();
        let mut cls: Vec<u16> = Vec::new();
        for (i, p) in pairs.iter().enumerate() {
            if let Some(o) = RoleDiscovery::offset(facet, &p.0, &p.1) {
                kept.push(p.clone());
                raw.push(o);
                cls.push(*classes.get(i).unwrap_or(&0));
            }
        }
        if kept.is_empty() {
            return Self { pairs: kept, offsets: raw };
        }

        let mut acc: HashMap<u16, Vec<c64>> = HashMap::new();
        for (c, o) in cls.iter().zip(raw.iter()) {
            let e = acc.entry(*c).or_insert_with(|| vec![c64::new(0.0, 0.0); PHASE_CHANNELS]);
            for k in 0..PHASE_CHANNELS {
                e[k] += c64::from_polar(1.0, o[k]);
            }
        }

        let offsets: Vec<Vec<f64>> = raw
            .iter()
            .zip(cls.iter())
            .map(|(o, c)| {
                let m = &acc[c];
                (0..PHASE_CHANNELS)
                    .map(|k| {
                        let shift = match m[k].norm() > 1e-12 {
                            true => m[k].arg(),
                            false => 0.0,
                        };
                        (o[k] - shift).rem_euclid(crate::config::TWO_PI)
                    })
                    .collect()
            })
            .collect();

        Self { pairs: kept, offsets }
    }

    /// Held-out agreement of these offsets to the fixed harmonic basis.
    ///
    /// Nothing is fitted, so every pair is held out and the comparison across
    /// context sources is a comparison of the contexts alone.
    pub fn harmonic_agreement(&self) -> f64 {
        let basis = RelationTypes::harmonic();
        let (mut sum, mut n) = (0.0f64, 0usize);
        for i in 0..self.len() {
            if let Some((_, a)) = self.best_match(i, &basis.rotations) {
                if a.is_finite() {
                    sum += a;
                    n += 1;
                }
            }
        }
        match n {
            0 => 0.0,
            _ => sum / n as f64,
        }
    }

    /// How often two pairs sharing a head land on the same harmonic mode.
    ///
    /// The diagnostic the whole context question turns on: 59.8% against a 9.1%
    /// chance level means the offset is carrying the head rather than the step.
    pub fn head_domination(&self) -> f64 {
        let basis = RelationTypes::harmonic();
        let mut by_mode: HashMap<usize, HashMap<&str, usize>> = HashMap::new();
        let mut total = 0usize;
        for i in 0..self.len() {
            if let Some((m, _)) = self.best_match(i, &basis.rotations) {
                *by_mode
                    .entry(m)
                    .or_default()
                    .entry(self.pairs[i].0.as_str())
                    .or_insert(0) += 1;
                total += 1;
            }
        }
        let shared: usize = by_mode
            .values()
            .map(|heads| heads.values().filter(|c| **c > 1).sum::<usize>())
            .sum();
        shared as f64 / total.max(1) as f64
    }

    /// The `(pair, offset)` view the clustering takes.
    fn usable(&self) -> Vec<(&(String, String), Vec<f64>)> {
        self.pairs.iter().zip(self.offsets.iter()).map(|(p, o)| (p, o.clone())).collect()
    }

    /// Nearest centroid and its agreement, for a pair already in the table.
    fn best_match(&self, index: usize, rotations: &[Vec<f64>]) -> Option<(usize, f64)> {
        let off = self.offsets.get(index)?;
        rotations
            .iter()
            .enumerate()
            .map(|(i, r)| (i, RoleDiscovery::agreement(off, r)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    }
}

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

/// The rotation of standing-wave mode `m` on a medium of `channels` channels.
///
/// Bentov, *Stalking the Wild Pendulum*, ch. 1: "Standing waves cannot exist
/// unless they divide their medium into an integral number of half waves. A
/// standing wave having a fractional wavelength cannot be sustained... The
/// dimensions of the plate are the factors that govern what is the size or
/// wavelength of the standing wave that can be sustained in the plate."
///
/// That is a derivation of the bounded-type claim rather than an assertion of
/// it. If a relation is a standing wave across the channel index, the medium
/// admits only integer modes, and a 64-channel torus sustains exactly 64 of them
/// — mode 64 aliases back onto mode 0. **The bound is the channel count.** It
/// was never a number to be chosen; `MAX_RELATION_TYPES = PHASE_CHANNELS` is
/// forced by the geometry.
///
/// The experimental consequence is sharper than the philosophical one. A
/// harmonic basis is *fixed a priori*: it fits nothing, has no free parameters,
/// and is identical on every corpus and every seed. K-means fits 64 arbitrary
/// 64-dimensional rotations to the data — 4,096 fitted numbers. If the fixed
/// basis holds its own against that, relations in this manifold are modes of the
/// medium rather than arbitrary directions in it, and the entire discovery
/// apparatus was solving a problem the geometry had already answered.
pub fn harmonic_rotation(m: usize, channels: usize) -> Vec<f64> {
    let n = channels.max(1);
    (0..n)
        .map(|k| (crate::config::TWO_PI * (m % n) as f64 * k as f64 / n as f64)
            .rem_euclid(crate::config::TWO_PI))
        .collect()
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
        Self::discover_with(facet, pairs, k, rounds, OffsetEstimator::Raw)
    }

    /// [`RelationTypes::discover`] under a chosen offset estimator.
    pub fn discover_with(
        facet: &Facet,
        pairs: &[(String, String)],
        k: usize,
        rounds: usize,
        est: OffsetEstimator,
    ) -> Self {
        let k = k.min(MAX_RELATION_TYPES);
        let table = OffsetTable::build(facet, pairs, est);
        let found = RoleDiscovery::cluster(&table.usable(), k, rounds);
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

    /// The standing-wave modes of the medium, as relation types.
    ///
    /// Nothing is fitted. Mode `m` is [`harmonic_rotation`], and there are
    /// exactly [`PHASE_CHANNELS`] of them because a bounded medium sustains only
    /// integer harmonics. Compare against [`RelationTypes::discover`], which
    /// fits 64 free rotations of 64 channels each to the same pairs.
    pub fn harmonic() -> Self {
        let rotations: Vec<Vec<f64>> = (0..PHASE_CHANNELS)
            .map(|m| harmonic_rotation(m, PHASE_CHANNELS))
            .collect();
        let evidence = (0..PHASE_CHANNELS)
            .map(|i| TypeEvidence {
                relation: RelationType(i as u8),
                members: Vec::new(),
                coherence: 0.0,
            })
            .collect();
        Self { rotations, evidence }
    }

    /// Held-out agreement to the fixed harmonic basis, against the same null.
    ///
    /// There is no fit/held split here because there is nothing to fit — every
    /// pair is held out. The null is the same permuted-filler control, put
    /// through the identical basis, so the gain means what it means everywhere
    /// else in this module.
    pub fn harmonic_saturation(
        facet: &Facet,
        pairs: &[(String, String)],
        est: OffsetEstimator,
    ) -> SaturationPoint {
        let basis = Self::harmonic();
        let score = |set: &[(String, String)]| -> f64 {
            let table = OffsetTable::build(facet, set, est);
            let mut sum = 0.0;
            let mut n = 0usize;
            for i in 0..table.len() {
                if let Some((_, a)) = table.best_match(i, &basis.rotations) {
                    if a.is_finite() {
                        sum += a;
                        n += 1;
                    }
                }
            }
            match n {
                0 => 0.0,
                _ => sum / n as f64,
            }
        };
        SaturationPoint {
            k: PHASE_CHANNELS,
            held_out: score(pairs),
            shuffled: score(&permute_fillers(pairs, 0x9E37_79B9_7F4A_7C15)),
            shuffled_b: score(&permute_fillers(pairs, 0x1234_5678_9ABC_DEF1)),
        }
    }

    /// Which mode a pair sits closest to, and how closely.
    ///
    /// The mode index *is* the relation type — an integer, not a fitted vector.
    pub fn harmonic_mode(
        facet: &Facet,
        head: &str,
        filler: &str,
        est_table: &OffsetTable,
        index: usize,
    ) -> Option<(usize, f64)> {
        let _ = (facet, head, filler);
        let basis = Self::harmonic();
        est_table.best_match(index, &basis.rotations)
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
        Self::saturation_at_with(facet, pairs, k, rounds, OffsetEstimator::Raw)
    }

    /// [`RelationTypes::saturation_at`] under a chosen offset estimator.
    ///
    /// The estimator is applied to the null exactly as to the signal. That is
    /// what makes the comparison legitimate: centering mechanically reduces head
    /// domination, so a centered arm scored against an uncentered null would
    /// "improve" for free. Both arms are centered, or neither.
    pub fn saturation_at_with(
        facet: &Facet,
        pairs: &[(String, String)],
        k: usize,
        rounds: usize,
        est: OffsetEstimator,
    ) -> SaturationPoint {
        let (fit, held, null_fit, null_held) = Self::split(pairs);
        let (null_fit_b, null_held_b) = (
            permute_fillers(&fit, 0x1234_5678_9ABC_DEF1),
            permute_fillers(&held, 0x0FED_CBA9_8765_4321),
        );
        SaturationPoint {
            k,
            held_out: Self::held_out_agreement(facet, &fit, &held, k, rounds, est),
            shuffled: Self::held_out_agreement(facet, &null_fit, &null_held, k, rounds, est),
            shuffled_b: Self::held_out_agreement(facet, &null_fit_b, &null_held_b, k, rounds, est),
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
        est: OffsetEstimator,
    ) -> f64 {
        let fit_table = OffsetTable::build(facet, fit, est);
        let found = RoleDiscovery::cluster(&fit_table.usable(), k, rounds);
        if found.is_empty() {
            return 0.0;
        }
        let rotations: Vec<Vec<f64>> = found.into_iter().map(|d| d.rotation).collect();

        // The held-out half is centered on *its own* statistics, not the fit
        // half's. Centering a held-out pair with means computed from the training
        // pairs would leak, and centering it with means that include itself would
        // not be held out either — this is the honest middle: the estimator is a
        // property of the set it is applied to, and the two sets are disjoint.
        let held_table = OffsetTable::build(facet, held, est);
        let mut sum = 0.0;
        let mut n = 0usize;
        for i in 0..held_table.len() {
            if let Some((_, a)) = held_table.best_match(i, &rotations) {
                if a.is_finite() {
                    sum += a;
                    n += 1;
                }
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
    use crate::config::TWO_PI;

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

    /// Centering must remove the head main effect and keep the relation.
    ///
    /// Built so the two are separable: every pair's offset is a per-head
    /// constant (the main effect) plus one of two relation rotations (the
    /// interaction). Raw offsets are dominated by the head constant and cluster
    /// by head. Head-centered offsets have the constant gone, so they must
    /// cluster by relation instead — the thing the real data does not do.
    #[test]
    fn test_head_centering_removes_the_head_effect_and_keeps_the_relation() {
        const HEADS: usize = 24;
        let mut f = Facet::new();
        let mut pairs: Vec<(String, String)> = Vec::new();

        for i in 0..HEADS {
            let h = format!("h{i}");
            f.get_or_init(&h);
            // A large per-head constant: this is the main effect that buries the
            // relation in the raw difference.
            let bias = TWO_PI * i as f64 / HEADS as f64;
            for (suffix, rel) in [("_x", 0.35), ("_y", -0.35)] {
                let name = format!("{h}{suffix}");
                let base: Vec<f64> = (0..PHASE_CHANNELS).map(|k| f.lexicon[&h].theta(k)).collect();
                let p = f.get_or_init(&name);
                for k in 0..PHASE_CHANNELS {
                    p.set_theta(k, base[k] + bias + rel);
                }
                p.sync_phase();
                pairs.push((h.clone(), name));
            }
        }

        let cluster_of = |est: OffsetEstimator, i: usize, suffix: &str| -> usize {
            let types = RelationTypes::discover_with(&f, &pairs, 2, 20, est);
            let table = OffsetTable::build(&f, &pairs, est);
            let idx = pairs
                .iter()
                .position(|p| p.1 == format!("h{i}{suffix}"))
                .expect("pair present");
            table
                .best_match(idx, &types.rotations)
                .expect("a nearest type")
                .0
        };

        // Raw: two heads with different biases are in different clusters even
        // though they stand in the same relation.
        let raw_same_relation_far_heads =
            cluster_of(OffsetEstimator::Raw, 0, "_x") == cluster_of(OffsetEstimator::Raw, 12, "_x");
        // Head-centered: same relation, same cluster, whatever the head.
        let centered_same_relation = cluster_of(OffsetEstimator::HeadCentered, 0, "_x")
            == cluster_of(OffsetEstimator::HeadCentered, 12, "_x");
        let centered_different_relation = cluster_of(OffsetEstimator::HeadCentered, 0, "_x")
            != cluster_of(OffsetEstimator::HeadCentered, 0, "_y");

        assert!(
            !raw_same_relation_far_heads,
            "the fixture must actually bury the relation under the head effect, \
             or the test proves nothing"
        );
        assert!(
            centered_same_relation,
            "head-centering should put the same relation in one cluster across heads"
        );
        assert!(
            centered_different_relation,
            "head-centering should still separate the two relations"
        );
    }

    /// Double-centering leaves nothing when there is nothing but main effects.
    ///
    /// Offsets built purely from a head term and a filler term have no
    /// interaction, so the two-way residual is zero and any clustering of it is
    /// clustering noise. If the real data behaves like this, the manifold
    /// encodes no relations and no readout can recover them.
    #[test]
    fn test_double_centering_annihilates_a_pure_main_effect_table() {
        let mut f = Facet::new();
        let heads: Vec<String> = (0..12).map(|i| format!("h{i}")).collect();
        let fillers: Vec<String> = (0..12).map(|i| format!("f{i}")).collect();
        for w in heads.iter().chain(fillers.iter()) {
            f.get_or_init(w);
        }
        // theta(filler) - theta(head) is additive in the two words by
        // construction: the offset table has main effects and no interaction.
        let pairs: Vec<(String, String)> = heads
            .iter()
            .flat_map(|h| fillers.iter().map(move |fl| (h.clone(), fl.clone())))
            .collect();

        let table = OffsetTable::build(&f, &pairs, OffsetEstimator::DoubleCentered);
        let mut worst = 0.0f64;
        for o in &table.offsets {
            for a in o {
                // Residuals should sit at 0 (mod 2pi).
                let d = (a + std::f64::consts::PI).rem_euclid(TWO_PI) - std::f64::consts::PI;
                worst = worst.max(d.abs());
            }
        }
        assert!(
            worst < 1e-6,
            "a purely additive offset table should double-center to zero; \
             largest residual {worst:.6}"
        );
    }

    /// The medium admits exactly as many modes as it has channels.
    ///
    /// This is where the bound comes from. Mode 64 on a 64-channel torus is mode
    /// 0 — it aliases — so there is no 65th standing wave to have, and the cap
    /// is geometry rather than a choice.
    #[test]
    fn test_the_harmonic_basis_has_exactly_channel_count_distinct_modes() {
        let basis = RelationTypes::harmonic();
        assert_eq!(basis.len(), PHASE_CHANNELS);

        // Every mode is distinct from every other.
        for a in 0..PHASE_CHANNELS {
            for b in (a + 1)..PHASE_CHANNELS {
                let same = (0..PHASE_CHANNELS).all(|k| {
                    let d = (basis.rotations[a][k] - basis.rotations[b][k]).abs();
                    d < 1e-9 || (d - TWO_PI).abs() < 1e-9
                });
                assert!(!same, "modes {a} and {b} are the same rotation");
            }
        }

        // And the (n+1)-th aliases onto the first.
        let wrapped = harmonic_rotation(PHASE_CHANNELS, PHASE_CHANNELS);
        for k in 0..PHASE_CHANNELS {
            let d = (wrapped[k] - basis.rotations[0][k]).abs();
            assert!(d < 1e-9 || (d - TWO_PI).abs() < 1e-9, "mode n should alias to mode 0");
        }
    }

    /// A pair built on a mode must be recognised as that mode, with nothing
    /// fitted.
    #[test]
    fn test_a_pair_built_on_a_mode_is_read_back_as_that_mode() {
        let basis = RelationTypes::harmonic();
        let mut f = Facet::new();
        f.get_or_init("head");
        let base: Vec<f64> = (0..PHASE_CHANNELS).map(|k| f.lexicon["head"].theta(k)).collect();

        for m in [1usize, 5, 17, 40] {
            let rot = harmonic_rotation(m, PHASE_CHANNELS);
            let name = format!("filler{m}");
            let p = f.get_or_init(&name);
            for k in 0..PHASE_CHANNELS {
                p.set_theta(k, base[k] + rot[k]);
            }
            p.sync_phase();

            let pairs = vec![("head".to_string(), name.clone())];
            let table = OffsetTable::build(&f, &pairs, OffsetEstimator::Raw);
            let (best, agree) = table.best_match(0, &basis.rotations).expect("a nearest mode");
            assert_eq!(best, m, "mode {m} read back as {best}");
            assert!(agree > 0.99, "agreement with its own mode should be ~1, got {agree:.4}");
        }
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

