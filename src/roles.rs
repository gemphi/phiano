//! Constitutive rules: *X counts as Y in context C*.
//!
//! Searle's constitutive rule is a **typed triple**, and money is his own
//! canonical example. *Money is a form of currency that enables transactions;
//! usually a paper or coin representation* is not a bag of eleven words. It is:
//!
//! ```text
//! genus(money, currency)        money counts as currency
//! function(money, transaction)  ... in the context of exchange
//! form(money, paper)            ... realised as paper
//! ```
//!
//! # Why this is the fix, and not just a nicer story
//!
//! The measurements found the same failure three times: rotating a word by its
//! **position** times the golden angle hurt inside definitions (−44 pp
//! pair/random), in the two-word language-model context (+11 perplexity) and in
//! sentence composition (0.171 → 0.101 MRR). Three tasks, three scales, one
//! direction.
//!
//! The diagnosis is not "order is bad". Positional rotation is a **nonce** role:
//! *currency* lands at angle 1·φ in one entry and 7·φ in another, so the same
//! concept is scattered to a different angle in every context it appears in.
//! Superposing scattered copies cancels rather than accumulates, which is
//! precisely what a shared representation must not do.
//!
//! A **role** rotation is the opposite. Every `genus` relation binds at the same
//! angle, everywhere, forever. *currency* as the genus of *money* and *animal*
//! as the genus of *cow* land at the same offset from their heads, so the
//! genus relation itself becomes a direction in the manifold — and directions
//! superpose constructively.
//!
//! That is the whole difference, and it is testable rather than rhetorical:
//! unbinding a composed word by a role should recover that role's filler, and it
//! cannot if the binding was positional.
//!
//! # Hierarchy
//!
//! The rule composes, which is what makes *a cow as a particular cow, a cow as
//! an animal, an animal as a species* one structure rather than three facts.
//! `genus` chains: `bessie →genus cow →genus animal →genus organism`. Following
//! the chain is repeated unbinding, so a hierarchy is a path through the
//! manifold rather than a table beside it.

use crate::config::{PHASE_CHANNELS, TWO_PI};
use crate::facet::Facet;
use crate::phasor::{fnv1a, SpectralPhasor};
use std::collections::HashMap;

/// A relation type. The rotation is derived from the name, so it is stable
/// across processes, corpora and runs without being stored anywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Role {
    /// *X counts as Y*: the kind X belongs to. `genus(money, currency)`.
    Genus,
    /// What X is for. `function(money, transaction)`.
    Function,
    /// What X is made of or realised as. `form(money, paper)`.
    Form,
    /// X is a particular of Y. `instance(bessie, cow)`.
    Instance,
    /// Where or when X holds — Searle's *context C*.
    Context,
    /// A defining property of X. `property(gold, heavy)`.
    Property,
}

impl Role {
    pub const ALL: [Role; 6] = [
        Role::Genus,
        Role::Function,
        Role::Form,
        Role::Instance,
        Role::Context,
        Role::Property,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Role::Genus => "genus",
            Role::Function => "function",
            Role::Form => "form",
            Role::Instance => "instance",
            Role::Context => "context",
            Role::Property => "property",
        }
    }

    /// The role's rotation, one angle per channel.
    ///
    /// Derived from the role's name by the same hash the lexicon uses, so it is
    /// the same rotation in every process without being persisted, and two
    /// roles are near-orthogonal by construction. **Shared** is the whole point:
    /// a positional rotation gives the same concept a different angle in every
    /// context, and superposing scattered copies cancels.
    pub fn rotation(self) -> [f64; PHASE_CHANNELS] {
        let mut out = [0.0f64; PHASE_CHANNELS];
        for (k, o) in out.iter_mut().enumerate() {
            let h = fnv1a(&format!("role:{}:{}", self.name(), k));
            *o = (h % 100_000) as f64 / 100_000.0 * TWO_PI;
        }
        out
    }
}

/// One constitutive rule: `head` counts as `filler` under `role`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub head: String,
    pub role: Role,
    pub filler: String,
}

/// The relations a vocabulary holds, and the composition they induce.
#[derive(Debug, Clone, Default)]
pub struct RuleSet {
    rules: Vec<Rule>,
    by_head: HashMap<String, Vec<usize>>,
}

impl RuleSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub fn add(&mut self, head: &str, role: Role, filler: &str) {
        if head == filler {
            return;
        }
        let i = self.rules.len();
        self.rules.push(Rule {
            head: head.to_string(),
            role,
            filler: filler.to_string(),
        });
        self.by_head.entry(head.to_string()).or_default().push(i);
    }

    pub fn rules_for(&self, head: &str) -> Vec<&Rule> {
        self.by_head
            .get(head)
            .map(|ix| ix.iter().map(|i| &self.rules[*i]).collect())
            .unwrap_or_default()
    }

    pub fn all(&self) -> &[Rule] {
        &self.rules
    }

    /// Counts per role, for reporting what an extractor actually found.
    pub fn role_counts(&self) -> Vec<(Role, usize)> {
        Role::ALL
            .iter()
            .map(|r| (*r, self.rules.iter().filter(|x| x.role == *r).count()))
            .collect()
    }
}

/// Binding and unbinding by role.
pub struct Roles;

impl Roles {
    /// Binds a filler into a role: rotate the filler's phases by the role's.
    ///
    /// Phase addition is circular convolution in this representation, so
    /// binding is invertible by subtraction — which is what makes the composed
    /// word queryable rather than merely a blend.
    pub fn bind(filler: &SpectralPhasor, role: Role) -> SpectralPhasor {
        let rot = role.rotation();
        let mut out = *filler;
        for k in 0..PHASE_CHANNELS {
            out.set_theta(k, filler.theta(k) + rot[k]);
        }
        out.sync_phase();
        out
    }

    /// Removes a role's rotation, recovering the filler's phases.
    pub fn unbind(bound: &SpectralPhasor, role: Role) -> SpectralPhasor {
        let rot = role.rotation();
        let mut out = *bound;
        for k in 0..PHASE_CHANNELS {
            out.set_theta(k, bound.theta(k) - rot[k]);
        }
        out.sync_phase();
        out
    }

    /// The relational identity of a word: the superposition of its bound
    /// relations.
    ///
    /// This is the operational form of "a word is defined by the relations it
    /// has". The head's own phases are *not* included — the identity is the
    /// relations, and mixing the head in would let a word be recovered from
    /// itself and make the recovery test vacuous.
    ///
    /// Returns `None` when the facet knows none of the fillers.
    pub fn identity(facet: &Facet, rules: &RuleSet, head: &str) -> Option<SpectralPhasor> {
        let mine = rules.rules_for(head);
        if mine.is_empty() {
            return None;
        }

        let mut acc = vec![(0.0f64, 0.0f64); PHASE_CHANNELS];
        let mut used = 0usize;
        for r in mine {
            let p = match facet.lexicon.get(&r.filler) {
                Some(p) => p,
                None => continue,
            };
            used += 1;
            let bound = Self::bind(p, r.role);
            for (k, a) in acc.iter_mut().enumerate() {
                let t = bound.theta(k);
                a.0 += t.cos();
                a.1 += t.sin();
            }
        }
        if used == 0 {
            return None;
        }

        let mut out = SpectralPhasor::seeded(head, 1.0, 1);
        for (k, (x, y)) in acc.iter().enumerate() {
            if x.hypot(*y) > 1e-12 {
                out.set_theta(k, y.atan2(*x));
            }
        }
        out.sync_phase();
        Some(out)
    }

    /// Asks a composed identity what fills a role, returning the nearest words.
    ///
    /// `identity ⊖ role` should land on the filler that was bound there. This is
    /// the query that positional binding cannot answer: with a nonce rotation
    /// there is no role to subtract.
    pub fn query(
        facet: &Facet,
        identity: &SpectralPhasor,
        role: Role,
        exclude: &[&str],
        k: usize,
    ) -> Vec<(String, f64)> {
        let probe = Self::unbind(identity, role);
        let mut scored: Vec<(&str, f64)> = facet
            .lexicon
            .iter()
            .filter(|(w, _)| !exclude.contains(&w.as_str()))
            .map(|(w, p)| (w.as_str(), probe.resonance(p)))
            .collect();
        scored.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(b.0))
        });
        scored
            .into_iter()
            .take(k)
            .map(|(w, s)| (w.to_string(), s))
            .collect()
    }

    /// Walks a role chain: *bessie → cow → animal → organism*.
    ///
    /// A hierarchy is repeated unbinding, so it is a path through the manifold
    /// rather than a table beside it. Stops at `depth`, at a cycle, or when the
    /// chain runs out.
    pub fn ascend(rules: &RuleSet, start: &str, role: Role, depth: usize) -> Vec<String> {
        let mut path = vec![start.to_string()];
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
        seen.insert(start.to_string());

        let mut cur = start.to_string();
        for _ in 0..depth {
            let next = rules
                .rules_for(&cur)
                .into_iter()
                .find(|r| r.role == role)
                .map(|r| r.filler.clone());
            match next {
                Some(n) if seen.insert(n.clone()) => {
                    path.push(n.clone());
                    cur = n;
                }
                _ => break,
            }
        }
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn facet_of(words: &[&str]) -> Facet {
        let mut f = Facet::new();
        for w in words {
            f.lexicon
                .insert((*w).to_string(), SpectralPhasor::seeded(w, 1.0, 1));
        }
        f
    }

    /// Binding must be exactly invertible, or a composed word cannot be queried.
    #[test]
    fn test_bind_unbind_is_exact() {
        let p = SpectralPhasor::seeded("currency", 1.0, 1);
        for role in Role::ALL {
            let back = Roles::unbind(&Roles::bind(&p, role), role);
            for k in 0..PHASE_CHANNELS {
                let d = (back.theta(k) - p.theta(k)).abs();
                // Phases are byte-quantised, so equality is to one quantum.
                assert!(
                    d < TWO_PI / 255.0 || (TWO_PI - d) < TWO_PI / 255.0,
                    "role {} channel {}: {} vs {}",
                    role.name(),
                    k,
                    back.theta(k),
                    p.theta(k)
                );
            }
        }
    }

    /// Roles must be distinguishable, or every relation is the same relation.
    #[test]
    fn test_roles_are_near_orthogonal() {
        let p = SpectralPhasor::seeded("currency", 1.0, 1);
        for a in Role::ALL {
            for b in Role::ALL {
                let r = Roles::bind(&p, a).resonance(&Roles::bind(&p, b));
                match a == b {
                    true => assert!(r > 0.99, "{} vs itself: {}", a.name(), r),
                    false => assert!(
                        r.abs() < 0.35,
                        "{} and {} are not distinguishable: {}",
                        a.name(),
                        b.name(),
                        r
                    ),
                }
            }
        }
    }

    /// **The claim.** A word composed from its relations must answer *what is
    /// its genus?* by unbinding — and must give a different answer for a
    /// different role.
    #[test]
    fn test_identity_answers_role_queries() {
        let f = facet_of(&[
            "money", "currency", "transaction", "paper", "cow", "animal", "milk", "hide",
            "tree", "plant", "shade", "wood",
        ]);
        let mut rules = RuleSet::new();
        rules.add("money", Role::Genus, "currency");
        rules.add("money", Role::Function, "transaction");
        rules.add("money", Role::Form, "paper");
        rules.add("cow", Role::Genus, "animal");
        rules.add("cow", Role::Function, "milk");
        rules.add("cow", Role::Form, "hide");
        rules.add("tree", Role::Genus, "plant");
        rules.add("tree", Role::Function, "shade");
        rules.add("tree", Role::Form, "wood");

        for (head, genus, function) in
            [("money", "currency", "transaction"), ("cow", "animal", "milk"), ("tree", "plant", "shade")]
        {
            let id = Roles::identity(&f, &rules, head).expect("composed");
            let g = Roles::query(&f, &id, Role::Genus, &[head], 3);
            let fun = Roles::query(&f, &id, Role::Function, &[head], 3);

            assert!(
                g.iter().any(|(w, _)| w == genus),
                "genus of {} should recover {}, got {:?}",
                head,
                genus,
                g
            );
            assert!(
                fun.iter().any(|(w, _)| w == function),
                "function of {} should recover {}, got {:?}",
                head,
                function,
                fun
            );
            assert_ne!(
                g[0].0, fun[0].0,
                "two roles of {} must not return the same filler",
                head
            );
        }
    }

    /// Hierarchy: *a particular cow, a cow, an animal*. The chain composes.
    #[test]
    fn test_genus_chain_ascends() {
        let mut rules = RuleSet::new();
        rules.add("bessie", Role::Instance, "cow");
        rules.add("bessie", Role::Genus, "cow");
        rules.add("cow", Role::Genus, "animal");
        rules.add("animal", Role::Genus, "organism");
        rules.add("organism", Role::Genus, "thing");

        let path = Roles::ascend(&rules, "bessie", Role::Genus, 5);
        assert_eq!(path, vec!["bessie", "cow", "animal", "organism", "thing"]);
    }

    /// A cycle must terminate rather than spin: dictionaries define circularly.
    #[test]
    fn test_cyclic_chain_terminates() {
        let mut rules = RuleSet::new();
        rules.add("tree", Role::Genus, "plant");
        rules.add("plant", Role::Genus, "tree");
        let path = Roles::ascend(&rules, "tree", Role::Genus, 10);
        assert_eq!(path, vec!["tree", "plant"]);
    }
}

/// A relation type **discovered from use**, not declared.
///
/// [`Role`] is six variants someone chose, and the extractor that fills them is
/// a list of regexes hardcoding the phrasings someone thought of. That is the
/// thing CLU's abstract data types were an argument against: a type is defined
/// by the operations its instances share, not by a label supplied from outside
/// and a hand-written list of the cases you happened to anticipate.
///
/// The manifold already has the invariant. If `genus` is a real relation, then
/// the per-channel offset `θ(filler) − θ(head)` is *approximately the same
/// vector* for `(dog, animal)` and for `(oak, tree)` — that shared offset is
/// what makes it one relation rather than two coincidences. So the types can be
/// recovered by clustering offsets, and nothing needs to be named in advance.
///
/// The test is not whether the clusters look tidy. It is whether they line up
/// with relation labels the clustering never saw.
#[derive(Debug, Clone)]
pub struct DiscoveredRole {
    /// The shared offset, one angle per channel — the role's rotation.
    pub rotation: Vec<f64>,
    /// Pairs assigned to this cluster, as `(head, filler)`.
    pub members: Vec<(String, String)>,
    /// Mean agreement between a member's offset and the centroid. Near 1.0 is a
    /// tight relation; near 0 is a bag of unrelated pairs sharing a bin.
    pub coherence: f64,
}

/// Recovers relation types by clustering phase offsets.
pub struct RoleDiscovery;

impl RoleDiscovery {
    /// The offset that takes `head` to `filler`, per channel.
    fn offset(facet: &Facet, head: &str, filler: &str) -> Option<Vec<f64>> {
        let (h, f) = (facet.lexicon.get(head)?, facet.lexicon.get(filler)?);
        Some((0..PHASE_CHANNELS).map(|k| f.theta(k) - h.theta(k)).collect())
    }

    /// Mean cosine between two offset vectors — 1.0 when the two pairs stand in
    /// the same relation, 0 when they are unrelated.
    fn agreement(a: &[f64], b: &[f64]) -> f64 {
        let mut s = 0.0;
        for (x, y) in a.iter().zip(b.iter()) {
            s += (x - y).cos();
        }
        s / a.len().max(1) as f64
    }

    /// Circular mean of a set of offset vectors.
    fn centroid(offsets: &[&Vec<f64>]) -> Vec<f64> {
        (0..PHASE_CHANNELS)
            .map(|k| {
                let (mut x, mut y) = (0.0f64, 0.0f64);
                for o in offsets {
                    x += o[k].cos();
                    y += o[k].sin();
                }
                match x.hypot(y) > 1e-12 {
                    true => y.atan2(x),
                    false => 0.0,
                }
            })
            .collect()
    }

    /// Clusters `pairs` into `k` discovered relations.
    ///
    /// K-means on the torus: assignment by offset agreement, update by circular
    /// mean. Seeds are chosen by farthest-point on the agreement metric rather
    /// than at random, so the result does not depend on a seed and two runs on
    /// the same data give the same types — which a discovery procedure has to,
    /// or the "types" are an artefact of initialisation.
    pub fn discover(
        facet: &Facet,
        pairs: &[(String, String)],
        k: usize,
        rounds: usize,
    ) -> Vec<DiscoveredRole> {
        let usable: Vec<(&(String, String), Vec<f64>)> = pairs
            .iter()
            .filter_map(|p| Self::offset(facet, &p.0, &p.1).map(|o| (p, o)))
            .collect();
        if usable.len() < k || k == 0 {
            return Vec::new();
        }

        // Farthest-point seeding: start at the first pair, then repeatedly take
        // the pair least like everything chosen so far.
        let mut seeds: Vec<usize> = vec![0];
        while seeds.len() < k {
            let next = (0..usable.len())
                .filter(|i| !seeds.contains(i))
                .min_by(|a, b| {
                    let sim = |i: &usize| {
                        seeds
                            .iter()
                            .map(|s| Self::agreement(&usable[*i].1, &usable[*s].1))
                            .fold(f64::NEG_INFINITY, f64::max)
                    };
                    sim(a).partial_cmp(&sim(b)).unwrap_or(std::cmp::Ordering::Equal)
                });
            match next {
                Some(n) => seeds.push(n),
                None => break,
            }
        }

        let mut centroids: Vec<Vec<f64>> = seeds.iter().map(|s| usable[*s].1.clone()).collect();
        let mut assign: Vec<usize> = vec![0; usable.len()];

        for _ in 0..rounds {
            let mut moved = false;
            for (i, (_, off)) in usable.iter().enumerate() {
                let best = centroids
                    .iter()
                    .enumerate()
                    .max_by(|a, b| {
                        Self::agreement(off, a.1)
                            .partial_cmp(&Self::agreement(off, b.1))
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|(c, _)| c)
                    .unwrap_or(0);
                if assign[i] != best {
                    assign[i] = best;
                    moved = true;
                }
            }
            for (c, cen) in centroids.iter_mut().enumerate() {
                let members: Vec<&Vec<f64>> = usable
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| assign[*i] == c)
                    .map(|(_, (_, o))| o)
                    .collect();
                if !members.is_empty() {
                    *cen = Self::centroid(&members);
                }
            }
            if !moved {
                break;
            }
        }

        centroids
            .into_iter()
            .enumerate()
            .map(|(c, rotation)| {
                let members: Vec<(String, String)> = usable
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| assign[*i] == c)
                    .map(|(_, (p, _))| ((*p).0.clone(), (*p).1.clone()))
                    .collect();
                let coherence = match members.is_empty() {
                    true => 0.0,
                    false => {
                        let sum: f64 = usable
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| assign[*i] == c)
                            .map(|(_, (_, o))| Self::agreement(o, &rotation))
                            .sum();
                        sum / members.len() as f64
                    }
                };
                DiscoveredRole { rotation, members, coherence }
            })
            .collect()
    }

    /// How well discovered clusters line up with labels they never saw.
    ///
    /// Purity: for each cluster, the share taken by its most common true label,
    /// weighted by cluster size. This is the only honest test of a discovery
    /// procedure — tidy-looking clusters prove nothing, and the labels must play
    /// no part in producing them.
    ///
    /// Chance purity is roughly the largest label's share of the data, so purity
    /// must be read against that and not against 1.0.
    pub fn purity(clusters: &[DiscoveredRole], labels: &HashMap<(String, String), String>) -> (f64, f64) {
        let mut correct = 0usize;
        let mut total = 0usize;
        let mut label_counts: HashMap<&str, usize> = HashMap::new();

        for c in clusters {
            let mut here: HashMap<&str, usize> = HashMap::new();
            for m in &c.members {
                if let Some(l) = labels.get(m) {
                    *here.entry(l.as_str()).or_insert(0) += 1;
                    *label_counts.entry(l.as_str()).or_insert(0) += 1;
                    total += 1;
                }
            }
            correct += here.values().copied().max().unwrap_or(0);
        }

        let chance = label_counts.values().copied().max().unwrap_or(0) as f64
            / total.max(1) as f64;
        (correct as f64 / total.max(1) as f64, chance)
    }
}

#[cfg(test)]
mod discovery_tests {
    use super::*;

    /// Discovery must recover relations it was never told about.
    ///
    /// Two synthetic relations are planted by construction — filler = head
    /// rotated by a fixed offset — and the clusterer is given the pairs with no
    /// labels. If it cannot separate two relations that are exactly separable,
    /// it will not find real ones.
    #[test]
    fn test_discovery_separates_planted_relations() {
        let mut f = Facet::new();
        let heads: Vec<String> = (0..24).map(|i| format!("h{}", i)).collect();
        let rot_a: Vec<f64> = (0..PHASE_CHANNELS).map(|k| (k as f64 * 0.37) % TWO_PI).collect();
        let rot_b: Vec<f64> = (0..PHASE_CHANNELS).map(|k| (k as f64 * 2.11 + 1.7) % TWO_PI).collect();

        let mut pairs = Vec::new();
        let mut labels: HashMap<(String, String), String> = HashMap::new();
        for (i, h) in heads.iter().enumerate() {
            let hp = SpectralPhasor::seeded(h, 1.0, 1);
            f.lexicon.insert(h.clone(), hp);

            let (rot, name) = match i % 2 {
                0 => (&rot_a, "alpha"),
                _ => (&rot_b, "beta"),
            };
            let fname = format!("f{}", i);
            let mut fp = hp;
            for k in 0..PHASE_CHANNELS {
                fp.set_theta(k, hp.theta(k) + rot[k]);
            }
            fp.sync_phase();
            f.lexicon.insert(fname.clone(), fp);

            pairs.push((h.clone(), fname.clone()));
            labels.insert((h.clone(), fname), name.to_string());
        }

        let clusters = RoleDiscovery::discover(&f, &pairs, 2, 20);
        assert_eq!(clusters.len(), 2);

        let (purity, chance) = RoleDiscovery::purity(&clusters, &labels);
        assert!(
            purity > 0.9,
            "two exactly-separable relations must be separated: purity {} vs chance {}",
            purity,
            chance
        );
        for c in &clusters {
            assert!(c.coherence > 0.9, "a recovered relation must be tight: {}", c.coherence);
        }
    }

    /// Discovery must be reproducible: seeded by farthest point, not by chance.
    #[test]
    fn test_discovery_is_deterministic() {
        let mut f = Facet::new();
        let mut pairs = Vec::new();
        for i in 0..20 {
            let (h, fl) = (format!("head{}", i), format!("fill{}", i));
            f.lexicon.insert(h.clone(), SpectralPhasor::seeded(&h, 1.0, 1));
            f.lexicon.insert(fl.clone(), SpectralPhasor::seeded(&fl, 1.0, 1));
            pairs.push((h, fl));
        }
        let a = RoleDiscovery::discover(&f, &pairs, 3, 10);
        let b = RoleDiscovery::discover(&f, &pairs, 3, 10);
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(b.iter()) {
            assert_eq!(x.members, y.members, "clustering must not move between runs");
        }
    }
}
