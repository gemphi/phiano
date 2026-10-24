use super::{Agent, Persona, World};
use std::path::Path;

/// The one concrete agent: a persona and the world it was told.
pub struct Being {
    pub persona: Persona,
    pub world: World,
}

impl Being {
    /// A persona that has been told nothing yet.
    pub fn new(persona: Persona) -> Self {
        Self { persona, world: World::new() }
    }

    /// A persona told everything its quest folders hold:
    /// `<root>/<slug>/<seed>/relations.tsv`, as `persona_quest` writes them.
    /// With no folders yet it is `new`, and `told()` says so with 0.
    pub fn from_quests(root: &Path, persona: Persona) -> Self {
        let (rows, _) = phiplm::relate::read_quests(&root.join(persona.slug()));
        let mut being = Self::new(persona);
        being.tell(&rows);
        // A persona's `sources` are also told: a directory of `.b` files is
        // read by the BCPL reader (docs/168), so a language's persona is
        // told from its own code rather than quested from an oracle.
        for src in being.persona.sources.clone() {
            let dir = Path::new(&src);
            if dir.is_dir() {
                // The coarse grain: what each function declares, gets and
                // calls. The prose reader's occurrence rows are a corpus's
                // grain, not a persona's.
                being.tell(&phiplm::code::bcpl::read_coarse(dir));
            }
        }
        being
    }

    /// A persona read from its file and told from `root`.
    pub fn from_file(root: &Path, file: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
        Ok(Self::from_quests(root, Persona::from_md(&text)?))
    }
}

/// The persona files under `dir`: every `*.md` but `README.md`, sorted.
pub fn persona_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut v: Vec<_> = std::fs::read_dir(dir)
        .map(|rd| rd.filter_map(|e| e.ok().map(|e| e.path())).collect())
        .unwrap_or_default();
    v.retain(|p| p.extension().is_some_and(|x| x == "md") && p.file_name().is_some_and(|n| n != "README.md"));
    v.sort();
    v
}

/// The being called `who` under `dir`, told from `root`: an exact name or
/// slug first (`bertrand-russell`), else the one slug with `who` as a word
/// (`feynman`); two or more is an error naming them (`russell`).
pub fn find(root: &Path, dir: &Path, who: &str) -> Result<Being, String> {
    let who = who.trim().to_lowercase().replace(' ', "-");
    let mut files = persona_files(dir);
    for sub in ["stage", "code"] {
        files.extend(persona_files(&dir.join(sub)));
    }
    let all: Vec<Persona> = files
        .iter()
        .filter_map(|f| std::fs::read_to_string(f).ok().and_then(|t| Persona::from_md(&t).ok()))
        .collect();
    if let Some(p) = all.iter().find(|p| p.slug() == who || p.name.to_lowercase() == who.replace('-', " ")) {
        return Ok(Being::from_quests(root, p.clone()));
    }
    let hits: Vec<&Persona> = all.iter().filter(|p| p.slug().split('-').any(|w| w == who)).collect();
    match hits.as_slice() {
        [p] => Ok(Being::from_quests(root, (*p).clone())),
        [] => Err(format!("no persona called {who} under {}", dir.display())),
        many => Err(format!("{who} is {}: say which", many.iter().map(|p| p.slug()).collect::<Vec<_>>().join(" or "))),
    }
}

impl Agent for Being {
    fn persona(&self) -> &Persona {
        &self.persona
    }
    fn world(&self) -> &World {
        &self.world
    }
    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sys::Row;

    #[test]
    fn a_being_is_told_watches_and_answers() {
        let p = Persona::from_md("---\nname: Sailor\nseeds: boat\nkinds: has\nkudos: 8\n---\n").unwrap();
        let mut b = Being::new(p);
        assert_eq!((b.told(), b.kudos(), b.name()), (0, 8, "Sailor"));
        b.tell(&[Row::new("boat", "has", "sail"), Row::new("boat", "has", "rope"), Row::new("boat", "has", "sail")]);
        assert_eq!(b.told(), 2, "an edge told twice is one edge, counted twice");
        assert_eq!(b.ask("boat", "has")[0].0, "sail");
        assert!(b.ask("boat", "is-a").is_empty(), "a kind never told calls for nothing");
        let before = b.told();
        b.watch(&crate::sys::Kit::new());
        assert!(b.told() > before, "watching the kit tells the body");
        assert!(b.world.lexicon.get("scope:body").is_some());
    }

    #[test]
    fn a_context_narrows_what_is_watched() {
        let p = Persona::from_md("---\nname: Face\nseeds: smile\nkinds: has\ncontext: body/face\n---\n").unwrap();
        let mut b = Being::new(p);
        b.watch(&crate::sys::Kit::new());
        assert!(b.world.lexicon.get("scope:body/face").is_some());
        assert!(b.world.lexicon.get("scope:track").is_none(), "the track is outside its context");
        assert!(b.told() > 0);
    }
}
