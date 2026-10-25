//! The AI race (docs/160): teams of beings, scored on what their worlds
//! have been told — edges, words, kinds — plus their standing in egg.
//!
//! The score is honest in one way that matters: a team whose personas have
//! never been quested has been told nothing, and the race says so rather
//! than inventing a lead. Kudos is a handicap declared in the persona
//! files, printed in its own column, never folded into `told`.
//! Engagement is rows: a team you work with (the phiace release, say) is
//! told more, and that is the only way to pull ahead.

use super::{Agent, Being};
use std::collections::BTreeMap;

/// One team's standing.
#[derive(Debug, Clone, PartialEq)]
pub struct Standing {
    pub team: String,
    pub members: Vec<String>,
    /// Graph across the team's worlds.
    pub told: usize,
    /// Words across the team's worlds.
    pub words: usize,
    /// Relation kinds across the team's worlds.
    pub kinds: usize,
    /// Standing in egg, summed over the members.
    pub kudos: u32,
}

impl Standing {
    /// What the race ranks by: told, then kudos — a team told nothing
    /// cannot win on kudos alone.
    pub fn score(&self) -> (usize, u32) {
        (self.told, self.kudos)
    }
}

/// The race: every being, grouped by its persona's team.
#[derive(Default)]
pub struct Race {
    pub beings: Vec<Being>,
}

impl Race {
    pub fn new(beings: Vec<Being>) -> Self {
        Self { beings }
    }

    /// Tell every member of `team` the same rows: engagement.
    pub fn engage(&mut self, team: &str, rows: &[super::Row]) -> usize {
        let team = team.to_lowercase();
        let mut n = 0;
        for b in self.beings.iter_mut().filter(|b| b.persona.team == team) {
            b.tell(rows);
            n += 1;
        }
        n
    }

    /// Every team's standing, best first. A persona with no team is a
    /// team of one, under its own slug.
    pub fn standings(&self) -> Vec<Standing> {
        let mut by: BTreeMap<String, Standing> = BTreeMap::new();
        for b in &self.beings {
            let team = match b.persona.team.is_empty() {
                true => b.persona.slug(),
                false => b.persona.team.clone(),
            };
            let s = by.entry(team.clone()).or_insert_with(|| Standing { team, members: Vec::new(), told: 0, words: 0, kinds: 0, kudos: 0 });
            s.members.push(b.name().to_string());
            s.told += b.told();
            s.words += b.world.lexicon.len();
            s.kinds += b.world.relations.len();
            s.kudos += b.kudos();
        }
        let mut v: Vec<Standing> = by.into_values().collect();
        v.sort_by(|a, b| b.score().cmp(&a.score()).then_with(|| a.team.cmp(&b.team)));
        v
    }

    /// The winner, or `None` when no team has been told anything — then
    /// there is no race yet, only kudos.
    pub fn winner(&self) -> Option<Standing> {
        self.standings().into_iter().next().filter(|s| s.told > 0)
    }

    /// The standings as a markdown table.
    pub fn table(&self) -> String {
        let mut out = String::from("| team | members | told (edges) | per member | words | kinds | kudos (egg) |\n|---|---|---|---|---|---|---|\n");
        for s in self.standings() {
            let per = s.told as f64 / s.members.len().max(1) as f64;
            out.push_str(&format!("| {} | {} | {} | {per:.0} | {} | {} | {} |\n", s.team, s.members.join(", "), s.told, s.words, s.kinds, s.kudos));
        }
        match self.winner() {
            Some(w) => out.push_str(&format!("\n**{}** leads: told the most. Kudos breaks ties only.\n", w.team)),
            None => out.push_str("\nNo team has been told anything yet: there is no race, only kudos. Run `persona_quest` first.\n"),
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sys::{Persona, Row};

    fn p(name: &str, team: &str, kudos: u32) -> Persona {
        Persona::from_md(&format!("---\nname: {name}\nseeds: x\nkinds: has\nteam: {team}\nkudos: {kudos}\n---\n")).unwrap()
    }

    #[test]
    fn kudos_alone_is_no_race_and_telling_is() {
        let mut race = Race::new(vec![Being::new(p("A", "meta", 100)), Being::new(p("B", "anthropic", 1))]);
        assert_eq!(race.winner(), None);
        assert_eq!(race.standings()[0].team, "meta", "kudos orders the standings while nothing is told");
        assert_eq!(race.engage("anthropic", &[Row::new("claude", "has", "constitution")]), 1);
        let w = race.winner().unwrap();
        assert_eq!((w.team.as_str(), w.told, w.kudos), ("anthropic", 1, 1));
        assert!(race.table().contains("**anthropic** leads"));
    }
}
