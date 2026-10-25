//! The student of code, on the base (docs/124). It learns an answer the
//! moment it is told: the answer's relations by the grammar
//! (`phiano::code`), counted (`phiano::relate::Tally`), and the answer
//! itself kept whole, as the builders' game student keeps a told order. It
//! writes by one mechanism, D4 movement over those counts:
//!
//! 1. *which answer to say again* — the told functions are a kind
//!    (`said-again`); the new name's words and the signature's types are the
//!    conditions, and they move the choice to the functions told under the
//!    same conditions (a type sibling shares every word; a name sibling most);
//! 2. *what to exchange in it* — a token of the said-again body is
//!    exchanged for another of its kind (an operator, a call) when the
//!    proportion holds in the counts: the source's own words rank the old
//!    token above the new one and the new name's words rank the new above the
//!    old (`above : > :: below : <`); types by their place in the signature;
//!    the name by the name;
//! 3. *worse* — a body that does not compile is not said.
//!
//! In Phient's world, a word (w) is nothing but what it relates to, under what
//! conditions (c): `below` is the exchange of `>` for `<` under `count`.

use super::rows::{given, join_ops, rows_of};
use phiano::relate::{Tally, ScopeTree, SAYS};
use phiecs::quest::Row;

pub const GIVEN: [&str; 4] = ["has-word", "takes", "returns", "within"];
pub(super) const EXCHANGED: [&str; 3] = ["uses-op", "calls", "has-literal"];
pub const TAU: f64 = 0.5;
/// A word is known once this many different functions have said it.
pub const KNOWN: usize = 3;

pub struct Told {
    pub name: String,
    pub key: String,
    pub body: Vec<String>,
    pub takes: Vec<String>,
    pub returns: Option<String>,
    pub words: Vec<String>,
}

#[derive(Default)]
pub struct Student {
    pub counts: Tally,
    pub told: Vec<Told>,
    /// The scope tree the answers lie in (docs/126), built from `within`.
    pub tree: ScopeTree,
    /// Read under the scopes a cast finds (false: flat, docs/124).
    pub levels: bool,
}

impl Student {
    pub fn new(levels: bool) -> Self {
        Student { counts: Tally::new(&GIVEN), told: Vec::new(), tree: ScopeTree::default(), levels }
    }

    /// One answer, told once: its tokens (signature up to `{`, then body
    /// with any helper after it), its rows, and its place: `path` relative
    /// to the answers' root (`family/task.rs`), each component a scope the
    /// function is `within`, and what it says (`SAYS`) for the tree's cast.
    pub fn learn(&mut self, text: &str, toks: &[String], path: &str) -> bool {
        let Some((mut rows, name)) = rows_of(text) else { return false };
        let (words, takes, returns) = given(&rows, &name);
        // Each answer is its own value under `said-again` (four `sum`s, one
        // per type, are four answers).
        let key = format!("{name}#{}", self.told.len());
        rows.push(Row { about: name.clone(), kind: "said-again".into(), other: key.clone(), weight: 1.0 });
        let scopes = phiano::relate::scopes(path, 1);
        for s in &scopes {
            rows.push(Row { about: name.clone(), kind: "within".into(), other: s.clone(), weight: 1.0 });
        }
        for w in scopes.windows(2) {
            rows.push(Row { about: w[1].clone(), kind: "within".into(), other: w[0].clone(), weight: 1.0 });
            self.tree.add(&w[0], &w[1]);
        }
        if scopes.len() == 1 {
            self.tree.add(&scopes[0], &key);
        }
        for c in words.iter().chain(&takes).chain(&returns) {
            rows.push(Row { about: name.clone(), kind: SAYS.into(), other: c.clone(), weight: 1.0 });
        }
        self.counts.learn(&rows);
        let a = super::analog::answer(&join_ops(toks));
        self.told.push(Told { name, key, body: a.body, takes, returns, words });
        true
    }

    /// The scopes `conds` are drawn into (the best path of a cast), and
    /// how much they move each told answer: `ln read_levels − ln prior`
    /// under `said-again`. Nothing when reading flat.
    pub fn drawn(&self, conds: &[String], values: &[String]) -> (Vec<String>, Vec<f64>) {
        if !self.levels {
            return (Vec::new(), vec![0.0; values.len()]);
        }
        let Some(found) = self.tree.cast(&self.counts, conds, f64::MIN, 2).into_iter().next() else { return (Vec::new(), vec![0.0; values.len()]) };
        let path = found.path;
        let levels: Vec<Vec<String>> = path.iter().map(|s| vec![s.clone()]).collect();
        let (s, _) = self.counts.read_levels(&levels, "said-again", values, TAU);
        let prior = self.counts.prior("said-again", values);
        (path, s.iter().zip(&prior).map(|(x, p)| x - p.ln()).collect())
    }

    /// The words of `name` the student does not know: told in fewer than
    /// `KNOWN` different functions, so their counts cannot tell them from the
    /// functions they came with (`below` heard only in `count_below` is heard
    /// as `count`). Each is to be asked for (`code_quest --uses`), with the
    /// name as the example.
    pub fn unknown(&self, name: &str) -> Vec<(String, String)> {
        name.split('_')
            .filter(|w| w.len() > 1)
            .filter(|w| self.told.iter().filter(|t| t.words.iter().any(|x| x == w)).map(|t| &t.name).collect::<std::collections::HashSet<_>>().len() < KNOWN)
            .map(|w| (w.to_string(), name.to_string()))
            .collect()
    }

    /// The told functions ranked for saying again under `conds` (the new
    /// name's words and types): each condition the function was told under
    /// moves the choice toward it by its count; a condition the function has
    /// that the new name lacks stands in proportion with one the new name
    /// has that the function lacks (`above` for `below`, `i32` for `f64`:
    /// the exchange is the writing), and only the surplus moves it away.
    pub(super) fn said_again(&self, conds: &[String]) -> Vec<(String, f64)> {
        let values = self.counts.values("said-again");
        let prior: Vec<f64> = self.counts.prior("said-again", &values).iter().map(|p| p.ln()).collect();
        let moves = |c: &String| -> Vec<f64> { self.counts.read(std::slice::from_ref(c), "said-again", &values, TAU).0.iter().zip(&prior).map(|(s, p)| s - p).collect() };
        // The scopes the conditions are drawn into move every answer first.
        let (_, mut s) = self.drawn(conds, &values);
        for (i, f) in values.iter().enumerate() {
            let Some(t) = self.told.iter().find(|t| t.key == *f) else { continue };
            let theirs: Vec<String> = t.words.iter().chain(&t.takes).chain(&t.returns).cloned().collect();
            let missing = conds.iter().filter(|c| !theirs.contains(c)).count();
            let mut extra: Vec<f64> = theirs.iter().filter(|c| !conds.contains(c)).map(|c| moves(c)[i].max(0.0)).collect();
            extra.sort_by(|a, b| b.total_cmp(a));
            s[i] = conds.iter().filter(|c| theirs.contains(c)).map(|c| moves(c)[i]).sum::<f64>() - extra.iter().skip(missing).sum::<f64>();
        }
        let mut out: Vec<(String, f64)> = values.into_iter().zip(s).collect();
        out.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        out
    }
}
