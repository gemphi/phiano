//! The writers' scoreboard: one row per writer — prompts, what parsed,
//! what compiled, tokens written, agreement with Phi-4's body, and what
//! works by the family's asserts.

/// prompts, parses, compiles, tokens, agreement, with an answer, works, checked.
#[derive(Default, Clone, Copy)]
pub struct Tally([f64; 8]);

impl Tally {
    /// One prompt written: `said` is the agreement with Phi-4's body when
    /// there was one; `works` is `None` when the family has no asserts.
    pub fn add(&mut self, parses: bool, builds: bool, body_len: usize, works: Option<bool>, said: Option<f64>) {
        let t = &mut self.0;
        t[0] += 1.0;
        t[1] += parses as usize as f64;
        t[2] += builds as usize as f64;
        t[3] += body_len as f64;
        if let Some(a) = said {
            t[4] += a;
            t[5] += 1.0;
        }
        t[6] += (works == Some(true)) as usize as f64;
        t[7] += works.is_some() as usize as f64;
    }

    pub fn row(&self, name: &str) -> String {
        let [n, ok, builds, len, ag, with, good, checked] = self.0;
        let agreement = if with > 0.0 { format!("{:.1}%", 100.0 * ag / with) } else { "–".into() };
        format!("| {name} | {ok} of {n} ({:.0}%) | {builds} ({:.0}%) | {good} of {checked} ({:.0}%) | {:.1} | {agreement} |\n", 100.0 * ok / n.max(1.0), 100.0 * builds / n.max(1.0), 100.0 * good / checked.max(1.0), len / n.max(1.0))
    }
}

/// Share of Phi-4's body reproduced from its start, token for token.
pub fn agree(out: &[String], phi4: &[String]) -> f64 {
    let n = out.iter().zip(phi4).take_while(|(a, b)| a == b).count();
    n as f64 / phi4.len().max(1) as f64
}
