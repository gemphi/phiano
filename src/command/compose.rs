use super::word::quoted;
use crate::command::Context;
use phient::generate::compose::Composition;

/// Compose - recursive composition via the Flower-Hayes cognitive process.
///
/// Generates 64 sector variations, evaluates them, keeps the better,
/// discards the worse, trains on the better (Kuramoto), and recurses.
///
/// Usage:
///   compose "prompt"                              - 3 rounds, no examples
///   compose "prompt" 5                            - 5 rounds
///   compose "prompt" 3 "example 1" "example 2"   - learn examples, 3 rounds
///   compose "prompt" 5 "ex1" "ex2" "ex3" "ex4"   - learn 4 examples, 5 rounds
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

        let parts = quoted(ctx.arg);

        if parts.is_empty() {
            println!("  Usage: compose \"prompt\" [rounds] [\"example 1\"] ...");
            return true;
        }

        let prompt = parts[0].clone();
        let mut max_rounds = 3usize;
        let mut examples: Vec<String> = Vec::new();

        for part in &parts[1..] {
            if let Ok(n) = part.parse::<usize>() {
                max_rounds = n.clamp(1, 20);
            } else {
                examples.push(part.clone());
            }
        }

        if examples.is_empty() {
            println!(
                "  [compose] \"{}\" - {} rounds, 64 sectors per round",
                prompt, max_rounds,
            );
        } else {
            println!(
                "  [compose] \"{}\" - {} rounds, learning {} examples first",
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
}
