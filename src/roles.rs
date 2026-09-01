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
