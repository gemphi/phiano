//! The phiecs reader: the words told as rows into a `World` and READ FROM
//! THE PHASORS — the difference under the condition (docs/187), never a
//! count. One angle per word per kind is the running mean of everything
//! the word was ever followed by, and crowds as more is fed; so the
//! CONDITION picks the channel: a pair is told under the construction
//! family the words so far show ("does not", "is a") when one is known,
//! and under plain `next` otherwise — the DAG of docs/174 on the circle.
//! Every family is a channel: past the base 64 a kind is an EXTENSION
//! channel a word pays for only where it touches it (docs/188), so the
//! conditions are not rationed — [`FAMILIES`] is a safety rail, not a
//! budget.

use super::Concept;
use crate::sys::{Agent, Being, Persona};
use phiecs::Row;
use phient::study::Reader;

/// Family channels opened at most — a rail far above any corpus here.
pub const FAMILIES: usize = 60_000;

pub struct Ecs {
    being: Being,
    /// Knows the construction families of what was said (the same reader
    /// the concept arm uses; here it only names the channel).
    family: Concept,
    opened: Vec<String>,
}

impl Ecs {
    pub fn new() -> Self {
        let p = Persona::from_md("---\nname: Arena\nseeds: thing\nkinds: next\n---\n").expect("persona");
        Self { being: Being::new(p), family: Concept::new(), opened: Vec::new() }
    }

    /// The channel for what follows `said`: its family's, if that family
    /// has a channel or one can still be opened; else `next`.
    fn channel(&mut self, said: &[String], open: bool) -> String {
        let Some((a, b)) = self.family.family(said) else { return "next".into() };
        let name = format!("{a} {b}");
        if self.opened.contains(&name) {
            return name;
        }
        if open && self.opened.len() < FAMILIES {
            self.opened.push(name.clone());
            return name;
        }
        "next".into()
    }
}

impl Default for Ecs {
    fn default() -> Self {
        Self::new()
    }
}

impl Reader for Ecs {
    fn name(&self) -> &str {
        "phiecs (phasors: word before → family → next)"
    }

    fn guess(&mut self, said: &[String]) -> Option<String> {
        let last = said.last()?.clone();
        // Deepest first (docs/170): the word BEFORE the last is the tightest
        // condition — its own channel; then the family; then plain `next`.
        // Each level answers only when the one below it is silent. (Summing
        // the levels was measured in docs/189 and LOST — 20.7% against
        // 24.3% — because the crowded shallow channel's many small votes
        // swamp the deep channel's one confident answer; kept as read.)
        if said.len() >= 2 {
            if let Some((w, _)) = self.being.ask(&last, &format!("after {}", said[said.len() - 2])).first() {
                return Some(w.clone());
            }
        }
        let kind = self.channel(said, false);
        if let Some((w, _)) = self.being.ask(&last, &kind).first() {
            return Some(w.clone());
        }
        self.being.ask(&last, "next").first().map(|(w, _)| w.clone())
    }

    fn tell(&mut self, words: &[String]) {
        let mut rows: Vec<Row> = Vec::with_capacity(words.len() * 3);
        for j in 1..words.len() {
            let kind = self.channel(&words[..j], true);
            rows.push(Row::new(&words[j - 1], "next", &words[j]));
            if kind != "next" {
                rows.push(Row::new(&words[j - 1], &kind, &words[j]));
            }
            if j >= 2 {
                rows.push(Row::new(&words[j - 1], &format!("after {}", words[j - 2]), &words[j]));
            }
        }
        self.being.tell(&rows);
        self.family.tell(words);
    }
}
