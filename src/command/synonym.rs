use crate::command::Context;
use crate::wave::Wave;

/// Synonym — finds the nearest resonant words for a given word.
///
/// Usage: `synonym <word> [count]`
///
/// Uses ray casting to find words with minimal energy delta to the target.
pub struct Synonym;

impl Synonym {
    /// Finds and prints the nearest resonant words for the given word.
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: synonym <word> [count]");
            return true;
        }

        let parts: Vec<&str> = ctx.arg.split_whitespace().collect();
        let word = parts[0].to_lowercase();
        let count: usize = parts
            .get(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        if !ctx.manifold.contains_word(&word) {
            println!("  I don't know '{}' yet. Try: define {}", word, word);
            return true;
        }

        let matches = Wave::ray_cast_word(ctx.manifold, &word, count);

        if matches.is_empty() {
            println!("  No resonant matches found.");
        } else {
            for (rank, (w, delta)) in matches.iter().enumerate() {
                println!("  Rank {}: {:<15} ΔC = {:.8}", rank + 1, w, delta);
            }
        }
        true
    }
}
