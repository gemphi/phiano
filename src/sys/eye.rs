//! The eye (docs/167): active vision as a sensor an agent watches.
//! phi-oml's thesis — movement generates data — lands here as one move:
//! the eye twitches, the flow field sorts the scene by parallax, and the
//! field is told as rows. Seeing is telling, as watching the kit is.

use super::{Agent, Row};
use phioml::Flow;

/// One twitch of the eye: `who` is told the scene under its own scope
/// `sight` (narrowed by the persona's `context` exactly as the kit is).
/// Returns how many rows the agent heard.
pub fn see(who: &mut dyn Agent, flow: &Flow, angle: f64) -> usize {
    let allowed = |s: &str| {
        let ctx = &who.persona().context;
        ctx.is_empty() || ctx.iter().any(|c| c == "sight" || c == "track" || s == format!("scope:{c}") || s.starts_with(&format!("scope:{c}/")))
    };
    let rows: Vec<Row> = flow.rows("sight", angle).into_iter().filter(|r| allowed("scope:sight") || allowed(&r.other)).collect();
    let n = rows.len();
    who.tell(&rows);
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sys::{Being, Persona};
    use phioml::Aoc;

    #[test]
    fn seeing_is_telling_and_asking_works_after() {
        let p = Persona::from_md("---\nname: Dancer\nseeds: step\nkinds: does, within, says\ncontext: sight\n---\n").unwrap();
        let mut b = Being::new(p);
        let mut f = Flow::new(3);
        f.see("hand", Aoc::new(0.4, 3.0));
        f.see("table", Aoc::new(1.1, 1.0));
        f.see("wall", Aoc::new(2.0, 0.2));
        let n = see(&mut b, &f, 0.25);
        assert_eq!(n, 9);
        assert_eq!(b.ask("hand", "does")[0].0, "near", "the eye's parallax became the agent's answer");
        assert!(b.told() > 0);
    }
}
