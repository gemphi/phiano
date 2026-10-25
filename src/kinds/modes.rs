#[allow(unused_imports)]
use super::*;

impl OffsetTable {

    /// The harmonic mode each pair falls on, as an index in `0..PHASE_CHANNELS`.
    ///
    /// Nothing is fitted — the modes are the medium's standing waves — so this is
    /// a *labelling* of the pairs that no training produced and no context saw.
    /// That is what makes it usable as the target of a prediction test: if a
    /// context class predicts these labels on held-out pairs, the context carries
    /// information about which relation holds.
    pub fn modes(&self) -> Vec<usize> {
        let basis = KindTable::harmonic();
        (0..self.len())
            .map(|i| self.best_match(i, &basis.rotations).map(|(m, _)| m).unwrap_or(0))
            .collect()
    }

    /// The pairs, in the order `modes` reports them.
    pub fn pairs(&self) -> &[(String, String)] {
        &self.pairs
    }

    /// How often two pairs sharing a head land on the same harmonic mode.
    ///
    /// The diagnostic the whole context question turns on: 59.8% against a 9.1%
    /// chance level means the offset is carrying the head rather than the step.
    pub fn head_domination(&self) -> f64 {
        let basis = KindTable::harmonic();
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
    pub(super) fn usable(&self) -> Vec<(&(String, String), Vec<f64>)> {
        self.pairs.iter().zip(self.offsets.iter()).map(|(p, o)| (p, o.clone())).collect()
    }

    /// Nearest centroid and its agreement, for a pair already in the table.
    pub(super) fn best_match(&self, index: usize, rotations: &[Vec<f64>]) -> Option<(usize, f64)> {
        let off = self.offsets.get(index)?;
        rotations
            .iter()
            .enumerate()
            .map(|(i, r)| (i, RoleDiscovery::agreement(off, r)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    }
}

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
