//! One channel per relation, and a magnitude that can carry something.
//!
//! `bin/dimensions` established two things about the shipped representation.
//!
//! **The channels cannot specialise.** Training is symmetric in them: seeds are
//! hashes, the Kuramoto step treats every channel alike, and `channel_kernel`
//! varies only decay and frequency. Nothing distinguishes channel 17 from channel
//! 42, so nothing can push channel 17 toward being *the gender channel*. Scoring
//! a pair by its best dimension came out at 43.9% against a 50% chance line,
//! because there are no dimensions to be best at.
//!
//! **The magnitude cannot matter.** Amplitude is one number per word, shared
//! across all 64 channels, so it factors out of a normalised inner product
//! exactly. Phase-only and full-complex scoring agreed to the decimal place in
//! every arm of `bin/constructed` — not approximately, identically. Half of every
//! phasor in this project is mathematically incapable of influencing a result.
//!
//! [`RelationalPhasor`] fixes both, and they are the same fix. A word carries a
//! phase *and a magnitude per channel*, and channel `r` belongs to relation `r`:
//! the angle says where the word sits under that relation, the magnitude says how
//! much evidence it has for standing in it at all. A word never observed under
//! relation `r` has magnitude near zero there and contributes nothing to
//! comparisons along it — which is what makes the complex number do work that the
//! angle alone cannot.
//!
//! # Where the relations come from
//!
//! Not from a list I wrote. Two observable, label-free sources, packed into the
//! 64 channels:
//!
//! * **Adjacency.** The relative offset between two words in a sentence, 1 to
//!   [`OFFSET_CHANNELS`]. "Next to" is a relation, and it is the one relation
//!   always available.
//! * **Markers.** The most frequent word standing *between* two others names the
//!   relation between them — `of`, `in`, `as`, `to`. This is the observation
//!   behind `roles::RELATIONAL_PREPOSITIONS`, except that the markers are taken
//!   by corpus frequency rather than chosen, so the language supplies the
//!   inventory.
//!
//! # Why this breaks the symmetry
//!
//! Training couples two words **on one channel only** — the channel of the
//! relation under which they were observed. A pair seen under `of` moves the
//! `of` channel and leaves the other 63 alone. After enough text, channel `r`
//! holds an arrangement of the vocabulary specific to relation `r`, which is
//! precisely what a symmetric update can never produce.
//!
//! # What it measured, and why it loses
//!
//! Built and run on WikiText-2 prose, 36,577 words, 56 marker channels plus 8
//! offset channels:
//!
//! ```text
//! similarity                                pair>random
//! shipped: mean of 64 channels                    61.5%
//! relational: evidence-weighted, all              50.1%
//! relational: best single dimension               38.8%
//! relational: mean of best 8                      41.6%
//! ```
//!
//! **The symmetry break works mechanically and loses on the benchmark.** Both
//! fixes do what they were built to do — the tests below prove an observation
//! moves one channel and that per-channel magnitude changes a similarity, neither
//! of which the shipped representation can do. And the result is at chance, ten
//! points behind the representation it was meant to improve.
//!
//! The reason is sparsity, and it is worth stating precisely because it is not a
//! bug. Two arbitrary words almost never co-occur under `of`. Of the ten relation
//! families, every one's best channel is an *offset* channel covering 100% of its
//! pairs, and the marker channels cover almost none — so the only dimensions on
//! which two arbitrary words can be compared are the positional ones, which carry
//! where a word sits rather than what it means. Only gender (+0.094) and agent
//! (+0.058) clear the -0.004 noise floor.
//!
//! **Relation-specific channels make comparison sparse, and sparse comparison
//! loses to dense averaging on a similarity task.** That is a real trade, not a
//! failure of the idea: this architecture puts each relation somewhere definite,
//! which is what *composition* needs — deliberately working within one relation —
//! and takes away the everything-at-once blend that *similarity* wants. The
//! benchmark here is a similarity benchmark, and it is measuring the cost of the
//! trade without measuring the benefit.
//!
//! Two failures found on the way, both caught by the measurement rather than by
//! reading the code. The first run scored 35.9%, *below* chance, because channel
//! traffic is wildly unequal — every adjacent pair feeds an offset channel while a
//! marker channel sees only pairs joined by that word — so the offset channels
//! outvoted everything by two orders of magnitude. [`RelationalFacet::balance`]
//! equalises the vote and recovered 14 points. The second was in the measurement:
//! averaging a channel's agreement over the whole family scores it zero wherever a
//! pair was never seen, so only universally-populated channels could ever come
//! first. Fixed, and the conclusion was unchanged — the offset channels really do
//! cover every pair.

use crate::config::{PHASE_CHANNELS, TWO_PI};
use crate::phasor::{fnv1a, mix64};
use crate::tokenizer::Tokenizer;
use std::collections::HashMap;

/// Channels reserved for relative offset. The rest go to markers.
pub const OFFSET_CHANNELS: usize = 8;

/// How far apart two words may be and still count as related.
pub const WINDOW: usize = 8;

/// Coupling strength per observation.
pub const RATE: f64 = 0.06;

/// Below this circular dispersion a channel has collapsed and is muted.
///
/// The same reasoning as `cognitive::grounding::DISPERSION_FLOOR`, applied per
/// channel because that is where this representation collapses.
pub const CHANNEL_DISPERSION_FLOOR: f64 = 0.35;

/// Below this vocabulary the collapse guard does not run.
pub const GUARD_MIN_VOCAB: usize = 200;

/// A word as 64 complex numbers: one per relation.
#[derive(Debug, Clone)]
pub struct RelationalPhasor {
    /// Angle per relation channel.
    pub theta: Vec<f64>,
    /// Evidence per relation channel. This is the magnitude, and unlike the
    /// shipped representation it varies across channels — which is the only way
    /// a magnitude can survive normalisation and affect a comparison.
    pub mag: Vec<f64>,
}

impl RelationalPhasor {
    /// A word's starting position: hash-seeded angles, zero evidence.
    ///
    /// Magnitudes start at zero rather than one. A word has no claim to any
    /// relation until it is seen in one, and starting every word with full
    /// magnitude everywhere would reproduce the flat representation this module
    /// exists to replace.
    pub fn seeded(word: &str) -> Self {
        let h = mix64(fnv1a(word));
        let theta = (0..PHASE_CHANNELS)
            .map(|k| {
                let hk = mix64(h ^ (k as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
                (hk >> 11) as f64 / (1u64 << 53) as f64 * TWO_PI
            })
            .collect();
        Self { theta, mag: vec![0.0; PHASE_CHANNELS] }
    }

    /// Normalised complex inner product, magnitude included.
    ///
    /// A channel where either word has no evidence contributes nothing, so two
    /// words are compared only on the relations they both participate in. That
    /// is the behaviour the shipped `resonance` cannot have: averaging 64
    /// cosines gives a channel the same vote whether the word was ever seen
    /// there or not.
    pub fn similarity(&self, other: &Self) -> f64 {
        let (mut dot, mut na, mut nb) = (0.0f64, 0.0f64, 0.0f64);
        for k in 0..PHASE_CHANNELS {
            let (a, b) = (self.mag[k], other.mag[k]);
            dot += a * b * (self.theta[k] - other.theta[k]).cos();
            na += a * a;
            nb += b * b;
        }
        match na > 1e-12 && nb > 1e-12 {
            true => (dot / (na.sqrt() * nb.sqrt())).clamp(-1.0, 1.0),
            false => 0.0,
        }
    }

    /// Agreement on one relation alone, weighted by the evidence for it.
    pub fn on(&self, other: &Self, k: usize) -> f64 {
        let w = (self.mag[k] * other.mag[k]).sqrt();
        match w > 1e-12 {
            true => (self.theta[k] - other.theta[k]).cos(),
            false => 0.0,
        }
    }

    /// Total evidence, for reporting how concentrated a word's relations are.
    pub fn evidence(&self) -> f64 {
        self.mag.iter().sum()
    }
}

/// Which channel each observable relation occupies.
///
/// Built from the corpus, capped at [`PHASE_CHANNELS`]: offsets first, then the
/// most frequent between-word markers until the channels run out. The bound is
/// the channel count, exactly as `relation_type::MAX_RELATION_TYPES` is.
#[derive(Debug, Clone)]
pub struct RelationVocab {
    marker_channel: HashMap<String, usize>,
}

impl RelationVocab {
    /// Counts what appears between word pairs and gives the commonest their own
    /// channels.
    pub fn learn(docs: &[Vec<String>]) -> Self {
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for toks in docs {
            for i in 0..toks.len() {
                for j in (i + 2)..(i + 1 + WINDOW).min(toks.len()) {
                    // Exactly one word between the pair names the relation.
                    if j == i + 2 {
                        *counts.entry(toks[i + 1].as_str()).or_insert(0) += 1;
                    }
                }
            }
        }
        let mut ranked: Vec<(&str, usize)> = counts.into_iter().collect();
        // Sorted by count, ties by word, so the assignment is reproducible.
        ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));

        let room = PHASE_CHANNELS - OFFSET_CHANNELS;
        let marker_channel = ranked
            .into_iter()
            .take(room)
            .enumerate()
            .map(|(i, (w, _))| (w.to_string(), OFFSET_CHANNELS + i))
            .collect();
        Self { marker_channel }
    }

    /// The channel for a pair at `i` and `j` in `toks`, if any.
    ///
    /// A marked relation takes precedence over a bare offset: `bread of wheat`
    /// is an `of` relation first and an offset-2 relation second, and the marker
    /// is the more specific claim.
    pub fn channel(&self, toks: &[String], i: usize, j: usize) -> Option<usize> {
        if j == i + 2 {
            if let Some(c) = self.marker_channel.get(&toks[i + 1]) {
                return Some(*c);
            }
        }
        let d = j - i;
        match d <= OFFSET_CHANNELS {
            true => Some(d - 1),
            false => None,
        }
    }

    pub fn markers(&self) -> usize {
        self.marker_channel.len()
    }

    /// The marker a channel stands for, for inspection.
    pub fn name(&self, k: usize) -> String {
        if k < OFFSET_CHANNELS {
            return format!("offset+{}", k + 1);
        }
        self.marker_channel
            .iter()
            .find(|(_, c)| **c == k)
            .map(|(w, _)| w.clone())
            .unwrap_or_else(|| format!("ch{k}"))
    }
}

/// The manifold: one relational phasor per word.
#[derive(Debug, Default)]
pub struct RelationalFacet {
    pub words: HashMap<String, RelationalPhasor>,
    pub counts: HashMap<String, u32>,
    /// Channels that survived the collapse guard.
    pub live: usize,
}

impl RelationalFacet {
    /// Trains by coupling each observed pair **on its relation's channel only**.
    ///
    /// This is the whole point. The shipped trainer moves all 64 channels on
    /// every observation, which is why no channel can come to mean anything. Here
    /// an observation is evidence about one relation, so it moves one channel and
    /// deposits its evidence there.
    pub fn train(docs: &[Vec<String>], vocab: &RelationVocab, epochs: usize) -> Self {
        let mut facet = Self::default();
        for toks in docs {
            for w in toks {
                facet.counts.entry(w.clone()).and_modify(|c| *c += 1).or_insert(1);
                facet
                    .words
                    .entry(w.clone())
                    .or_insert_with(|| RelationalPhasor::seeded(w));
            }
        }

        for _ in 0..epochs.max(1) {
            for toks in docs {
                for i in 0..toks.len() {
                    for j in (i + 1)..(i + 1 + WINDOW).min(toks.len()) {
                        if toks[i] == toks[j] {
                            continue;
                        }
                        let k = match vocab.channel(toks, i, j) {
                            Some(k) => k,
                            None => continue,
                        };
                        // Kuramoto on one channel: each word moves a little
                        // toward the other, and both gain evidence for this
                        // relation.
                        let (ta, tb) = (
                            facet.words[&toks[i]].theta[k],
                            facet.words[&toks[j]].theta[k],
                        );
                        let d = (tb - ta).sin();
                        if let Some(p) = facet.words.get_mut(&toks[i]) {
                            p.theta[k] = (ta + RATE * d).rem_euclid(TWO_PI);
                            p.mag[k] += 1.0;
                        }
                        if let Some(p) = facet.words.get_mut(&toks[j]) {
                            p.theta[k] = (tb - RATE * d).rem_euclid(TWO_PI);
                            p.mag[k] += 1.0;
                        }
                    }
                }
            }
        }

        // Evidence is a count; compress it so one very frequent relation does not
        // dominate every comparison a word takes part in.
        for p in facet.words.values_mut() {
            for m in p.mag.iter_mut() {
                *m = (1.0 + *m).ln();
            }
        }

        facet.balance();
        facet.silence_collapsed();
        facet
    }

    /// Equalises how loudly each channel votes.
    ///
    /// Traffic is wildly unequal: every adjacent pair feeds an offset channel,
    /// while a marker channel only sees pairs joined by that one word. Left raw,
    /// the offset channels carry two orders of magnitude more evidence and decide
    /// every comparison — the first run of `bin/relational` had all ten relation
    /// families peaking on offset channels and not one on a marker.
    ///
    /// Dividing each channel by its own mean evidence makes a channel's vote
    /// proportional to how much *this word* uses it relative to how much anyone
    /// does, rather than to how common the relation is.
    fn balance(&mut self) {
        let n = self.words.len().max(1) as f64;
        let mut mean = vec![0.0f64; PHASE_CHANNELS];
        for p in self.words.values() {
            for k in 0..PHASE_CHANNELS {
                mean[k] += p.mag[k];
            }
        }
        for m in mean.iter_mut() {
            *m = (*m / n).max(1e-9);
        }
        for p in self.words.values_mut() {
            for k in 0..PHASE_CHANNELS {
                p.mag[k] /= mean[k];
            }
        }
    }

    /// Mutes channels whose phases have collapsed onto one angle.
    ///
    /// Kuramoto coupling is attraction-dominated, and restricting it to a channel
    /// does not change that — it concentrates the failure. A channel every pair
    /// feeds pulls its whole vocabulary to a point, after which every word agrees
    /// with every other there and the channel is pure noise with a loud voice.
    /// That is what put the first run *below chance*: the collapsed offset
    /// channels agreed about everything, so the only discrimination left came
    /// from sparse marker channels holding near-random angles.
    ///
    /// The project already has this guard for the manifold as a whole
    /// (`cognitive::grounding::DISPERSION_FLOOR`); this applies it per channel,
    /// which is where the new representation can fail.
    fn silence_collapsed(&mut self) {
        // Dispersion over a handful of points is not a measurement, and muting a
        // channel on that basis destroys information rather than protecting it.
        // The same back-off `Wave::pool_floor` applies to retrieval.
        if self.words.len() < GUARD_MIN_VOCAB {
            self.live = PHASE_CHANNELS;
            return;
        }
        let n = self.words.len().max(1) as f64;
        let mut keep = vec![true; PHASE_CHANNELS];
        for k in 0..PHASE_CHANNELS {
            let (mut x, mut y) = (0.0f64, 0.0f64);
            for p in self.words.values() {
                if p.mag[k] > 0.0 {
                    x += p.theta[k].cos();
                    y += p.theta[k].sin();
                }
            }
            let dispersion = 1.0 - x.hypot(y) / n;
            keep[k] = dispersion >= CHANNEL_DISPERSION_FLOOR;
        }
        self.live = keep.iter().filter(|k| **k).count();
        for p in self.words.values_mut() {
            for k in 0..PHASE_CHANNELS {
                if !keep[k] {
                    p.mag[k] = 0.0;
                }
            }
        }
    }

    pub fn similarity(&self, a: &str, b: &str) -> f64 {
        match (self.words.get(a), self.words.get(b)) {
            (Some(x), Some(y)) => x.similarity(y),
            _ => 0.0,
        }
    }

    pub fn on(&self, a: &str, b: &str, k: usize) -> f64 {
        match (self.words.get(a), self.words.get(b)) {
            (Some(x), Some(y)) => x.on(y, k),
            _ => 0.0,
        }
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn docs(raw: &[&str]) -> Vec<Vec<String>> {
        raw.iter().map(|s| Tokenizer::tokenize(s)).collect()
    }

    /// The point of the module: an observation moves the channel of the relation
    /// it observed, and no other.
    ///
    /// The invariant is *evidence implies movement, and only that* — not "one
    /// channel moves". A single sentence exhibits several relations at once:
    /// `alpha of beta` puts `alpha` next to `of` (an offset relation) as well as
    /// two away from `beta` (the `of` relation), so two channels legitimately
    /// fire. The first version of this test asserted the narrower thing and
    /// failed on correct behaviour.
    #[test]
    fn test_training_moves_only_the_channels_it_has_evidence_for() {
        let d = docs(&["alpha of beta", "alpha of beta"]);
        let v = RelationVocab::learn(&d);
        let k = v.channel(&d[0], 0, 2).expect("a channel for the `of` relation");

        let before = RelationalPhasor::seeded("alpha");
        let f = RelationalFacet::train(&d, &v, 3);
        let after = &f.words["alpha"];

        assert!(after.mag[k] > 0.0, "the `of` relation must leave evidence");
        assert!(
            (after.theta[k] - before.theta[k]).abs() > 1e-9,
            "the relation's own channel must move"
        );

        let mut moved = 0usize;
        for c in 0..PHASE_CHANNELS {
            let shifted = (after.theta[c] - before.theta[c]).abs() > 1e-12;
            match after.mag[c] > 0.0 {
                // Evidence and movement go together, in both directions.
                true => {
                    moved += 1;
                    assert!(shifted, "channel {c} has evidence but never moved");
                }
                false => assert!(shifted == false, "channel {c} moved with no evidence"),
            }
        }
        assert!(
            moved < PHASE_CHANNELS / 4,
            "a three-word sentence exhibits a handful of relations, not {moved} of \
             {PHASE_CHANNELS} — that would be the symmetric update this replaces"
        );
    }

    /// Two words seen under the same relation agree on that channel, and two seen
    /// under different relations do not.
    #[test]
    fn test_the_channel_holds_its_own_relation() {
        let mut raw: Vec<String> = Vec::new();
        for _ in 0..40 {
            raw.push("king of realm".into());
            raw.push("queen of realm".into());
            raw.push("bread in oven".into());
        }
        let d: Vec<Vec<String>> = raw.iter().map(|s| Tokenizer::tokenize(s)).collect();
        let v = RelationVocab::learn(&d);
        let f = RelationalFacet::train(&d, &v, 6);

        let of = v.channel(&Tokenizer::tokenize("king of realm"), 0, 2).unwrap();
        let in_ = v.channel(&Tokenizer::tokenize("bread in oven"), 0, 2).unwrap();
        assert_ne!(of, in_, "different markers must take different channels");

        // `king` and `queen` are both pulled toward `realm` on the `of` channel,
        // so they converge there.
        assert!(
            f.on("king", "queen", of) > 0.9,
            "co-related words should agree on their shared relation: {}",
            f.on("king", "queen", of)
        );
        // And `king` has no evidence at all on the `in` channel.
        assert_eq!(f.words["king"].mag[in_], 0.0);
        assert_eq!(f.on("king", "bread", in_), 0.0);
    }

    /// Magnitude must be able to change a comparison — the failure that made the
    /// shipped representation's complex numbers inert.
    #[test]
    fn test_per_channel_magnitude_changes_the_similarity() {
        let mut a = RelationalPhasor::seeded("a");
        let mut b = RelationalPhasor::seeded("b");
        for k in 0..PHASE_CHANNELS {
            a.theta[k] = 0.0;
            b.theta[k] = match k == 3 {
                true => 0.0,
                false => std::f64::consts::PI,
            };
            a.mag[k] = 1.0;
            b.mag[k] = 1.0;
        }
        // Evidence spread evenly: the one agreeing channel is outvoted.
        let spread = a.similarity(&b);

        // The same angles, with the evidence concentrated where they agree.
        for k in 0..PHASE_CHANNELS {
            let w = match k == 3 {
                true => 8.0,
                false => 0.05,
            };
            a.mag[k] = w;
            b.mag[k] = w;
        }
        let concentrated = a.similarity(&b);

        assert!(
            concentrated > spread + 0.5,
            "magnitude must move the score: {spread:.4} -> {concentrated:.4}"
        );
        // A per-word scalar could not do this: it cancels in the normalisation.
        assert!(spread < 0.0 && concentrated > 0.0, "the sign should flip");
    }

    #[test]
    fn test_relation_vocab_is_bounded_and_reproducible() {
        let d = docs(&[
            "a of b", "c of d", "e in f", "g in h", "i on j", "k at l", "m of n",
        ]);
        let v1 = RelationVocab::learn(&d);
        let v2 = RelationVocab::learn(&d);
        assert!(v1.markers() + OFFSET_CHANNELS <= PHASE_CHANNELS);
        for k in 0..PHASE_CHANNELS {
            assert_eq!(v1.name(k), v2.name(k), "channel {k} assignment is not stable");
        }
        // Offsets own the low channels.
        assert_eq!(v1.name(0), "offset+1");
    }
}
