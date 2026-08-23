use crate::command::Context;
use crate::compose::Composition;

/// Compose — recursive composition via the Flower-Hayes cognitive process.
///
/// Generates 64 sector variations, evaluates them, keeps the better,
/// discards the worse, trains on the better (Kuramoto), and recurses.
///
/// Usage:
///   compose "prompt"                              — 3 rounds, no examples
///   compose "prompt" 5                            — 5 rounds
///   compose "prompt" 3 "example 1" "example 2"   — learn examples, 3 rounds
///   compose "prompt" 5 "ex1" "ex2" "ex3" "ex4"   — learn 4 examples, 5 rounds
///
/// The prompt determines where the river starts.
/// Examples are trained on before composing (teacher's specimens).
/// Rounds control how many recursive refinement cycles to run.
pub struct Compose;

impl Compose {
    /// Composes text via recursive sector tournament.
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: compose \"prompt\" [rounds] [\"example 1\"] [\"example 2\"] ...");
            println!("  Generates 64 sector variations, evaluates, trains on better, recurses.");
            println!("  Examples shape the facet before composing (learn by example).");
            return true;
        }

        let parts = Self::parse_args(ctx.arg);

        if parts.is_empty() {
            println!("  Usage: compose \"prompt\" [rounds] [\"example 1\"] ...");
            return true;
        }

        let prompt = parts[0].clone();
        let mut max_rounds = 3usize;
        let mut examples: Vec<String> = Vec::new();

        for part in &parts[1..] {
            if let Ok(n) = part.parse::<usize>() {
                max_rounds = n.max(1).min(20);
            } else {
                examples.push(part.clone());
            }
        }

        if examples.is_empty() {
            println!(
                "  [compose] \"{}\" — {} rounds, 64 sectors per round",
                prompt, max_rounds,
            );
        } else {
            println!(
                "  [compose] \"{}\" — {} rounds, learning {} examples first",
                prompt,
                max_rounds,
                examples.len(),
            );
        }

        let composition = Composition::compose(
            ctx.manifold,
            ctx.trainer,
            &prompt,
            &examples,
            max_rounds,
        );

        println!("{}", composition);
        true
    }

    /// Parses arguments: splits by whitespace but respects quoted strings.
    ///
    /// Examples:
    ///   "fireflies in the dark" 12 "like stars dancing"
    ///   → ["fireflies in the dark", "12", "like stars dancing"]
    fn parse_args(arg: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut quote_char = '"';

        for ch in arg.chars() {
            if !in_quotes && (ch == '"' || ch == '\'') {
                in_quotes = true;
                quote_char = ch;
                current.clear();
            } else if in_quotes && ch == quote_char {
                in_quotes = false;
                if !current.is_empty() {
                    parts.push(current.clone());
                }
                current.clear();
            } else if !in_quotes && ch.is_whitespace() {
                if !current.is_empty() {
                    parts.push(current.clone());
                }
                current.clear();
            } else {
                current.push(ch);
            }
        }

        if !current.is_empty() {
            parts.push(current);
        }

        parts
    }
}
