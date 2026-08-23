/// ImpersonationResult — the output of an impersonation cycle.

use std::fmt;

pub struct ImpersonationResult {
    /// The persona name.
    pub persona_name: String,
    /// The prompt that was given.
    pub prompt: String,
    /// The composed text in the persona's style.
    pub text: String,
    /// The sector that won.
    pub winning_sector: u16,
    /// The color of the winning sector.
    pub winning_color: String,
    /// Quality score of the composition.
    pub quality_score: f64,
    /// How well it matches the persona's fingerprint (0.0-1.0).
    pub persona_fit: f64,
    /// Number of rounds completed.
    pub rounds: usize,
}

impl fmt::Display for ImpersonationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  ── impersonation: {} ──", self.persona_name)?;
        writeln!(f, "  prompt: \"{}\"", self.prompt)?;
        writeln!(
            f,
            "  sector {} ({}) | quality {:.4} | persona fit {:.4} | {} rounds",
            self.winning_sector, self.winning_color, self.quality_score, self.persona_fit, self.rounds,
        )?;
        writeln!(f)?;
        for line in self.text.lines() {
            writeln!(f, "    {}", line)?;
        }
        Ok(())
    }
}
