use crate::command::Context;
use crate::command::Parser;
use crate::envision::Envision;
use crate::eval::Evaluator;
use crate::trainer::MultiEpochResult;

/// Learn - trains the facet on a sentence (online learning mode).
///
/// Usage:
///   learn "some text to learn from"
///   learn multi "text" [epochs] [warmup]  - multi-epoch with convergence
///
/// If the user types text that doesn't match any command, the `default`
/// method is called instead, which trains, evaluates, and envisions.
pub struct Learn;

impl Learn {
    /// Trains the facet on the provided text.
    pub fn apply(&self, ctx: &mut Context) -> bool {
        if ctx.arg.is_empty() {
            println!("  Usage: learn \"some text to learn from\"");
            println!("         learn multi \"text\" [epochs] [warmup]");
            return true;
        }

        if ctx.arg.starts_with("multi ") {
            return self.multi(ctx);
        }

        let text = Parser::strip_quotes(ctx.arg);
        let updated = ctx.trainer.train_online(ctx.manifold, &text);
        println!(
            "  [trained] {} tokens, facet: {} words",
            updated,
            ctx.manifold.vocabulary_size(),
        );
        true
    }

    /// Multi-epoch training with warmup and convergence detection.
    fn multi(&self, ctx: &mut Context) -> bool {
        let rest = ctx.arg.strip_prefix("multi ").unwrap_or("");
        let parts: Vec<&str> = rest.split_whitespace().collect();
        if parts.is_empty() {
            println!("  Usage: learn multi \"text\" [epochs] [warmup]");
            return true;
        }

        let text = Parser::strip_quotes(parts[0]);
        let epochs: usize = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(10);
        let warmup: usize = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(3);

        let result: MultiEpochResult = ctx.trainer.train_multi_epoch(
            ctx.manifold, &text, epochs, warmup,
        );
        println!(
            "  [multi-train] {} - facet: {} words",
            result,
            ctx.manifold.vocabulary_size(),
        );
        true
    }

    /// Default handler for unrecognized input - treats it as text to learn.
    ///
    /// Trains on the input, evaluates it, and runs the envision phase
    /// to detect knowledge gaps.
    pub fn default(&self, ctx: &mut Context) -> bool {
        let updated = ctx.trainer.train_online(ctx.manifold, ctx.line);
        println!(
            "  [learned] {} tokens, facet: {} words",
            updated,
            ctx.manifold.vocabulary_size(),
        );

        let eval = Evaluator::new().eval(ctx.manifold, ctx.line);
        println!("{}", eval);

        if let Some(v) = Envision::new().detect_gaps(ctx.manifold, None, ctx.line) {
            println!("{}", v);
        }
        true
    }
}
