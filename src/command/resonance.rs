use crate::command::Context;
use crate::command::Parser;
use crate::wave::Wave;

/// Resonance - finds words that resonate with a sentence wave.
///
/// Usage: `resonance "some text" [count]`
///
/// Computes the text wave, then ray casts to find words with minimal
/// energy delta to that wave.
pub struct Resonance;

impl Resonance {
    /// Finds and prints words resonating with the given text.
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: resonance \"some text\" [count]");
            return true;
        }

        let parts: Vec<&str> = ctx.arg.split_whitespace().collect();
        let text = Parser::strip_quotes(parts[0]);
        let count: usize = parts
            .get(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        let wave = Wave::text(ctx.manifold, &text);
        let matches = Wave::ray_cast(ctx.manifold, wave, count);

        if matches.is_empty() {
            println!("  No resonant words found.");
        } else {
            for (rank, (w, delta)) in matches.iter().enumerate() {
                println!("  Rank {}: {:<15} ΔC = {:.8}", rank + 1, w, delta);
            }
        }
        true
    }
}
