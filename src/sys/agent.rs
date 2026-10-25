use super::{Kit, Persona, Row, World};

/// The one interface. Everything an agent can do is a default over its
/// world, so a new kind of agent implements four things and gets the rest.
pub trait Agent {
    fn persona(&self) -> &Persona;
    fn world(&self) -> &World;
    fn world_mut(&mut self) -> &mut World;

    fn name(&self) -> &str {
        &self.persona().name
    }

    /// Rows in, counted at once: a quest's table, a paragraph, a kit.
    fn tell(&mut self, rows: &[Row]) {
        self.world_mut().tell(rows);
    }

    /// The kit's rows in — a body, a track, a device — the same move as
    /// `tell`, from a sensor rather than a sentence. A persona with a
    /// `context` watches only those scopes (`body/face`, `track`): the rows
    /// of the scopes themselves and of every thing within them.
    fn watch(&mut self, kit: &Kit) {
        let ctx = self.persona().context.clone();
        if ctx.is_empty() {
            return self.tell(&kit.rows);
        }
        let allowed = |s: &str| ctx.iter().any(|c| s == format!("scope:{c}") || s.starts_with(&format!("scope:{c}/")));
        let things: std::collections::HashSet<&str> =
            kit.rows.iter().filter(|r| r.kind == "within" && allowed(&r.other)).map(|r| r.about.as_str()).collect();
        let rows: Vec<Row> = kit.rows.iter().filter(|r| allowed(&r.about) || things.contains(r.about.as_str())).cloned().collect();
        self.tell(&rows);
    }

    /// What `word` calls for under `kind`, by evidence, heaviest first.
    /// Empty when the word or the kind was never told.
    fn ask(&self, word: &str, kind: &str) -> Vec<(String, f64)> {
        let w = self.world();
        let (Some(word), Some(kind)) = (w.lexicon.get(word), w.relations.get(kind)) else { return Vec::new() };
        // Read from the phasors — the difference under the kind — not from
        // the edge counts (docs/187: nothing learns from counts).
        let mut out: Vec<(String, f64)> = w.reads(word, kind).into_iter().map(|(o, wt)| (w.spelling(o).to_string(), wt)).collect();
        out.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        out
    }

    /// How much the agent has been told: its edges.
    fn told(&self) -> usize {
        self.world().edges.len()
    }

    /// Its standing in egg.
    fn kudos(&self) -> u32 {
        self.persona().kudos
    }
}
